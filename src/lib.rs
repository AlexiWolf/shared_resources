//! This crate provides a shared resources container which is thread-safe, and lock-free.
//!
//! The [`Resources`] struct, is a container of [`Resource`] objects. Resources are inserted at 
//! run-time. The container stores up to 1 instance of each type.  Stored resources can be accessed
//! by the rest of the system through an immutable reference. Borrowing rules are checked at 
//! run-time.
//!
//! Thread-safe access is provided by the [`ResourcesSync`] struct.  It's similar to the 
//! [`Resources`] struct, except it only allows access to thread-safe resources, and can, itself, 
//! be sent to other threads.
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
//!
//! ## Multi-threaded Use
//!
//! The default [`Resources`] container is not [`Send`], or [`Sync`], so it *cannot* be sent 
//! between threads.  Instead, you must create a [`ResourcesSync`], which is a thread-safe handle 
//! to the resources container that can be sent to other threads.  Any resource accessed through 
//! the handle must, itself, be [`Send`] / [`Sync`].
//!
//! ```
//! # use shared_resources::*;
//! #
//! # fn main() -> Result<(), AccessError> {
//! #
//! # struct ExampleResource(&'static str);
//! # let resource = ExampleResource("Hello, World!");
//! # let mut resources = Resources::default();
//! # resources.insert(resource);
//! #
//! // Create a ResourcesSync from the Resources container.
//! let resources_sync = resources.sync();
//!
//! std::thread::scope(|scope| {
//!     scope
//!         .spawn(|| {
//!             let mut resource = resources_sync.get_mut::<ExampleResource>().unwrap();
//!             resource.0 = "Goodbye, World!";
//!         })
//!         .join()
//!         .unwrap();
//!     scope
//!         .spawn(|| {
//!             let resource = resources_sync.get::<ExampleResource>().unwrap();
//!             assert_eq!(resource.0, "Goodbye, World!");
//!         })
//!         .join()
//!         .unwrap();
//! });
//! #
//! # Ok(())
//! # }
//!
//! ```

mod errors;
pub use errors::*;
mod resource;
pub use resource::*;
mod store;
pub use store::*;
