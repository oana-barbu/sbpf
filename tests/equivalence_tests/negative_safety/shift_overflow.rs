#![allow(clippy::arithmetic_side_effects)]
#![cfg(all(feature = "jit", not(target_os = "windows"), target_arch = "x86_64"))]

extern crate test_utils;

use solana_sbpf::{
    assembler::assemble,
    error::EbpfError,
    program::BuiltinProgram,
    verifier::{RequisiteVerifier, VerifierError},
    vm::Config,
};
use std::sync::Arc;
use test_utils::TestContextObject;

#[test]
fn test_arsh32_imm_high() {
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(Config::default()));
    let executable = assemble::<TestContextObject>(
        "
        mov32 r0, 0xf8
        lsh32 r0, 28
        arsh32 r0, 48
        exit",
        loader,
    )
    .unwrap();
    let result = executable.verify::<RequisiteVerifier>();
    assert!(matches!(
        result,
        Err(EbpfError::VerifierError(VerifierError::ShiftWithOverflow(_, _, _)))
    ));
}

#[test]
fn test_arsh32_imm_neg() {
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(Config::default()));
    let executable = assemble::<TestContextObject>(
        "
        mov32 r0, 0xf8
        lsh32 r0, 28
        arsh32 r0, -16
        exit",
        loader,
    )
    .unwrap();
    let result = executable.verify::<RequisiteVerifier>();
    assert!(matches!(
        result,
        Err(EbpfError::VerifierError(VerifierError::ShiftWithOverflow(_, _, _)))
    ));
}

#[test]
fn test_arsh64_imm_high() {
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(Config::default()));
    let executable = assemble::<TestContextObject>(
        "
        mov32 r0, 1
        lsh r0, 63
        arsh r0, 124
        exit",
        loader,
    )
    .unwrap();
    let result = executable.verify::<RequisiteVerifier>();
    assert!(matches!(
        result,
        Err(EbpfError::VerifierError(VerifierError::ShiftWithOverflow(_, _, _)))
    ));
}

#[test]
fn test_arsh64_imm_neg() {
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(Config::default()));
    let executable = assemble::<TestContextObject>(
        "
        mov32 r0, 1
        lsh r0, 63
        arsh r0, -4
        exit",
        loader,
    )
    .unwrap();
    let result = executable.verify::<RequisiteVerifier>();
    assert!(matches!(
        result,
        Err(EbpfError::VerifierError(VerifierError::ShiftWithOverflow(_, _, _)))
    ));
}

#[test]
fn test_lsh32_imm_high() {
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(Config::default()));
    let executable = assemble::<TestContextObject>(
        "
        mov r0, 0x11
        lsh32 r0, 60
        exit",
        loader,
    )
    .unwrap();
    let result = executable.verify::<RequisiteVerifier>();
    assert!(matches!(
        result,
        Err(EbpfError::VerifierError(VerifierError::ShiftWithOverflow(_, _, _)))
    ));
}

#[test]
fn test_lsh32_imm_neg() {
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(Config::default()));
    let executable = assemble::<TestContextObject>(
        "
        mov r0, 0x11
        lsh32 r0, -4
        exit",
        loader,
    )
    .unwrap();
    let result = executable.verify::<RequisiteVerifier>();
    assert!(matches!(
        result,
        Err(EbpfError::VerifierError(VerifierError::ShiftWithOverflow(_, _, _)))
    ));
}

#[test]
fn test_lsh64_imm_high() {
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(Config::default()));
    let executable = assemble::<TestContextObject>(
        "
        mov r0, 0x1
        lsh r0, 68
        exit",
        loader,
    )
    .unwrap();
    let result = executable.verify::<RequisiteVerifier>();
    assert!(matches!(
        result,
        Err(EbpfError::VerifierError(VerifierError::ShiftWithOverflow(_, _, _)))
    ));
}

#[test]
fn test_lsh64_imm_neg() {
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(Config::default()));
    let executable = assemble::<TestContextObject>(
        "
        mov r0, 0x1
        lsh r0, -60
        exit",
        loader,
    )
    .unwrap();
    let result = executable.verify::<RequisiteVerifier>();
    assert!(matches!(
        result,
        Err(EbpfError::VerifierError(VerifierError::ShiftWithOverflow(_, _, _)))
    ));
}

#[test]
fn test_rsh32_imm_high() {
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(Config::default()));
    let executable = assemble::<TestContextObject>(
        "
        mov32 r0, 0x10000000
        rsh32 r0, 60
        exit",
        loader,
    )
    .unwrap();
    let result = executable.verify::<RequisiteVerifier>();
    assert!(matches!(
        result,
        Err(EbpfError::VerifierError(VerifierError::ShiftWithOverflow(_, _, _)))
    ));
}

#[test]
fn test_rsh32_imm_neg() {
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(Config::default()));
    let executable = assemble::<TestContextObject>(
        "
        mov32 r0, 0x10000000
        rsh32 r0, -4
        exit",
        loader,
    )
    .unwrap();
    let result = executable.verify::<RequisiteVerifier>();
    assert!(matches!(
        result,
        Err(EbpfError::VerifierError(VerifierError::ShiftWithOverflow(_, _, _)))
    ));
}

#[test]
fn test_rsh64_imm_high() {
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(Config::default()));
    let executable = assemble::<TestContextObject>(
        "
        mov r0, 0x10
        rsh r0, 68
        exit",
        loader,
    )
    .unwrap();
    let result = executable.verify::<RequisiteVerifier>();
    assert!(matches!(
        result,
        Err(EbpfError::VerifierError(VerifierError::ShiftWithOverflow(_, _, _)))
    ));
}

#[test]
fn test_rsh64_imm_neg() {
    let loader = Arc::new(BuiltinProgram::<TestContextObject>::new_loader(Config::default()));
    let executable = assemble::<TestContextObject>(
        "
        mov r0, 0x10
        rsh r0, -60
        exit",
        loader,
    )
    .unwrap();
    let result = executable.verify::<RequisiteVerifier>();
    assert!(matches!(
        result,
        Err(EbpfError::VerifierError(VerifierError::ShiftWithOverflow(_, _, _)))
    ));
}
