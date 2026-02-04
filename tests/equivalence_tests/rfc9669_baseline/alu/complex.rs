#![allow(clippy::arithmetic_side_effects)]
#![cfg(all(feature = "jit", not(target_os = "windows"), target_arch = "x86_64"))]

use solana_sbpf::error::ProgramResult;
use test_utils::{test_interpreter_and_jit_asm, TestContextObject};

#[test]
fn test_alu_arith() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 10
        sub32 r0, r0
        mov32 r1, 1
        mov32 r2, 2
        mov32 r3, 3
        mov32 r4, 4
        mov32 r5, 5
        mov32 r6, 6
        mov32 r7, 7
        mov32 r8, 8
        mov32 r9, 9
        jne r0, 0, exit
        add32 r0, 23
        add32 r0, r7
        jne r0, 30, exit
        sub32 r0, 13
        sub32 r0, r1
        jne r0, 16, exit
        mul32 r0, 7
        mul32 r0, r3
        jne r0, 336, exit
        div32 r0, 2
        div32 r0, r4
        jne r0, 42, exit
        exit",
        [],
        TestContextObject::new(25),
        ProgramResult::Ok(0x2a),
    );
}

#[test]
fn test_alu64_arith() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 10
        sub r0, r0
        mov r1, 1
        mov r2, 2
        mov r3, 3
        mov r4, 4
        mov r5, 5
        mov r6, 6
        mov r7, 7
        mov r8, 8
        mov r9, 9
        jne r0, 0, exit
        add r0, 23
        add r0, r7
        jne r0, 30, exit
        sub r0, 13
        sub r0, r1
        jne r0, 16, exit
        mul r0, 7
        mul r0, r3
        jne r0, 336, exit
        div r0, 2
        div r0, r4
        exit",
        [],
        TestContextObject::new(24),
        ProgramResult::Ok(0x2a),
    );
}

#[test]
fn test_prime() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 67
        mov r0, 0x1
        mov r2, 0x2
        jgt r1, 0x2, L3
        L1:
        ja exit
        L2:
        add r2, 0x1
        mov r0, 0x1
        jge r2, r1, exit
        L3:
        mov r3, r1
        div r3, r2
        mul r3, r2
        mov r4, r1
        sub r4, r3
        mov r0, 0x0
        jne r4, 0x0, L2
        exit",
        [],
        TestContextObject::new(16),
        ProgramResult::Ok(0x1),
    );
}
