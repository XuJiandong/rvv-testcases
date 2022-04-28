use core::{arch::asm, convert::TryInto};
use eint::{Eint, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    intrinsic::vwop_wx,
    misc::avl_iterator,
    runner::{run_vop_vx, WideningCategory},
};

fn expected_op_addu(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    let sew_byte = lhs.len() / 2;
    match sew_byte {
        8 => {
            let l = u128::from_le_bytes(lhs.try_into().unwrap());
            let r = rhs as u128;

            let (res, _) = l.overflowing_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::get(lhs);
            let r = E512::from(rhs);

            let (r, _) = l.overflowing_add_u(r);
            r.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_addu_wx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_wx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwaddu.wx v24, v8, t0", in (reg) x);
        });
    }

    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_addu,
        op,
        WideningCategory::VdVs2,
        "vwaddu.wx",
    );
}

fn expected_op_add(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    let sew_byte = lhs.len() / 2;
    match sew_byte {
        8 => {
            let l = i128::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as i64) as i128;

            let (res, _) = l.overflowing_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::get(lhs);
            let r = E512::from(rhs as i64);

            let (r, _) = l.overflowing_add_s(r);
            r.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_add_wx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_wx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwadd.wx v24, v8, t0", in (reg) x);
        });
    }

    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_add,
        op,
        WideningCategory::VdVs2,
        "vwadd.wx",
    );
}

fn expected_op_subu(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    let sew_byte = lhs.len() / 2;
    match sew_byte {
        8 => {
            let l = u128::from_le_bytes(lhs.try_into().unwrap());
            let r = rhs as u128;

            let (res, _) = l.overflowing_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::get(lhs);
            let r = E512::from(rhs);

            let (r, _) = l.overflowing_sub_u(r);
            r.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_subu_wx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_wx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwsubu.wx v24, v8, t0", in (reg) x);
        });
    }

    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_subu,
        op,
        WideningCategory::VdVs2,
        "vwsubu.wx",
    );
}

fn expected_op_sub(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    let sew_byte = lhs.len() / 2;
    match sew_byte {
        8 => {
            let l = i128::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as i64) as i128;

            let (res, _) = l.overflowing_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::get(lhs);
            let r = E512::from(rhs as i64);

            let (r, _) = l.overflowing_sub_s(r);
            r.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_sub_wx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_wx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwsub.wx v24, v8, t0", in (reg) x);
        });
    }

    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_sub,
        op,
        WideningCategory::VdVs2,
        "vwsub.wx",
    );
}

pub fn test_vwop_wx() {
    for sew in [64, 256] {
        for lmul in [-4, -2, 1, 2, 4] {
            for avl in avl_iterator(sew, 4) {
                test_vw_addu_wx(sew, lmul, avl);
                test_vw_add_wx(sew, lmul, avl);
                test_vw_subu_wx(sew, lmul, avl);
                test_vw_sub_wx(sew, lmul, avl);
            }
        }
    }
}
