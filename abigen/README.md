## Using Abigen
To generate ABI bindings for your smart contract you can add `abi/contract.abi.json` file containing the smart contract ABI, as well as the following `build.rs` file to the root of your project. This will ensure that `src/abi/contract.rs` module containing Rust bindings for your smart contract is always generated in your project:

**build.rs**

```rust
fn main() {
    substreams_antelope::Abigen::new("Contract", "abi/gems.blend.abi.json")
        .expect("failed to load abi")
        .generate()
        .expect("failed to generate contract")
        .write_to_file("src/abi/gems.blend.abi.rs")
        .expect("failed to write contract");
}
```