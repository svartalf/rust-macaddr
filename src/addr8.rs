#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// MAC address in *EUI-64* format.
#[repr(C)]
#[derive(Debug, Default, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MacAddr8([u8; 8]);

impl MacAddr8 {
    /// Creates a new `MacAddr8` address from the bytes.
    ///
    /// ## Example
    ///
    /// ```edition2018
    /// # use macaddr::MacAddr8;
    /// let addr = MacAddr8::new(0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF);
    /// ```
    #[allow(clippy::many_single_char_names, clippy::too_many_arguments)]
    pub const fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> MacAddr8 {
        Self([a, b, c, d, e, f, g, h])
    }

    /// Returns `true` if the address is nil.
    ///
    /// ## Example
    ///
    /// ```edition2018
    /// # use macaddr::MacAddr8;
    /// let addr = MacAddr8::new(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
    ///
    /// assert_eq!(addr.is_nil(), true);
    /// ```
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_nil(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }

    /// Returns `true` if the address is broadcast.
    ///
    /// ## Example
    ///
    /// ```edition2018
    /// # use macaddr::MacAddr8;
    /// let addr = MacAddr8::new(0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF);
    ///
    /// assert_eq!(addr.is_broadcast(), true);
    /// ```
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn is_broadcast(&self) -> bool {
        self.0.iter().all(|&b| b == 0xFF)
    }

    /// Returns `true` if the address is unicast.
    ///
    /// ## Example
    ///
    /// ```edition2018
    /// # use macaddr::MacAddr8;
    /// let addr = MacAddr8::new(0x00, 0x01, 0x44, 0x55, 0x66, 0x77, 0xCD, 0xEF);
    ///
    /// assert_eq!(addr.is_unicast(), true);
    /// ```
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub const fn is_unicast(&self) -> bool {
        self.0[0] & 1 == 0
    }

    /// Returns `true` if the address is multicast.
    ///
    /// ## Example
    ///
    /// ```edition2018
    /// # use macaddr::MacAddr8;
    /// let addr = MacAddr8::new(0x01, 0x00, 0x0C, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC);
    ///
    /// assert_eq!(addr.is_multicast(), true);
    /// ```
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub const fn is_multicast(&self) -> bool {
        self.0[0] & 1 == 1
    }

    /// Returns `true` if the address is universally administered address (UAA).
    ///
    /// ## Example
    ///
    /// ```edition2018
    /// # use macaddr::MacAddr8;
    /// let addr = MacAddr8::new(0x01, 0x00, 0x0C, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC);
    ///
    /// assert_eq!(addr.is_universal(), true);
    /// ```
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub const fn is_universal(&self) -> bool {
        self.0[0] & 1 << 1 == 0
    }

    /// Returns `true` if the address is locally administered (LAA).
    ///
    /// ## Example
    ///
    /// ```edition2018
    /// # use macaddr::MacAddr8;
    /// let addr = MacAddr8::new(0x02, 0x00, 0x0C, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC);
    ///
    /// assert_eq!(addr.is_local(), true);
    /// ```
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub const fn is_local(&self) -> bool {
        self.0[0] & 1 << 1 == 2
    }

    /// Consumes `MacAddr8` address and returns raw bytes.
    pub const fn into_bytes(self) -> [u8; 8] {
        self.0
    }
}

impl From<[u8; 8]> for MacAddr8 {
    fn from(bytes: [u8; 8]) -> Self {
        Self(bytes)
    }
}

impl AsRef<[u8]> for MacAddr8 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for MacAddr8 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

#[cfg(feature = "std")]
mod std {
    use std::fmt;

    use super::MacAddr8;

    impl fmt::Display for MacAddr8 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_fmt(format_args!(
                    // Canonical form
                    "{:X}-{:X}-{:X}-{:X}-{:X}-{:X}-{:X}-{:X}",
                    self.0[0],
                    self.0[1],
                    self.0[2],
                    self.0[3],
                    self.0[4],
                    self.0[5],
                    self.0[6],
                    self.0[7],
                ))
        }
    }
}
