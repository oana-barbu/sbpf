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
fn test_exit_not_last() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 0
        ja L2
        L1:
        mov r2, 0
        exit
        L2:
        mov r0, 0
        ja L1",
        [],
        TestContextObject::new(6),
        ProgramResult::Ok(0x0),
    );
}

#[test]
fn test_exit() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0
        exit",
        [],
        TestContextObject::new(2),
        ProgramResult::Ok(0x0),
    );
}

#[test]
fn test_rfc9669_exit() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 1
        exit",
        [],
        TestContextObject::new(2),
        ProgramResult::Ok(0x1),
    );
}
