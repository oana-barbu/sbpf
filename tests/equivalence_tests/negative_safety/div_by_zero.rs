#![allow(clippy::arithmetic_side_effects)]
#![cfg(all(feature = "jit", not(target_os = "windows"), target_arch = "x86_64"))]

extern crate test_utils;

use solana_sbpf::{
    assembler::assemble,
    ebpf,
    error::{EbpfError, ProgramResult},
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
fn test_div32_by_zero_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 1
        mov32 r1, 0
        div32 r0, r1
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Err(EbpfError::DivideByZero),
    );
}

#[test]
fn test_div32_by_zero_reg_high_bits() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 1
        lddw r1, 0x100000000
        div32 r0, r1
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Err(EbpfError::DivideByZero),
    );
}

#[test]
fn test_div64_by_zero_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 1
        mov32 r1, 0
        div r0, r1
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Err(EbpfError::DivideByZero),
    );
}

#[test]
fn test_mod32_by_zero_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 1
        mov32 r1, 0
        mod32 r0, r1
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Err(EbpfError::DivideByZero),
    );
}

#[test]
fn test_mod64_by_zero_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 1
        mov32 r1, 0
        mod r0, r1
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Err(EbpfError::DivideByZero),
    );
}
