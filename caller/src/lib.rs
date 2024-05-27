use crate::bindings::exports::test::caller::api::{Guest, NewType};
use crate::bindings::test::callee_stub::stub_callee::Api;
use crate::bindings::golem::rpc::types::Uri;


mod bindings;

struct Component;

impl Guest for Component {
    fn run_self(name:  String) -> NewType {
        let component_id = "callee_component_id".to_string();
        let uri = Uri { value: format!("worker://{component_id}/{}", "foo") };
        let callee_api = Api::new(&uri);
        let result = callee_api.run_callee(name.as_str());
        NewType{
            value: result
        }
    }

    fn run_remote(remote: NewType) -> String {
        format!("From remote {}", remote.value.to_string())
    }
}