use std::{collections::BTreeMap, future::Future};

use futures::FutureExt;

use crate::{error::TreeFutureError, tree_future_output::TreeFutureOutput, BoxedResultFuture};

/// Create a type which can hold multiple futures and return them in order of completion using a BTreeMap
pub struct TreeFuture<E, T> {
    futures: BTreeMap<E, BoxedResultFuture<T>>,
}

impl<E, T> TreeFuture<E, T>
where
    T: TreeFutureOutput + 'static,
    E: Ord + std::hash::Hash,
{
    pub fn new() -> Self {
        TreeFuture {
            futures: BTreeMap::new(),
        }
    }
    /// insert a future into the collection mapping it's value to be `Ok(T)`
    pub fn insert_future(&mut self, task_id: E, future: impl Future<Output = T> + Send + 'static) {
        self.futures
            .insert(task_id, future.map(|res| Ok(res)).boxed());
    }

    /// Insert a future into the collection which can be abortable
    /// Useful in scenarios when you wish to cancel some future before it completes
    ///
    /// **Note:** Requires `T` to be `'static` if you wish to use this method
    ///
    /// **Note:** This method utilizes the `abortable` method from the futures crate which can be costly in terms of performance
    pub fn insert_abortable_future(
        &mut self,
        task_id: E,
        future: impl Future<Output = T> + Send + 'static,
    ) -> futures::stream::AbortHandle {
        let (abortable_future, abort_handle) = futures::future::abortable(future);
        self.futures.insert(
            task_id,
            abortable_future
                .map(|res| res.map_err(|err| Box::new(err) as Box<dyn TreeFutureError + 'static>))
                .boxed(),
        );

        abort_handle
    }

    /// drop a future instantally, removing it from collection
    pub fn remove_future(&mut self, task_id: E) -> Option<BoxedResultFuture<T>> {
        self.futures.remove(&task_id)
    }
}

impl<E, T> std::ops::Deref for TreeFuture<E, T>
where
    T: TreeFutureOutput,
    E: Ord,
{
    type Target = BTreeMap<E, BoxedResultFuture<T>>;
    fn deref(&self) -> &Self::Target {
        &self.futures
    }
}

impl<E, T> std::ops::DerefMut for TreeFuture<E, T>
where
    T: TreeFutureOutput,
    E: Ord,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.futures
    }
}

impl<E, T> futures::stream::Stream for TreeFuture<E, T>
where
    T: TreeFutureOutput,
    E: Ord,
{
    /// Return Key as starting item and value as the result of the future polled
    type Item = (E, Result<T, Box<dyn TreeFutureError>>);
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        // here we pop the future in order of btree priority and poll it
        if let Some((task_id, mut future)) = self.futures.pop_first() {
            match future.as_mut().poll(cx) {
                std::task::Poll::Ready(val) => {
                    let result = (task_id, val);
                    std::task::Poll::Ready(Some(result))
                }
                std::task::Poll::Pending => std::task::Poll::Pending,
            }
        } else {
            std::task::Poll::Ready(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream::StreamExt;

    #[test]
    fn test_basic_insertion() {
        let async_block = async move {
            let mut tree_future = TreeFuture::new();

            let _ = tree_future.insert_future(1, futures::future::ready(10));
            let _ = tree_future.insert_future(2, futures::future::ready(10));
            let _ = tree_future.insert_future(3, futures::future::ready(10));

            let (id, val) = tree_future.next().await.unwrap();
            assert_eq!(id, 1);
            assert_eq!(val.unwrap(), 10);

            let (id, val) = tree_future.next().await.unwrap();
            assert_eq!(id, 2);
            assert_eq!(val.unwrap(), 10);

            let (id, val) = tree_future.next().await.unwrap();
            assert_eq!(id, 3);
            assert_eq!(val.unwrap(), 10);
        };
        let _ = futures::executor::block_on(async_block);
    }

    #[test]
    fn test_cancellation() {
        let async_block = async move {
            let mut tree_future = TreeFuture::new();

            let _ = tree_future.insert_future(1, futures::future::ready(10));
            let abort_handle = tree_future.insert_abortable_future(2, futures::future::ready(10));
            let _ = tree_future.insert_future(3, futures::future::ready(10));

            let (id, val) = tree_future.next().await.unwrap();
            assert_eq!(id, 1);
            assert_eq!(val.unwrap(), 10);

            let _ = abort_handle.abort();
            let (id, val) = tree_future.next().await.unwrap();
            assert_eq!(id, 2);
            assert!(val.is_err());

            let (id, val) = tree_future.next().await.unwrap();
            assert_eq!(id, 3);
            assert_eq!(val.unwrap(), 10);
        };
        let _ = futures::executor::block_on(async_block);
    }

    #[test]
    fn test_removal() {
        let async_block = async move {
            let mut tree_future = TreeFuture::new();

            let _ = tree_future.insert_future(1, futures::future::ready(10));
            let _ = tree_future.insert_future(2, futures::future::ready(10));
            let _ = tree_future.insert_future(3, futures::future::ready(10));

            let (id, val) = tree_future.next().await.unwrap();
            assert_eq!(id, 1);
            assert_eq!(val.unwrap(), 10);

            let removed_future = tree_future.remove_future(2).unwrap();
            let future_value = removed_future.await.unwrap();
            assert_eq!(future_value, 10);

            let (id, val) = tree_future.next().await.unwrap();
            assert_eq!(id, 3);
            assert_eq!(val.unwrap(), 10);
        };
        let _ = futures::executor::block_on(async_block);
    }
}
