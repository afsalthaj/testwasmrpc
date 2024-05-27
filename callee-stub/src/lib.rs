#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
impl crate::bindings::exports::test::callee_stub::stub_callee::GuestApi for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn run_callee(&self, input: String) -> String {
        let result = self
            .rpc
            .invoke_and_await(
                "test:callee/api/run-callee",
                &[WitValue::builder().string(&input)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}", "test:callee/api/run-callee"
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
