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
fn test_mov() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 2
        mov32 r0, r1
        jne r0, 2, end
        lddw r2, 0xFFFFFF00000002
        mov32 r0, r2
        jne r0, 2, end
        mov32 r0, 1
        end:
        exit",
        [],
        TestContextObject::new(8),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_mov64_sign_extend() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, -10
        exit",
        [],
        TestContextObject::new(2),
        ProgramResult::Ok(0xFFFFFFFFFFFFFFF6),
    );
}

#[test]
fn test_mov64() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 1
        mov r0, r1
        mov r0, r0
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jit_bounce() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 1
        mov r6, r0
        mov r7, r6
        mov r8, r7
        mov r9, r8
        mov r0, r9
        exit",
        [],
        TestContextObject::new(7),
        ProgramResult::Ok(0x1),
    );
}
