/// A marker trait coupling the output of the future with the tree future
pub trait TreeFutureOutput {
    /// trait method to help return default for item type, we avoid using std::default::Default to avoid conflicts
    fn return_default() -> Self;
}

impl TreeFutureOutput for u8 {
    fn return_default() -> Self {
        0
    }
}
impl TreeFutureOutput for i8 {
    fn return_default() -> Self {
        0
    }
}

impl TreeFutureOutput for i16 {
    fn return_default() -> Self {
        0
    }
}

impl TreeFutureOutput for i32 {
    fn return_default() -> Self {
        0
    }
}

impl TreeFutureOutput for i64 {
    fn return_default() -> Self {
        0
    }
}

impl TreeFutureOutput for i128 {
    fn return_default() -> Self {
        0
    }
}

impl TreeFutureOutput for isize {
    fn return_default() -> Self {
        0
    }
}

impl TreeFutureOutput for u16 {
    fn return_default() -> Self {
        0
    }
}

impl TreeFutureOutput for u32 {
    fn return_default() -> Self {
        0
    }
}

impl TreeFutureOutput for u64 {
    fn return_default() -> Self {
        0
    }
}

impl TreeFutureOutput for u128 {
    fn return_default() -> Self {
        0
    }
}

impl TreeFutureOutput for usize {
    fn return_default() -> Self {
        0
    }
}
impl TreeFutureOutput for String {
    fn return_default() -> Self {
        String::new()
    }
}

impl TreeFutureOutput for &str {
    fn return_default() -> Self {
        ""
    }
}
