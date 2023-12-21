use std::{any::TypeId, collections::HashMap};

use atomic_refcell::*;

use crate::*;

/// Provides a shared [`Resource`] container.
#[derive(Default)]
pub struct Resources {
    inner: UnsafeResources,
}

impl Resources {
    /// Inserts a [`Resource`] of type `T` into the store.
    ///
    /// If an instance of `T` already exists, it is quietly replaced with the new instance.
    ///
    /// Call [`Resources::remove()`] first, if you want to retrieve the existing instance.
    pub fn insert<T: Resource>(&mut self, resource: T) {
        // Safety: `Resources` is `!Send` / `!Sync`, so it is not possible for it to modify the
        // `UnsafeResources` store from another thread.
        unsafe { self.inner.insert(Box::from(resource)) }
    }

    /// Removes the instance of type `T` from the store, if it exists.
    pub fn remove<T: Resource>(&mut self) -> Option<T> {
        // Safety: `Resources` is `!Send` / `!Sync`, so it is not possible for it to modify the
        // `UnsafeResources` store from another thread.
        let type_id = TypeId::of::<T>();
        unsafe {
            let resource = self.inner.remove(&type_id)?.downcast::<T>().ok()?;
            Some(*resource)
        }
    }

    /// Returns an immutable reference to the stored `T`, if it exists.
    ///
    /// # Errors
    ///
    /// - Returns [`AccessError::NoSuchResource'] if an instance of type `T` does not exist.
    /// - Returns [`AccessError::AlreadyBorrowed'] if there is an existing mutable reference to
    ///   `T`.
    pub fn get<T: Resource>(&self) -> Result<AtomicRef<T>, AccessError> {
        // Safety: `Resources` is `!Send` / `!Sync`, so it is not possible for it to access the
        // `UnsafeResources` store from another thread.
        let type_id = TypeId::of::<T>();
        match unsafe { self.inner.get(&type_id) } {
            Some(cell) => Ok(cell.try_borrow()?),
            None => Err(AccessError::NoSuchResource),
        }
    }

    /// Returns an immutable reference to the stored `T`, if it exists.
    ///
    /// # Errors
    ///
    /// - Returns [`AccessError::NoSuchResource'] if an instance of type `T` does not exist.
    /// - Returns [`AccessError::AlreadyBorrowed'] if there is an existing reference to `T`.
    pub fn get_mut<T: Resource>(&self) -> Result<AtomicRefMut<T>, AccessError> {
        // Safety: `Resources` is `!Send` / `!Sync`, so it is not possible for it to modify the
        // `UnsafeResources` store on another thread.
        let type_id = TypeId::of::<T>();
        match unsafe { self.inner.get(&type_id) } {
            Some(cell) => Ok(cell.try_borrow_mut()?),
            None => Err(AccessError::NoSuchResource),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestResource(&'static str);

    #[test]
    fn should_access_stored_resources() {
        let mut resources = Resources::default();
        resources.insert(TestResource("Hello, World!"));

        {
            let mut resource = resources.get_mut::<TestResource>().unwrap();
            resource.0 = "Goodbye, World!";
        }

        let resource = resources.get::<TestResource>().unwrap();
        assert_eq!(resource.0, "Goodbye, World!");
    }

    #[test]
    fn should_insert_and_remove_resources() {
        let mut resources = Resources::default();
        resources.insert(TestResource("Hello, World!"));

        {
            let resource = resources.get::<TestResource>().unwrap();
            assert_eq!(resource.0, "Hello, World!");
        }

        assert!(resources.remove::<TestResource>().is_some());
    }
}

/// Provides a [`Resource`] container which does run-time borrow-checking, but *does not* ensure
/// [`!Send`] / [`!Sync`] types are not accessed across threads.
#[derive(Default)]
struct UnsafeResources {
    resources: HashMap<TypeId, ResourceCell>,
}

impl UnsafeResources {
    /// # Safety
    ///
    /// [`!Send`] types cannot be inserted from any thread that doesn't own the resource store.
    pub unsafe fn insert(&mut self, resource: Box<dyn Resource>) {
        let type_id = resource.type_id();
        self.resources.insert(type_id, ResourceCell::new(resource));
    }

    /// # Safety
    ///
    /// [`!Send`] types cannot be removed from any thread that doesn't own the resource store.
    pub unsafe fn remove(&mut self, type_id: &TypeId) -> Option<Box<dyn Resource>> {
        self.resources.remove(type_id).map(|cell| cell.into_inner())
    }

    /// # Safety
    ///
    /// [`!Send`] / [`!Sync`] types cannot be accessed from any thread that doesn't own the
    /// resource store.
    pub unsafe fn get(&self, type_id: &TypeId) -> Option<&ResourceCell> {
        self.resources.get(type_id)
    }
}
