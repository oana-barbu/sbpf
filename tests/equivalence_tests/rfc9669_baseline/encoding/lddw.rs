#![allow(clippy::arithmetic_side_effects)]
#![cfg(all(feature = "jit", not(target_os = "windows"), target_arch = "x86_64"))]

use solana_sbpf::error::ProgramResult;
use test_utils::{test_interpreter_and_jit_asm, TestContextObject};

#[test]
fn test_lddw() {
    test_interpreter_and_jit_asm!(
        "
        lddw r0, 0x1122334455667788
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x1122334455667788),
    );
}

#[test]
fn test_lddw2() {
    test_interpreter_and_jit_asm!(
        "
        lddw r0, 2147483648
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x0000000080000000),
    );
}

#[test]
fn test_rfc9669_lddw() {
    test_interpreter_and_jit_asm!(
        "
        lddw r0, 0x1122334455667788
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x1122334455667788),
    );
}
