#![allow(clippy::arithmetic_side_effects)]
#![cfg(all(feature = "jit", not(target_os = "windows"), target_arch = "x86_64"))]

//! Tests for SBPF-specific ALU behavior that differs from standard eBPF.

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
fn test_sub32_overflow() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0x7FFFFFFF
        sub32 r0, -1
        exit",
        [],
        TestContextObject::new(3),
        // Standard eBPF: 0x0000000080000000 (zero-extended)
        // SBPF: 0xFFFFFFFF80000000 (sign-extended)
        ProgramResult::Ok(0xFFFFFFFF80000000),
    );
}
