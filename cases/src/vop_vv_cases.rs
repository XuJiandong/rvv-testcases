use core::arch::asm;
use core::convert::TryInto;

use alloc::boxed::Box;
use eint::{Eint, E256};
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vop_vv;

use rvv_testcases::misc::{avl_iterator, U1024, U256, U512};
use rvv_testcases::runner::{run_vop_vv, ExpectedOp, WideningCategory};

fn expected_op_add(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let (res, _) = lhs[0].overflowing_add(rhs[0]);
            result[0] = res;
        }
        2 => {
            let (r, _) = u16::from_le_bytes(lhs.try_into().unwrap())
                .overflowing_add(u16::from_le_bytes(rhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let (r, _) = u32::from_le_bytes(lhs.try_into().unwrap())
                .overflowing_add(u32::from_le_bytes(rhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let (r, _) = u64::from_le_bytes(lhs.try_into().unwrap())
                .overflowing_add(u64::from_le_bytes(rhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let (r, _) = u128::from_le_bytes(lhs.try_into().unwrap())
                .overflowing_add(u128::from_le_bytes(rhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let (r, _) =
                U256::from_little_endian(lhs).overflowing_add(U256::from_little_endian(rhs));
            r.to_little_endian(result);
        }
        64 => {
            let (r, _) =
                U512::from_little_endian(lhs).overflowing_add(U512::from_little_endian(rhs));
            r.to_little_endian(result);
        }
        128 => {
            let (r, _) =
                U1024::from_little_endian(lhs).overflowing_add(U1024::from_little_endian(rhs));
            r.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

fn test_vadd_vv(sew: u64, lmul: i64, avl: u64) {
    fn add(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vadd.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_add)),
        add,
        WideningCategory::None,
        "vadd.vv",
    );
}

fn expected_op_mul(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            let (res, _) = l.overflowing_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = U256::from_little_endian(rhs);
            let (res, _) = l.overflowing_mul(r);
            let res2: U256 = res.into();
            res2.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

fn test_vmul_vv(sew: u64, lmul: i64, avl: u64) {
    fn mul(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmul.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_mul)),
        mul,
        WideningCategory::None,
        "vmul.vv",
    );
}

fn expected_op_and(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        8 => {
            let res = u64::from_le_bytes(lhs.try_into().unwrap())
                & u64::from_le_bytes(rhs.try_into().unwrap());
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = U256::from_little_endian(rhs);
            let res = l & r;
            let res2: U256 = res.into();
            res2.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

fn test_vand_vv(sew: u64, lmul: i64, avl: u64) {
    fn and(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vand.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_and)),
        and,
        WideningCategory::None,
        "vand.vv",
    );
}

fn expected_op_or(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        8 => {
            let res = u64::from_le_bytes(lhs.try_into().unwrap())
                | u64::from_le_bytes(rhs.try_into().unwrap());
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = U256::from_little_endian(rhs);
            let res = l | r;
            let res2: U256 = res.into();
            res2.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

fn test_vor_vv(sew: u64, lmul: i64, avl: u64) {
    fn or(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vor.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_or)),
        or,
        WideningCategory::None,
        "vor.vv",
    );
}

fn expected_op_xor(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        8 => {
            let res = u64::from_le_bytes(lhs.try_into().unwrap())
                ^ u64::from_le_bytes(rhs.try_into().unwrap());
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = U256::from_little_endian(rhs);
            let res = l ^ r;
            let res2: U256 = res.into();
            res2.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

fn test_vxor_vv(sew: u64, lmul: i64, avl: u64) {
    fn xor(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vxor.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_xor)),
        xor,
        WideningCategory::None,
        "vxor.vv",
    );
}

fn expected_op_mulh(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = i64::from_le_bytes(rhs.try_into().unwrap()) as i128;
            let res = ((l * r) >> 64) as i64;
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let (_, res) = l.widening_mul_s(r);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

fn test_vmulh_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmulh.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_mulh)),
        op,
        WideningCategory::None,
        "vmulh.vv",
    );
}

fn expected_op_mulhu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = u64::from_le_bytes(rhs.try_into().unwrap()) as u128;
            let res = ((l * r) >> 64) as u64;
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let (_, res) = l.widening_mul_u(r);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

fn test_vmulhu_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmulhu.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_mulhu)),
        op,
        WideningCategory::None,
        "vmulhu.vv",
    );
}

fn expected_op_mulhsu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = u64::from_le_bytes(rhs.try_into().unwrap()) as u128;
            let (res, _) = l.overflowing_mul(r);
            let res = (res >> 64) as u64;
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let (_, res) = l.widening_mul_su(r);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

fn test_vmulhsu_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmulhsu.vv v24, v8, v16");
        });
    }

    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_mulhsu)),
        op,
        WideningCategory::None,
        "vmulhsu.vv",
    );
}

pub fn test_vop_vv() {
    for sew in [64, 256] {
        for lmul in [-8, -2, 1, 4, 8] {
            for avl in avl_iterator(sew, 4) {
                test_vadd_vv(sew, lmul, avl);
                test_vmul_vv(sew, lmul, avl);
                test_vand_vv(sew, lmul, avl);
                test_vor_vv(sew, lmul, avl);
                test_vxor_vv(sew, lmul, avl);
                test_vmulh_vv(sew, lmul, avl);
                test_vmulhu_vv(sew, lmul, avl);
                test_vmulhsu_vv(sew, lmul, avl);
            }
        }
    }
}
