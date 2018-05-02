use storage::column::column_data::Datum;
use exec::Block;

mod scalar_func;

pub enum Expr {
  Literal(Datum),
  ColumnRef(String),
  ScalarFunc(ScalarFuncExpr)
}

struct ScalarFuncExpr {

}

struct EvalContext {
}

impl Expr {
  pub fn eval(&mut self, context: &EvalContext, input: &Block) -> Block {
    unimplemented!()
  }
}

