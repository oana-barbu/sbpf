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
fn test_ldxb_all() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, r1
        ldxb r9, [r0+0]
        lsh r9, 0
        ldxb r8, [r0+0x1]
        lsh r8, 4
        ldxb r7, [r0+2]
        lsh r7, 8
        ldxb r6, [r0+3]
        lsh r6, 12
        ldxb r5, [r0+4]
        lsh r5, 16
        ldxb r4, [r0+5]
        lsh r4, 20
        ldxb r3, [r0+6]
        lsh r3, 24
        ldxb r2, [r0+7]
        lsh r2, 28
        ldxb r1, [r0+8]
        lsh r1, 32
        ldxb r0, [r0+9]
        lsh r0, 36
        or r0, r1
        or r0, r2
        or r0, r3
        or r0, r4
        or r0, r5
        or r0, r6
        or r0, r7
        or r0, r8
        or r0, r9
        exit",
        [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09],
        TestContextObject::new(31),
        ProgramResult::Ok(0x9876543210),
    );
}

#[test]
fn test_ldxb() {
    test_interpreter_and_jit_asm!(
        "
        ldxb r0, [r1+0x2]
        exit",
        [0xaa, 0xbb, 0x11, 0xcc, 0xdd],
        TestContextObject::new(2),
        ProgramResult::Ok(0x11),
    );
}

#[test]
fn test_rfc9669_ldxb() {
    test_interpreter_and_jit_asm!(
        "
        stb [r10-1], 0xff
        ldxb r0, [r10-1]
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0xff),
    );
}

#[test]
fn test_ldxh_all() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, r1
        ldxh r9, [r0+0]
        be16 r9
        lsh r9, 0
        ldxh r8, [r0+2]
        be16 r8
        lsh r8, 4
        ldxh r7, [r0+4]
        be16 r7
        lsh r7, 8
        ldxh r6, [r0+6]
        be16 r6
        lsh r6, 12
        ldxh r5, [r0+8]
        be16 r5
        lsh r5, 16
        ldxh r4, [r0+10]
        be16 r4
        lsh r4, 20
        ldxh r3, [r0+12]
        be16 r3
        lsh r3, 24
        ldxh r2, [r0+14]
        be16 r2
        lsh r2, 28
        ldxh r1, [r0+16]
        be16 r1
        lsh r1, 32
        ldxh r0, [r0+18]
        be16 r0
        lsh r0, 36
        or r0, r1
        or r0, r2
        or r0, r3
        or r0, r4
        or r0, r5
        or r0, r6
        or r0, r7
        or r0, r8
        or r0, r9
        exit",
        [0x00, 0x00, 0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00, 0x09],
        TestContextObject::new(41),
        ProgramResult::Ok(0x9876543210),
    );
}

#[test]
fn test_ldxh_all2() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, r1
        ldxh r9, [r0+0]
        be16 r9
        ldxh r8, [r0+2]
        be16 r8
        ldxh r7, [r0+4]
        be16 r7
        ldxh r6, [r0+6]
        be16 r6
        ldxh r5, [r0+8]
        be16 r5
        ldxh r4, [r0+10]
        be16 r4
        ldxh r3, [r0+12]
        be16 r3
        ldxh r2, [r0+14]
        be16 r2
        ldxh r1, [r0+16]
        be16 r1
        ldxh r0, [r0+18]
        be16 r0
        or r0, r1
        or r0, r2
        or r0, r3
        or r0, r4
        or r0, r5
        or r0, r6
        or r0, r7
        or r0, r8
        or r0, r9
        exit",
        [0x00, 0x01, 0x00, 0x02, 0x00, 0x04, 0x00, 0x08, 0x00, 0x10, 0x00, 0x20, 0x00, 0x40, 0x00, 0x80, 0x01, 0x00, 0x02, 0x00],
        TestContextObject::new(31),
        ProgramResult::Ok(0x3ff),
    );
}

#[test]
fn test_ldxh_same_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, r1
        sth [r0+0], 0x1234
        ldxh r0, [r0+0]
        exit",
        [0xff, 0xff],
        TestContextObject::new(4),
        ProgramResult::Ok(0x1234),
    );
}

#[test]
fn test_ldxh() {
    test_interpreter_and_jit_asm!(
        "
        ldxh r0, [r1+2]
        exit",
        [0xaa, 0xbb, 0x11, 0x22, 0xcc, 0xdd],
        TestContextObject::new(2),
        ProgramResult::Ok(0x2211),
    );
}

#[test]
fn test_rfc9669_ldxh() {
    test_interpreter_and_jit_asm!(
        "
        sth [r10-2], 0x8001
        ldxh r0, [r10-2]
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x8001),
    );
}

#[test]
fn test_ldxw_all() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, r1
        ldxw r9, [r0+0]
        be32 r9
        ldxw r8, [r0+4]
        be32 r8
        ldxw r7, [r0+8]
        be32 r7
        ldxw r6, [r0+12]
        be32 r6
        ldxw r5, [r0+16]
        be32 r5
        ldxw r4, [r0+20]
        be32 r4
        ldxw r3, [r0+24]
        be32 r3
        ldxw r2, [r0+28]
        be32 r2
        ldxw r1, [r0+32]
        be32 r1
        ldxw r0, [r0+36]
        be32 r0
        or r0, r1
        or r0, r2
        or r0, r3
        or r0, r4
        or r0, r5
        or r0, r6
        or r0, r7
        or r0, r8
        or r0, r9
        exit",
        [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00],
        TestContextObject::new(31),
        ProgramResult::Ok(0x030f0f),
    );
}

#[test]
fn test_ldxw() {
    test_interpreter_and_jit_asm!(
        "
        ldxw r0, [r1+2]
        exit",
        [0xaa, 0xbb, 0x11, 0x22, 0x33, 0x44, 0xcc, 0xdd],
        TestContextObject::new(2),
        ProgramResult::Ok(0x44332211),
    );
}

#[test]
fn test_rfc9669_ldxw() {
    test_interpreter_and_jit_asm!(
        "
        stw [r10-4], 0x11223344
        ldxw r0, [r10-4]
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x11223344),
    );
}

#[test]
fn test_ldxdw() {
    test_interpreter_and_jit_asm!(
        "
        ldxdw r0, [r1+2]
        exit",
        [0xaa, 0xbb, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0xcc, 0xdd],
        TestContextObject::new(2),
        ProgramResult::Ok(0x8877665544332211),
    );
}

#[test]
fn test_rfc9669_ldxdw() {
    test_interpreter_and_jit_asm!(
        "
        lddw r1, 0x1122334455667788
        stxdw [r10-8], r1
        ldxdw r0, [r10-8]
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x1122334455667788),
    );
}

#[test]
fn test_mem_len() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, r2
        exit",
        [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02],
        TestContextObject::new(2),
        ProgramResult::Ok(0x8),
    );
}
