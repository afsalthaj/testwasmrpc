## Tests with wasm-rpc


A locally built golem-cli that depends on a specific local version of wasm-rpc is used to initialise the workspace


### Simple Scenario

A caller depends on a callee module.
This case, callee-stub is created and is then added as a dependency to caller.
This works just as fine

```bash
../golem/target/debug/golem-cli stubgen initialize-workspace --targets callee --callers caller --wasm-rpc-path-override /Users/afsalthaj/github/wasm-rpc/wasm-rpc

```