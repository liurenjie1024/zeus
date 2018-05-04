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
  item: Expr,
  desc: bool
}

pub struct TopNNode {
  sort_items: Vec<SortItem>,
  limit: i32,
  input: Box<ExecNode>
}

impl ExecNode for TopNNode {
  fn open(&mut self, context: &mut ExecContext) -> Result<()> {
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
      item: Expr::new(rpc_sort_item.get_expr())?,
      desc: rpc_sort_item.get_desc()
    })
  }
}

impl TopNNode {
  pub fn new(plan_node: &PlanNode, server_context: &ServerContext) -> Result<Box<ExecNode>> {
    ensure!(plan_node.get_plan_node_type() == PlanNodeType::TOPN_NODE,
      "Can't create topn node from {:?}", plan_node.get_plan_node_type());
    ensure!(plan_node.get_children() == 1,
      "Input size of topn node should be 1 rather {}", plan_node.get_children().len());

    let input = plan_node.get_children().first().unwrap().to(server_context)?;

    let mut sort_items = Vec::new();

    plan_node.get_topn_node().get_sort_item()
      .iter()
      .try_fold(&mut sort_items, |res, item| Ok(res.push(SortItem::new(item)?)));

    Ok(box TopNNode {
      sort_items,
      limit: plan_node.get_topn_node().get_limit(),
      input
    })
  }
}

