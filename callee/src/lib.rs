use crate::bindings::exports::test::callee::api::{CalleeType, Guest};
use crate::bindings::test::caller_stub::stub_caller::{Api, Uri};

mod bindings;

struct Component;

impl Guest for Component {
    fn run(caller_component_id: String) -> CalleeType {
        let uri = Uri { value: format!("worker://{caller_component_id}/{}", "myworker") };

        let api = Api::new(&uri);

        // Cyclic call
        let result = api.run("foo");

        CalleeType {
            value: result.value
        }
    }
}
