use std::vec::Vec;
use std::cmp;
use std::collections::BinaryHeap;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::default::Default;

use rpc::zeus_plan::PlanNodeType;
use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::TopNNode_SortItem;
use rpc::zeus_meta::ColumnType;
use storage::column::ColumnBuilder;
use storage::column::vec_column_data::Datum;
use super::ServerContext;
use super::ExecNode;
use super::ExecContext;
use super::Block;
use super::expression::Expr;
use super::expression::EvalContext;
use util::errors::*;

struct SortItem {
  item: Expr,
  desc: bool
}

pub struct TopNExecNode {
  sort_items: Vec<SortItem>,
  limit: i32,
  input: Box<ExecNode>,
  executed: bool
}

impl ExecNode for TopNExecNode {
  fn open(&mut self, context: &mut ExecContext) -> Result<()> {
    self.input.open(context)
  }

  fn next(&mut self) -> Result<Block> {
    if self.executed {
      return Err(ErrorKind::EOF.into())
    }

    let eval_context = EvalContext::default();
    self.executed = true;


    // Get source blocks
    let mut source_blocks = BlockChain::default();
    loop {
      let block = self.input.next()?;
      let eof = block.eof;

      source_blocks.add_block(block)?;
      if eof {
        break;
      }
    }

    // Calculate sort by blocks
    let sort_item_blocks = source_blocks.blocks_slice().iter()
      .try_fold(Vec::new(), |mut blocks, b| -> Result<Vec<Block>> {
        blocks.push(self.get_sort_by_block(&eval_context, b)?);
        Ok(blocks)
      })?;

    let sort_item_blocks = BlockChain {
      blocks: sort_item_blocks,
      sort_order: self.sort_items.iter().map(|x| x.desc).collect()
    };


    // Heap meta data
    let row_count = source_blocks.blocks_slice().iter()
      .map(|b| b.len())
      .fold(0usize, |acc, x| acc + x);

    let limit = match self.limit {
      x if x <= 0 => None,
      y => Some(y as usize)
    };

    let mut heap = TopNHeap::new(row_count, limit);

    // insert data to heap
    sort_item_blocks.iter()
      .try_for_each(|r| heap.push(r))?;


    // sorted rows
    let rows = heap.into_sorted_vec().iter()
      .map(|x| x.change_block_chain(&source_blocks))
      .collect::<Vec<BlockChainRow>>();

    let column_cnt = source_blocks.column_size();

    let mut column_vec = Vec::new();
    for column_idx in 0..column_cnt {
      let column_data = rows.iter()
        .try_fold(Vec::new(), |mut c, r| -> Result<Vec<Datum>> {
          c.push(r.get_data(column_idx)?);
          Ok(c)
        })?;

      let column_type = source_blocks.column_type(column_idx).unwrap();
      let mut column = ColumnBuilder::new_vec(column_type, column_data);

      if let Some(name) = source_blocks.column_name(column_idx) {
        column = column.set_name(name);
      }

      column_vec.push(column.build());
    }

    Ok(Block::new(column_vec, true))
  }

  fn close(&mut self) -> Result<()> {
    self.input.close()
  }
}

impl TopNExecNode {
  fn get_sort_by_block(&mut self, eval_ctx: &EvalContext, input: &Block) -> Result<Block> {
    self.sort_items.iter_mut()
      .try_fold(Block::default(), |mut b, sort_item| -> Result<Block> {
        let sort_column = sort_item.item.eval(eval_ctx, input)?;
        b.merge(sort_column)?;
        Ok(b)
      })
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

impl TopNExecNode {
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

    Ok(box TopNExecNode {
      sort_items,
      limit: plan_node.get_topn_node().get_limit(),
      input,
      executed: false
    })
  }
}

#[derive(Default)]
struct BlockChain {
  blocks: Vec<Block>,
  sort_order: Vec<bool> // true is desc, else is asc
}

impl BlockChain {
  fn blocks_slice(&self) -> &[Block] {
    &self.blocks
  }

  fn sort_order_slice(&self) -> &[bool] {
    &self.sort_order
  }

  fn iter(&self) -> impl Iterator<Item = BlockChainRow> {
    self.blocks.iter()
      .enumerate()
      .flat_map(move |x| (0..(x.1.len())).map(move |y| {
        BlockChainRow {
          block_chain: self,
          block: x.0,
          row: y
        }
      }))
  }

  fn get_data(&self, block: usize, row: usize, col: usize) -> Result<Datum> {
    self.blocks.get(block)
      .and_then(|b| b.columns.get(col))
      .and_then(|c| c.get(row))
      .ok_or_else(|| format!("No data found in block {}, row {}, column {}", block, row, col).into())
  }

  fn column_size(&self) -> usize {
    self.blocks.get(0)
      .map(|b| b.columns.len())
      .unwrap_or(0usize)
  }

