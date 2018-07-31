use exec::expression::Expr;
use exec::expression::ScalarFuncExpr;
use storage::column::vec_column_data::Datum;
use rpc::zeus_expr::ScalarFuncId;
use parquet::file::metadata::RowGroupMetaDataPtr;
use parquet::file::statistics::Statistics;

impl Expr {
  pub(super) fn filter(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    match self {
      Expr::ScalarFunc(scalar_func) => scalar_func.filter(row_group_metadata.clone()),
      _ => true
    }
  }
}

impl ScalarFuncExpr {
  fn filter(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    match self.get_id() {
      // logical operators
      ScalarFuncId::AND => self.eval_and(row_group_metadata),
      ScalarFuncId::OR => self.eval_or(row_group_metadata),

      // cmp operators
      ScalarFuncId::GT_INT32 => self.eval_gt_i32(row_group_metadata),
      ScalarFuncId::GT_INT64 => self.eval_gt_i64(row_group_metadata),
      ScalarFuncId::GT_FLOAT4 => self.eval_gt_f32(row_group_metadata),
      ScalarFuncId::GT_FLOAT8 => self.eval_gt_f64(row_group_metadata),

      ScalarFuncId::GE_INT32 => self.eval_ge_i32(row_group_metadata),
      ScalarFuncId::GE_INT64 => self.eval_ge_i64(row_group_metadata),
      ScalarFuncId::GE_FLOAT4 => self.eval_ge_f32(row_group_metadata),
      ScalarFuncId::GE_FLOAT8 => self.eval_ge_f64(row_group_metadata),
      
      ScalarFuncId::EQ_INT32 => self.eval_eq_i32(row_group_metadata),
      ScalarFuncId::EQ_INT64 => self.eval_eq_i64(row_group_metadata),
      ScalarFuncId::EQ_FLOAT4 => self.eval_eq_f32(row_group_metadata),
      ScalarFuncId::EQ_FLOAT8 => self.eval_eq_f64(row_group_metadata),
      
      ScalarFuncId::LT_INT32 => self.eval_lt_i32(row_group_metadata),
      ScalarFuncId::LT_INT64 => self.eval_lt_i64(row_group_metadata),
      ScalarFuncId::LT_FLOAT4 => self.eval_lt_f32(row_group_metadata),
      ScalarFuncId::LT_FLOAT8 => self.eval_lt_f64(row_group_metadata),

      ScalarFuncId::LE_INT32 => self.eval_le_i32(row_group_metadata),
      ScalarFuncId::LE_INT64 => self.eval_le_i64(row_group_metadata),
      ScalarFuncId::LE_FLOAT4 => self.eval_le_f32(row_group_metadata),
      ScalarFuncId::LE_FLOAT8 => self.eval_le_f64(row_group_metadata),

      _ => true
    }
  }
}

// Logical implementations
impl ScalarFuncExpr {
  fn eval_and(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    assert_eq!(ScalarFuncId::AND, self.get_id());

    self.get_args()
      .iter()
      .all(|e| e.filter(row_group_metadata.clone()))
  }

  fn eval_or(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    assert_eq!(ScalarFuncId::OR, self.get_id());

    self.get_args()
      .iter()
      .any(|e| e.filter(row_group_metadata.clone()))
  }
}

