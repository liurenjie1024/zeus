use super::block::Block;
use util::errors::*;

pub struct ExecContext {}

impl Default for ExecContext {
  fn default() -> Self {
    ExecContext {}
  }
}

pub trait ExecNode {
  fn open(&mut self, ctx: ExecContext) -> Result<()>;
  fn next(&mut self) -> Result<Block>;
  fn close(&mut self) -> Result<()>;
}