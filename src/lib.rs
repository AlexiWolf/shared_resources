#[derive(Default)]
pub struct Resources {

}

impl Resources {
    pub fn insert<T: Resource>(&mut self) {

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
