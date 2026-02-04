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

// Syscall dispatch by numeric ID is not supported in sbpf.
#[test]
#[ignore]
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

// Indirect syscall dispatch by register is not supported in sbpf.
#[test]
#[ignore]
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
