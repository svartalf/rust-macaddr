#![no_main]

use std::str::FromStr;

use libfuzzer_sys::fuzz_target;
use macaddr::MacAddr;

fuzz_target!(|s: String| {
    let _ = MacAddr::from_str(&s);
});
