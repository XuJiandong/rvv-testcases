use core::{arch::asm, convert::TryInto};
use eint::{Eint, E256};
use rvv_asm::rvv_asm;
use rvv_testcases::runner::{run_template_v_vim, run_template_v_vvm, run_template_v_vxm, MaskType};

fn test_vmerge_vvm() {
    fn exp_op(lhs: &[u8], rhs: &[u8], result: &mut [u8], mask: bool) {
        if mask {
            result.copy_from_slice(rhs);
        } else {
            result.copy_from_slice(lhs);
        }
    }
    fn op(_: &[u8], _: &[u8], _: MaskType) {
        unsafe {
            rvv_asm!("vmerge.vvm v24, v8, v16, v0");
        }
    }

    run_template_v_vvm(exp_op, op, "vmerge.vvm");
}

fn test_vmerge_vxm() {
    fn exp_op(lhs: &[u8], x: u64, result: &mut [u8], mask: bool) {
        if mask {
            match result.len() {
                8 => result.copy_from_slice(&x.to_le_bytes()),
                32 => E256::from(x as i64).put(result),
                _ => panic!("Abort"),
            }
        } else {
            result.copy_from_slice(lhs);
        }
    }
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            rvv_asm!("mv t0, {}", "vmerge.vxm v24, v8, t0, v0", in (reg) x);
        }
    }

    run_template_v_vxm(exp_op, op, "vmerge.vxm");
}

fn test_vmerge_vim() {
    fn exp_op(lhs: &[u8], x: i64, result: &mut [u8], mask: bool) {
        if mask {
            match result.len() {
                8 => result.copy_from_slice(&x.to_le_bytes()),
                32 => E256::from(x).put(result),
                _ => panic!("Abort"),
            }
        } else {
            result.copy_from_slice(lhs);
        }
    }
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => {
                    rvv_asm!("vmerge.vim v24, v8, -16, v0");
                }
                -15 => {
                    rvv_asm!("vmerge.vim v24, v8, -15, v0");
                }
                -14 => {
                    rvv_asm!("vmerge.vim v24, v8, -14, v0");
                }
                -13 => {
                    rvv_asm!("vmerge.vim v24, v8, -13, v0");
                }
                -12 => {
                    rvv_asm!("vmerge.vim v24, v8, -12, v0");
                }
                -11 => {
                    rvv_asm!("vmerge.vim v24, v8, -11, v0");
                }
                -10 => {
                    rvv_asm!("vmerge.vim v24, v8, -10, v0");
                }
                -9 => {
                    rvv_asm!("vmerge.vim v24, v8, -9, v0");
                }
                -8 => {
                    rvv_asm!("vmerge.vim v24, v8, -8, v0");
                }
                -7 => {
                    rvv_asm!("vmerge.vim v24, v8, -7, v0");
                }
                -6 => {
                    rvv_asm!("vmerge.vim v24, v8, -6, v0");
                }
                -5 => {
                    rvv_asm!("vmerge.vim v24, v8, -5, v0");
                }
                -4 => {
                    rvv_asm!("vmerge.vim v24, v8, -4, v0");
                }
                -3 => {
                    rvv_asm!("vmerge.vim v24, v8, -3, v0");
                }
                -2 => {
                    rvv_asm!("vmerge.vim v24, v8, -2, v0");
                }
                -1 => {
                    rvv_asm!("vmerge.vim v24, v8, -1, v0");
                }
                0 => {
                    rvv_asm!("vmerge.vim v24, v8, 0, v0");
                }
                1 => {
                    rvv_asm!("vmerge.vim v24, v8, 1, v0");
                }
                2 => {
                    rvv_asm!("vmerge.vim v24, v8, 2, v0");
                }
                3 => {
                    rvv_asm!("vmerge.vim v24, v8, 3, v0");
                }
                4 => {
                    rvv_asm!("vmerge.vim v24, v8, 4, v0");
                }
                5 => {
                    rvv_asm!("vmerge.vim v24, v8, 5, v0");
                }
                6 => {
                    rvv_asm!("vmerge.vim v24, v8, 6, v0");
                }
                7 => {
                    rvv_asm!("vmerge.vim v24, v8, 7, v0");
                }
                8 => {
                    rvv_asm!("vmerge.vim v24, v8, 8, v0");
                }
                9 => {
                    rvv_asm!("vmerge.vim v24, v8, 9, v0");
                }
                10 => {
                    rvv_asm!("vmerge.vim v24, v8, 10, v0");
                }
                11 => {
                    rvv_asm!("vmerge.vim v24, v8, 11, v0");
                }
                12 => {
                    rvv_asm!("vmerge.vim v24, v8, 12, v0");
                }
                13 => {
                    rvv_asm!("vmerge.vim v24, v8, 13, v0");
                }
                14 => {
                    rvv_asm!("vmerge.vim v24, v8, 14, v0");
                }
                15 => {
                    rvv_asm!("vmerge.vim v24, v8, 15, v0");
                }
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_vim(exp_op, op, "vmerge.vim");
}

pub fn test_integer_merge() {
    test_vmerge_vvm();
    test_vmerge_vxm();
    test_vmerge_vim();
}
