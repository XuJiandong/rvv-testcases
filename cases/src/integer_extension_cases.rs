use alloc::boxed::Box;
use core::{arch::asm, convert::TryInto};
use eint::{Eint, E128, E256};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    intrinsic::vop_nv,
    misc::{avl_iterator, conver_to_i256},
    runner::{run_vop_vv, ExpectedOp, WideningCategory},
};

// use ckb_std::syscalls::debug;
// use rvv_testcases::log;2

fn expected_op_vzext_vf2(lhs: &[u8], _: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() / 2);

    match result.len() {
        8 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap()) as u64;
            result.copy_from_slice(&l.to_le_bytes());
        }
        32 => {
            E256::from(E128::get(lhs)).put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vzext_vf2(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_nv(
            lhs,
            rhs,
            result,
            sew,
            avl,
            lmul,
            || unsafe {
                rvv_asm!("vzext.vf2 v24, v8");
            },
            WideningCategory::NarrowVs2(2),
        );
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_vzext_vf2)),
        op,
        WideningCategory::NarrowVs2(2),
        "vzext.vf2",
    );
}

fn expected_op_vsext_vf2(lhs: &[u8], _: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() / 2);

    match result.len() {
        8 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap()) as i64;
            result.copy_from_slice(&l.to_le_bytes());
        }
        32 => {
            conver_to_i256(E128::get(lhs)).put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vsext_vf2(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_nv(
            lhs,
            rhs,
            result,
            sew,
            avl,
            lmul,
            || unsafe {
                rvv_asm!("vsext.vf2 v24, v8");
            },
            WideningCategory::NarrowVs2(2),
        );
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_vsext_vf2)),
        op,
        WideningCategory::NarrowVs2(2),
        "vsext.vf2",
    );
}

fn expected_op_vzext_vf4(lhs: &[u8], _: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() / 4);

    match result.len() {
        8 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap()) as u64;
            result.copy_from_slice(&l.to_le_bytes());
        }
        32 => {
            E256::from(u64::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vzext_vf4(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_nv(
            lhs,
            rhs,
            result,
            sew,
            avl,
            lmul,
            || unsafe {
                rvv_asm!("vzext.vf4 v24, v8");
            },
            WideningCategory::NarrowVs2(4),
        );
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_vzext_vf4)),
        op,
        WideningCategory::NarrowVs2(4),
        "vzext.vf4",
    );
}

fn expected_op_vsext_vf4(lhs: &[u8], _: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() / 4);

    match result.len() {
        8 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap()) as i64;
            result.copy_from_slice(&l.to_le_bytes());
        }
        32 => {
            E256::from(i64::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vsext_vf4(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_nv(
            lhs,
            rhs,
            result,
            sew,
            avl,
            lmul,
            || unsafe {
                rvv_asm!("vsext.vf4 v24, v8");
            },
            WideningCategory::NarrowVs2(4),
        );
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_vsext_vf4)),
        op,
        WideningCategory::NarrowVs2(4),
        "vsext.vf4",
    );
}

fn expected_op_vzext_vf8(lhs: &[u8], _: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() / 8);
    match result.len() {
        8 => {
            let l = lhs[0] as u64;
            result.copy_from_slice(&l.to_le_bytes());
        }
        32 => {
            E256::from(u32::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vzext_vf8(sew: u64, lmul: i64, avl: u64) {
    if lmul <= -2 {
        return;
    }
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_nv(
            lhs,
            rhs,
            result,
            sew,
            avl,
            lmul,
            || unsafe {
                rvv_asm!("vzext.vf8 v24, v8");
            },
            WideningCategory::NarrowVs2(8),
        );
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_vzext_vf8)),
        op,
        WideningCategory::NarrowVs2(8),
        "vzext.vf8",
    );
}

fn expected_op_vsext_vf8(lhs: &[u8], _: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() / 8);

    match result.len() {
        8 => {
            let l = lhs[0] as i64;
            result.copy_from_slice(&l.to_le_bytes());
        }
        32 => {
            E256::from(i32::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vsext_vf8(sew: u64, lmul: i64, avl: u64) {
    if lmul <= -2 {
        return;
    }
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_nv(
            lhs,
            rhs,
            result,
            sew,
            avl,
            lmul,
            || unsafe {
                rvv_asm!("vsext.vf8 v24, v8");
            },
            WideningCategory::NarrowVs2(8),
        );
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_vsext_vf8)),
        op,
        WideningCategory::NarrowVs2(8),
        "vsext.vf8",
    );
}

pub fn test_integer_extension() {
    for sew in [64, 256] {
        for lmul in [-2, 1, 4, 8] {
            for avl in avl_iterator(sew, 4) {
                test_vzext_vf2(sew, lmul, avl);
                test_vsext_vf2(sew, lmul, avl);
                test_vzext_vf4(sew, lmul, avl);
                test_vsext_vf4(sew, lmul, avl);
                test_vzext_vf8(sew, lmul, avl);
                test_vsext_vf8(sew, lmul, avl);
            }
        }
    }
}
