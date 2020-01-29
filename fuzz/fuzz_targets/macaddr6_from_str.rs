#![no_main]

use std::str::FromStr;

use libfuzzer_sys::fuzz_target;
use macaddr::MacAddr6;

fuzz_target!(|s: String| {
    let _ = MacAddr6::from_str(&s);
});
