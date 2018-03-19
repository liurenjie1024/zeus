use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicUsize;
use std::boxed::Box;
use std::ptr;
use std::iter::Iterator;
use std::iter::IntoIterator;
use std::marker::Send;
use std::marker::Sync;
use std::marker::PhantomData;

struct ListNode<T> {
  value: T,
  next: AtomicPtr<ListNode<T>>,
}

impl<T> ListNode<T> {
  pub fn new(t: T) -> ListNode<T> {
    ListNode {
      value: t,
      next: AtomicPtr::new(ptr::null_mut()),
    }
  }
}

pub struct ListIter<'a, T: 'a> {
  cur_ptr: *const ListNode<T>,
  marker: PhantomData<&'a ListNode<T>>,
}

impl<'a, T: 'a> Iterator for ListIter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.cur_ptr.is_null() {
      None
    } else {
      let cur_node = unsafe { &*self.cur_ptr };

      self.cur_ptr = cur_node.next.load(Ordering::SeqCst);
      Some(&cur_node.value)
    }
  }
}

// A lock free, single writer, multi reader list. It doesn't support delete.
pub struct ConcurrentList<T> {
  root: AtomicPtr<ListNode<T>>,
  end: AtomicPtr<ListNode<T>>,
  size: AtomicUsize,
}

impl<T> ConcurrentList<T> {
  pub fn new() -> ConcurrentList<T> {
    ConcurrentList {
      root: AtomicPtr::new(ptr::null_mut()),
      end: AtomicPtr::new(ptr::null_mut()),
      size: AtomicUsize::new(0),
    }
  }

  pub fn append(
    &self,
    t: T,
  )
  {
    let new_node = Box::into_raw(Box::new(ListNode::new(t)));

    unsafe {
      let end_ptr = self.end.load(Ordering::SeqCst);
      if !end_ptr.is_null() {
        let end_node = &mut *end_ptr;
        end_node.next.store(new_node, Ordering::SeqCst);
        self.end.store(new_node, Ordering::SeqCst);
      } else {
        self.end.store(new_node, Ordering::SeqCst);
        self.root.store(new_node, Ordering::SeqCst);
      }
    }

    self.size.store(self.size() + 1, Ordering::SeqCst)
  }

  pub fn size(&self) -> usize {
    self.size.load(Ordering::SeqCst)
  }
}

impl<'a, T> IntoIterator for &'a ConcurrentList<T> {
  type Item = &'a T;
  type IntoIter = ListIter<'a, T>;

  fn into_iter(self) -> ListIter<'a, T> {
    ListIter {
      cur_ptr: self.root.load(Ordering::SeqCst),
      marker: PhantomData,
    }
  }
}

impl<T> Drop for ConcurrentList<T> {
  fn drop(&mut self) {
    let mut cur_ptr = self.root.load(Ordering::SeqCst);
    self.root.store(ptr::null_mut(), Ordering::SeqCst);
    self.end.store(ptr::null_mut(), Ordering::SeqCst);
    self.size.store(0, Ordering::SeqCst);

    while !cur_ptr.is_null() {
      let next_ptr = unsafe { (&*cur_ptr).next.load(Ordering::SeqCst) };

      let _ = unsafe { Box::from_raw(cur_ptr) };

      cur_ptr = next_ptr;
    }
  }
}

unsafe impl<T: Send + Sync> Send for ConcurrentList<T> {}
unsafe impl<T: Send + Sync> Sync for ConcurrentList<T> {}

#[cfg(test)]
mod tests {
  use std::cell::Cell;

  use super::ConcurrentList;

  struct CountDroppable<'a> {
    cell: &'a Cell<i32>,
  }

  impl<'a> Drop for CountDroppable<'a> {
    fn drop(&mut self) {
      self.cell.set(self.cell.get() + 1);
    }
  }

  #[test]
  fn test_append() {
    let list: ConcurrentList<i32> = ConcurrentList::new();

    for x in 1..5 {
      list.append(x);
    }

    assert_eq!(4, list.size());
  }

  #[test]
  fn test_iter() {
    let list: ConcurrentList<i32> = ConcurrentList::new();

    for x in 1..5 {
      list.append(x);
    }

    assert_eq!(4, list.size());

    let mut iter = (&list).into_iter();

    assert_eq!(Some(&1), iter.next());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&3), iter.next());
    assert_eq!(Some(&4), iter.next());
  }

  #[test]
  fn test_drop() {
    let count = Cell::new(0);
    {
      let list: ConcurrentList<CountDroppable> = ConcurrentList::new();

      for _ in 1..5 {
        list.append(CountDroppable {
          cell: &count,
        });
      }
    }

    assert_eq!(4, count.get());
  }
}
