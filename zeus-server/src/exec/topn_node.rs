use std::vec::Vec;
use std::cmp;
use std::collections::BinaryHeap;
use std::cmp::Ord;
use std::cmp::Ordering;

use rpc::zeus_plan::PlanNodeType;
use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::TopNNode_SortItem;
use super::ServerContext;
use super::ExecNode;
use super::ExecContext;
use super::Block;
use super::expression::Expr;
use super::expression::EvalContext;
use util::errors::*;

struct SortItem {
  _item: Expr,
  _desc: bool
}

pub struct TopNNode {
  _sort_items: Vec<SortItem>,
  _limit: i32,
  input: Box<ExecNode>,
  executed: bool
}

impl ExecNode for TopNNode {
  fn open(&mut self, context: &mut ExecContext) -> Result<()> {
    self.input.open(&mut context)
  }

  fn next(&mut self) -> Result<Block> {
    if self.executed {
      return Ok(Block::default())
    }

    let eval_context = EvalContext::default();

    let mut source_blocks = Vec::new();
    loop {
      let block = self.input.next()?;
      let eof = block.eof;

      source_blocks.push(block);
      if eof {
        break;
      }
    }

    let sort_item_blocks = source_blocks.iter()
      .try_fold(Vec::new(), |mut blocks, b| -> Result<Block> {
        blocks.push(self.get_sort_by_blocks(eval_context, b)?);
        Ok(blocks)
      })?;

    let sort_item_blocks = BlockChain {
      blocks: sort_item_blocks,
      desc: self._sort_items.iter().map(|x| x._desc).collect()
    };

    let row_count = source_blocks.iter()
      .map(|b| b.len())
      .fold(0usize, |acc, x| acc + x);

    let limit = match self._limit {
      x if x <=0 => None,
      y => Some(y as usize)
    };

    let mut heap = TopNHeap::new(row_count, limit);

    sort_item_blocks.iter()
      .for_each(|r| heap.push(r));


  }

  fn close(&mut self) -> Result<()> {
    self.input.close()
  }
}

impl TopNNode {
  fn get_sort_by_blocks(&mut self, eval_ctx: &EvalContext, input: &Block) -> Result<Block> {
    self._sort_items.iter()
      .try_fold(Block::default(), |mut b, sort_item| -> Result<Block> {
        let sort_block = sort_item._item.eval(eval_ctx, input)?;
        b.merge(sort_block);
        Ok(b)
      })
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

struct BlockChain {
  blocks: Vec<Block>,
  desc: Vec<bool>
}

impl BlockChain {
  fn iter(&self) -> impl Iterator<Item = BlockChainRow> {
    self.blocks.iter()
      .enumerate()
      .flat_map(|x| (0..(x.1.len())).map(|y| {
        BlockChainRow {
          block_chain: &self,
          block: x.0,
          row: y
        }
      }))
  }
}


struct BlockChainRow<'a> {
  block_chain: &'a BlockChain,
  block: usize,
  row: usize
}

impl<'a> Ord for BlockChainRow<'a> {
  fn cmp(&self, other: &Self) -> Ordering {
    unimplemented!()
  }
}

impl<'a> PartialOrd for BlockChainRow<'a> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<'a> PartialEq<Self> for BlockChainRow<'a> {
  fn eq(&self, other: &Self) -> bool {
    unimplemented!()
  }
}

impl<'a> Eq for BlockChainRow<'a> {
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

  fn push(&mut self, row: BlockChainRow) -> Result<()> {
    unimplemented!()
  }

  fn into_sorted_vec(self) -> Vec<BlockChainRow> {
    unimplemented!()
  }
}

