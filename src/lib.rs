use atomic_refcell::*;
use downcast_rs::{Downcast, impl_downcast};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccessError {

}

pub trait Resource: Downcast + 'static {}

impl<T> Resource for T where T: 'static {}

impl_downcast!(Resource);

#[derive(Default)]
pub struct Resources {

}

impl Resources {
    pub fn insert<T: Resource>(&mut self, resource: T) {
        
    }

    pub fn get_mut<T: Resource>(&self) -> Result<AtomicRefMut<T>, AccessError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestResource(&'static str);

    #[test]
    fn it_works() {
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
