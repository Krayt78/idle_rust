pub struct Item {
        pub name: String,
        pub description: String,
    }

    impl Item {
        pub fn new(name: String, description: String) -> Self {
            Self { name, description }
        }
    }