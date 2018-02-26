use std::io::Cursor;
use std::vec::Vec;

use bytes::{Buf, BigEndian};

use db::column::ColumnFactory;
use db::column::Column;
use db::column::{BoolColumn, ByteColumn, IntColumn, FloatColumn, LongColumn, TimestampColumn};
use rpc::zeus_meta::FieldType;
use util::error::Result;

struct NumericColumnFactory {
    field_type: FieldType,
    column_size: usize
}

impl ColumnFactory for NumericColumnFactory {
    fn create_column(&mut self, raw_data: &[u8]) -> Result<Box<Column>>  {
        unimplemented!()
    }
}

impl NumericColumnFactory {
    fn create_bool_column(&self, raw_data: &[u8]) -> Result<Box<BoolColumn>> {
        let mut data: Vec<bool> = Vec::with_capacity(self.column_size);


        for x in 0..self.column_size {
            let bit = (raw_data[x/8] >> (x%8)) & 0x01;
            data.push(bit > 0);
        }

        let column = BoolColumn::create(self.field_type, data)?;
        Ok(Box::new(column))
    }

    fn create_byte_column(&self, raw_data: &[u8]) -> Result<Box<ByteColumn>> {
        let mut data: Vec<u8> = Vec::with_capacity(self.column_size);

        for x in raw_data {
            data.push(*x);
        }

        Ok(Box::new(ByteColumn::create(self.field_type, data)?))
    }

    fn create_float_column(&self, raw_data: &[u8]) -> Result<Box<FloatColumn>> {
        let mut data: Vec<f32> = Vec::with_capacity(self.column_size);

        let mut cursor = Cursor::new(raw_data);

        for _ in 0..self.column_size {
            data.push(cursor.get_f32::<BigEndian>());
        }

        Ok(Box::new(FloatColumn::create(self.field_type, data)?))
    }


    fn create_int_column(&self, raw_data: &[u8]) -> Result<Box<IntColumn>> {
        let mut data: Vec<i32> = Vec::with_capacity(self.column_size);

        let mut cursor = Cursor::new(raw_data);

        for _ in 0..self.column_size {
            data.push(cursor.get_i32::<BigEndian>());
        }

        Ok(Box::new(IntColumn::create(self.field_type, data)?))
    }

    fn create_long_column(&self, raw_data: &[u8]) -> Result<Box<LongColumn>> {
        let mut data: Vec<i64> = Vec::with_capacity(self.column_size);

        let mut cursor = Cursor::new(raw_data);

        for _ in 0..self.column_size {
            data.push(cursor.get_i64::<BigEndian>());
        }

        Ok(Box::new(LongColumn::create(self.field_type, data)?))
    }

    fn create_timestamp_column(&self, raw_data: &[u8]) -> Result<Box<TimestampColumn>> {
        let mut data: Vec<u64> = Vec::with_capacity(self.column_size);

        let mut cursor = Cursor::new(raw_data);

        for _ in 0..self.column_size {
            data.push(cursor.get_u64::<BigEndian>());
        }

        Ok(Box::new(TimestampColumn::create(self.field_type, data)?))
    }
}



