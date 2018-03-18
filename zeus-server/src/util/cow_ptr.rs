use std::sync::Arc;
use std::boxed::Box;
use std::any::Any;
use std::ops::Deref;
use std::ops::DerefMut;

pub trait ToBoxedOwned {
  fn to_boxed_owned(&self) -> Box<Any>;
}

pub enum CowPtr<T>
where T: ToBoxedOwned + 'static + ?Sized
{
  Borrowed(Arc<T>),
  Owned(Box<T>),
}

unsafe impl<T: Send + ToBoxedOwned + 'static + ?Sized> Send for CowPtr<T> {}

impl<T: ToBoxedOwned + 'static + ?Sized> Deref for CowPtr<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    match *self {
      CowPtr::Borrowed(ref arc) => arc.deref(),
      CowPtr::Owned(ref b) => b.deref(),
    }
  }
}

impl<T: ToBoxedOwned + 'static> CowPtr<T> {
  pub fn to_owned(self) -> CowPtr<T> {
    match self {
      CowPtr::Borrowed(arc) => {
        let owner = arc.to_boxed_owned().downcast::<T>().unwrap();
        CowPtr::Owned(owner)
      },
      CowPtr::Owned(..) => self,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::ToBoxedOwned;
  use super::CowPtr;
  use std::sync::Arc;
  use std::any::Any;
  use std::fmt::Debug;

  #[derive(PartialEq, Eq, Debug, Copy, Clone)]
  struct TestStruct {
    x: i32,
  }

  impl ToBoxedOwned for TestStruct {
    fn to_boxed_owned(&self) -> Box<Any> {
      Box::new(TestStruct {
        x: self.x,
      })
    }
  }

  #[test]
  fn test_to_owned() {
    let arc = Arc::new(TestStruct {
      x: 56,
    });

    let mut cow_ptr = CowPtr::Borrowed(arc);

    cow_ptr = cow_ptr.to_owned();

    match cow_ptr {
      CowPtr::Borrowed(..) => assert!(false, "Borrowed should not appear here!"),
      CowPtr::Owned(ref ptr) => assert_eq!(56, ptr.x),
    }

    cow_ptr = cow_ptr.to_owned();

    match cow_ptr {
      CowPtr::Borrowed(..) => assert!(false, "Borrowed should not appear here!"),
      CowPtr::Owned(ref ptr) => assert_eq!(56, ptr.x),
    }
  }

  #[test]
  fn test_borrowed_ref() {
    let cow_ptr = CowPtr::Borrowed(Arc::new(TestStruct {
      x: 56,
    }));

    assert_eq!(56, cow_ptr.x);
  }

  #[test]
  fn test_owned_ref() {
    let cow_ptr = CowPtr::Owned(Box::new(TestStruct {
      x: 56,
    }));

    assert_eq!(56, cow_ptr.x);
  }
}
