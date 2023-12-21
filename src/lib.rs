use std::{collections::HashMap, any::TypeId};

use atomic_refcell::*;
use downcast_rs::{Downcast, impl_downcast};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccessError {
    NoSuchResource,
}

pub trait Resource: Downcast + 'static {}

impl<T> Resource for T where T: 'static {}

impl_downcast!(Resource);

struct ResourceCell {

}

#[derive(Default)]
pub struct Resources {
    inner: UnsafeResources,
}

impl Resources {
    pub fn insert<T: Resource>(&mut self, resource: T) {
        let type_id = resource.type_id();

        // Safety: `Resources` is `!Send` / `!Sync`, so it is not possible for it to modify the
        // `UnsafeResources` store on another thread.
        unsafe { self.inner.insert(type_id, Box::from(resource)) }
    }

    pub fn get<T: Resource>(&self) -> Result<AtomicRefMut<T>, AccessError> {
        Err(AccessError::NoSuchResource)
    }

    pub fn get_mut<T: Resource>(&self) -> Result<AtomicRefMut<T>, AccessError> {
        Err(AccessError::NoSuchResource)
    }
}

#[derive(Default)]
struct UnsafeResources {
    resources: HashMap<TypeId, ResourceCell>,
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
}
