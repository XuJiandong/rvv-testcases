use alloc::boxed::Box;
use core::{arch::asm, convert::TryInto};
use eint::{Eint, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    intrinsic::{vop_vv_destructive, vop_vv_destructive_wide, vwop_vx},
    misc::{avl_iterator, conver_to_i512},
    runner::{run_vop_vv, run_vop_vx, ExpectedOp, WideningCategory},
};

fn expected_op_macc_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());

            let extra = u64::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(r);
            let (res2, _) = res.overflowing_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);

            let extra = E256::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmacc_vv(sew: u64, lmul: i64, avl: u64) {
    fn macc(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv_destructive(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmacc.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_macc_vv)),
        macc,
        WideningCategory::None,
        "vmacc.vv",
    );
}

fn expected_op_wmaccu_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    match lhs.len() {
        2 => {
            let l: u32 = u16::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: u32 = u16::from_le_bytes(rhs.try_into().unwrap()).into();

            let extra: u32 = u32::from_le_bytes(result.try_into().unwrap());
            let res2 = l * r + extra;
            result.copy_from_slice(&res2.to_le_bytes());
        }
        4 => {
            let l: u64 = u32::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: u64 = u32::from_le_bytes(rhs.try_into().unwrap()).into();

            let extra: u64 = u64::from_le_bytes(result.try_into().unwrap());
            let res2 = l * r + extra;
            result.copy_from_slice(&res2.to_le_bytes());
        }
        8 => {
            let l: u128 = u64::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: u128 = u64::from_le_bytes(rhs.try_into().unwrap()).into();

            let extra: u128 = u128::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(E256::get(rhs));

            let extra = E512::get(result);

            let (res, _) = l.overflowing_mul_u(r);
            let (res2, _) = res.overflowing_add_u(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vwmaccu_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv_destructive_wide(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwmaccu.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_wmaccu_vv)),
        op,
        WideningCategory::VdOnly,
        "vwmaccu.vv",
    );
}

fn expected_op_wmaccu_vx(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert!(lhs.len() * 2 == result.len());
    match lhs.len() {
        8 => {
            let l: u128 = u64::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: u128 = rhs.into();
            let extra: u128 = u128::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(rhs);
            let extra = E512::get(result);

            let (res, _) = l.overflowing_mul_u(r);
            let (res2, _) = res.overflowing_add_u(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vwmaccu_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vx(lhs, rhs, result, sew, avl, lmul, |rhs: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwmaccu.vx v24, v8, t0", in (reg) rhs);
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_wmaccu_vx,
        op,
        WideningCategory::VdOnly,
        "vwmaccu.vx",
    );
}

fn expected_op_wmacc_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    match lhs.len() {
        8 => {
            let l: i128 = i64::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: i128 = i64::from_le_bytes(rhs.try_into().unwrap()).into();

            let extra = i128::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = conver_to_i512(E256::get(rhs));

            let extra = E512::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vwmacc_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv_destructive_wide(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwmacc.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_wmacc_vv)),
        op,
        WideningCategory::VdOnly,
        "vwmacc.vv",
    );
}

fn expected_op_wmacc_vx(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert!(lhs.len() * 2 == result.len());
    match lhs.len() {
        8 => {
            let l: i128 = i64::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: i128 = (rhs as i64) as i128;

            let extra = i128::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = E512::from(rhs as i64);

            let extra = E512::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vwmacc_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vx(lhs, rhs, result, sew, avl, lmul, |rhs: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwmacc.vx v24, v8, t0", in (reg) rhs);
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_wmacc_vx,
        op,
        WideningCategory::VdOnly,
        "vwmacc.vv",
    );
}

fn expected_op_wmaccsu_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = i64::from_le_bytes(rhs.try_into().unwrap()) as i128;
            let extra = i128::from_le_bytes(result.try_into().unwrap());

            let res1 = l.wrapping_mul(r);
            let res2 = res1.wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = E512::from(E256::get(lhs));
            let r = conver_to_i512(E256::get(rhs));
            let extra = E512::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vwmaccsu_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv_destructive_wide(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwmaccsu.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_wmaccsu_vv)),
        op,
        WideningCategory::VdOnly,
        "vwmaccsu.vv",
    );
}

fn expected_op_wmaccsu_vx(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert!(lhs.len() * 2 == result.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = (rhs as i64) as i128;
            let extra = i128::from_le_bytes(result.try_into().unwrap());

            let res1 = l.wrapping_mul(r);
            let res2 = res1.wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(rhs as i64);
            let extra = E512::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vwmaccsu_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vx(lhs, rhs, result, sew, avl, lmul, |rhs: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwmaccsu.vx v24, v8, t0", in (reg) rhs);
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_wmaccsu_vx,
        op,
        WideningCategory::VdOnly,
        "vwmaccsu.vx",
    );
}

fn expected_op_wmaccus_vx(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert!(lhs.len() * 2 == result.len());
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = rhs  as i128;
            let extra = i128::from_le_bytes(result.try_into().unwrap());

            let res1 = l.wrapping_mul(r);
            let res2 = res1.wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = E512::from(rhs);
            let extra = E512::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vwmaccus_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vx(lhs, rhs, result, sew, avl, lmul, |rhs: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwmaccus.vx v24, v8, t0", in (reg) rhs);
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_wmaccus_vx,
        op,
        WideningCategory::VdOnly,
        "vwmaccus.vv",
    );
}

pub fn test_widening_width_multiply_add() {
    for sew in [64, 256] {
        for lmul in [-4, -2, 1, 2, 4] {
            for avl in avl_iterator(sew, 4) {
                test_vmacc_vv(sew, lmul, avl);

                test_vwmaccu_vv(sew, lmul, avl);
                test_vwmaccu_vx(sew, lmul, avl);

                test_vwmacc_vv(sew, lmul, avl);
                test_vwmacc_vx(sew, lmul, avl);

                test_vwmaccsu_vv(sew, lmul, avl);
                test_vwmaccsu_vx(sew, lmul, avl);
                test_vwmaccus_vx(sew, lmul, avl);
            }
        }
    }
}
