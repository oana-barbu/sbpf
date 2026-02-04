#![allow(clippy::arithmetic_side_effects)]
#![cfg(all(feature = "jit", not(target_os = "windows"), target_arch = "x86_64"))]

extern crate test_utils;

use crate::common::v2_config;
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

// Syscall dispatch by numeric ID is not supported in sbpf.
#[test]
#[ignore]
fn test_call_unwind_fail() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, -1
        call 5
        mov r0, 2
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x2),
    );
}

// Indirect syscall dispatch by register is not supported in sbpf.
#[test]
#[ignore]
fn test_callx() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, -1
        mov r2, 5
        call r2
        mov r0, 2
        exit",
        [],
        TestContextObject::new(5),
        ProgramResult::Ok(0x2),
    );
}

// Uses large immediate 0xffffffff; similar to test_mov32_imm and test_add32_sub32 in execution.rs
#[test]
#[ignore]
fn test_rfc9669_add32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 0xffffffff
        add32 r1, 1
        jne32 r1, 0, fail
        mov32 r2, 10
        add32 r2, 32
        jne32 r2, 42, fail
        mov32 r3, 40
        mov32 r4, 2
        add32 r3, r4
        jne32 r3, 42, fail
        mov r5, -1
        mov32 r5, 1
        rsh r5, 32
        jne32 r5, 0, fail
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

// Uses large immediate 0xffffffff; similar to test_add32_sub32 in execution.rs
#[test]
#[ignore]
fn test_rfc9669_sub32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 0
        sub32 r1, 1
        jne32 r1, 0xffffffff, fail
        mov32 r2, 50
        sub32 r2, 8
        jne32 r2, 42, fail
        mov32 r3, 50
        mov32 r4, 8
        sub32 r3, r4
        jne32 r3, 42, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(14),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediate 0xffffffd6; similar to test_neg in execution.rs
#[test]
#[ignore]
fn test_rfc9669_neg32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 42
        neg32 r1
        jne32 r1, 0xffffffd6, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        [],
        TestContextObject::new(7),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediate 0xff00ff00; similar to test_alu32_logic in execution.rs
#[test]
#[ignore]
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

// Uses large immediates 0x80000000 and 0xffffffff; similar to test_arsh32_imm in execution.rs
#[test]
#[ignore]
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

// Uses large immediate 0xffffffff in jne; similar to test_arsh64 in execution.rs
#[test]
#[ignore]
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

// Uses large immediate 0x80000000; similar to test_alu32_logic in execution.rs
#[test]
#[ignore]
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

// Uses large immediate 0x80000000; similar to test_alu32_logic in execution.rs
#[test]
#[ignore]
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

// Uses large immediate 0xffffffff; similar to test_alu64_logic in execution.rs
#[test]
#[ignore]
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

// Uses large immediate 0xf0f0f0f0; similar to test_alu64_logic in execution.rs
#[test]
#[ignore]
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

// Uses large immediates 0xffffffff and 0x80000000; similar to test_mov32_imm in execution.rs
#[test]
#[ignore]
fn test_rfc9669_mov32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, 42
        jne32 r1, 42, fail
        mov32 r2, -1
        jne32 r2, 0xffffffff, fail
        mov32 r3, 0x80000000
        mov32 r4, r3
        jne32 r3, r4, fail
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

// Uses large immediate 0xffffffff; similar to test_mov64_imm in execution.rs
#[test]
#[ignore]
fn test_rfc9669_mov64() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 42
        jne r1, 42, fail
        mov r2, -1
        jne r2, 0xffffffff, fail
        mov r3, r2
        jne r2, r3, fail
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

// Uses large immediate 0x80000000; similar to test_mul in execution.rs
#[test]
#[ignore]
fn test_mul32_intmin_by_negone_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x80000000
        mul32 r0, -1
        exit",
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x80000000),
    );
}

// Uses large immediate 0x80000000; similar to test_mul in execution.rs
#[test]
#[ignore]
fn test_mul32_intmin_by_negone_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0x80000000
        mov r1, -1
        mul32 r0, r1
        exit",
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x80000000),
    );
}