// Comparator implementations
impl ScalarFuncExpr {
  fn eval_ge_i32(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Int32(v), Statistics::Int32(t)) => t.max() >= v,
          _ => true
        }
      })
  }

  fn eval_ge_i64(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Int64(v), Statistics::Int64(t)) => t.max() >= v,
          _ => true
        }
      })
  }

  fn eval_ge_f32(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Float4(v), Statistics::Float(t)) => t.max() >= v,
          _ => true
        }
      })
  }

  fn eval_ge_f64(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Float8(v), Statistics::Double(t)) => t.max() >= v,
          _ => true
        }
      })
  }
  
  fn eval_gt_i32(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Int32(v), Statistics::Int32(t)) => t.max() > v,
          _ => true
        }
      })
  }

  fn eval_gt_i64(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Int64(v), Statistics::Int64(t)) => t.max() > v,
          _ => true
        }
      })
  }

  fn eval_gt_f32(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Float4(v), Statistics::Float(t)) => t.max() > v,
          _ => true
        }
      })
  }

  fn eval_gt_f64(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Float8(v), Statistics::Double(t)) => t.max() > v,
          _ => true
        }
      })
  }

  fn eval_eq_i32(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Int32(v), Statistics::Int32(t)) => (v>=t.min()) && (v <= t.max()),
          _ => true
        }
      })
  }

  fn eval_eq_i64(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Int64(v), Statistics::Int64(t)) => (v>=t.min()) && (v <= t.max()),
          _ => true
        }
      })
  }

  fn eval_eq_f32(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Float4(v), Statistics::Float(t)) => (v>=t.min()) && (v <= t.max()),
          _ => true
        }
      })
  }

  fn eval_eq_f64(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Float8(v), Statistics::Double(t)) => (v>=t.min()) && (v <= t.max()),
          _ => true
        }
      })
  }

  fn eval_le_i32(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Int32(v), Statistics::Int32(t)) => t.min() <= v,
          _ => true
        }
      })
  }

  fn eval_le_i64(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Int64(v), Statistics::Int64(t)) => t.min() <= v,
          _ => true
        }
      })
  }

  fn eval_le_f32(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Float4(v), Statistics::Float(t)) => t.min() <= v,
          _ => true
        }
      })
  }

  fn eval_le_f64(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Float8(v), Statistics::Double(t)) => t.min() <= v,
          _ => true
        }
      })
  }

  fn eval_lt_i32(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Int32(v), Statistics::Int32(t)) => t.min() < v,
          _ => true
        }
      })
  }

  fn eval_lt_i64(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Int64(v), Statistics::Int64(t)) => t.min() < v,
          _ => true
        }
      })
  }

  fn eval_lt_f32(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Float4(v), Statistics::Float(t)) => t.min() < v,
          _ => true
        }
      })
  }

  fn eval_lt_f64(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    self.eval_cmp(row_group_metadata,
      |data, stat| {
        match (data, stat) {
          (Datum::Float8(v), Statistics::Double(t)) => t.min() < v,
          _ => true
        }
      })
  }

  fn eval_cmp(&self,
    row_group_metadata: RowGroupMetaDataPtr,
    comparator: fn(&Datum, &Statistics) -> bool) -> bool {
    if self.get_args().len() != 2 {
      error!("Invalid argument number {} for compare operator.", self.get_args().len());
      return true;
    }

    let (field_name, literal) = match (self.get_args().get(0), self.get_args().get(1)) {
      (Some(Expr::ColumnRef(column_ref)), Some(Expr::Literal(literal))) =>
        (column_ref.get_column_name(), literal.get_data()),
      (Some(Expr::Literal(literal)), Some(Expr::ColumnRef(column_ref))) =>
        (column_ref.get_column_name(), literal.get_data()),
      _ =>
        return true
    };

    row_group_metadata.columns()
      .iter()
      .find(|c| c.column_path().string() == field_name)
      .and_then(|c| c.statistics())
      .filter(|s| s.has_min_max_set())
      .map(|s| comparator(&literal, s))
      .unwrap_or(true)
  }
}

#[cfg(test)]
mod tests {
  use std::rc::Rc;

  use parquet::file::metadata::RowGroupMetaData;
  use parquet::file::metadata::RowGroupMetaDataPtr;
  use parquet::schema::types::SchemaDescriptor;
  use parquet::schema::types::Type;
  use parquet::basic::Type as PhysicalType;
  use parquet::basic::LogicalType;

  use parquet_format::RowGroup;
  use parquet_format::ColumnMetaData;
  use parquet_format::Type as ParquetType;
  use parquet_format::CompressionCodec;
  use parquet_format::Statistics;
  use parquet_format::ColumnChunk;

  use rpc::zeus_meta::ColumnType;
  use rpc::zeus_expr::ScalarFuncId;

  use exec::expression::Expr;
  use exec::expression::ScalarFuncExpr;
  use exec::expression::LiteralExpr;
  use exec::expression::ColumnRefExpr;

  use storage::column::vec_column_data::Datum;


  use util::errors::*;

