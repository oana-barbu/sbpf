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
fn test_rfc9669_jsge() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, -1
        jsge r1, -1, ok1
        ja fail
        ok1:
        mov r2, -1
        mov r3, -1
        jsge r2, r3, ok2
        ja fail
        ok2:
        mov r4, -1
        jsge32 r4, -1, ok3
        ja fail
        ok3:
        mov r5, -1
        mov r6, -1
        jsge32 r5, r6, ok4
        ja fail
        ok4:
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(12),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_jsgt() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, -1
        jsgt r1, -2, ok1
        ja fail
        ok1:
        mov r2, -1
        mov r3, 1
        jsgt r2, r3, fail
        ja ok2
        ok2:
        mov r4, -1
        jsgt32 r4, 0, fail
        ja ok3
        ok3:
        mov r5, -1
        mov r6, 0
        jsgt32 r5, r6, fail
        ja ok4
        ok4:
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(15),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_jsle() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, -1
        jsle r1, -1, ok1
        ja fail
        ok1:
        mov r2, -2
        mov r3, -1
        jsle r2, r3, ok2
        ja fail
        ok2:
        mov r4, -1
        jsle32 r4, 0, ok3
        ja fail
        ok3:
        mov r5, -1
        mov r6, 0
        jsle32 r5, r6, ok4
        ja fail
        ok4:
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(12),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_jslt() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, -2
        jslt r1, -1, ok1
        ja fail
        ok1:
        mov r2, -2
        mov r3, -1
        jslt r2, r3, ok2
        ja fail
        ok2:
        mov r4, -1
        jslt32 r4, 0, ok3
        ja fail
        ok3:
        mov r5, -1
        mov r6, 0
        jslt32 r5, r6, ok4
        ja fail
        ok4:
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(12),
        ProgramResult::Ok(0x1),
    );
}
