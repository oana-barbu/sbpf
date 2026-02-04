#![allow(clippy::arithmetic_side_effects)]
#![cfg(all(feature = "jit", not(target_os = "windows"), target_arch = "x86_64"))]

use solana_sbpf::error::ProgramResult;
use test_utils::{test_interpreter_and_jit_asm, TestContextObject};

#[test]
fn test_stack() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 51
        stdw [r10-16], 0xab
        stdw [r10-8], 0xcd
        and r1, 1
        lsh r1, 3
        mov r2, r10
        add r2, r1
        ldxdw r0, [r2-16]
        exit",
        [],
        TestContextObject::new(9),
        ProgramResult::Ok(0xcd),
    );
}
