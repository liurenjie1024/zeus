use std::vec::Vec;
use std::any::Any;
use std::sync::Arc;
use std::iter::Iterator;

use rpc::zeus_meta::ColumnType;
use rpc::zeus_data::ColumnValue;
use storage::column::Column;
use storage::column::ColumnValueIter;
use util::errors::*;
use util::cow_ptr::ToBoxedOwned;

pub struct ColumnString {
  offsets: Arc<Vec<usize>>,
  chars: Arc<Vec<u8>>,
}

impl ColumnString {
  fn size(&self) -> usize {
    self.offsets.len()
  }

  fn field_type(&self) -> ColumnType {
    ColumnType::STRING
  }
  fn into_iter(&self) -> ColumnStringIterator {
    ColumnStringIterator {
      cur_pos: 1usize,
      offsets: self.offsets.clone(),
      chars: self.chars.clone()
    }
  }
}

impl ToBoxedOwned for ColumnString {
  fn to_boxed_owned(&self) -> Box<Any> {
    Box::new(ColumnString {
      offsets: self.offsets.clone(),
      chars: self.chars.clone(),
    })
  }
}

impl ColumnString {
  pub fn new(
    offsets: Vec<usize>,
    chars: Vec<u8>,
  ) -> Result<ColumnString>
  {
    Ok(ColumnString {
      offsets: Arc::new(offsets),
      chars: Arc::new(chars),
    })
  }
}

struct ColumnStringIterator {
  cur_pos: usize,
  offsets: Arc<Vec<usize>>,
  chars: Arc<Vec<u8>>
}

impl Iterator for ColumnStringIterator {
  type Item = ColumnValue;

  fn next(&mut self) -> Option<Self::Item> {
    let ret = self.offsets.get(self.cur_pos)
      .map(|end| self.offsets.get(self.cur_pos-1).unwrap()..end)
      .map(|range|(*range.start)..(*range.end))
      .map(|range|self.chars[range].to_vec())
      .map(|s| {
        String::from_utf8(s)
          .map_err(|e| error!("Failed to convert string: {:?}", e))
      }).and_then(|r| r.ok())
      .map(|s| {
        let mut c = ColumnValue::new();
        c.set_string_value(s);
        c
      });

    self.cur_pos += 1;
    ret
  }
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;
  use std::iter::Iterator;

  use rpc::zeus_data::ColumnValue;
  use storage::column::Column;
  use super::ColumnString;

  struct ColumnStringBuilder {
    offsets: Vec<usize>,
    chars: Vec<u8>
  }

  impl ColumnStringBuilder {
    fn new() -> ColumnStringBuilder {
      ColumnStringBuilder {
        offsets: vec![0usize],
        chars: Vec::new()
      }
    }

    fn add(&mut self, data: String) {
      let mut bytes = data.into_bytes();
      let last_offset = *(self.offsets.last().unwrap());
      {
        self.offsets.push(last_offset + bytes.len());
      }
      self.chars.append(&mut bytes);
    }

    fn build(self) -> ColumnString {
      ColumnString {
        offsets: Arc::new(self.offsets),
        chars: Arc::new(self.chars)
      }
    }
  }

  #[test]
  fn test_column_string_iterator() {
    let mut builder = ColumnStringBuilder::new();
    builder.add("123".to_string());
    builder.add("456".to_string());
    builder.add("7".to_string());

    let column = builder.build();

    let values: Vec<ColumnValue> = column.into_iter()
      .collect();

    let columns: Vec<ColumnValue> = vec!["123", "456", "7"]
      .iter()
      .map(|s| to_column_value(s))
      .collect();

    assert_eq!(columns, values);
  }

  fn to_column_value(s: &str) -> ColumnValue {
    let mut c = ColumnValue::new();
    c.set_string_value(s.to_string());
    c
  }
}