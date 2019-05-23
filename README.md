# macaddr

[![Latest Version](https://img.shields.io/crates/v/macaddr.svg)](https://crates.io/crates/macaddr)
[![Latest Version](https://docs.rs/macaddr/badge.svg)](https://docs.rs/macaddr)
[![Build Status](https://travis-ci.org/svartalf/rust-macaddr.svg?branch=master)](https://travis-ci.org/svartalf/rust-macaddr)
[![Coverage Status](https://coveralls.io/repos/github/svartalf/rust-macaddr/badge.svg?branch=master)](https://coveralls.io/github/svartalf/rust-macaddr?branch=master)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)

This crate provides Rust types for a [MAC address](https://en.wikipedia.org/wiki/MAC_address)
identifiers, both in IEEE *EUI-48* and *EUI-64* formats.

You can think about it as the [`std::net::SocketAddr`](https://doc.rust-lang.org/std/net/enum.SocketAddr.html)
enum, but for MAC addresses instead.

It is intended to be as small and reusable as possible,
so it can be used by other crates easily,
providing the unified and neat interface.

And it is `serde`- and `no_std`-friendly also!
