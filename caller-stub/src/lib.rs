#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
impl crate::bindings::exports::test::caller_stub::stub_caller::GuestApi for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn run_self(&self, name: String) -> crate::bindings::test::caller::api::NewType {
        let result = self
            .rpc
            .invoke_and_await(
                "test:caller/api/run-self",
                &[WitValue::builder().string(&name)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}", "test:caller/api/run-self"
                ),
            );
        ({
            let record = result.tuple_element(0).expect("tuple not found");
            crate::bindings::test::caller::api::NewType {
                value: record
                    .field(0usize)
                    .expect("record field not found")
                    .string()
                    .expect("string not found")
                    .to_string(),
            }
        })
    }
    fn run_remote(&self, remote: crate::bindings::test::caller::api::NewType) -> String {
        let result = self
            .rpc
            .invoke_and_await(
                "test:caller/api/run-remote",
                &[WitValue::builder().record().item().string(&remote.value).finish()],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}", "test:caller/api/run-remote"
                ),
            );
        (result
            .tuple_element(0)
            .expect("tuple not found")
            .string()
            .expect("string not found")
            .to_string())
    }
}
