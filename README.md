# [`Substreams`](https://substreams.streamingfast.io/) for [**Antelope**](http://antelope.io/)

[<img alt="github" src="https://img.shields.io/badge/Github-substreams.antelope-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/pinax-network/substreams-antelope)
[<img alt="crates.io" src="https://img.shields.io/crates/v/substreams-antelope.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/substreams-antelope)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-substreams.antelope-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/substreams-antelope)
[<img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/pinax-network/substreams-antelope/ci.yml?branch=develop&style=for-the-badge" height="20">](https://github.com/pinax-network/substreams-antelope/actions?query=branch%3Adevelop)

> This library contains the generated protobuffer for the [Antelope blocks](https://github.com/pinax-network/firehose-antelope/blob/develop/proto/sf/antelope/type/v1/type.proto) as well as helper methods to extract and parse block data.

## 📖 Documentation

### https://docs.rs/substreams-antelope

### Further resources

- [Substreams documentation](https://substreams.streamingfast.io)


## Install

```
$ cargo add substreams-antelope
```

## Usage

Refer to [Docs.rs](https://docs.rs/substreams-antelope/latest/substreams_antelope/struct.Block.html#implementations) for helper methods on `Block` that extract action and transaction iterators from the Antelope block.

**Cargo.toml**

```toml
[dependencies]
substreams = "0.5"
substreams-antelope = "0.4"
```

**src/lib.rs**

```rust
use substreams::prelude::*;
use substreams::errors::Error;
use substreams_antelope::{Block, ActionTraces};

#[substreams::handlers::map]
fn map_action_traces(block: Block) -> Result<ActionTraces, Error> {
    let mut action_traces = vec![];

    for trx in block.transaction_traces() {
        for trace in trx.action_traces {
            action_traces.push(trace);
        }
    }
    Ok(ActionTraces { action_traces })
}
```

Or, using `actions()` helper method to filter all actions of `Statelog` type from `myaccount` account. As a parameter you can specify a list of contract account names to include actions from, that can be empty if you want actions with this signature from any contract account.

**src/lib.rs**

```rust
#[substreams::handlers::map]
fn map_actions(param_account: String, block: substreams_antelope::Block) -> Result<Actions, substreams::errors::Error> {
    Ok(Actions {
        transfers: block.actions::<abi::contract::actions::Transfer>(&["eosio.token"])
            .map(|(action, trace)| Transfer {
                // action.to, action.from, action.memo, action.quantity are available here.
            })
            .collect(),
    })
}
```


## Using Abigen
To generate ABI bindings for your smart contract you can add `abi/contract.abi.json` file containing the smart contract ABI, as well as the following `build.rs` file to the root of your project. This will ensure that `src/abi/contract.rs` module containing Rust bindings for your smart contract is always generated in your project:

**build.rs**

```rust
fn main() {
    substreams_antelope::Abigen::new("Contract", "abi/eosio.token.json")
        .expect("failed to load abi")
        .generate()
        .expect("failed to generate contract")
        .write_to_file("src/abi/eosio.token.rs")
        .expect("failed to write contract");
}
```

## Release
- Bump up version in workspace `Cargo.toml`
- Commit changes
- Tag a release: https://github.com/pinax-network/substreams-antelope/releases
- Publish packages in this order: `core`, `abigen`, `substreams-antelope`.

TODO: automate releases with github actions