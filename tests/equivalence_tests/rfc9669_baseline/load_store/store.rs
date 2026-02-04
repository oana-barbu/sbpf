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
fn test_stb() {
    test_interpreter_and_jit_asm!(
        "
        stb [r1+2], 0x11
        ldxb r0, [r1+2]
        exit",
        [0xaa, 0xbb, 0xff, 0xcc, 0xdd],
        TestContextObject::new(3),
        ProgramResult::Ok(0x11),
    );
}

#[test]
fn test_rfc9669_stb() {
    test_interpreter_and_jit_asm!(
        "
        stb [r10-1], 0x42
        ldxb r0, [r10-1]
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x42),
    );
}

#[test]
fn test_sth() {
    test_interpreter_and_jit_asm!(
        "
        sth [r1+2], 0x2211
        ldxh r0, [r1+2]
        exit",
        [0xaa, 0xbb, 0xff, 0xff, 0xcc, 0xdd],
        TestContextObject::new(3),
        ProgramResult::Ok(0x2211),
    );
}

#[test]
fn test_rfc9669_sth() {
    test_interpreter_and_jit_asm!(
        "
        sth [r10-2], 0x1234
        ldxh r0, [r10-2]
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x1234),
    );
}

#[test]
fn test_stw() {
    test_interpreter_and_jit_asm!(
        "
        stw [r1+2], 0x44332211
        ldxw r0, [r1+2]
        exit",
        [0xaa, 0xbb, 0xff, 0xff, 0xff, 0xff, 0xcc, 0xdd],
        TestContextObject::new(3),
        ProgramResult::Ok(0x44332211),
    );
}

#[test]
fn test_rfc9669_stw() {
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
fn test_stdw() {
    test_interpreter_and_jit_asm!(
        "
        stdw [r1+2], 0x44332211
        ldxdw r0, [r1+2]
        exit",
        [0xaa, 0xbb, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xcc, 0xdd],
        TestContextObject::new(3),
        ProgramResult::Ok(0x0000000044332211),
    );
}

#[test]
fn test_rfc9669_stdw() {
    test_interpreter_and_jit_asm!(
        "
        stdw [r10-8], 0x11223344
        ldxdw r0, [r10-8]
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x11223344),
    );
}

#[test]
fn test_stxb_all() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0xf0
        mov r2, 0xf2
        mov r3, 0xf3
        mov r4, 0xf4
        mov r5, 0xf5
        mov r6, 0xf6
        mov r7, 0xf7
        mov r8, 0xf8
        stxb [r1+0], r0
        stxb [r1+1], r2
        stxb [r1+2], r3
        stxb [r1+3], r4
        stxb [r1+4], r5
        stxb [r1+5], r6
        stxb [r1+6], r7
        stxb [r1+7], r8
        ldxdw r0, [r1+0]
        be64 r0
        exit",
        [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        TestContextObject::new(19),
        ProgramResult::Ok(0xf0f2f3f4f5f6f7f8),
    );
}

#[test]
fn test_stxb_all2() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, r1
        mov r1, 0xf1
        mov r9, 0xf9
        stxb [r0+0], r1
        stxb [r0+1], r9
        ldxh r0, [r0+0]
        be16 r0
        exit",
        [0xff, 0xff],
        TestContextObject::new(8),
        ProgramResult::Ok(0xf1f9),
    );
}

#[test]
fn test_stxb_chain() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, r1
        ldxb r9, [r0+0]
        stxb [r0+1], r9
        ldxb r8, [r0+1]
        stxb [r0+2], r8
        ldxb r7, [r0+2]
        stxb [r0+3], r7
        ldxb r6, [r0+3]
        stxb [r0+4], r6
        ldxb r5, [r0+4]
        stxb [r0+5], r5
        ldxb r4, [r0+5]
        stxb [r0+6], r4
        ldxb r3, [r0+6]
        stxb [r0+7], r3
        ldxb r2, [r0+7]
        stxb [r0+8], r2
        ldxb r1, [r0+8]
        stxb [r0+9], r1
        ldxb r0, [r0+9]
        exit",
        [0x2a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        TestContextObject::new(21),
        ProgramResult::Ok(0x2a),
    );
}

#[test]
fn test_stxb() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r2, 0x11
        stxb [r1+2], r2
        ldxb r0, [r1+2]
        exit",
        [0xaa, 0xbb, 0xff, 0xcc, 0xdd],
        TestContextObject::new(4),
        ProgramResult::Ok(0x11),
    );
}

#[test]
fn test_rfc9669_stxb() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 0x42
        stxb [r10-1], r1
        ldxb r0, [r10-1]
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x42),
    );
}

#[test]
fn test_stxh() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r2, 0x2211
        stxh [r1+2], r2
        ldxh r0, [r1+2]
        exit",
        [0xaa, 0xbb, 0xff, 0xff, 0xcc, 0xdd],
        TestContextObject::new(4),
        ProgramResult::Ok(0x2211),
    );
}

#[test]
fn test_rfc9669_stxh() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 0x1234
        stxh [r10-2], r1
        ldxh r0, [r10-2]
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x1234),
    );
}

#[test]
fn test_stxw() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r2, 0x44332211
        stxw [r1+2], r2
        ldxw r0, [r1+2]
        exit",
        [0xaa, 0xbb, 0xff, 0xff, 0xff, 0xff, 0xcc, 0xdd],
        TestContextObject::new(4),
        ProgramResult::Ok(0x44332211),
    );
}

#[test]
fn test_rfc9669_stxw() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 0x11223344
        stxw [r10-4], r1
        ldxw r0, [r10-4]
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x11223344),
    );
}

#[test]
fn test_rfc9669_stxdw() {
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
