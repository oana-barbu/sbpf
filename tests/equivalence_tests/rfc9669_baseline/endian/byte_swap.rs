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
fn test_be16_high() {
    test_interpreter_and_jit_asm!(
        "
        ldxdw r0, [r1+0]
        be16 r0
        exit",
        [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
        TestContextObject::new(3),
        ProgramResult::Ok(0x1122),
    );
}

#[test]
fn test_be16() {
    test_interpreter_and_jit_asm!(
        "
        ldxh r0, [r1+0]
        be16 r0
        exit",
        [0x11, 0x22],
        TestContextObject::new(3),
        ProgramResult::Ok(0x1122),
    );
}

#[test]
fn test_be32_high() {
    test_interpreter_and_jit_asm!(
        "
        ldxdw r0, [r1+0]
        be32 r0
        exit",
        [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
        TestContextObject::new(3),
        ProgramResult::Ok(0x11223344),
    );
}

#[test]
fn test_be32() {
    test_interpreter_and_jit_asm!(
        "
        ldxw r0, [r1+0]
        be32 r0
        exit",
        [0x11, 0x22, 0x33, 0x44],
        TestContextObject::new(3),
        ProgramResult::Ok(0x11223344),
    );
}

#[test]
fn test_be64() {
    test_interpreter_and_jit_asm!(
        "
        ldxdw r0, [r1+0]
        be64 r0
        exit",
        [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
        TestContextObject::new(3),
        ProgramResult::Ok(0x1122334455667788),
    );
}

#[test]
fn test_rfc9669_be16() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x1122
        be16 r0
        jne r0, 0x2211, fail
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
fn test_rfc9669_be32() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x11223344
        be32 r0
        jne r0, 0x44332211, fail
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
fn test_rfc9669_be64() {
    test_interpreter_and_jit_asm!(
        "
        lddw r0, 0x1122334455667788
        be64 r0
        lddw r1, 0x8877665544332211
        jne r0, r1, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(10),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_le16_high() {
    test_interpreter_and_jit_asm!(
        "
        ldxdw r0, [r1+0]
        le16 r0
        exit",
        [0x22, 0x11, 0x00, 0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA],
        TestContextObject::new(3),
        ProgramResult::Ok(0x1122),
    );
}

#[test]
fn test_le16() {
    test_interpreter_and_jit_asm!(
        "
        ldxh r0, [r1+0]
        le16 r0
        exit",
        [0x22, 0x11],
        TestContextObject::new(3),
        ProgramResult::Ok(0x1122),
    );
}

#[test]
fn test_le32_high() {
    test_interpreter_and_jit_asm!(
        "
        ldxdw r0, [r1+0]
        le32 r0
        exit",
        [0x44, 0x33, 0x22, 0x11, 0x00, 0xFF, 0xEE, 0xDD],
        TestContextObject::new(3),
        ProgramResult::Ok(0x11223344),
    );
}

#[test]
fn test_le32() {
    test_interpreter_and_jit_asm!(
        "
        ldxw r0, [r1+0]
        le32 r0
        exit",
        [0x44, 0x33, 0x22, 0x11],
        TestContextObject::new(3),
        ProgramResult::Ok(0x11223344),
    );
}

#[test]
fn test_le64() {
    test_interpreter_and_jit_asm!(
        "
        ldxdw r0, [r1+0]
        le64 r0
        exit",
        [0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11],
        TestContextObject::new(3),
        ProgramResult::Ok(0x1122334455667788),
    );
}

#[test]
fn test_rfc9669_le16() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x1122
        le16 r0
        jne r0, 0x1122, fail
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
fn test_rfc9669_le32() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x11223344
        le32 r0
        jne r0, 0x11223344, fail
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
fn test_rfc9669_le64() {
    test_interpreter_and_jit_asm!(
        "
        lddw r0, 0x1122334455667788
        le64 r0
        lddw r1, 0x1122334455667788
        jne r0, r1, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(10),
        ProgramResult::Ok(0x1),
    );
}
