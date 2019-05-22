#![cfg_attr(not(feature = "std"), no_std)]

mod addr;
mod addr6;
mod addr8;

pub use self::addr::MacAddr;
pub use self::addr6::MacAddr6;
pub use self::addr8::MacAddr8;