  fn column_type(&self, column_idx: usize) -> Option<ColumnType> {
    self.blocks.get(0)
      .and_then(|b| b.columns.get(column_idx))
      .map(|c| c.field_type())
  }

  fn column_name(&self, column_idx: usize) -> Option<String> {
    self.blocks.get(0)
      .and_then(|b| b.columns.get(column_idx))
      .and_then(|c| c.name())
      .map(|n| n.to_string())
  }

  fn add_block(&mut self, block: Block) -> Result<()> {
    let is_same_type = self.blocks.last()
      .map(|b| b.is_same_type(&block))
      .unwrap_or(true);

    if is_same_type {
      Ok(self.blocks.push(block))
    } else {
      Err(ErrorKind::BlockTypeNotMatch("Not match".to_string()).into())
    }
  }
}


struct BlockChainRow<'a> {
  block_chain: &'a BlockChain,
  block: usize,
  row: usize
}

impl<'a> BlockChainRow<'a> {
  fn try_cmp(&self, other: &Self) -> Result<Ordering> {
    let column_size = self.block_chain.column_size();
    let sort_order_size = self.block_chain.sort_order_slice().len();
    for column_idx in 0..column_size {
      let this_data = self.get_data(column_idx)?;
      let other_data = other.get_data(column_idx)?;

      let desc: bool = self.block_chain.sort_order_slice()
        .get(column_idx)
        .map(|&x| x)
        .ok_or::<Error>(ErrorKind::IndexOutOfBound(column_idx, sort_order_size).into())?;

      let mut ord = Datum::try_cmp(&this_data, &other_data)?;

      if desc {
        ord = ord.reverse();
      }
      if ord != Ordering::Equal {
        return Ok(ord);
      }
    }

    Ok(Ordering::Equal)
  }
}

impl<'a> Ord for BlockChainRow<'a> {
  fn cmp(&self, other: &Self) -> Ordering {
    let result = self.try_cmp(other);

    match result {
      Ok(ord) => ord,
      Err(e) => {
        error!("Failed to compare block chain row, this should not happen: {}", e);
        Ordering::Equal
      }
    }
  }
}

impl<'a> PartialOrd for BlockChainRow<'a> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(&other))
  }
}

impl<'a> PartialEq<Self> for BlockChainRow<'a> {
  fn eq(&self, other: &Self) -> bool {
    self.cmp(&other) == Ordering::Equal
  }
}

impl<'a> Eq for BlockChainRow<'a> {
}

impl<'a> BlockChainRow<'a> {
  fn get_data(&self, column_idx: usize) -> Result<Datum> {
    self.block_chain.get_data(self.block, self.row, column_idx)
  }

  fn change_block_chain<'b>(&self, block_chain: &'b BlockChain) -> BlockChainRow<'b> {
    BlockChainRow {
      block_chain,
      block: self.block,
      row: self.row
    }
  }
}

struct TopNHeap<'a> {
  heap: BinaryHeap<BlockChainRow<'a>>,
  limit: Option<usize>
}

impl<'a> TopNHeap<'a> {
  fn new(capacity: usize, limit: Option<usize>) -> TopNHeap<'a> {
    let heap_capacity = match limit {
      Some(x) => cmp::min(capacity, x),
      None => capacity
    };

    TopNHeap {
      heap: BinaryHeap::with_capacity(heap_capacity),
      limit
    }
  }

  fn push(&mut self, row: BlockChainRow<'a>) -> Result<()> {
    self.heap.push(row);

    if self.limit.is_some() && self.limit.unwrap() < self.heap.len() {
      self.heap.pop();
    }

    Ok(())
  }

  fn into_sorted_vec(self) -> Vec<BlockChainRow<'a>> {
    self.heap.into_sorted_vec()
  }
}

#[cfg(test)]
mod tests {
  use std::default::Default;
  use std::vec::Vec;

  use storage::column::ColumnBuilder;
  use storage::column::vec_column_data::Datum;
  use exec::tests::MemoryBlocks;
  use exec::Block;
  use exec::ExecNode;
  use exec::ExecContext;
  use super::TopNExecNode;
  use rpc::zeus_plan::{PlanNode, PlanNodeType};
  use rpc::zeus_plan::TopNNode;
  use rpc::zeus_plan::TopNNode_SortItem;
  use rpc::zeus_expr::{Expression, ExpressionType, ColumnRef};
  use rpc::zeus_meta::ColumnType;
  use server::ServerContext;

