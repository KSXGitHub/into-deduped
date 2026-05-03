//! Utility methods and functions that take an owned `Vec` and return a deduplicated owned `Vec`.

mod functions;
mod methods;

pub use functions::*;
pub use methods::*;

mod sealed {
    pub trait IsVec: Sized {}
    impl<Item> IsVec for Vec<Item> {}
}
