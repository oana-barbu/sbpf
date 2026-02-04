#![allow(clippy::arithmetic_side_effects)]
#![cfg(all(feature = "jit", not(target_os = "windows"), target_arch = "x86_64"))]

extern crate test_utils;

use crate::common::v2_config;
use solana_sbpf::{
    assembler::assemble,
    ebpf,
    error::{EbpfError, ProgramResult},
    memory_region::MemoryRegion,
    program::BuiltinProgram,
    static_analysis::Analysis,
    verifier::RequisiteVerifier,
    vm::ContextObject,
};
use std::sync::Arc;
use test_utils::{
    compare_register_trace, create_vm, test_interpreter_and_jit, test_interpreter_and_jit_asm,
    TestContextObject,
};

#[test]
fn test_sdiv64_intmin_by_negone_imm() {
    test_interpreter_and_jit_asm!(
        "
        ldxdw r0, [r1+0]
        sdiv r0, -1
        exit",
        v2_config(),
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80],
        TestContextObject::new(2),
        ProgramResult::Err(EbpfError::DivideOverflow),
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
        v2_config(),
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80],
        TestContextObject::new(3),
        ProgramResult::Err(EbpfError::DivideOverflow),
    );
}

#[test]
fn test_srem64_intmin_by_negone_imm() {
    test_interpreter_and_jit_asm!(
        "
        ldxdw r0, [r1+0]
        srem64 r0, -1
        exit",
        v2_config(),
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80],
        TestContextObject::new(2),
        ProgramResult::Err(EbpfError::DivideOverflow),
    );
}

#[test]
fn test_srem64_intmin_by_negone_reg() {
    test_interpreter_and_jit_asm!(
        "
        ldxdw r0, [r1+0]
        mov r1, -1
        srem64 r0, r1
        exit",
        v2_config(),
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80],
        TestContextObject::new(3),
        ProgramResult::Err(EbpfError::DivideOverflow),
    );
}
