
use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::PlanNodeType;
use super::ServerContext;
use super::ExecNode;
use super::ExecContext;
use super::Block;
use util::errors::*;

pub struct LimitExecNode {
  limit: i32,
  input: Box<ExecNode>,
  cur: i32
}

impl ExecNode for LimitExecNode {
  fn open(&mut self, context: &mut ExecContext) -> Result<()> {
    self.input.open(context)?;
    self.cur = 0;
    Ok(())
  }

  fn next(&mut self) -> Result<Block> {
    if self.cur >= self.limit {
      Ok(Block::empty_block())
    } else {
      let mut ret = self.input.next()?;
      let ret_len = ret.len();
      let left = (self.limit - self.cur) as usize;
      if ret_len >= left  {
        self.cur = self.limit;
        ret.eof = true;
        Ok(ret.take(left))
      } else {
        self.cur += ret_len as i32;
        Ok(ret)
      }
    }
  }

  fn close(&mut self) -> Result<()> {
    self.input.close()?;
    self.cur = 0;
    Ok(())
  }
}

impl LimitExecNode {
  pub fn new(plan_node: &PlanNode, _server_context: &ServerContext, mut children: Vec<Box<ExecNode>>)
    -> Result<Box<ExecNode>> {
    ensure!(plan_node.get_plan_node_type() == PlanNodeType::LIMIT_NODE,
      "Can't build limit node from plan node type: {:?}", plan_node.get_plan_node_type());
    ensure!(children.len() == 1,
      "Invalid number of inputs of limit node: {}", children.len());

    let input = children.pop().unwrap();

    Ok(box LimitExecNode {
      limit: plan_node.get_limit_node().get_limit(),
      input,
      cur: 0
    })
  }
}

#[cfg(test)]
mod tests {
  use std::default::Default;
  use std::convert::From;

  use storage::column::Column;
  use storage::column::column_data::ColumnData;
  use storage::column::column_data::Datum;
  use exec::tests::MemoryBlocks;
  use exec::ColumnWithInfo;
  use exec::Block;
  use exec::ExecNode;
  use exec::ExecContext;
  use super::LimitExecNode;
  use rpc::zeus_meta::ColumnType;
  use rpc::zeus_plan::LimitNode;
  use rpc::zeus_plan::PlanNode;
  use rpc::zeus_plan::PlanNodeType;
  use server::ServerContext;

  fn create_memory_block() -> Box<ExecNode> {
    let column1 = Column::new(ColumnType::BOOL, ColumnData::from(vec![true, false]));
    let column2 = Column::new(ColumnType::INT64, ColumnData::from(vec![12i64, 14i64]));
    let block1 = vec![ColumnWithInfo::from(column1), ColumnWithInfo::from(column2)];
    let block1 = Block::from(block1);

    let column3 = Column::new(ColumnType::BOOL, ColumnData::from(vec![false, true]));
    let column4 = Column::new(ColumnType::INT64, ColumnData::from(vec![100000i64, 54321i64]));
    let block2 = vec![ColumnWithInfo::from(column3), ColumnWithInfo::from(column4)];
    let block2 = Block::from(block2);

    box MemoryBlocks {
      blocks: vec![block1, block2],
    }
  }

  fn create_limit_plan_node() -> PlanNode {
    let mut limit_node = LimitNode::new();
    limit_node.set_limit(1);


    let mut plan_node = PlanNode::new();
    plan_node.set_node_id(1);
    plan_node.set_plan_node_type(PlanNodeType::LIMIT_NODE);
    plan_node.set_limit_node(limit_node);

    plan_node
  }

  #[test]
  fn test_create_limit_node_success() {
    let res = LimitExecNode::new(&create_limit_plan_node(), &ServerContext::default(),
                                 vec![create_memory_block()]);
    assert!(res.is_ok());
  }

  #[test]
  fn test_create_limit_node_wrong_node_type() {
    let mut plan_node = create_limit_plan_node();
    plan_node.set_plan_node_type(PlanNodeType::SCAN_NODE);
    let res = LimitExecNode::new(&plan_node, &ServerContext::default(),
                                 vec![create_memory_block()]);
    assert!(res.is_err());
  }

  #[test]
  fn test_create_limit_node_wrong_child_num() {
    let plan_node = create_limit_plan_node();
    let children = vec![create_memory_block(), create_memory_block()];
    let res = LimitExecNode::new(&plan_node, &ServerContext::default(), children);
    assert!(res.is_err());
  }

  #[test]
  fn test_query_from_limit_node() {
    let plan_node = create_limit_plan_node();

    let res = LimitExecNode::new(&plan_node, &ServerContext::default(),
                                 vec![create_memory_block()]);
    assert!(res.is_ok());

    let mut exec_node = res.unwrap();

    let mut exec_context = ExecContext {};
    assert!(exec_node.open(&mut exec_context).is_ok());

    let block = exec_node.next();
    assert!(block.is_ok());
    let block = block.unwrap();

    assert!(block.eof);
    assert_eq!(1, block.len());
    assert_eq!(2, block.columns.len());

    assert_eq!(Datum::from(false) , block.columns.get(0).unwrap().column.get(0).unwrap());
    assert_eq!(Datum::from(100000i64), block.columns.get(1).unwrap().column.get(0).unwrap());


    let block = exec_node.next();
    assert!(block.is_ok());
    let block = block.unwrap();

    assert!(block.eof);
    assert_eq!(0, block.len());
    assert_eq!(0, block.columns.len());
  }
}