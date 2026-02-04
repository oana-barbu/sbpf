#![allow(clippy::arithmetic_side_effects)]
#![cfg(all(feature = "jit", not(target_os = "windows"), target_arch = "x86_64"))]

extern crate test_utils;

use solana_sbpf::{
    assembler::assemble,
    ebpf,
    error::ProgramResult,
    memory_region::MemoryRegion,
    program::BuiltinProgram,
    static_analysis::Analysis,
    verifier::RequisiteVerifier,
    vm::{Config, ContextObject},
};
use std::sync::Arc;
use test_utils::{
    compare_register_trace, create_vm, test_interpreter_and_jit, test_interpreter_and_jit_asm,
    TestContextObject,
};

#[test]
fn test_mul32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 3
        mul32 r0, 4
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0xc),
    );
}

#[test]
fn test_mul32_intmin_by_negone_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x80000000
        mul32 r0, -1
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x80000000),
    );
}

#[test]
fn test_mul32_intmin_by_negone_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x80000000
        mov r1, -1
        mul32 r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x80000000),
    );
}

#[test]
fn test_mul32_reg_overflow() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x40000001
        mov r1, 4
        mul32 r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x4),
    );
}

#[test]
fn test_mul32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 3
        mov r1, 4
        mul32 r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0xc),
    );
}

#[test]
fn test_mul64_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x40000001
        mul r0, 4
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x100000004),
    );
}

#[test]
fn test_mul64_intmin_by_negone_imm() {
    test_interpreter_and_jit_asm!(
        "
        ldxdw r0, [r1+0]
        mul r0, -1
        exit",
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80],
        TestContextObject::new(3),
        ProgramResult::Ok(0x8000000000000000),
    );
}

#[test]
fn test_mul64_intmin_by_negone_reg() {
    test_interpreter_and_jit_asm!(
        "
        ldxdw r0, [r1+0]
        mov r1, -1
        mul r0, r1
        exit",
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80],
        TestContextObject::new(4),
        ProgramResult::Ok(0x8000000000000000),
    );
}

#[test]
fn test_mul64_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x40000001
        mov r1, 4
        mul r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x100000004),
    );
}

#[test]
fn test_rfc9669_mul32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 6
        mul32 r1, 7
        jne32 r1, 42, fail
        mov32 r2, 6
        mov32 r3, 7
        mul32 r2, r3
        jne32 r2, 42, fail
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
fn test_rfc9669_mul64() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 6
        mul r1, 7
        jne r1, 42, fail
        mov r2, 6
        mov r3, 7
        mul r2, r3
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
fn test_div32_high_divisor() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 12
        lddw r1, 0x100000004
        div32 r0, r1
        exit",
        [],
        TestContextObject::new(5),
        ProgramResult::Ok(0x3),
    );
}

#[test]
fn test_div32_imm() {
    test_interpreter_and_jit_asm!(
        "
        lddw r0, 0x10000000c
        div32 r0, 4
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x3),
    );
}

#[test]
fn test_div32_reg() {
    test_interpreter_and_jit_asm!(
        "
        lddw r0, 0x10000000c
        mov r1, 4
        div32 r0, r1
        exit",
        [],
        TestContextObject::new(5),
        ProgramResult::Ok(0x3),
    );
}

#[test]
fn test_div64_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0xc
        lsh r0, 32
        div r0, 4
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x300000000),
    );
}

#[test]
fn test_div64_negative_imm() {
    test_interpreter_and_jit_asm!(
        "
        lddw r0, 0xFFFFFFFFFFFFFFFF
        div r0, -10
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_div64_negative_reg() {
    test_interpreter_and_jit_asm!(
        "
        lddw r0, 0xFFFFFFFFFFFFFFFF
        mov32 r1, -10
        div r0, r1
        exit",
        [],
        TestContextObject::new(5),
        ProgramResult::Ok(0x10000000A),
    );
}

#[test]
fn test_div64_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0xc
        lsh r0, 32
        mov r1, 4
        div r0, r1
        exit",
        [],
        TestContextObject::new(5),
        ProgramResult::Ok(0x300000000),
    );
}

