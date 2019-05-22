use crate::{MacAddr6, MacAddr8};

/// A MAC address, either in *EUI-48* or *EUI-64* format.
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum MacAddr {
    V6(MacAddr6),
    V8(MacAddr8),
}

impl MacAddr {
    pub fn is_v6(&self) -> bool {
        match self {
            MacAddr::V6(_) => true,
            MacAddr::V8(_) => false,
        }
    }

    pub fn is_v8(&self) -> bool {
        match self {
            MacAddr::V6(_) => false,
            MacAddr::V8(_) => true,
        }
    }
}

impl From<MacAddr6> for MacAddr {
    fn from(addr: MacAddr6) -> Self {
        MacAddr::V6(addr)
    }
}

impl From<MacAddr8> for MacAddr {
    fn from(addr: MacAddr8) -> Self {
        MacAddr::V8(addr)
    }
}
