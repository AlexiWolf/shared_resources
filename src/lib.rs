#[derive(Default)]
pub struct Resources {

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
