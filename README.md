# nmos-rs

`nmos-rs` is a WIP Rust implementation of the AMWA Networked Media Open Specifications (NMOS) API.

## Repo Overview

This repository hosts the following libraries:

  - `nmos-schema` - Rust types generated automatically from NMOS JSON Schemas.
  - `nmos-model` - Handcrafted typed model of NMOS resources.
  - `nmos-node` - Node implementation build around model including HTTP APIs.

### MSRV policy

Minimum Supported Rust Version is **1.56** due to the use of Rust 2021 Edition features.

## Getting Started

### Rust

Examples can be found at `node/examples`. You can run the examples with `cargo run --example name`. See the [list of examples](node/examples).

### Windows Support

Windows is currently unsupported due to a missing implementation in the MDNS abstraction crate [zeroconf][zeroconf].
To properly support MDNS on modern Windows platforms, it makes most
sense to use Dnssd in the WinRT API, for which bindings exist in the
[windows-rs][windows-rs] crate. (``Windows.Networking.ServiceDiscovery.Dnssd``)

## TODO:
- IS-04 v1.1-v1.3 node support.
- IS-05 node support.
- Automated testing with the AMWA NMOS testing tool.
- Simple registry implementation?
- You tell me!

[zeroconf]: https://crates.io/crates/zeroconf
[windows-rs]: https://crates.io/crates/windows