// Uses large immediate 0xb1858436; similar to test_mod in execution.rs
#[test]
#[ignore]
fn test_mod64() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0xb1858436
        lsh r0, 32
        or r0, 0x100dc5c8
        mov32 r1, 0xdde263e
        lsh r1, 32
        or r1, 0x3cbef7f3
        mod r0, r1
        mod r0, 0x658f1778
        exit",
        [],
        TestContextObject::new(9),
        ProgramResult::Ok(0x30ba5a04),
    );
}

// Uses large immediate 0x80000000 and tests divide by zero; similar to test_div in execution.rs
#[test]
#[ignore]
fn test_rfc9669_div64() {
    test_interpreter_and_jit_asm!(
        "
        lddw r1, 0x0000000000000054
        mov r2, 2
        div r1, r2
        jne r1, 42, fail
        mov r3, 123
        mov r4, 0
        div r3, r4
        jne r3, 0, fail
        lddw r5, 0x8000000000000000
        div r5, 0x80000000
        jne r5, 0, fail
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

// Uses large immediate 0xffffffd6 in jne32 comparison; similar to test_sdiv32_imm in this crate
#[test]
#[ignore]
fn test_rfc9669_sdiv32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, -84
        sdiv32 r1, 2
        jne32 r1, 0xffffffd6, fail
        mov32 r2, -126
        mov32 r3, 3
        sdiv32 r2, r3
        jne32 r2, 0xffffffd6, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        v2_config(),
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediate 0xffffffd6 in jne comparison; similar to test_sdiv64_imm in this crate
#[test]
#[ignore]
fn test_rfc9669_sdiv64() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, -84
        sdiv r1, 2
        jne r1, 0xffffffd6, fail
        mov r2, -126
        mov r3, 3
        sdiv r2, r3
        jne r2, 0xffffffd6, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        v2_config(),
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediate 0xffffffff in jne32 comparison; similar to test_srem32_neg_by_pos_imm in this crate
#[test]
#[ignore]
fn test_rfc9669_srem32() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r1, -10
        srem32 r1, 3
        jne32 r1, 0xffffffff, fail
        mov32 r2, -10
        mov32 r3, 3
        srem32 r2, r3
        jne32 r2, 0xffffffff, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        v2_config(),
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediate 0xffffffff in jne comparison; similar to test_srem64_neg_by_pos_imm in this crate
#[test]
#[ignore]
fn test_rfc9669_srem64() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, -10
        srem64 r1, 3
        jne r1, 0xffffffff, fail
        mov r2, -10
        mov r3, 3
        srem64 r2, r3
        jne r2, 0xffffffff, fail
        mov r0, 1
        exit
        fail:
        mov r0, 0
        exit",
        v2_config(),
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediate 0x80000000; similar to test_sdiv32_imm in this crate
#[test]
#[ignore]
fn test_sdiv32_intmin_by_negone_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0x80000000
        sdiv32 r0, -1
        exit",
        v2_config(),
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x80000000),
    );
}

// Uses large immediate 0x80000000; similar to test_sdiv32_reg in this crate
#[test]
#[ignore]
fn test_sdiv32_intmin_by_negone_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0x80000000
        mov32 r1, -1
        sdiv32 r0, r1
        exit",
        v2_config(),
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x80000000),
    );
}

// Uses large immediate 0x80000000; similar to test_srem32_neg_by_neg_imm in this crate
#[test]
#[ignore]
fn test_srem32_intmin_by_negone_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0x80000000
        srem32 r0, -1
        exit",
        v2_config(),
        [],
        TestContextObject::new(3),
        ProgramResult::Ok(0x0),
    );
}

// Uses large immediate 0x80000000; similar to test_srem32_neg_by_neg_reg in this crate
#[test]
#[ignore]
fn test_srem32_intmin_by_negone_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0x80000000
        mov32 r1, -1
        srem32 r0, r1
        exit",
        v2_config(),
        [],
        TestContextObject::new(4),
        ProgramResult::Ok(0x0),
    );
}

// Uses large immediates 0xfffffffe, 0xffffffff; similar to test_rfc9669_jsge in this crate
#[test]
#[ignore]
fn test_jsge_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov r1, 0xfffffffe
        jsge r1, 0xffffffff, end
        jsge r1, 0, +4
        mov32 r0, 1
        mov r1, 0xffffffff
        jsge r1, 0xffffffff, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(9),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediates 0xfffffffe, 0xffffffff; similar to test_rfc9669_jsge in this crate
