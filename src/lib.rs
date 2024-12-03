/*!
 A way to manage multiple futures and their execution processing & results in a tree like structure.

 Below is a example of how to use the `TreeFuture` struct to manage multiple futures and their results.
```rust
    use futures::stream::StreamExt;
    use collections_futures::TreeFuture;

    let mut tree_future = TreeFuture::new();
    let some_async_block = async move {
        // insert some futures which will return value upon completion
        tree_future.insert_future(1, async { 10 });
        tree_future.insert_future(2, async { 20 });

        let mut sum = 0;
        while let Some((task_id, result)) = tree_future.next().await {
            println!("Task ID: {}, Result: {:?}", task_id, result);
            sum += result.unwrap();
        }
        sum
    };
    assert_eq!(futures::executor::block_on(some_async_block), 30);
```

  Or if you wish to utilize abort handle for a specific future within this collection which you already supplied you may do so like this, during time of initialization
```rust
    use futures::stream::StreamExt;
    use collections_futures::TreeFuture;

    let mut tree_future = TreeFuture::new();

    let async_block = async move {
        let abort_handle = tree_future.insert_abortable_future(1, async { 30 });

        abort_handle.abort(); // abort the future with task_id 1

        // returns (i32, Err(futures::stream::Aborted)) tuple
        let (task_id, future_err) = tree_future.next().await.unwrap();

        future_err
    };

    assert!(futures::executor::block_on(async_block).is_err());
```

*/

pub(crate) mod error;
pub(crate) mod tree_future;
pub(crate) mod tree_future_local;
pub(crate) mod tree_future_output;

use std::future::Future;

pub use error::TreeFutureError;
pub use tree_future::TreeFuture;
pub use tree_future_local::TreeFutureLocal;
pub use tree_future_output::TreeFutureOutput;

//TODO: currently lifetime issues doesn't shows up, but need to prepare for it in future with different lifetimes!
/// Boxed future type which can hold any future and maps them to returns a `Result<T, Box<dyn TreeFutureError>>`
pub type BoxedResultFuture<T> =
    std::pin::Pin<Box<dyn Future<Output = Result<T, Box<dyn TreeFutureError>>> + Send + 'static>>;

pub type BoxedResultFutureLocal<T> =
    std::pin::Pin<Box<dyn Future<Output = Result<T, Box<dyn TreeFutureError>>>>>;
