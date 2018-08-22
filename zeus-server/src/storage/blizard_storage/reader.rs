use std::vec::Vec;
use std::sync::Arc;
use std::slice;
use std::mem;

use parquet::file::reader::FileReader as ParquetFileReader;
use parquet::basic::LogicalType as ParquetLogicalType;
use parquet::basic::Type as ParquetPhysicalType;
use parquet::schema::types::Type as ParquetType;
use parquet::file::reader::RowGroupReader;
use parquet::data_type::DataType as ParquetDataType;
use parquet::data_type::BoolType as ParquetBoolType;
use parquet::data_type::Int32Type as ParquetInt32Type;
use parquet::data_type::Int64Type as ParquetInt64Type;
use parquet::data_type::FloatType as ParquetFloatType;
use parquet::data_type::DoubleType as ParquetDoubleType;
use parquet::data_type::ByteArrayType as ParquetByteArrayType;
use parquet::column::reader::ColumnReader;
use parquet::column::reader::ColumnReaderImpl;
use parquet::column::reader::get_typed_column_reader;
use arrow::datatypes::Schema;
use arrow::datatypes::DataType;
use arrow::datatypes::Field;
use arrow::record_batch::RecordBatch;
use arrow::array::Array;
use arrow::array_data::ArrayDataBuilder;
use arrow::buffer::Buffer;
use arrow::memory;
use arrow::datatypes::ArrowPrimitiveType;
use arrow::array::PrimitiveArray;
use arrow::array::BinaryArray;

use util::errors::*;

struct ParquetReader<R>
  where R: ParquetFileReader
{
  reader: R
}

impl<R> ParquetReader<R>
  where R: ParquetFileReader
{
  pub fn new(reader: R) -> ParquetReader<R> {
    ParquetReader {
      reader
    }
  }

  pub fn num_rows(&self) -> Result<usize> {
    Ok(self.reader.metadata().file_metadata().num_rows() as usize)
  }

  pub fn num_of_row_groups(&self) -> Result<usize> {
    Ok(self.reader.num_row_groups())
  }

  pub fn read_schema(&self) -> Result<Schema> {
    let fields = self.reader.metadata().file_metadata()
      .schema()
      .get_fields()
      .iter()
      .try_fold(Vec::new(), |mut fields, parquet_type| -> Result<Vec<Field>> {
        let data_type = parquet_to_arrow_type(&**parquet_type)?;
        fields.push(Field::new(parquet_type.name(), data_type, false));
        Ok(fields)
      })?;

    Ok(Schema::new(fields))
  }

  pub fn get_row_group(&self, idx: usize) -> Result<Box<dyn RowGroupReader>> {
    Ok(self.reader.get_row_group(idx)?)
  }

  pub fn read_row_group(&self, row_group_idx: usize, column_indices: Vec<usize>)
    -> Result<RecordBatch> {
    let row_group = self.get_row_group(row_group_idx)?;
    let file_schema = self.read_schema()?;
    let row_num = row_group.metadata().num_rows() as usize;

    column_indices.iter()
      .try_fold(RecordBatchBuilder::new(), |builder, column_idx| -> Result<RecordBatchBuilder> {
        let column_idx = *column_idx;
        let field = &file_schema.columns()[column_idx];
        let reader = row_group.get_column_reader(column_idx)?;
        let data_type = field.data_type();


        let arr = match data_type {
          DataType::Boolean =>
            read_primitive_column::<ParquetBoolType, bool>(row_num, reader, data_type),
          DataType::Int32 =>
            read_primitive_column::<ParquetInt32Type , i32>(row_num, reader, data_type),
          DataType::Int64 =>
            read_primitive_column::<ParquetInt64Type , i64>(row_num, reader, data_type),
          DataType::Float32 =>
            read_primitive_column::<ParquetFloatType , f32>(row_num, reader, data_type),
          DataType::Float64 =>
            read_primitive_column::<ParquetDoubleType , f64>(row_num, reader, data_type),
          DataType::Utf8 =>
            read_utf8_array(row_num, reader),
          _ => bail!("Unconvertable data type: {:?}", data_type)
        }?;

        Ok(builder.add_field(field.clone()).add_column(arr))
      }).and_then(RecordBatchBuilder::build)
  }
}

