#![allow(clippy::arithmetic_side_effects)]
#![cfg(all(feature = "jit", not(target_os = "windows"), target_arch = "x86_64"))]

use solana_sbpf::error::ProgramResult;
use test_utils::{test_interpreter_and_jit_asm, TestContextObject};

#[test]
fn test_jsge_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov r1, 0xfffffffe
        jsge r1, 0xffffffff, exit
        jsge r1, 0, +4
        mov32 r0, 1
        mov r1, 0xffffffff
        jsge r1, 0xffffffff, +1
        mov32 r0, 2
        exit",
        [],
        TestContextObject::new(9),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jsge_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov r1, 0xfffffffe
        mov r2, 0xffffffff
        mov32 r3, 0
        jsge r1, r2, exit
        jsge r1, r3, exit
        jsge r1, r1, +1
        exit
        mov32 r0, 1
        mov r1, r2
        jsge r1, r2, +1
        mov32 r0, 2
        exit",
        [],
        TestContextObject::new(13),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jsge32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xfffffffe
        or r1, r9
        jsge32 r1, 0xffffffff, exit
        jsge32 r1, 0, exit
        mov32 r0, 1
        mov r1, 0xffffffff
        jsge32 r1, 0xffffffff, +1
        mov32 r0, 2
        exit",
        [],
        TestContextObject::new(12),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jsge32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xfffffffe
        or r1, r9
        mov r2, 0xffffffff
        mov32 r3, 0
        jsge32 r1, r2, exit
        jsge32 r1, r3, exit
        jsge32 r1, r1, +1
        exit
        mov32 r0, 1
        mov r1, r2
        jsge32 r1, r2, +1
        mov32 r0, 2
        exit",
        [],
        TestContextObject::new(16),
        ProgramResult::Ok(0x1),
    );
}

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
        TestContextObject::new(18),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jsgt_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov r1, 0xfffffffe
        jsgt r1, 0xffffffff, exit
        mov32 r0, 1
        mov32 r1, 0
        jsgt r1, 0xffffffff, +1
        mov32 r0, 2
        exit",
        [],
        TestContextObject::new(8),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jsgt_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov r1, 0xfffffffe
        mov r2, 0xffffffff
        jsgt r1, r2, exit
        jsgt r1, r1, exit
        mov32 r0, 1
        mov32 r1, 0
        jsgt r1, r2, +1
        mov32 r0, 2
        exit",
        [],
        TestContextObject::new(10),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jsgt32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xfffffffe
        or r1, r9
        jsgt32 r1, 0xffffffff, exit
        mov32 r0, 1
        mov32 r1, 0
        jsgt32 r1, 0xffffffff, +1
        mov32 r0, 2
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jsgt32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xfffffffe
        or r1, r9
        mov r2, 0xffffffff
        jsgt32 r1, r2, exit
        jsgt32 r1, r1, exit
        mov32 r0, 1
        mov32 r1, 0
        jsgt32 r1, r2, +1
        mov32 r0, 2
        exit",
        [],
        TestContextObject::new(13),
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
        TestContextObject::new(18),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jsle_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov r1, 0xfffffffe
        jsle r1, 0xfffffffd, exit
        jsle r1, 0xffffffff, +1
        exit
        mov32 r0, 1
        jsle r1, 0xfffffffe, +1
        mov32 r0, 2
        exit",
        [],
        TestContextObject::new(9),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jsle_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov r1, 0xffffffff
        mov r2, 0xfffffffe
        mov32 r3, 0
        jsle r1, r2, exit
        jsle r1, r3, +1
        exit
        jsle r1, r1, +1
        exit
        mov32 r0, 1
        mov r1, r2
        jsle r1, r2, +1
        mov32 r0, 2
        exit",
        [],
        TestContextObject::new(14),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jsle32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xfffffffe
        or r1, r9
        jsle32 r1, 0xfffffffd, exit
        jsle32 r1, 0xffffffff, +1
        exit
        mov32 r0, 1
        jsle32 r1, 0xfffffffe, +1
        mov32 r0, 2
        exit",
        [],
        TestContextObject::new(12),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jsle32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xffffffff
        or r1, r9
        mov r2, 0xfffffffe
        mov32 r3, 0
        jsle32 r1, r2, exit
        jsle32 r1, r3, +1
        exit
        jsle32 r1, r1, +1
        exit
        mov32 r0, 1
        mov r1, r2
        jsle32 r1, r2, +1
        mov32 r0, 2
        exit",
        [],
        TestContextObject::new(17),
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
        TestContextObject::new(18),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jslt_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov r1, 0xfffffffe
        jslt r1, 0xfffffffd, exit
        jslt r1, 0xfffffffe, exit
        jslt r1, 0xffffffff, +1
        exit
        mov32 r0, 1
        exit",
        [],
        TestContextObject::new(8),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jslt_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov r1, 0xfffffffe
        mov r2, 0xfffffffd
        mov r3, 0xffffffff
        jslt r1, r1, exit
        jslt r1, r2, exit
        jslt r1, r3, +1
        exit
        mov32 r0, 1
        exit",
        [],
        TestContextObject::new(10),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jslt32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xfffffffe
        or r1, r9
        jslt32 r1, 0xfffffffd, exit
        jslt32 r1, 0xfffffffe, exit
        jslt32 r1, 0xffffffff, +1
        exit
        mov32 r0, 1
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jslt32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xfffffffe
        or r1, r9
        mov r2, 0xfffffffd
        mov r3, 0xffffffff
        jslt32 r1, r1, exit
        jslt32 r1, r2, exit
        jslt32 r1, r3, +1
        exit
        mov32 r0, 1
        exit",
        [],
        TestContextObject::new(13),
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
        TestContextObject::new(18),
        ProgramResult::Ok(0x1),
    );
}
