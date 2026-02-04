#![allow(clippy::arithmetic_side_effects)]
#![cfg(all(feature = "jit", not(target_os = "windows"), target_arch = "x86_64"))]

extern crate test_utils;

use solana_sbpf::{
    assembler::assemble,
    ebpf,
    error::{EbpfError, ProgramResult},
    memory_region::MemoryRegion,
    program::{BuiltinProgram, SBPFVersion},
    static_analysis::Analysis,
    verifier::{RequisiteVerifier, VerifierError},
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

fn v2_config() -> Config {
    Config {
        enabled_sbpf_versions: SBPFVersion::V2..=SBPFVersion::V2,
        ..Config::default()
    }
}

#[test]
fn test_sdiv32_by_zero_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 1
        mov32 r1, 0
        sdiv32 r0, r1
        exit",
        v2_config(),
        [],
        TestContextObject::new(3),
        ProgramResult::Err(EbpfError::DivideByZero),
    );
}

#[test]
fn test_sdiv64_by_zero_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 1
        mov r1, 0
        sdiv r0, r1
        exit",
        v2_config(),
        [],
        TestContextObject::new(3),
        ProgramResult::Err(EbpfError::DivideByZero),
    );
}

#[test]
fn test_srem32_by_zero_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, -13
        mov32 r1, 0
        srem32 r0, r1
        exit",
        v2_config(),
        [],
        TestContextObject::new(3),
        ProgramResult::Err(EbpfError::DivideByZero),
    );
}

#[test]
fn test_srem64_by_zero_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, -13
        mov r1, 0
        srem r0, r1
        exit",
        v2_config(),
        [],
        TestContextObject::new(3),
        ProgramResult::Err(EbpfError::DivideByZero),
    );
}

#[test]
fn test_sdiv32_by_zero_imm() {
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(v2_config()));
    let executable = assemble::<TestContextObject>(
        "
        mov32 r0, 1
        sdiv32 r0, 0
        exit",
        loader,
    )
    .unwrap();
    let result = executable.verify::<RequisiteVerifier>();
    assert!(matches!(
        result,
        Err(EbpfError::VerifierError(VerifierError::DivisionByZero(_)))
    ));
}

#[test]
fn test_sdiv64_by_zero_imm() {
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(v2_config()));
    let executable = assemble::<TestContextObject>(
        "
        mov r0, 1
        sdiv r0, 0
        exit",
        loader,
    )
    .unwrap();
    let result = executable.verify::<RequisiteVerifier>();
    assert!(matches!(
        result,
        Err(EbpfError::VerifierError(VerifierError::DivisionByZero(_)))
    ));
}

#[test]
fn test_srem32_by_zero_imm() {
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(v2_config()));
    let executable = assemble::<TestContextObject>(
        "
        mov32 r0, -13
        srem32 r0, 0
        exit",
        loader,
    )
    .unwrap();
    let result = executable.verify::<RequisiteVerifier>();
    assert!(matches!(
        result,
        Err(EbpfError::VerifierError(VerifierError::DivisionByZero(_)))
    ));
}

#[test]
fn test_srem64_by_zero_imm() {
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(v2_config()));
    let executable = assemble::<TestContextObject>(
        "
        mov r0, -13
        srem r0, 0
        exit",
        loader,
    )
    .unwrap();
    let result = executable.verify::<RequisiteVerifier>();
    assert!(matches!(
        result,
        Err(EbpfError::VerifierError(VerifierError::DivisionByZero(_)))
    ));
}