  fn create_memory_block() -> Box<ExecNode> {
    let column1 = ColumnBuilder::new_vec(ColumnType::BOOL, Datum::vec_of(vec![true, true]))
      .set_name("a")
      .build();
    let column2 = ColumnBuilder::new_vec(ColumnType::INT64, Datum::vec_of(vec![16i64, 10000i64]))
      .set_name("b")
      .build();
    let column3 = ColumnBuilder::new_vec(
      ColumnType::STRING, Datum::vec_of(vec!["hate".to_string(), "cpp".to_string()]))
      .set_name("c")
      .build();
    let block1 = vec![column1, column2, column3];
    let block1 = Block::from(block1);

    let column1 = ColumnBuilder::new_vec(ColumnType::BOOL, Datum::vec_of(vec![false, false]))
      .set_name("a")
      .build();
    let column2 = ColumnBuilder::new_vec(ColumnType::INT64, Datum::vec_of(vec![10000i64, 12i64]))
      .set_name("b")
      .build();
    let column3 = ColumnBuilder::new_vec(
      ColumnType::STRING, Datum::vec_of(vec!["love".to_string(), "rust".to_string()]))
      .set_name("c")
      .build();
    let block2 = vec![column1, column2, column3];
    let block2 = Block::from(block2);

    box MemoryBlocks {
      blocks: vec![block1, block2],
    }
  }

  fn create_topn_plan_node() -> PlanNode {
    // create expression of a
    let expr_b = {
      let mut tmp = ColumnRef::new();
      tmp.set_name("b".to_string());

      let mut expr = Expression::new();
      expr.set_expression_type(ExpressionType::COLUMN_REF);
      expr.set_column(tmp);

      let mut sort_item = TopNNode_SortItem::new();
      sort_item.set_expr(expr);
      sort_item.set_desc(true);

      sort_item
    };


    // create expression b
    let expr_c = {
      let mut tmp = ColumnRef::new();
      tmp.set_name("c".to_string());

      let mut expr = Expression::new();
      expr.set_expression_type(ExpressionType::COLUMN_REF);
      expr.set_column(tmp);

      let mut sort_item = TopNNode_SortItem::new();
      sort_item.set_expr(expr);
      sort_item.set_desc(false);

      sort_item
    };


    let mut topn_node = TopNNode::new();
    topn_node.mut_sort_item().push(expr_b);
    topn_node.mut_sort_item().push(expr_c);

    let mut plan_node = PlanNode::new();
    plan_node.set_node_id(1);
    plan_node.set_plan_node_type(PlanNodeType::TOPN_NODE);
    plan_node.set_topn_node(topn_node);

    plan_node
  }

  #[test]
  fn test_create_topn_exec_node() {
    let plan_node = create_topn_plan_node();
    let server_context = ServerContext::default();
    let children = vec![create_memory_block()];

    assert!(TopNExecNode::new(&plan_node, &server_context, children).is_ok());
  }

  #[test]
  fn test_create_topn_exec_node_wrong_type() {
    let mut plan_node = create_topn_plan_node();
    plan_node.set_plan_node_type(PlanNodeType::SCAN_NODE);

    let server_context = ServerContext::default();
    let children = vec![create_memory_block()];

    assert!(TopNExecNode::new(&plan_node, &server_context, children).is_err());
  }

  #[test]
  fn test_create_topn_exec_node_wrong_children() {
    let plan_node = create_topn_plan_node();

    let server_context = ServerContext::default();
    let children = vec![create_memory_block(), create_memory_block()];

    assert!(TopNExecNode::new(&plan_node, &server_context, children).is_err());
  }

  #[test]
  fn test_execute_topn_exec_node() {
    let plan_node = create_topn_plan_node();
    let server_context = ServerContext::default();
    let children = vec![create_memory_block()];

    let mut topn_node = TopNExecNode::new(&plan_node, &server_context, children).unwrap();
    let mut exec_context = ExecContext::default();

    assert!(topn_node.open(&mut exec_context).is_ok());

    // First block
    let ret = topn_node.next();
    assert!(ret.is_ok());

    let ret = ret.unwrap();
    assert_eq!(4, ret.len());
    assert_eq!(3, ret.columns.len());
    assert_eq!(vec![Datum::Bool(true), Datum::Bool(false), Datum::Bool(true), Datum::Bool(false)],
               ret.columns[0].iter().collect::<Vec<Datum>>());
    assert_eq!(Some("a"), ret.columns[0].name());
    assert_eq!(vec![Datum::Int64(10000i64), Datum::Int64(10000i64),
                    Datum::Int64(16i64), Datum::Int64(12i64)],
               ret.columns[1].iter().collect::<Vec<Datum>>());
    assert_eq!(Some("b"), ret.columns[1].name());
    assert_eq!(vec![Datum::UTF8("cpp".to_string()), Datum::UTF8("love".to_string()),
                    Datum::UTF8("hate".to_string()), Datum::UTF8("rust".to_string())],
               ret.columns[2].iter().collect::<Vec<Datum>>());
    assert_eq!(Some("c"), ret.columns[2].name());
    assert!(ret.eof);

    // Second block
    let ret = topn_node.next();
    assert!(ret.is_err());
  }
}
