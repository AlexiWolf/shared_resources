#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccessError {
    NoSuchResource,
    AlreadyBorrowed,
}
