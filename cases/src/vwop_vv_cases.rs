use alloc::boxed::Box;
use core::{arch::asm, convert::TryInto};
use eint::{Eint, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    intrinsic::vwop_vv,
    misc::{avl_iterator, conver_to_i512},
    runner::{run_vop_vv, ExpectedOp, WideningCategory},
};

fn expected_op_addu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = u64::from_le_bytes(rhs.try_into().unwrap()) as u128;
            let res = l.wrapping_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(E256::get(rhs));
            l.overflowing_add_u(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_addu_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwaddu.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_addu)),
        op,
        WideningCategory::VdOnly,
        "vwaddu.vv",
    );
}

fn expected_op_add(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = i64::from_le_bytes(rhs.try_into().unwrap()) as i128;
            let res = l.wrapping_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = conver_to_i512(E256::get(rhs));
            l.overflowing_add_s(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_add_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwadd.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_add)),
        op,
        WideningCategory::VdOnly,
        "vwadd.vv",
    );
}

fn expected_op_mulu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = u64::from_le_bytes(rhs.try_into().unwrap()) as u128;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(E256::get(rhs));
            l.overflowing_mul_u(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_mulu_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwmulu.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_mulu)),
        op,
        WideningCategory::VdOnly,
        "vwmulu.vv",
    );
}

fn expected_op_mul(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = i64::from_le_bytes(rhs.try_into().unwrap()) as i128;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = conver_to_i512(E256::get(rhs));
            l.overflowing_mul_s(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_mul_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwmul.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_mul)),
        op,
        WideningCategory::VdOnly,
        "vwmul.vv",
    );
}

fn expected_op_mulsu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = u64::from_le_bytes(rhs.try_into().unwrap()) as i128;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = E512::from(E256::get(rhs));
            l.overflowing_mul_u(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_mulsu_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwmulsu.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_mulsu)),
        op,
        WideningCategory::VdOnly,
        "vwmulsu.vv",
    );
}

fn expected_op_subu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = u64::from_le_bytes(rhs.try_into().unwrap()) as u128;
            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(E256::get(rhs));
            l.overflowing_sub_u(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_subu_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwsubu.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_subu)),
        op,
        WideningCategory::VdOnly,
        "vwsubu.vv",
    );
}

fn expected_op_sub(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = i64::from_le_bytes(rhs.try_into().unwrap()) as i128;
            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = conver_to_i512(E256::get(rhs));
            l.overflowing_sub_s(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_sub_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwsub.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_sub)),
        op,
        WideningCategory::VdOnly,
        "vwsub.vv",
    );
}

pub fn test_vwop_vv() {
    for sew in [64, 256] {
        for lmul in [-2, 1, 2] {
            for avl in avl_iterator(sew, 4) {
                test_vw_addu_vv(sew, lmul, avl);
                test_vw_add_vv(sew, lmul, avl);
                test_vw_mulu_vv(sew, lmul, avl);
                test_vw_mul_vv(sew, lmul, avl);
                test_vw_mulsu_vv(sew, lmul, avl);
                test_vw_subu_vv(sew, lmul, avl);
                test_vw_sub_vv(sew, lmul, avl);
            }
        }
    }
}
