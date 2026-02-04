#![allow(clippy::arithmetic_side_effects)]
#![cfg(all(feature = "jit", not(target_os = "windows"), target_arch = "x86_64"))]

use solana_sbpf::error::ProgramResult;
use test_utils::{test_interpreter_and_jit_asm, TestContextObject};

#[test]
fn test_add() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov32 r1, 2
        add32 r0, 1
        add32 r0, r1
        add32 r0, r0
        add32 r0, -3
        exit",
        [],
        TestContextObject::new(7),
        ProgramResult::Ok(0x3),
    );
}

#[test]
fn test_add64() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0
        mov r1, 2
        add r0, 1
        add r0, r1
        add r0, r0
        add r0, -3
        exit",
        [],
        TestContextObject::new(7),
        ProgramResult::Ok(0x3),
    );
}

#[test]
fn test_rfc9669_add32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 0xffffffff
        add32 r1, 1
        jne32 r1, 0, fail
        mov32 r2, 10
        add32 r2, 32
        jne32 r2, 42, fail
        mov32 r3, 40
        mov32 r4, 2
        add32 r3, r4
        jne32 r3, 42, fail
        mov r5, -1
        mov32 r5, 1
        rsh r5, 32
        jne32 r5, 0, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(18),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_add64() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 40
        add r1, 2
        jne r1, 42, fail
        mov r2, 41
        mov r3, 1
        add r2, r3
        jne r2, 42, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_sub32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 0
        sub32 r1, 1
        jne32 r1, 0xffffffff, fail
        mov32 r2, 50
        sub32 r2, 8
        jne32 r2, 42, fail
        mov32 r3, 50
        mov32 r4, 8
        sub32 r3, r4
        jne32 r3, 42, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(14),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_sub64() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 50
        sub r1, 8
        jne r1, 42, fail
        mov r2, 50
        mov r3, 8
        sub r2, r3
        jne r2, 42, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_neg() {
    test_interpreter_and_jit_asm!(
        "
        lddw r0, 0x100000002
        neg32 r0
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0xfffffffe),
    );
}

#[test]
fn test_neg32_intmin_imm() {
    test_interpreter_and_jit_asm!(
        "
        lddw r0, 0x80000000
        neg32 r0
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x80000000),
    );
}

#[test]
fn test_neg32_intmin_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x80000000
        neg32 r0
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x80000000),
    );
}

#[test]
fn test_neg64_intmin_imm() {
    test_interpreter_and_jit_asm!(
        "
        ldxdw r0, [r1+0]
        neg r0
        exit",
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80],
        TestContextObject::new(3),
        ProgramResult::Ok(0x8000000000000000),
    );
}

#[test]
fn test_neg64_intmin_reg() {
    test_interpreter_and_jit_asm!(
        "
        ldxdw r0, [r1+0]
        mov r1, -1
        neg r0
        exit",
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80],
        TestContextObject::new(4),
        ProgramResult::Ok(0x8000000000000000),
    );
}

#[test]
fn test_neg64() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 2
        neg r0
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0xfffffffffffffffe),
    );
}

#[test]
fn test_rfc9669_neg32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 42
        neg32 r1
        jne32 r1, 0xffffffd6, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(7),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_neg64() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 42
        neg r1
        lddw r2, 0xffffffffffffffd6
        jne r1, r2, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(9),
        ProgramResult::Ok(0x1),
    );
}
