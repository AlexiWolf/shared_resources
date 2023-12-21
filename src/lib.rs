use std::{collections::HashMap, any::TypeId};

use atomic_refcell::*;
use downcast_rs::{Downcast, impl_downcast};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccessError {
    NoSuchResource,
    AlreadyBorrowed,
}

pub trait Resource: Downcast + 'static {}

impl<T> Resource for T where T: 'static {}

impl_downcast!(Resource);

struct ResourceCell {
    inner: AtomicRefCell<Box<dyn Resource>>,
}

impl ResourceCell {
    fn new(resource: Box<dyn Resource>) -> Self {
        Self {
            inner: AtomicRefCell::new(resource),
        }
    }

    fn into_inner(self) -> Box<dyn Resource> {
        self.inner.into_inner()
    }

    pub fn try_borrow<T: Resource>(&self) -> Result<AtomicRef<T>, AccessError> {
        match self.inner.try_borrow() {
            Ok(borrow) => Ok(AtomicRef::map(borrow, |inner| inner.downcast_ref().unwrap())),
            Err(_) => Err(AccessError::AlreadyBorrowed),
        }
    }

    pub fn try_borrow_mut<T: Resource>(&self) -> Result<AtomicRefMut<T>, AccessError> {
        match self.inner.try_borrow_mut() {
            Ok(borrow) => Ok(AtomicRefMut::map(borrow, |inner| inner.downcast_mut().unwrap())),
            Err(_) => Err(AccessError::AlreadyBorrowed),
        }
    }
}

#[derive(Default)]
pub struct Resources {
    inner: UnsafeResources,
}

impl Resources {
    pub fn insert<T: Resource>(&mut self, resource: T) {
        // Safety: `Resources` is `!Send` / `!Sync`, so it is not possible for it to modify the
        // `UnsafeResources` store from another thread.
        unsafe { self.inner.insert(Box::from(resource)) }
    }

    pub fn remove<T: Resource>(&mut self) -> Option<T> {
        // Safety: `Resources` is `!Send` / `!Sync`, so it is not possible for it to modify the
        // `UnsafeResources` store from another thread.
        let type_id = TypeId::of::<T>();
        unsafe {
            let resource = self
                .inner
                .remove(&type_id)?
                .downcast::<T>()
                .ok()?;
            Some(*resource)
        }
    }

    pub fn get<T: Resource>(&self) -> Result<AtomicRef<T>, AccessError> {
        // Safety: `Resources` is `!Send` / `!Sync`, so it is not possible for it to access the
        // `UnsafeResources` store from another thread.
        let type_id = TypeId::of::<T>();
        match unsafe { self.inner.get(&type_id) } {
            Some(cell) => Ok(
                cell
                    .try_borrow()?
            ),
            None => Err(AccessError::NoSuchResource),
        }
    }

    pub fn get_mut<T: Resource>(&self) -> Result<AtomicRefMut<T>, AccessError> {
        // Safety: `Resources` is `!Send` / `!Sync`, so it is not possible for it to modify the
        // `UnsafeResources` store on another thread.
        let type_id = TypeId::of::<T>();
        match unsafe { self.inner.get(&type_id) } {
            Some(cell) => Ok(
                cell
                    .try_borrow_mut()?
            ),
            None => Err(AccessError::NoSuchResource),
        }
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
