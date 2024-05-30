#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct Api {
    rpc: WasmRpc,
}
impl Api {}
impl crate::bindings::exports::test::callee2_stub::stub_callee2::GuestApi for Api {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn run(&self, name: String) -> crate::bindings::test::callee2::api::Callee2Type {
        let result = self
            .rpc
            .invoke_and_await(
                "test:callee2/api/run",
                &[WitValue::builder().string(&name)],
            )
            .expect(
                &format!("Failed to invoke-and-await remote {}", "test:callee2/api/run"),
            );
        ({
            let record = result.tuple_element(0).expect("tuple not found");
            crate::bindings::test::callee2::api::Callee2Type {
                value: record
                    .field(0usize)
                    .expect("record field not found")
                    .string()
                    .expect("string not found")
                    .to_string(),
            }
        })
    }
}
