use crate::bindings::exports::test::caller::api::{Guest, NewType};
use crate::bindings::test::callee_stub::stub_callee::{Api, Uri};
use crate::bindings::test::caller_stub::stub_caller::Api as CyclicApi;
use crate::bindings::test::caller_stub::stub_caller::Uri as CyclicUri; // Sort of same as above doesn't matter

mod bindings;

struct Component;

impl Guest for Component {
    fn add(x: i32, y: i32) -> i64 {
        let uri = CyclicUri { value: format!("worker://callercomponentitself/{}", "myworker") };

        let api = CyclicApi::new(&uri);

        // Cyclic call
        let result = api.run("foo");

        dbg!(result.value);

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