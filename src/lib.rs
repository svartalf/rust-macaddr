//! This crate provides types for a [MAC address] identifiers,
//! both in IEEE *EUI-48* and *EUI-64* formats.
//!
//! You can think about it as the `std::net::SocketAddr` enum,
//! but for MAC addresses instead.
//!
//! It is intended to be as small and reusable as possible,
//! so it can be used by other crates easily,
//! providing the unified and neat interface.
//!
//! ## Serde support
//!
//! [Serde] support can be enabled with a `"serde_std"` feature
//! (disabled by default) if used in `std`-enabled builds.
//!
//! This feature is called like this because of [this Cargo bug].\
//! `"serde"` feature is exists also, but it is intended to be used
//! in the `no_std` builds.
//!
//! ## No-std support
//!
//! This crate can be used in a `no_std` builds with
//! disabled `"std"` feature (enabled by default).
//!
//! Enabled `"serde"` feature will add support for `no_std`
//! serde serialization and deserialization.
//!
//! [Serde]: https://serde.rs
//! [MAC address]: https://en.wikipedia.org/wiki/MAC_address
//! [this Cargo bug]: https://github.com/rust-lang/cargo/issues/3494
#![cfg_attr(not(feature = "std"), no_std)]
#![doc(html_root_url = "https://docs.rs/macaddr/0.1.1")]

mod addr;
mod addr6;
mod addr8;
mod parser;

pub use self::addr::MacAddr;
pub use self::addr6::MacAddr6;
pub use self::addr8::MacAddr8;
pub use self::parser::ParseError;
