mod resource;
pub use resource::*;
mod store;
pub use store::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccessError {
    NoSuchResource,
    AlreadyBorrowed,
}