fn read_primitive_column<P, T>(row_num: usize, reader: ColumnReader, arrow_type: &DataType)
  -> Result<Arc<dyn Array>>
  where P: ParquetDataType + 'static,
        T: Sized + ArrowPrimitiveType
{
  let mut column_reader = get_typed_column_reader::<P>(reader);
  let buffer_ptr = memory::allocate_aligned((row_num * mem::size_of::<T>()) as i64)
    .map_err(|e| ErrorKind::Arrow(e))?;

  let data_ref = unsafe {
    slice::from_raw_parts_mut(mem::transmute(buffer_ptr), row_num)
  };

  let (read_num, _) = column_reader.read_batch(row_num, None, None, data_ref)?;
  if read_num != row_num {
    bail!("Expected {} rows, but read {} rows", row_num, read_num);
  }

  let buffer = Buffer::from_raw_parts(buffer_ptr, row_num);

  let array_data = ArrayDataBuilder::new(arrow_type.clone())
    .len(row_num as i64)
    .null_count(0i64)
    .add_buffer(buffer)
    .build();

  Ok(Arc::new(PrimitiveArray::<T>::from(array_data)))
}

fn read_utf8_array(row_num: usize, mut reader: ColumnReader) -> Result<Arc<dyn Array>> {
  //TODO: Read utf8 is slow, optimize this
  let mut vec = Vec::with_capacity(row_num);
  vec.resize_default(row_num);
  let mut r = get_typed_column_reader::<ParquetByteArrayType>(reader);
  let (num_read, _) = r.read_batch(row_num, None, None, &mut vec)?;
  if num_read != row_num {
    bail!("Expected {} rows, but read {} rows", row_num, num_read);
  }

  let strs = vec.into_iter()
    .try_fold(Vec::new(), |mut ret, byte_arr| -> Result<Vec<String>> {
      let mut bytes = Vec::new();
      bytes.extend_from_slice(byte_arr.data());
      let local_str = String::from_utf8(bytes)?;
      ret.push(local_str );
      Ok(ret)
    })?;

  let str_refs = strs.iter()
    .map(String::as_str)
    .collect::<Vec<&str>>();

  Ok(Arc::new(BinaryArray::from(str_refs)))
}

fn parquet_to_arrow_type(parquet_type: &ParquetType) -> Result<DataType> {
  match parquet_type {
    ParquetType::PrimitiveType {
      ref basic_info,
      ref physical_type,
      ..
    } => {
      match physical_type {
        ParquetPhysicalType::BOOLEAN => Ok(DataType::Boolean),
        ParquetPhysicalType::INT32 => Ok(DataType::Int32),
        ParquetPhysicalType::INT64 => Ok(DataType::Int64),
        ParquetPhysicalType::FLOAT => Ok(DataType::Float32),
        ParquetPhysicalType::DOUBLE => Ok(DataType::Float64),
        ParquetPhysicalType::BYTE_ARRAY if basic_info.logical_type() == ParquetLogicalType::UINT_8 =>
          Ok(DataType::Utf8),
        _ => bail!("Unable to convert physical type {:?}", parquet_type)
      }
    },
    _ => bail!("Unable to convert physical type {:?}", parquet_type)
  }
}

struct RecordBatchBuilder {
  fields: Vec<Field>,
  arrays: Vec<Arc<Array>>
}

impl RecordBatchBuilder {
  pub fn new() -> Self {
    RecordBatchBuilder {
      fields: Vec::new(),
      arrays: Vec::new()
    }
  }

  pub fn add_field(mut self, field: Field) -> Self {
    self.fields.push(field);
    self
  }

  pub fn add_column(mut self, array: Arc<Array>) -> Self {
    self.arrays.push(array);
    self
  }

  pub fn build(self) -> Result<RecordBatch> {
    self.sanity_check()
      .map(|s| {
        let schema = Arc::new(Schema::new(s.fields));
        RecordBatch::new(schema, s.arrays)
      })
  }

  fn sanity_check(self) -> Result<Self> {
    ensure!(self.fields.len() == self.arrays.len(),
    "Number of fileds {} does not match with number of columns {}", self.fields.len(),
    self.arrays.len());
    ensure!(self.fields.len() > 0, "Record batch can't be empty!");
    ensure!(self.arrays[0].len() > 0, "Row num can't be zero");

    let row_num = self.arrays[0].len();

    for arr in &self.arrays {
      ensure!(arr.len() == row_num, "Column length {} does not match with row num {}", arr.len(),
        row_num)
    }

    Ok(self)
  }
}


