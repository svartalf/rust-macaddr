#![no_main]

use std::str::FromStr;

use libfuzzer_sys::fuzz_target;
use macaddr::MacAddr8;

fuzz_target!(|s: String| {
    let _ = MacAddr8::from_str(&s);
});
