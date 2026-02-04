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
fn test_alu_bit() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov32 r1, 1
        mov32 r2, 2
        mov32 r3, 3
        mov32 r4, 4
        mov32 r5, 5
        mov32 r6, 6
        mov32 r7, 7
        mov32 r8, 8
        jne r0, 0, end
        or32 r0, r5
        or32 r0, 0xa0
        or32 r0, r0
        jne r0, 0xa5, end
        and32 r0, 0xa3
        mov32 r9, 0x91
        and32 r0, r9
        and32 r0, r0
        jne r0, 0x81, end
        lsh32 r0, 22
        lsh32 r0, r8
        jne r0, 0x40000000, end
        rsh32 r0, 19
        rsh32 r0, r7
        jne r0, 0x10, end
        xor32 r0, 0x03
        xor32 r0, r2
        end:
        exit",
        [],
        TestContextObject::new(28),
        ProgramResult::Ok(0x11),
    );
}

#[test]
fn test_alu64_bit() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0
        mov r1, 1
        mov r2, 2
        mov r3, 3
        mov r4, 4
        mov r5, 5
        mov r6, 6
        mov r7, 7
        mov r8, 8
        jne r0, 0, end
        or r0, r5
        or r0, 0xa0
        or r0, r0
        jne r0, 0xa5, end
        and r0, 0xa3
        mov r9, 0x91
        and r0, r9
        and r0, r0
        jne r0, 0x81, end
        lsh r0, 32
        lsh r0, 22
        lsh r0, r8
        rsh r0, 32
        rsh r0, 19
        rsh r0, r7
        jne r0, 0x10, end
        xor r0, 0x03
        xor r0, r2
        end:
        exit",
        [],
        TestContextObject::new(29),
        ProgramResult::Ok(0x11),
    );
}

#[test]
fn test_rfc9669_and32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 0xff00ff00
        and32 r1, 0x0f0f0f0f
        jne32 r1, 0x0f000f00, fail
        mov32 r2, 0x55555555
        mov32 r3, 0x0f0f0f0f
        and32 r2, r3
        jne32 r2, 0x05050505, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_and64() {
    test_interpreter_and_jit_asm!(
        "
        lddw r1, 0xff00ff00ff00ff00
        lddw r2, 0x0f0f0f0f0f0f0f0f
        and r1, 0x0f0f0f0f
        lddw r3, 0x000000000f000f00
        jne r1, r3, fail
        lddw r4, 0x5555555555555555
        lddw r5, 0x0f0f0f0f0f0f0f0f
        and r4, r5
        lddw r6, 0x0505050505050505
        jne r4, r6, fail
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
fn test_rfc9669_or32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 0xaa
        or32 r1, 0x55
        jne32 r1, 0xff, fail
        mov32 r2, 0xf0
        mov32 r3, 0x0f
        or32 r2, r3
        jne32 r2, 0xff, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(9),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_or64() {
    test_interpreter_and_jit_asm!(
        "
        lddw r1, 0xaaaaaaaaaaaaaaaa
        or r1, 0x55555555
        lddw r2, 0xaaaaaaaaffffffff
        jne r1, r2, fail
        lddw r3, 0xf0f0f0f0f0f0f0f0
        lddw r4, 0x0f0f0f0f0f0f0f0f
        or r3, r4
        jne r3, 0xffffffff, fail
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
fn test_rfc9669_xor32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 0xff
        xor32 r1, 0xf0
        jne32 r1, 0x0f, fail
        mov32 r2, 0xff
        mov32 r3, 0xf0
        xor32 r2, r3
        jne32 r2, 0x0f, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(9),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_xor64() {
    test_interpreter_and_jit_asm!(
        "
        lddw r1, 0xffffffffffffffff
        xor r1, 0xf0f0f0f0
        lddw r2, 0xffffffff0f0f0f0f
        jne r1, r2, fail
        lddw r3, 0xffffffffffffffff
        lddw r4, 0xf0f0f0f0f0f0f0f0
        xor r3, r4
        lddw r5, 0x0f0f0f0f0f0f0f0f
        jne r3, r5, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(13),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_arsh32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0xf8
        lsh32 r0, 28
        arsh32 r0, 16
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0xffff8000),
    );
}

#[test]
fn test_arsh32_reg_high() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0xf8
        mov32 r1, 48
        lsh32 r0, 28
        arsh32 r0, r1
        exit",
        [],
        TestContextObject::new(5),
        ProgramResult::Ok(0xffff8000),
    );
}

#[test]
fn test_arsh32_reg_neg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0xf8
        mov32 r1, -16
        lsh32 r0, 28
        arsh32 r0, r1
        exit",
        [],
        TestContextObject::new(5),
        ProgramResult::Ok(0xffff8000),
    );
}