#[test]
fn test_rfc9669_div32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 84
        div32 r1, 2
        jne32 r1, 42, fail
        mov32 r2, 126
        mov32 r3, 3
        div32 r2, r3
        jne32 r2, 42, fail
        mov32 r4, 123
        mov32 r5, 0
        div32 r4, r5
        jne32 r4, 0, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(15),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_div64() {
    test_interpreter_and_jit_asm!(
        "
        lddw r1, 0x0000000000000054
        mov r2, 2
        div r1, r2
        jne r1, 42, fail
        mov r3, 123
        mov r4, 0
        div r3, r4
        jne r3, 0, fail
        lddw r5, 0x8000000000000000
        div r5, 0x80000000
        jne r5, 0, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(17),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_sdiv32_by_zero_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 1
        sdiv32 r0, 0
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x0),
    );
}

#[test]
fn test_sdiv32_by_zero_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 1
        mov32 r1, 0
        sdiv32 r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x0),
    );
}

#[test]
fn test_sdiv32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, -12
        sdiv32 r0, 4
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0xfffffffd),
    );
}

#[test]
fn test_sdiv32_intmin_by_negone_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0x80000000
        sdiv32 r0, -1
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x80000000),
    );
}

#[test]
fn test_sdiv32_intmin_by_negone_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0x80000000
        mov32 r1, -1
        sdiv32 r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x80000000),
    );
}

#[test]
fn test_sdiv32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, -12
        mov32 r1, 4
        sdiv32 r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0xfffffffd),
    );
}

#[test]
fn test_sdiv64_by_zero_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 1
        sdiv r0, 0
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x0),
    );
}

#[test]
fn test_sdiv64_by_zero_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 1
        mov r1, 0
        sdiv r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x0),
    );
}

#[test]
fn test_sdiv64_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, -12
        sdiv r0, 4
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0xfffffffffffffffd),
    );
}

#[test]
fn test_sdiv64_intmin_by_negone_imm() {
    test_interpreter_and_jit_asm!(
        "
        ldxdw r0, [r1+0]
        sdiv r0, -1
        exit",
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80],
        TestContextObject::new(3),
        ProgramResult::Ok(0x8000000000000000),
    );
}

#[test]
fn test_sdiv64_intmin_by_negone_reg() {
    test_interpreter_and_jit_asm!(
        "
        ldxdw r0, [r1+0]
        mov r1, -1
        sdiv r0, r1
        exit",
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80],
        TestContextObject::new(4),
        ProgramResult::Ok(0x8000000000000000),
    );
}

#[test]
fn test_sdiv64_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, -12
        mov r1, 4
        sdiv r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0xfffffffffffffffd),
    );
}

#[test]
fn test_rfc9669_sdiv32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, -84
        sdiv32 r1, 2
        jne32 r1, 0xffffffd6, fail
        mov32 r2, -126
        mov32 r3, 3
        sdiv32 r2, r3
        jne32 r2, 0xffffffd6, fail
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
fn test_rfc9669_sdiv64() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, -84
        sdiv r1, 2
        jne r1, 0xffffffd6, fail
        mov r2, -126
        mov r3, 3
        sdiv r2, r3
        jne r2, 0xffffffd6, fail
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
fn test_mod() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 5748
        mod32 r0, 92
        jne r0, 44, exit
        mov32 r1, 13
        mod32 r0, r1
        exit",
        [],
        TestContextObject::new(6),
        ProgramResult::Ok(0x5),
    );
}

#[test]
fn test_mod32() {
    test_interpreter_and_jit_asm!(
        "
        lddw r0, 0x100000003
        mod32 r0, 3
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x0),
    );
}

#[test]
fn test_mod64() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0xb1858436
        lsh r0, 32
        or r0, 0x100dc5c8
        mov32 r1, 0xdde263e
        lsh r1, 32
        or r1, 0x3cbef7f3
        mod r0, r1
        mod r0, 0x658f1778
        exit",
        [],
        TestContextObject::new(9),
        ProgramResult::Ok(0x30ba5a04),
    );
}

