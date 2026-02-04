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
fn test_call_local() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0
        mov r1, 1
        mov r2, 2
        mov r3, 3
        mov r4, 4
        mov r5, 5
        mov r6, 6
        mov r7, 7
        mov r8, 8
        mov r9, 9
        call func1
        jne r0, 15, failed
        jne r6, 6, failed
        jne r7, 7, failed
        jne r8, 8, failed
        jne r9, 9, failed
        mov r0, 1
        exit
        failed:
        mov r0, -1
        exit
        func1:
        mov r0, 0
        add r0, r1
        add r0, r2
        add r0, r3
        add r0, r4
        add r0, r5
        mov r6, 0
        mov r7, 0
        mov r8, 0
        mov r9, 0
        exit",
        [],
        TestContextObject::new(29),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_call_unwind_fail() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, -1
        call 5
        mov r0, 2
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x2),
    );
}

#[test]
fn test_callx() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, -1
        mov r2, 5
        call r2
        mov r0, 2
        exit",
        [],
        TestContextObject::new(5),
        ProgramResult::Ok(0x2),
    );
}

#[test]
fn test_rfc9669_call_local() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 1
        mov r2, 2
        mov r3, 3
        mov r4, 4
        mov r5, 5
        call func1
        jne r0, 15, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit
        func1:
        mov r0, 0
        add r0, r1
        add r0, r2
        add r0, r3
        add r0, r4
        add r0, r5
        exit",
        [],
        TestContextObject::new(16),
        ProgramResult::Ok(0x1),
    );
}
