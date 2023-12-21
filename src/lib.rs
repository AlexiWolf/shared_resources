use std::{collections::HashMap, any::TypeId};

use atomic_refcell::*;
use downcast_rs::{Downcast, impl_downcast};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccessError {

}

pub trait Resource: Downcast + 'static {}

impl<T> Resource for T where T: 'static {}

impl_downcast!(Resource);

struct ResourceCell {

}

#[derive(Default)]
pub struct Resources {

}

impl Resources {
    pub fn insert<T: Resource>(&mut self, resource: T) {
        
    }

    pub fn get<T: Resource>(&self) -> Result<AtomicRefMut<T>, AccessError> {
        todo!()
    }

    pub fn get_mut<T: Resource>(&self) -> Result<AtomicRefMut<T>, AccessError> {
        todo!()
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
