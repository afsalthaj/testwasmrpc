use crate::bindings::exports::test::caller::api::{Guest, NewType};


mod bindings;

struct Component;

impl Guest for Component {
    fn run(name: String) -> NewType {
        NewType {
            value: format!("Hello, {}!", name)
        }
    }
}