  fn create_row_group_metadata() -> Result<RowGroupMetaDataPtr> {
    let schema_desc = {
      //create dsp id type, int32, int32
      let dsp_id = Rc::new(Type::primitive_type_builder("dsp_id", PhysicalType::INT32)
        .with_logical_type(LogicalType::INT_32)
        .build()?);

      //create logtime type, timestamp, int64
      let logtime = Rc::new(Type::primitive_type_builder("logtime", PhysicalType::INT64)
        .with_logical_type(LogicalType::TIMESTAMP_MILLIS)
        .build()?);

      //create adspace id type, utf8, byte array
      let adspace_id = Rc::new(Type::primitive_type_builder("adspace_id", PhysicalType::BYTE_ARRAY)
        .with_logical_type(LogicalType::UTF8)
        .build()?);

      let struct_type = Rc::new(Type::group_type_builder("")
        .with_fields(&mut vec![dsp_id, logtime, adspace_id])
        .build()?);

      Rc::new(SchemaDescriptor::new(struct_type))
    };

    let row_group = {

      // dsp id column chunk
      let dsp_id_column_chunk = {
        let stat = Statistics::new(None, None, None, None, Some(6i32.to_bytes().to_vec()),
          Some(4i32.to_bytes().to_vec()));

        let metadata = ColumnMetaData::new(ParquetType::INT32, vec![], vec!["dsp_id".to_string()],
          CompressionCodec::UNCOMPRESSED, 10, 10 , 10, None, 0, None, None, stat, None);

        ColumnChunk::new(None, 1i64, Some(metadata), None, None, None, None)
      };

      // logtime column chunk
      let logtime_column_chunk = {
        let stat = Statistics::new(None, None, None, None, Some(8i64.to_bytes().to_vec()),
          Some(1i64.to_bytes().to_vec()));

        let metadata = ColumnMetaData::new(ParquetType::INT64, vec![], vec!["logtime".to_string()],
          CompressionCodec::UNCOMPRESSED, 10, 10 , 10, None, 0, None, None, stat, None);

        ColumnChunk::new(None, 2i64, Some(metadata), None, None, None, None)
      };

      // adspace id column chunk
      let adspace_id_column_chunk = {
        let metadata = ColumnMetaData::new(ParquetType::INT64, vec![],
          vec!["adspace_id".to_string()], CompressionCodec::UNCOMPRESSED, 10, 10 , 10, None, 0,
          None, None, None, None);

        ColumnChunk::new(None, 3i64, Some(metadata), None, None, None, None)
      };

      RowGroup::new(vec![dsp_id_column_chunk, logtime_column_chunk, adspace_id_column_chunk],
        10, 2, None)
    };

    Ok(Rc::new(RowGroupMetaData::from_thrift(schema_desc, row_group)?))
  }

  #[test]
  fn test_filter_non_scalar_func() {
    let row_group = create_row_group_metadata().unwrap();

    // literal expression should return true
    let literal_expr = Expr::Literal(LiteralExpr::new(
      ColumnType::INT32,
      Datum::Int32(4),
      "a".to_string()).unwrap());
    assert_eq!(true, literal_expr.filter(row_group.clone()));

    // column ref expression should return true
    let column_ref_expr = Expr::ColumnRef(ColumnRefExpr::new(
      "a".to_string(),
      ColumnType::INT32,
      "a".to_string()));
    assert_eq!(true, column_ref_expr.filter(row_group.clone()));
  }

