#![allow(dead_code)]
use core::arch::asm;
use core::convert::TryInto;

use alloc::boxed::Box;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vop_vv_destructive;

use rvv_testcases::misc::{avl_iterator, U256, U512};
use rvv_testcases::runner::{run_vop_vv, ExpectedOp, WideningCategory};

fn expected_op(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = U256::from_little_endian(rhs);
            let extra = U256::from_little_endian(result);
            let (res, _) = l.overflowing_mul(r);
            let (res2, _) = res.overflowing_add(extra);
            res2.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_single_width_integer_multiply_add() {
    fn macc(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv_destructive(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmacc.vv v21, v1, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-8, -2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                ExpectedOp::Normal(Box::new(expected_op)),
                macc,
                WideningCategory::None,
                "vmacc.vv",
            );
        }
    }
}

fn expected_op_wide_u(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
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
        32 => {
            let l: U512 = U256::from_little_endian(lhs).into();
            let r: U512 = U256::from_little_endian(rhs).into();
            let extra = U512::from_little_endian(result);
            let (res, _) = l.overflowing_mul(r);
            let (res2, _) = res.overflowing_add(extra);
            res2.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_widening_width_uinteger_multiply_add() {
    fn wmaccu(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv_destructive(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwmaccu.vv v21, v1, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-8, -2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                ExpectedOp::Normal(Box::new(expected_op_wide_u)),
                wmaccu,
                WideningCategory::VdOnly,
                "vwmaccu.vv",
            );
        }
    }
}
