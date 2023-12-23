/// Represents an error that occurred when accessing the [`Resources`](crate::Resources) store.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccessError {
    /// There is no [`Resource`](crate::Resource) of the requested type in the store.
    NoSuchResource,

    /// The requested [`Resource`](crate::Resource) has already been borrowed.
    AlreadyBorrowed,
}

impl std::fmt::Display for AccessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessError::NoSuchResource => write!(f, "There is no resource of the requested type."),
            AccessError::AlreadyBorrowed => {
                write!(f, "The requested resource is already borrowed.")
            }
        }
    }
}
