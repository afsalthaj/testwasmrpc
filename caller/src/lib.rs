use crate::bindings::exports::test::caller::api::Guest;
use crate::bindings::test::callee_stub::stub_callee::Api;
use crate::bindings::golem::rpc::types::Uri;


mod bindings;

struct Component;

impl Guest for Component {
    fn run_self(name: String) -> String {
        let component_id = "callee_component_id".to_string();
        let uri = Uri { value: format!("worker://{component_id}/{}", "foo") };
        let callee_api = Api::new(&uri);
        let result = callee_api.run_callee(name.as_str());
        result
    }

    fn run_remote(remote: String) -> String {
        format!("From remote {}", remote.to_string())
    }
}