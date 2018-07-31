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
      ScalarFuncId::NOT => self.eval_not(row_group_metadata),
      
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
      .any(|e| !e.filter(row_group_metadata.clone()))
  }

  fn eval_not(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    assert_eq!(ScalarFuncId::NOT, self.get_id());
    self.get_args()
      .first()
      .map(|e| !e.filter(row_group_metadata))
      .unwrap_or(true)
  }

  fn eval_or(&self, row_group_metadata: RowGroupMetaDataPtr) -> bool {
    assert_eq!(ScalarFuncId::OR, self.get_id());

    self.get_args()
      .iter()
      .all(|e| e.filter(row_group_metadata.clone()))
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






