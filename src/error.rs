/// Trait for generating objects as errors
pub trait TreeFutureError: std::fmt::Debug {}

impl TreeFutureError for futures::stream::Aborted {}

impl TreeFutureError for (dyn std::error::Error + Send + 'static) {}
