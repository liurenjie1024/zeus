use std::sync::Arc;

trait ToBoxedOwned {
    fn to_boxed_owned(&self) -> Box<Self>;
}

pub enum CowPtr<T> {
    Borrowed(Arc<T>),
    Owned(Box<T>)
}
