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

#[test]
fn test_jeq_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov32 r1, 0xa
        jeq r1, 0xb, end
        mov32 r0, 1
        mov32 r1, 0xb
        jeq r1, 0xb, end
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(8),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jeq_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov32 r1, 0xa
        mov32 r2, 0xb
        jeq r1, r2, end
        jeq r1, r1, +1
        exit
        mov32 r0, 1
        mov32 r1, 0xb
        jeq r1, r2, end
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jeq32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0x0
        mov32 r1, 0xa
        jeq32 r1, 0xb, end
        mov32 r0, 1
        mov r1, 0xb
        or r1, r9
        jeq32 r1, 0xb, end
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jeq32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xa
        mov32 r2, 0xb
        jeq32 r1, r2, end
        jeq32 r1, r1, +1
        exit
        mov32 r0, 1
        mov32 r1, 0xb
        or r1, r9
        jeq32 r1, r2, end
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(14),
        ProgramResult::Ok(0x1),
    );
}

#[test]
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

#[test]
fn test_jne_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov32 r1, 0xb
        mov32 r2, 0xb
        jne r1, r2, end
        jne r1, r1, end
        mov32 r0, 1
        mov32 r1, 0xa
        jne r1, r2, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(10),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jne32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xb
        or r1, r9
        jne32 r1, 0xb, +4
        mov32 r0, 1
        mov32 r1, 0xa
        or r1, r9
        jne32 r1, 0xb, +1
        mov32 r0, 2
        exit",
        [],
        TestContextObject::new(12),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jne32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xb
        or r1, r9
        mov32 r2, 0xb
        jne32 r1, r2, +5
        jne32 r1, r1, +4
        mov32 r0, 1
        mov32 r1, 0xa
        or r1, r9
        jne32 r1, r2, +1
        mov32 r0, 2
        exit",
        [],
        TestContextObject::new(14),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_jne() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 41
        jne r1, 42, ok1
        ja fail
        ok1:
        mov r2, 100
        mov r3, 101
        jne r2, r3, ok2
        ja fail
        ok2:
        mov r4, -1
        jne32 r4, 0, ok3
        ja fail
        ok3:
        mov r5, -1
        mov r6, 0
        jne32 r5, r6, ok4
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
fn test_jge_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov32 r1, 0xa
        jge r1, 0xb, end
        mov32 r0, 1
        mov32 r1, 0xc
        jge r1, 0xb, end
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(8),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jge_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov32 r1, 0xa
        mov32 r2, 0x0b
        jge r1, r2, end
        jge r1, r1, +1
        exit
        mov32 r0, 1
        mov32 r1, 0xc
        jge r1, r2, end
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jge32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xa
        jge32 r1, 0xb, end
        mov32 r0, 1
        mov32 r1, 0xc
        or r1, r9
        jge32 r1, 0xb, end
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jge32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0xa
        mov32 r2, 0xb
        jge32 r1, r2, end
        jge32 r1, r1, +1
        exit
        mov32 r0, 1
        mov32 r1, 0xc
        or r1, r9
        jge32 r1, r2, end
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(14),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_jge() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 42
        jge r1, 42, ok1
        ja fail
        ok1:
        mov r2, 100
        mov r3, 100
        jge r2, r3, ok2
        ja fail
        ok2:
        mov r4, 0
        jge32 r4, 0, ok3
        ja fail
        ok3:
        mov r5, 0
        mov r6, 0
        jge32 r5, r6, ok4
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
fn test_jgt_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov32 r1, 5
        jgt r1, 6, end
        jgt r1, 5, end
        jgt r1, 4, L1
        exit
        L1:
        mov32 r0, 1
        end:
        exit",
        [],
        TestContextObject::new(8),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jgt_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0
        mov r1, 5
        mov r2, 6
        mov r3, 4
        jgt r1, r2, end
        jgt r1, r1, end
        jgt r1, r3, taken
        exit
        taken:
        mov r0, 1
        end:
        exit",
        [],
        TestContextObject::new(10),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jgt32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 5
        or r1, r9
        jgt32 r1, 6, end
        jgt32 r1, 5, end
        jgt32 r1, 4, taken
        exit
        taken:
        mov32 r0, 1
        end:
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jgt32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov r0, 0
        mov r1, 5
        mov32 r1, 5
        or r1, r9
        mov r2, 6
        mov r3, 4
        jgt32 r1, r2, end
        jgt32 r1, r1, end
        jgt32 r1, r3, taken
        exit
        taken:
        mov r0, 1
        end:
        exit",
        [],
        TestContextObject::new(14),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_jgt() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 43
        jgt r1, 42, ok1
        ja fail
        ok1:
        mov r2, 100
        mov r3, 99
        jgt r2, r3, ok2
        ja fail
        ok2:
        mov r4, -1
        jgt32 r4, 1, ok3
        ja fail
        ok3:
        mov r5, -1
        mov r6, 1
        jgt32 r5, r6, ok4
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
fn test_jle_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov32 r1, 5
        jle r1, 4, end
        jle r1, 6, +1
        exit
        taken:
        jle r1, 5, +1
        exit
        taken2:
        mov32 r0, 1
        end:
        exit",
        [],
        TestContextObject::new(9),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jle_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0
        mov r1, 5
        mov r2, 4
        mov r3, 6
        jle r1, r2, end
        jle r1, r1, +1
        exit
        jle r1, r3, +1
        exit
        mov r0, 1
        end:
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jle32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 5
        or r1, r9
        jle32 r1, 4, end
        jle32 r1, 6, +1
        exit
        jle32 r1, 5, +1
        exit
        mov32 r0, 1
        end:
        exit",
        [],
        TestContextObject::new(12),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jle32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov r0, 0
        mov r1, 5
        mov r2, 4
        mov r3, 6
        or r1, r9
        jle32 r1, r2, end
        jle32 r1, r1, +1
        exit
        jle32 r1, r3, +1
        exit
        mov r0, 1
        end:
        exit",
        [],
        TestContextObject::new(14),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_jle() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 42
        jle r1, 42, ok1
        ja fail
        ok1:
        mov r2, 99
        mov r3, 99
        jle r2, r3, ok2
        ja fail
        ok2:
        mov r4, 1
        jle32 r4, 1, ok3
        ja fail
        ok3:
        mov r5, 1
        mov r6, 1
        jle32 r5, r6, ok4
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
fn test_jlt_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov32 r1, 5
        jlt r1, 4, end
        jlt r1, 5, end
        jlt r1, 6, +1
        exit
        mov32 r0, 1
        end:
        exit",
        [],
        TestContextObject::new(8),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jlt_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r0, 0
        mov r1, 5
        mov r2, 4
        mov r3, 6
        jlt r1, r2, end
        jlt r1, r1, end
        jlt r1, r3, +1
        exit
        mov r0, 1
        end:
        exit",
        [],
        TestContextObject::new(10),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jlt32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 5
        or r1, r9
        jlt32 r1, 4, end
        jlt32 r1, 5, end
        jlt32 r1, 6, +1
        exit
        mov32 r0, 1
        end:
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jlt32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov r0, 0
        mov r1, 5
        mov r2, 4
        mov r3, 6
        or r1, r9
        jlt32 r1, r2, end
        jlt32 r1, r1, end
        jlt32 r1, r3, +1
        exit
        mov r0, 1
        end:
        exit",
        [],
        TestContextObject::new(13),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_rfc9669_jlt() {
    test_interpreter_and_jit_asm!(
        "
        mov r1, 41
        jlt r1, 42, ok1
        ja fail
        ok1:
        mov r2, 99
        mov r3, 100
        jlt r2, r3, ok2
        ja fail
        ok2:
        mov r4, 1
        jlt32 r4, 2, ok3
        ja fail
        ok3:
        mov r5, 1
        mov r6, 2
        jlt32 r5, r6, ok4
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
fn test_jset_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov32 r1, 0x7
        jset r1, 0x8, end
        mov32 r0, 1
        mov32 r1, 0x9
        jset r1, 0x8, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(8),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jset_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov32 r0, 0
        mov32 r1, 0x7
        mov32 r2, 0x8
        jset r1, r2, end
        jset r1, r1, +1
        exit
        mov32 r0, 1
        mov32 r1, 0x9
        jset r1, r2, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jset32_imm() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0x7
        or r1, r9
        jset32 r1, 0x8, end
        mov32 r0, 1
        mov32 r1, 0x9
        jset32 r1, 0x8, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(11),
        ProgramResult::Ok(0x1),
    );
}

#[test]
fn test_jset32_reg() {
    test_interpreter_and_jit_asm!(
        "
        mov r9, 1
        lsh r9, 32
        mov32 r0, 0
        mov32 r1, 0x7
        or r1, r9
        mov32 r2, 0x8
        jset32 r1, r2, end
        jset32 r1, r1, +1
        exit
        mov32 r0, 1
        mov32 r1, 0x9
        jset32 r1, r2, +1
        mov32 r0, 2
        end:
        exit",
        [],
        TestContextObject::new(14),
        ProgramResult::Ok(0x1),
    );
}

#[test]
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
