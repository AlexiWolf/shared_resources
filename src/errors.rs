/// Represents an error that occurred when accessing the [`Resources`](crate::Resources) store.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccessError {
    /// There is no [`Resource`](crate::Resource) of the requested type in the store.
    NoSuchResource,

    /// The requested [`Resource`](crate::Resource) has already been borrowed.
    AlreadyBorrowed,
}