#[test]
fn test_rfc9669_mod32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 100
        mod32 r1, 42
        jne32 r1, 16, fail
        mov32 r2, 100
        mov32 r3, 42
        mod32 r2, r3
        jne32 r2, 16, fail
        mov32 r4, 100
        mov32 r5, 0
        mod32 r4, r5
        jne32 r4, 100, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(15),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_mod64() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 100
        mod r1, 42
        jne r1, 16, fail
        mov r2, 100
        mov r3, 42
        mod r2, r3
        jne r2, 16, fail
        mov r4, 100
        mov r5, 0
        mod r4, r5
        jne r4, 100, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(15),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_srem32_intmin_by_negone_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0x80000000
        srem32 r0, -1
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x0),
    );
}

#[test]
fn test_srem32_intmin_by_negone_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0x80000000
        mov32 r1, -1
        srem32 r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x0),
    );
}

#[test]
fn test_srem32_neg_by_neg_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, -13
        srem32 r0, -4
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0xffffffff),
    );
}

#[test]
fn test_srem32_neg_by_neg_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, -13
        mov32 r1, -4
        srem32 r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0xffffffff),
    );
}

#[test]
fn test_srem32_neg_by_pos_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, -13
        srem32 r0, 4
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0xffffffff),
    );
}

#[test]
fn test_srem32_neg_by_pos_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, -13
        mov32 r1, 4
        srem32 r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0xffffffff),
    );
}

#[test]
fn test_srem32_neg_by_zero_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, -13
        srem32 r0, 0
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0xfffffff3),
    );
}

#[test]
fn test_srem32_neg_by_zero_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, -13
        mov32 r1, 0
        srem32 r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0xfffffff3),
    );
}

#[test]
fn test_srem32_pos_by_neg_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 13
        srem32 r0, -4
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_srem32_pos_by_neg_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 13
        mov32 r1, -4
        srem32 r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_srem64_intmin_by_negone_imm() {
    test_interpreter_and_jit_asm!(
        "
        lddw r0, 0x8000000000000000
        srem64 r0, -1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x0),
    );
}

#[test]
fn test_srem64_intmin_by_negone_reg() {
    test_interpreter_and_jit_asm!(
        "
        lddw r0, 0x8000000000000000
        mov r1, -1
        srem64 r0, r1
        exit",
        [],
        TestContextObject::new(5),
        ProgramResult::Ok(0x0),
    );
}

#[test]
fn test_srem64_neg_by_neg_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, -13
        srem64 r0, -4
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0xffffffffffffffff),
    );
}

#[test]
fn test_srem64_neg_by_neg_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, -13
        mov r1, -4
        srem64 r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0xffffffffffffffff),
    );
}

#[test]
fn test_srem64_neg_by_pos_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, -13
        srem64 r0, 4
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0xffffffffffffffff),
    );
}

#[test]
fn test_srem64_neg_by_pos_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, -13
        mov r1, 4
        srem64 r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0xffffffffffffffff),
    );
}

#[test]
fn test_srem64_neg_by_zero_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, -13
        srem64 r0, 0
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0xfffffffffffffff3),
    );
}

#[test]
fn test_srem64_neg_by_zero_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, -13
        mov r1, 0
        srem64 r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0xfffffffffffffff3),
    );
}

#[test]
fn test_srem64_pos_by_neg_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 13
        srem64 r0, -4
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_srem64_pos_by_neg_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 13
        mov r1, -4
        srem64 r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_srem32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, -10
        srem32 r1, 3
        jne32 r1, 0xffffffff, fail
        mov32 r2, -10
        mov32 r3, 3
        srem32 r2, r3
        jne32 r2, 0xffffffff, fail
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
fn test_rfc9669_srem64() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, -10
        srem64 r1, 3
        jne r1, 0xffffffff, fail
        mov r2, -10
        mov r3, 3
        srem64 r2, r3
        jne r2, 0xffffffff, fail
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
