use core::arch::asm;

use alloc::boxed::Box;
use eint::{Eint, E256, E8};
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::{vop_vv, vop_vx};
use rvv_testcases::misc::{avl_iterator, U256};
use rvv_testcases::runner::{run_vop_vv, run_vop_vx, ExpectedOp, WideningCategory};

fn expected_op(lhs: &[u8], _: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    match result.len() {
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = l << 10; // fixed
            r.to_little_endian(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}

fn test_vsll_vi(sew: u64, lmul: i64, avl: u64) {
    fn sll(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |_| unsafe {
            rvv_asm!("vsll.vi v24, v8, 10");
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op,
        sll,
        WideningCategory::None,
        "vsll.vi",
    );
}

fn expected_op_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    match result.len() {
        1 => {
            let l = E8::get(lhs);
            let r = E8::get(rhs);
            let res = l.wrapping_shl(r.u32());
            res.put(result);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = l.wrapping_shl(r.u32());
            res.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}

fn test_vsll_vv(sew: u64, lmul: i64, avl: u64) {
    fn sll_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vsll.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_vv)),
        sll_vv,
        WideningCategory::None,
        "vsll.vv",
    );
}

fn expected_op_vsll_vx(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            result[0] = lhs[0] + x as u8;
        }
        32 => {
            let l = E256::get(lhs);
            let res = l.wrapping_shl(x as u32);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

fn test_vsll_vx(sew: u64, lmul: i64, avl: u64) {
    fn sll_vx(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", 
                     "vsll.vx v24, v8, t0",
                     in (reg) x);
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_vsll_vx,
        sll_vx,
        WideningCategory::None,
        "vsll.vx",
    );
}

pub fn test_single_width_shift() {
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            test_vsll_vi(sew, lmul, avl);
            test_vsll_vv(sew, lmul, avl);
            test_vsll_vx(sew, lmul, avl);
        }
    }
}
