use crate::bindings::exports::test::callee::api::{CalleeType, Guest};
use crate::bindings::test::callee2_stub::stub_callee2::{Api, Uri};

mod bindings;

struct Component;

impl Guest for Component {
    fn run(name: String) -> CalleeType {
        let uri = Uri { value: format!("worker://{}/{}", "component_id", "myworker") };
        let api = Api::new(&uri);
        api.run("foo");
        CalleeType {
            value: format!("Hello, {}!", name),
        }
    }
}
