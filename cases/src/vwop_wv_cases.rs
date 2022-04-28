use alloc::boxed::Box;
use core::{arch::asm, convert::TryInto};
use eint::{Eint, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    intrinsic::vwop_wv,
    misc::{avl_iterator, conver_to_i512},
    runner::{run_vop_vv, ExpectedOp, WideningCategory},
};

fn expected_op_addu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), 2 * rhs.len());
    assert_eq!(lhs.len(), result.len());

    let sew_byte = lhs.len() / 2;
    match sew_byte {
        8 => {
            let l = u128::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap()) as u128;

            let (res, _) = l.overflowing_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::get(lhs);
            let r = E512::from(E256::get(rhs));

            let (res, _) = l.overflowing_add_u(r);
            res.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_addu_wv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_wv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwaddu.wv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_addu)),
        op,
        WideningCategory::VdVs2,
        "vwaddu.wv",
    );
}

fn expected_op_add(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), 2 * rhs.len());
    assert_eq!(lhs.len(), result.len());

    let sew_byte = lhs.len() / 2;
    match sew_byte {
        8 => {
            let l = i128::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap()) as i128;

            let (res, _) = l.overflowing_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::get(lhs);
            let r = conver_to_i512(E256::get(rhs));

            let (res, _) = l.overflowing_add_s(r);
            res.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_add_wv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_wv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwadd.wv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_add)),
        op,
        WideningCategory::VdVs2,
        "vwadd.wv",
    );
}

fn expected_op_subu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), 2 * rhs.len());
    assert_eq!(lhs.len(), result.len());

    let sew_byte = lhs.len() / 2;
    match sew_byte {
        8 => {
            let l = u128::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap()) as u128;

            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::get(lhs);
            let r = E512::from(E256::get(rhs));

            let (res, _) = l.overflowing_sub_u(r);
            res.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_subu_wv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_wv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwsubu.wv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_subu)),
        op,
        WideningCategory::VdVs2,
        "vwsubu.wv",
    );
}

fn expected_op_sub(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), 2 * rhs.len());
    assert_eq!(lhs.len(), result.len());

    let sew_byte = lhs.len() / 2;
    match sew_byte {
        8 => {
            let l = i128::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap()) as i128;

            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::get(lhs);
            let r = conver_to_i512(E256::get(rhs));

            let (res, _) = l.overflowing_sub_s(r);
            res.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_sub_wv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_wv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwsub.wv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_sub)),
        op,
        WideningCategory::VdVs2,
        "vwsub.wv",
    );
}

pub fn test_vwop_wv() {
    for sew in [64, 256] {
        for lmul in [-4, -2, 1, 2, 4] {
            for avl in avl_iterator(sew, 4) {
                test_vw_addu_wv(sew, lmul, avl);
                test_vw_add_wv(sew, lmul, avl);

                test_vw_subu_wv(sew, lmul, avl);
                test_vw_sub_wv(sew, lmul, avl);
            }
        }
    }
}
