//! [![github]](https://github.com/pinax-network/substreams-antelope)&ensp;[![crates-io]](https://crates.io/crates/substreams-antelope)&ensp;[![docs-rs]](crate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! This library contains the generated protobuffer for the [Antelope blocks](https://github.com/pinax-network/firehose-antelope/blob/develop/proto/sf/antelope/type/v1/type.proto) as well as helper methods to extract and parse block data.
//!
//! ## 🛠 Feature Roadmap
//!
//! - [x] [Antelope blocks](https://github.com/pinax-network/firehose-antelope/blob/develop/proto/sf/antelope/type/v1/type.proto)
//! - [x] Block helper methods
//!     - [x] all_transaction_traces
//!     - [x] executed_transaction_traces
//!     - [x] transaction_traces_count
//!     - [x] executed_input_action_count
//!     - [x] executed_total_action_count

/// Modules for generated protobuffer
#[path = "pb/sf.antelope.type.v1.rs"]
#[rustfmt::skip]
#[allow(dead_code)]
pub mod pb;

pub mod action;
pub mod block;
pub mod errors;

pub use action::Action;
pub use errors::Error;
