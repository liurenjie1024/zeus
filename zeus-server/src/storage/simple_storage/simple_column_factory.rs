use std::io::Cursor;
use std::vec::Vec;
use std::vec::Splice;
use std::mem::size_of;

use bytes::{Buf, BigEndian};

use storage::column::ColumnFactory;
use storage::column::Column;
use storage::column::{BoolColumn, ByteColumn, IntColumn, FloatColumn, LongColumn, TimestampColumn};
use storage::column::StringColumn;
use rpc::zeus_meta::FieldType;
use storage::ErrorKind;
use util::error::Result;
use util::error::Error;

struct NumericColumnFactory {
    field_type: FieldType,
    column_size: usize
}

impl ColumnFactory for NumericColumnFactory {
    fn create_column(&mut self, raw_data: &[u8]) -> Result<Box<Column>>  {
        match self.field_type {
            FieldType::BOOL => self.create_bool_column(raw_data),
            FieldType::BYTE => self.create_byte_column(raw_data),
            FieldType::FLOAT => self.create_float_column(raw_data),
            FieldType::INT32 => self.create_int_column(raw_data),
            FieldType::INT64 => self.create_long_column(raw_data),
            FieldType::TIMESTAMP => self.create_timestamp_column(raw_data),
            FieldType::STRING => Err(Error::DBError(ErrorKind::InvalidFieldType))
        }
    }
}

impl NumericColumnFactory {
    fn create_bool_column(&self, raw_data: &[u8]) -> Result<Box<Column>> {
        let mut data: Vec<bool> = Vec::with_capacity(self.column_size);


        for x in 0..self.column_size {
            let bit = (raw_data[x/8] >> (x%8)) & 0x01;
            data.push(bit > 0);
        }

        Ok(Box::new(BoolColumn::create(self.field_type, data)?))
    }

    fn create_byte_column(&self, raw_data: &[u8]) -> Result<Box<Column>> {
        let mut data: Vec<u8> = Vec::with_capacity(self.column_size);

        for x in raw_data {
            data.push(*x);
        }

        Ok(Box::new(ByteColumn::create(self.field_type, data)?))
    }

    fn create_float_column(&self, raw_data: &[u8]) -> Result<Box<Column>> {
        let mut data: Vec<f32> = Vec::with_capacity(self.column_size);

        let mut cursor = Cursor::new(raw_data);

        for _ in 0..self.column_size {
            data.push(cursor.get_f32::<BigEndian>());
        }

        Ok(Box::new(FloatColumn::create(self.field_type, data)?))
    }


    fn create_int_column(&self, raw_data: &[u8]) -> Result<Box<Column>> {
        let mut data: Vec<i32> = Vec::with_capacity(self.column_size);

        let mut cursor = Cursor::new(raw_data);

        for _ in 0..self.column_size {
            data.push(cursor.get_i32::<BigEndian>());
        }

        Ok(Box::new(IntColumn::create(self.field_type, data)?))
    }

    fn create_long_column(&self, raw_data: &[u8]) -> Result<Box<Column>> {
        let mut data: Vec<i64> = Vec::with_capacity(self.column_size);

        let mut cursor = Cursor::new(raw_data);

        for _ in 0..self.column_size {
            data.push(cursor.get_i64::<BigEndian>());
        }

        Ok(Box::new(LongColumn::create(self.field_type, data)?))
    }

    fn create_timestamp_column(&self, raw_data: &[u8]) -> Result<Box<Column>> {
        let mut data: Vec<u64> = Vec::with_capacity(self.column_size);

        let mut cursor = Cursor::new(raw_data);

        for _ in 0..self.column_size {
            data.push(cursor.get_u64::<BigEndian>());
        }

        Ok(Box::new(TimestampColumn::create(self.field_type, data)?))
    }
}

struct StringColumnFactory {
    column_size: usize
}

impl ColumnFactory for StringColumnFactory {
    fn create_column(&mut self, raw_data: &[u8]) -> Result<Box<Column>> {
        let mut indexes: Vec<usize> = Vec::with_capacity(self.column_size);
        let mut cursor = Cursor::new(raw_data);

        for _ in 0..self.column_size {
            indexes.push(cursor.get_i32::<BigEndian>() as usize);
        }

        let index_size = self.column_size * size_of::<i32>();
        let mut str_data: Vec<u8> = Vec::with_capacity(raw_data.len() - index_size);
        str_data.splice(.., raw_data[index_size..].iter().cloned());

        Ok(Box::new(StringColumn::new(indexes, str_data)?))
    }
}

pub fn create_column_factory(field_type: FieldType, column_size: usize) -> Box<ColumnFactory> {
    match field_type {
        FieldType::STRING => Box::new(StringColumnFactory {
            column_size
        }),
        FieldType::BOOL  | FieldType::BYTE  |
        FieldType::FLOAT | FieldType::INT32 |
        FieldType::INT64 | FieldType::TIMESTAMP => Box::new(NumericColumnFactory {
            field_type,
            column_size
        })
    }
}