  #[test]
  fn test_filter_gt_func() {
    // in this row group 4 <= dsp_id <= 6
    let row_group = create_row_group_metadata().unwrap();


    // dsp_id > 3 need to scan this group
    {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(3),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      let gt_dsp_id_expr = Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::GT_INT32,
        vec![dsp_id_expr, literal_expr],
        "a".to_string()).unwrap());
      assert_eq!(true, gt_dsp_id_expr.filter(row_group.clone()));
    }


    // dsp_id > 5 need to scan this group
    {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(5),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      let gt_dsp_id_expr = Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::GT_INT32,
        vec![dsp_id_expr, literal_expr],
        "a".to_string()).unwrap());
      assert_eq!(true, gt_dsp_id_expr.filter(row_group.clone()));
    }

    // dsp_id > 6  should skip this group
    {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(6),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      let gt_dsp_id_expr = Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::GT_INT32,
        vec![dsp_id_expr, literal_expr],
        "a".to_string()).unwrap());
      assert_eq!(false, gt_dsp_id_expr.filter(row_group.clone()));
    }
  }

  #[test]
  fn test_filter_ge_func() {
    // in this row group 4 <= dsp_id <= 6
    let row_group = create_row_group_metadata().unwrap();


    // dsp_id >= 3 need to scan this group
    {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(3),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      let gt_dsp_id_expr = Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::GE_INT32,
        vec![dsp_id_expr, literal_expr],
        "a".to_string()).unwrap());
      assert_eq!(true, gt_dsp_id_expr.filter(row_group.clone()));
    }


    // dsp_id >= 6 need to scan this group
    {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(6),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      let gt_dsp_id_expr = Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::GE_INT32,
        vec![dsp_id_expr, literal_expr],
        "a".to_string()).unwrap());
      assert_eq!(true, gt_dsp_id_expr.filter(row_group.clone()));
    }

    // dsp_id >= 7  should skip this group
    {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(7),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      let gt_dsp_id_expr = Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::GE_INT32,
        vec![dsp_id_expr, literal_expr],
        "a".to_string()).unwrap());
      assert_eq!(false, gt_dsp_id_expr.filter(row_group.clone()));
    }
  }

  #[test]
  fn test_filter_eq_func() {
    // in this row group 4 <= dsp_id <= 6
    let row_group = create_row_group_metadata().unwrap();


    // dsp_id == 3 need to scan this group
    {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(3),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      let gt_dsp_id_expr = Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::EQ_INT32,
        vec![dsp_id_expr, literal_expr],
        "a".to_string()).unwrap());
      assert_eq!(false, gt_dsp_id_expr.filter(row_group.clone()));
    }


    // dsp_id == 6 need to scan this group
    {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(6),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      let gt_dsp_id_expr = Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::EQ_INT32,
        vec![dsp_id_expr, literal_expr],
        "a".to_string()).unwrap());
      assert_eq!(true, gt_dsp_id_expr.filter(row_group.clone()));
    }

    // dsp_id == 7  should skip this group
    {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(7),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      let gt_dsp_id_expr = Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::EQ_INT32,
        vec![dsp_id_expr, literal_expr],
        "a".to_string()).unwrap());
      assert_eq!(false, gt_dsp_id_expr.filter(row_group.clone()));
    }
  }

  #[test]
  fn test_filter_lt_func() {
    // in this row group 4 <= dsp_id <= 6
    let row_group = create_row_group_metadata().unwrap();


    // dsp_id < 7 need to scan this group
    {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(7),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      let gt_dsp_id_expr = Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::LT_INT32,
        vec![dsp_id_expr, literal_expr],
        "a".to_string()).unwrap());
      assert_eq!(true, gt_dsp_id_expr.filter(row_group.clone()));
    }


    // dsp_id < 5 need to scan this group
    {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(5),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      let gt_dsp_id_expr = Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::LT_INT32,
        vec![dsp_id_expr, literal_expr],
        "a".to_string()).unwrap());
      assert_eq!(true, gt_dsp_id_expr.filter(row_group.clone()));
    }

    // dsp_id < 3  should skip this group
    {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(3),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      let gt_dsp_id_expr = Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::LT_INT32,
        vec![dsp_id_expr, literal_expr],
        "a".to_string()).unwrap());
      assert_eq!(false, gt_dsp_id_expr.filter(row_group.clone()));
    }
  }


  #[test]
  fn test_filter_le_func() {
    // in this row group 4 <= dsp_id <= 6
    let row_group = create_row_group_metadata().unwrap();


    // dsp_id <= 7 need to scan this group
    {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(7),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      let gt_dsp_id_expr = Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::LE_INT32,
        vec![dsp_id_expr, literal_expr],
        "a".to_string()).unwrap());
      assert_eq!(true, gt_dsp_id_expr.filter(row_group.clone()));
    }


    // dsp_id <= 4 need to scan this group
    {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(5),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      let gt_dsp_id_expr = Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::LE_INT32,
        vec![dsp_id_expr, literal_expr],
        "a".to_string()).unwrap());
      assert_eq!(true, gt_dsp_id_expr.filter(row_group.clone()));
    }

    // dsp_id <= 3  should skip this group
    {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(3),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      let gt_dsp_id_expr = Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::LE_INT32,
        vec![dsp_id_expr, literal_expr],
        "a".to_string()).unwrap());
      assert_eq!(false, gt_dsp_id_expr.filter(row_group.clone()));
    }
  }


  #[test]
  fn test_filter_and_func_all_true() {
    let row_group = create_row_group_metadata().unwrap();

    let literal_expr = Expr::Literal(LiteralExpr::new(
      ColumnType::INT32,
      Datum::Int32(3),
      "a".to_string()).unwrap());

    let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
      "dsp_id".to_string(),
      ColumnType::INT32,
      "dsp_id".to_string()));

    let and_expr =  Expr::ScalarFunc(ScalarFuncExpr::new(
      ColumnType::BOOL,
      ScalarFuncId::AND,
      vec![literal_expr, dsp_id_expr],
      "a".to_string()).unwrap());

    assert_eq!(true, and_expr.filter(row_group.clone()));
  }

  #[test]
  fn test_filter_and_func_one_false() {
    let row_group = create_row_group_metadata().unwrap();

    // dsp_id > 10 should return false
    let gt_expr = {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(10),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::GT_INT32,
        vec![literal_expr, dsp_id_expr],
        "a".to_string()).unwrap())
    };


    // literal expr should return true
    let literal_expr = Expr::Literal(LiteralExpr::new(
      ColumnType::INT32,
      Datum::Int32(10),
      "a".to_string()).unwrap());


    let and_expr =  Expr::ScalarFunc(ScalarFuncExpr::new(
      ColumnType::BOOL,
      ScalarFuncId::AND,
      vec![literal_expr, gt_expr],
      "a".to_string()).unwrap());

    assert_eq!(false, and_expr.filter(row_group.clone()));
  }


  #[test]
  fn test_filter_or_func_one_true() {
    let row_group = create_row_group_metadata().unwrap();

    // dsp_id > 10 should return false
    let gt_expr = {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(10),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::GT_INT32,
        vec![literal_expr, dsp_id_expr],
        "a".to_string()).unwrap())
    };


    // literal expr should return true
    let literal_expr = Expr::Literal(LiteralExpr::new(
      ColumnType::INT32,
      Datum::Int32(10),
      "a".to_string()).unwrap());


    let or_expr =  Expr::ScalarFunc(ScalarFuncExpr::new(
      ColumnType::BOOL,
      ScalarFuncId::OR,
      vec![literal_expr, gt_expr],
      "a".to_string()).unwrap());

    assert_eq!(true, or_expr.filter(row_group.clone()));
  }

  #[test]
  fn test_filter_or_func_all_false() {
    // 4 <= dsp_id <= 6, 1 <= logtime <= 8
    let row_group = create_row_group_metadata().unwrap();

    // dsp_id > 10 should return false
    let gt_expr1 = {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT32,
        Datum::Int32(10),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "dsp_id".to_string(),
        ColumnType::INT32,
        "dsp_id".to_string()));

      Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::GT_INT32,
        vec![literal_expr, dsp_id_expr],
        "a".to_string()).unwrap())
    };

    // logtime > 10 should return false
    let gt_expr2 = {
      let literal_expr = Expr::Literal(LiteralExpr::new(
        ColumnType::INT64,
        Datum::Int64(10),
        "a".to_string()).unwrap());

      let dsp_id_expr = Expr::ColumnRef(ColumnRefExpr::new(
        "logtime".to_string(),
        ColumnType::INT64,
        "logtime".to_string()));

      Expr::ScalarFunc(ScalarFuncExpr::new(
        ColumnType::BOOL,
        ScalarFuncId::GT_INT64,
        vec![literal_expr, dsp_id_expr],
        "a".to_string()).unwrap())
    };

    let or_expr =  Expr::ScalarFunc(ScalarFuncExpr::new(
      ColumnType::BOOL,
      ScalarFuncId::OR,
      vec![gt_expr1, gt_expr2],
      "a".to_string()).unwrap());

    assert_eq!(false, or_expr.filter(row_group.clone()));
  }
}





