use core::{arch::asm, convert::TryInto};
use eint::{Eint, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    intrinsic::vwop_vx,
    misc::{avl_iterator, conver_to_i512},
    runner::{run_vop_vx, WideningCategory},
};

fn expected_op_addu(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len() * 2, result.len());

    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = rhs as u128;
            let res = l.wrapping_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(rhs);
            l.overflowing_add_u(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vwaddu_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwaddu.vx v24, v8, t0", in (reg) x);
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_addu,
        op,
        WideningCategory::VdOnly,
        "vwaddu.vx",
    );
}

fn expected_op_add(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len() * 2, result.len());

    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = (rhs as i64) as u128;
            let res = l.wrapping_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = E512::from(rhs as i64);
            l.overflowing_add_s(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vwadd_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwadd.vx v24, v8, t0", in (reg) x);
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_add,
        op,
        WideningCategory::VdOnly,
        "vwadd.vx",
    );
}

fn expected_op_subu(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len() * 2, result.len());

    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = rhs as u128;
            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(rhs);
            l.overflowing_sub_u(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vwsubu_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwsubu.vx v24, v8, t0", in (reg) x);
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_subu,
        op,
        WideningCategory::VdOnly,
        "vwsubu.vx",
    );
}

fn expected_op_sub(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len() * 2, result.len());

    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = (rhs as i64) as u128;
            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = E512::from(rhs as i64);
            l.overflowing_sub_s(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vwsub_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwsub.vx v24, v8, t0", in (reg) x);
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_sub,
        op,
        WideningCategory::VdOnly,
        "vwsub.vx",
    );
}

fn expected_op_mulu(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len() * 2, result.len());

    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = rhs as u128;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(rhs);
            l.overflowing_mul_u(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vwmulu_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwmulu.vx v24, v8, t0", in (reg) x);
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_mulu,
        op,
        WideningCategory::VdOnly,
        "vwmulu.vx",
    );
}

fn expected_op_mul(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len() * 2, result.len());

    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = (rhs as i64) as u128;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = E512::from(rhs as i64);
            l.overflowing_mul_s(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vwmul_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwmul.vx v24, v8, t0", in (reg) x);
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_mul,
        op,
        WideningCategory::VdOnly,
        "vwmul.vx",
    );
}

fn expected_op_mulsu(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len() * 2, result.len());

    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = rhs as u128;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = E512::from(rhs);
            l.overflowing_mul_u(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vwmulsu_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwmulsu.vx v24, v8, t0", in (reg) x);
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_mulsu,
        op,
        WideningCategory::VdOnly,
        "vwmulsu.vx",
    );
}

pub fn test_vwop_vx() {
    for sew in [64, 256] {
        for lmul in [-2, 1, 2] {
            for avl in avl_iterator(sew, 4) {
                test_vwaddu_vx(sew, lmul, avl);
                test_vwadd_vx(sew, lmul, avl);
                test_vwsubu_vx(sew, lmul, avl);
                test_vwsub_vx(sew, lmul, avl);
                test_vwmulu_vx(sew, lmul, avl);
                test_vwmul_vx(sew, lmul, avl);
                test_vwmulsu_vx(sew, lmul, avl);
            }
        }
    }
}
