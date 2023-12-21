use downcast_rs::{Downcast, impl_downcast};

pub trait Resource: Downcast + 'static {}

impl<T> Resource for T where T: 'static {}

impl_downcast!(Resource);

#[derive(Default)]
pub struct Resources {

}

impl Resources {
    pub fn insert<T: Resource>(&mut self, resource: T) {
        
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
    }
}