#[test]
#[ignore]
fn test_jsge_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov r1, 0xfffffffe
        mov r2, 0xffffffff
        mov32 r3, 0
        jsge r1, r2, end
        jsge r1, r3, end
        jsge r1, r1, +1
        exit
        mov32 r0, 1
        mov r1, r2
        jsge r1, r2, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(13),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediates 0xfffffffe, 0xffffffff; similar to test_rfc9669_jsge in this crate
#[test]
#[ignore]
fn test_jsge32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xfffffffe
        or r1, r9
        jsge32 r1, 0xffffffff, end
        jsge32 r1, 0, end
        mov32 r0, 1
        mov r1, 0xffffffff
        jsge32 r1, 0xffffffff, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(12),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediates 0xfffffffe, 0xffffffff; similar to test_rfc9669_jsge in this crate
#[test]
#[ignore]
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
        jsge32 r1, r2, end
        jsge32 r1, r3, end
        jsge32 r1, r1, +1
        exit
        mov32 r0, 1
        mov r1, r2
        jsge32 r1, r2, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(16),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediates 0xfffffffe, 0xffffffff; similar to test_rfc9669_jsgt in this crate
#[test]
#[ignore]
fn test_jsgt_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov r1, 0xfffffffe
        jsgt r1, 0xffffffff, end
        mov32 r0, 1
        mov32 r1, 0
        jsgt r1, 0xffffffff, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(8),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediates 0xfffffffe, 0xffffffff; similar to test_rfc9669_jsgt in this crate
#[test]
#[ignore]
fn test_jsgt_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov r1, 0xfffffffe
        mov r2, 0xffffffff
        jsgt r1, r2, end
        jsgt r1, r1, end
        mov32 r0, 1
        mov32 r1, 0
        jsgt r1, r2, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(10),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediates 0xfffffffe, 0xffffffff; similar to test_rfc9669_jsgt in this crate
#[test]
#[ignore]
fn test_jsgt32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xfffffffe
        or r1, r9
        jsgt32 r1, 0xffffffff, end
        mov32 r0, 1
        mov32 r1, 0
        jsgt32 r1, 0xffffffff, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediates 0xfffffffe, 0xffffffff; similar to test_rfc9669_jsgt in this crate
#[test]
#[ignore]
fn test_jsgt32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xfffffffe
        or r1, r9
        mov r2, 0xffffffff
        jsgt32 r1, r2, end
        jsgt32 r1, r1, end
        mov32 r0, 1
        mov32 r1, 0
        jsgt32 r1, r2, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(13),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediates 0xfffffffe, 0xffffffff; similar to test_rfc9669_jsle in this crate
#[test]
#[ignore]
fn test_jsle_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov r1, 0xfffffffe
        jsle r1, 0xfffffffd, end
        jsle r1, 0xffffffff, +1
        exit
        mov32 r0, 1
        jsle r1, 0xfffffffe, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(9),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediates 0xfffffffe, 0xffffffff; similar to test_rfc9669_jsle in this crate
#[test]
#[ignore]
fn test_jsle_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov r1, 0xffffffff
        mov r2, 0xfffffffe
        mov32 r3, 0
        jsle r1, r2, end
        jsle r1, r3, +1
        exit
        jsle r1, r1, +1
        exit
        mov32 r0, 1
        mov r1, r2
        jsle r1, r2, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(14),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediates 0xfffffffe, 0xffffffff; similar to test_rfc9669_jsle in this crate
#[test]
#[ignore]
fn test_jsle32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xfffffffe
        or r1, r9
        jsle32 r1, 0xfffffffd, end
        jsle32 r1, 0xffffffff, +1
        exit
        mov32 r0, 1
        jsle32 r1, 0xfffffffe, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(12),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediates 0xfffffffe, 0xffffffff; similar to test_rfc9669_jsle in this crate
#[test]
#[ignore]
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
        jsle32 r1, r2, end
        jsle32 r1, r3, +1
        exit
        jsle32 r1, r1, +1
        exit
        mov32 r0, 1
        mov r1, r2
        jsle32 r1, r2, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(17),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediates 0xfffffffe, 0xffffffff; similar to test_rfc9669_jslt in this crate
