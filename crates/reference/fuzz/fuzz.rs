#![no_main]

extern crate libfuzzer_sys;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    buildkit_rs_reference::Reference::parse_normalized_named(data).ok();
});
