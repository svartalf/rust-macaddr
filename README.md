# rust-macaddr

[![Latest Version](https://img.shields.io/crates/v/macaddr.svg)](https://crates.io/crates/macaddr)
[![Latest Version](https://docs.rs/macaddr/badge.svg)](https://docs.rs/macaddr)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)


This crate provides Rust types for a [MAC address] identifiers,
both in IEEE *EUI-48* and *EUI-64* formats.

You can think about it as the `[std::net::SocketAddr](https://doc.rust-lang.org/std/net/enum.SocketAddr.html)`
enum, but for MAC addresses instead.

It is intended to be as small and reusable as possible,
so it can be used by other crates easily,
providing the unified and neat interface.
