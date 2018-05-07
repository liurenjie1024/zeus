use std::vec::Vec;

use rpc::zeus_plan::PlanNodeType;
use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::TopNNode_SortItem;
use super::ServerContext;
use super::ExecNode;
use super::ExecContext;
use super::Block;
use super::expression::Expr;
use util::errors::*;

struct SortItem {
  _item: Expr,
  _desc: bool
}

pub struct TopNNode {
  _sort_items: Vec<SortItem>,
  _limit: i32,
  input: Box<ExecNode>
}

impl ExecNode for TopNNode {
  fn open(&mut self, _context: &mut ExecContext) -> Result<()> {
    unimplemented!()
  }

  fn next(&mut self) -> Result<Block> {
    unimplemented!()
  }

  fn close(&mut self) -> Result<()> {
    unimplemented!()
  }
}

impl SortItem {
  fn new(rpc_sort_item: &TopNNode_SortItem) -> Result<SortItem> {
    Ok(SortItem {
      _item: Expr::new(rpc_sort_item.get_expr())?,
      _desc: rpc_sort_item.get_desc()
    })
  }
}

impl TopNNode {
  pub fn new(plan_node: &PlanNode, _server_context: &ServerContext, mut children: Vec<Box<ExecNode>>)
    -> Result<Box<ExecNode>> {
    ensure!(plan_node.get_plan_node_type() == PlanNodeType::TOPN_NODE,
      "Can't create topn node from {:?}", plan_node.get_plan_node_type());
    ensure!(children.len() == 1,
      "Input size of topn node should be 1 rather {}", children.len());

    let input = children.pop().unwrap();

    let sort_items = plan_node.get_topn_node().get_sort_item()
      .iter()
      .try_fold(Vec::new(), |mut res, item| -> Result<Vec<SortItem>> {
        res.push(SortItem::new(item)?);
        Ok(res)
      })?;

    Ok(box TopNNode {
      _sort_items: sort_items,
      _limit: plan_node.get_topn_node().get_limit(),
      input
    })
  }
}

