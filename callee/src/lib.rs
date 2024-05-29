use crate::bindings::exports::test::callee::api::{CalleeType, Guest};

mod bindings;

struct Component;

impl Guest for Component {
    fn run(name: String) -> CalleeType {
        CalleeType {
            value: format!("Hello, {}!", name),
        }
    }
}
