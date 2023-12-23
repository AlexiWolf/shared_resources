//! Provides a shared resource container.
//!
//! # Examples
//!
//! ```
//! # use shared_resources::*;
//! #
//! # fn main() -> Result<(), AccessError> {
//! #
//! // Create a new resource.  
//! struct ExampleResource(&'static str);
//! let resource = ExampleResource("Hello, World!");
//!
//! // Create a new resource store.
//! let mut resources = Resources::default();
//!
//! // Add a resource to the store.
//! // The resource store will take ownership over the resource.
//! resources.insert(resource);
//!
//! // An immutable borrow.
//! {
//!     // Resources::get() will return an AccessError if:
//!     // - The requested type does not exist in the store.
//!     // - There's an existing mutable borrow.
//!     let resource = resources.get::<ExampleResource>()?;
//!     assert_eq!(resource.0, "Hello, World!");
//! }
//!
//! // A mutable borrow.
//! {
//!     // Resources::get_mut() will return an AccessError if:
//!     // - The requested type does not exist in the store.
//!     // - There's an existing mutable, or immutable borrow.
//!     let mut resource = resources.get_mut::<ExampleResource>()?;
//!     resource.0 = "Goodbye, World!";
//! }
//!
//! // Remove the resource from the store.
//! // The resource is returned to the caller.
//! let resource = resources.remove::<ExampleResource>().unwrap();
//! assert_eq!(resource.0, "Goodbye, World!");
//! #
//! # Ok(())
//! # }
//! ```

mod errors;
pub use errors::*;
mod resource;
pub use resource::*;
mod store;
pub use store::*;
