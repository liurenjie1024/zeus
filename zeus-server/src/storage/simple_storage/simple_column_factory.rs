use storage::column::ColumnFactory;
use rpc::zeus_meta::ColumnType;

pub fn create_column_factory(
  _field_type: ColumnType,
  _column_size: usize,
) -> Box<ColumnFactory>
{
  unimplemented!()
}