#[test]
fn test_arsh32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0xf8
        mov32 r1, 16
        lsh32 r0, 28
        arsh32 r0, r1
        exit",
        [],
        TestContextObject::new(5),
        ProgramResult::Ok(0xffff8000),
    );
}

#[test]
fn test_arsh64_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 1
        lsh r0, 63
        arsh r0, 60
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0xfffffffffffffff8),
    );
}

#[test]
fn test_arsh64_reg_high() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 1
        lsh r0, 63
        mov32 r1, 124
        arsh r0, r1
        exit",
        [],
        TestContextObject::new(5),
        ProgramResult::Ok(0xfffffffffffffff8),
    );
}

#[test]
fn test_arsh64_reg_neg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 1
        lsh r0, 63
        mov32 r1, -4
        arsh r0, r1
        exit",
        [],
        TestContextObject::new(5),
        ProgramResult::Ok(0xfffffffffffffff8),
    );
}

#[test]
fn test_arsh64_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 1
        lsh r0, 63
        mov32 r1, 60
        arsh r0, r1
        exit",
        [],
        TestContextObject::new(5),
        ProgramResult::Ok(0xfffffffffffffff8),
    );
}

#[test]
fn test_rfc9669_arsh32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 0x80000000
        arsh32 r1, 31
        jne32 r1, 0xffffffff, fail
        mov32 r2, 0x80000000
        mov32 r3, 31
        arsh32 r2, r3
        jne32 r2, 0xffffffff, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_arsh64() {
    test_interpreter_and_jit_asm!(
        "
        lddw r1, 0x8000000000000000
        arsh r1, 63
        jne r1, 0xffffffff, fail
        lddw r2, 0x8000000000000000
        mov r3, 63
        arsh r2, r3
        jne r2, 0xffffffff, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_lsh32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x11
        lsh32 r0, 28
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x10000000),
    );
}

#[test]
fn test_lsh32_reg_high() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x11
        mov r7, 60
        lsh32 r0, r7
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x10000000),
    );
}

#[test]
fn test_lsh32_reg_neg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x11
        mov r7, -4
        lsh32 r0, r7
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x10000000),
    );
}

#[test]
fn test_lsh32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x11
        mov r7, 28
        lsh32 r0, r7
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x10000000),
    );
}

#[test]
fn test_lsh64_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x1
        lsh r0, 4
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x10),
    );
}

#[test]
fn test_lsh64_reg_high() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x1
        mov r7, 68
        lsh r0, r7
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x10),
    );
}

#[test]
fn test_lsh64_reg_neg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x1
        mov r7, -60
        lsh r0, r7
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x10),
    );
}

#[test]
fn test_lsh64_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x1
        mov r7, 4
        lsh r0, r7
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x10),
    );
}

#[test]
fn test_rfc9669_lsh32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 1
        lsh32 r1, 31
        jne32 r1, 0x80000000, fail
        mov32 r2, 1
        mov32 r3, 31
        lsh32 r2, r3
        jne32 r2, 0x80000000, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_lsh64() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 1
        lsh r1, 63
        lddw r2, 0x8000000000000000
        jne r1, r2, fail
        mov r3, 1
        mov r4, 63
        lsh r3, r4
        jne r3, r2, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(10),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rsh32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0x10000000
        rsh32 r0, 28
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rsh32_reg_high() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0x10000000
        mov32 r7, 60
        rsh32 r0, r7
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rsh32_reg_neg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0x10000000
        mov32 r7, -4
        rsh32 r0, r7
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rsh32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0x10000000
        mov32 r7, 28
        rsh32 r0, r7
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rsh64_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x10
        rsh r0, 4
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rsh64_reg_high() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x10
        mov r7, 68
        rsh r0, r7
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rsh64_reg_neg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x10
        mov r7, -60
        rsh r0, r7
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rsh64_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x10
        mov r7, 4
        rsh r0, r7
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_rsh32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 0x80000000
        rsh32 r1, 31
        jne32 r1, 1, fail
        mov32 r2, 0x80000000
        mov32 r3, 31
        rsh32 r2, r3
        jne32 r2, 1, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_rsh64() {
    test_interpreter_and_jit_asm!(
        "
        lddw r1, 0x8000000000000000
        rsh r1, 63
        jne r1, 1, fail
        lddw r2, 0x8000000000000000
        mov r3, 63
        rsh r2, r3
        jne r2, 1, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(9),
        ProgramResult::Ok(0x1),
    );
}
