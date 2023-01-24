# [`Substreams`](https://substreams.streamingfast.io/) for [**Antelope**](http://antelope.io/)

[<img alt="github" src="https://img.shields.io/badge/Github-substreams-antelope-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/pinax-network/substreams-antelope)
[<img alt="crates.io" src="https://img.shields.io/crates/v/antelope.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/antelope)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-antelope-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/antelope)
[<img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/pinax-network/substreams-antelope/ci.yml?branch=develop&style=for-the-badge" height="20">](https://github.com/pinax-network/substreams-antelope/actions?query=branch%3Adevelop)

> This library contains the generated protobuffer for the [Antelope blocks](https://github.com/pinax-network/firehose-antelope/blob/develop/proto/sf/antelope/type/v1/type.proto) as well as helper methods to extract and parse block data.

## 📖 Documentation

### https://docs.rs/substreams-antelope

### Further resources

- [Substreams documentation](https://substreams.streamingfast.io)

## 🛠 Feature Roadmap

- [x] [Antelope blocks](https://github.com/pinax-network/firehose-antelope/blob/develop/proto/sf/antelope/type/v1/type.proto)
- [x] Block helper methods
    - [x] all_transaction_traces
    - [x] executed_transaction_traces
    - [x] transaction_traces_count
    - [x] executed_input_action_count
    - [x] executed_total_action_count

## Install

```
$ cargo add substreams-antelope
```

## Quickstart

**Cargo.toml**

```toml
[dependencies]
substreams = "0.5"
substreams-antelope = "0.1"
```

**src/lib.rs**

```rust
use substreams::prelude::*;
use substreams::errors::Error;
use substreams_antelope::{Block, ActionTraces};

#[substreams::handlers::map]
fn map_action_traces(block: Block) -> Result<ActionTraces, Error> {
    let mut action_traces = vec![];  

    for trx in block.clone().all_transaction_traces() {
        for trace in trx.action_traces.clone() {
            action_traces.push(trace);
        }
    }
    Ok(ActionTraces { action_traces })
}
```
