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
fn test_add32_self() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 5
        add32 r0, r0
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0xa),
    );
}

#[test]
fn test_add64_self() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 5
        add r0, r0
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0xa),
    );
}

#[test]
fn test_sub32_self() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 5
        sub32 r0, r0
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x0),
    );
}

#[test]
fn test_sub64_self() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 5
        sub r0, r0
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x0),
    );
}


#[test]
fn test_neg32_intmin() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, -2147483648
        neg32 r0
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x80000000),
    );
}

#[test]
fn test_neg64_intmin() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 1
        lsh r0, 63
        neg r0
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x8000000000000000),
    );
}