#[test]
#[ignore]
fn test_jslt_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov r1, 0xfffffffe
        jslt r1, 0xfffffffd, end
        jslt r1, 0xfffffffe, end
        jslt r1, 0xffffffff, +1
        exit
        mov32 r0, 1
        end:
        exit",
        [],
        TestContextObject::new(8),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediates 0xfffffffe, 0xffffffff; similar to test_rfc9669_jslt in this crate
#[test]
#[ignore]
fn test_jslt_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov r1, 0xfffffffe
        mov r2, 0xfffffffd
        mov r3, 0xffffffff
        jslt r1, r1, end
        jslt r1, r2, end
        jslt r1, r3, +1
        exit
        mov32 r0, 1
        end:
        exit",
        [],
        TestContextObject::new(10),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediates 0xfffffffe, 0xffffffff; similar to test_rfc9669_jslt in this crate
#[test]
#[ignore]
fn test_jslt32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xfffffffe
        or r1, r9
        jslt32 r1, 0xfffffffd, end
        jslt32 r1, 0xfffffffe, end
        jslt32 r1, 0xffffffff, +1
        exit
        mov32 r0, 1
        end:
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediates 0xfffffffe, 0xffffffff; similar to test_rfc9669_jslt in this crate
#[test]
#[ignore]
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
        jslt32 r1, r1, end
        jslt32 r1, r2, end
        jslt32 r1, r3, +1
        exit
        mov32 r0, 1
        end:
        exit",
        [],
        TestContextObject::new(13),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediate 0x80000000; similar to jeq-imm, jgt-imm, jlt-imm, jne-imm, jset-imm tests in this crate
#[test]
#[ignore]
fn test_j_signed_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 2
        lddw r1, 0xFFFFFFFF80000000
        jeq r1, 0x80000000, +1
        exit
        lddw r1, 0xFFFFFFFF80000000
        jlt r1, 0x80000001, +1
        exit
        lddw r1, 0xFFFFFFFF80000001
        jgt r1, 0x80000000, +1
        exit
        lddw r1, 0x80000000
        jne r1, 0x80000000, +1
        exit
        lddw r1, 0xFFFFFFFF00000000
        jset r1, 0x80000000, +1
        exit
        mov32 r0, 1
        exit",
        [],
        TestContextObject::new(18),
        ProgramResult::Ok(0x1),
    );
}

// Uses large immediate 0xffffffff; similar to jeq-imm and jeq32-imm tests in this crate
#[test]
#[ignore]
fn test_rfc9669_jeq() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 42
        jeq r1, 42, ok1
        ja fail
        ok1:
        mov r2, 100
        mov r3, 100
        jeq r2, r3, ok2
        ja fail
        ok2:
        mov r4, -1
        jeq32 r4, 0xffffffff, ok3
        ja fail
        ok3:
        mov r5, -1
        mov r6, 0xffffffff
        jeq32 r5, r6, ok4
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

// Uses large immediate 0x80000000; similar to jset-imm and jset32-imm tests in this crate
#[test]
#[ignore]
fn test_rfc9669_jset() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 0x10
        jset r1, 0x10, ok1
        ja fail
        ok1:
        mov r2, 0x10
        mov r3, 0x08
        jset r2, r3, fail
        ja ok2
        ok2:
        mov r4, 0x80000000
        jset32 r4, 0x80000000, ok3
        ja fail
        ok3:
        mov r5, 0x80000000
        mov r6, 0x80000000
        jset32 r5, r6, ok4
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

// Uses large immediate 0x88776655; similar to test_rfc9669_stxdw in this crate
#[test]
#[ignore]
fn test_stxdw() {
    test_interpreter_and_jit_asm!(
        "
        mov r2, 0x88776655
        lsh r2, 32
        or r2, 0x44332211
        stxdw [r1+2], r2
        ldxdw r0, [r1+2]
        exit",
        [0xaa, 0xbb, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xcc, 0xdd],
        TestContextObject::new(6),
        ProgramResult::Ok(0x8877665544332211),
    );
}

// sbpf does not populate r2 with memory length (returns 0 instead of expected 8)
#[test]
#[ignore]
fn test_mem_len() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, r2
        exit",
        [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02],
        TestContextObject::new(2),
        ProgramResult::Ok(0x8),
    );
}
