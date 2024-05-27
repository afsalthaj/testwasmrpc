use crate::bindings::exports::test::caller::api::Guest;

mod bindings;

struct Component;

impl Guest for Component {
    fn run_self(name: String) -> String {
        // Supposed to be in another worker
        Self::run_remote(name)
    }

    fn run_remote(remote: String) -> String {
        format!("From remote {}", remote.to_string())
    }
}