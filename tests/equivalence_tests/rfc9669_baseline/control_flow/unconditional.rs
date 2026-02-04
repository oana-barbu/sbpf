#![allow(clippy::arithmetic_side_effects)]
#![cfg(all(feature = "jit", not(target_os = "windows"), target_arch = "x86_64"))]

use solana_sbpf::error::ProgramResult;
use test_utils::{test_interpreter_and_jit_asm, TestContextObject};

#[test]
fn test_rfc9669_ja() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 1
        ja done
        done:
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x1),
    );
}
