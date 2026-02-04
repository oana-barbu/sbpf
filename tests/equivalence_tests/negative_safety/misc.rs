#![allow(clippy::arithmetic_side_effects)]
#![cfg(all(feature = "jit", not(target_os = "windows"), target_arch = "x86_64"))]

extern crate test_utils;

use solana_sbpf::{
    assembler::assemble,
    program::{BuiltinProgram, SBPFVersion},
    vm::Config,
};
use std::sync::Arc;
use test_utils::TestContextObject;

#[test]
fn test_sdiv32_rejected_in_v4() {
    let config = Config {
        enabled_sbpf_versions: SBPFVersion::V4..=SBPFVersion::V4,
        ..Config::default()
    };
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(config));

    let result = assemble::<TestContextObject>(
        "
        mov32 r0, -12
        mov32 r1, 4
        sdiv32 r0, r1
        exit",
        loader,
    );
    assert_eq!(
        result.unwrap_err(),
        "Invalid instruction \"sdiv32\"".to_string()
    );
}
