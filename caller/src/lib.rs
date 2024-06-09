use crate::bindings::exports::test::caller::api::{Guest, NewType};
use crate::bindings::test::callee_stub::stub_callee::{Api, Uri};

mod bindings;

struct Component;

impl Guest for Component {
    fn add(x: i32, y: i32) -> i64 {
        (x + y) as i64
    }

    fn run(component_id: String) -> NewType {
        let uri = Uri { value: format!("worker://{component_id}/{}", "myworker") };

        let api = Api::new(&uri);

        // Cyclic call
        let result = api.run("foo");

        NewType {
            value: result.value
        }
    }
}