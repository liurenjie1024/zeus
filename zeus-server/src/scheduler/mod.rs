use futures::BoxFuture;
use futures::IntoFuture;

use util::error::Result;

pub trait Executor: Sync + Send {
    fn submit<F, R>(&self, f: F) -> BoxFuture<R::Item, R::Error>
    where
        F: FnOnce() -> R + Send + 'static,
        R: IntoFuture + 'static,
        R::Future: Send + 'static,
        R::Item: Send + 'static,
        R::Error: Send + 'static;

    fn shutdown() -> Result<usize>;
}