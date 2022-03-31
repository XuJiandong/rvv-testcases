#![allow(dead_code)]

use core::arch::asm;
use core::convert::TryInto;

use alloc::boxed::Box;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vredop_vs;

// use ckb_std::syscalls::debug;
// use rvv_testcases::log;
use rvv_testcases::misc::{avl_iterator, less_i256, greater_i256, U256};
use rvv_testcases::runner::{run_vop_vv, ExpectedOp, WideningCategory};

fn expected_op_sum(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            let res = u64::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 {
                l.wrapping_add(r)
            } else {
                l.wrapping_add(res)
            };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
            // log!("accumulate l = {:x}, r = {:x}, result = {:x}", l, r, res);
        }
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = U256::from_little_endian(rhs);
            let res = U256::from_little_endian(result);
            let res2 = if index == 0 {
                let (res, _) = l.overflowing_add(r);
                res
            } else {
                let (res, _) = l.overflowing_add(res);
                res
            };
            res2.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vredop_vv() {
    fn sum(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vredop_vs(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vredsum.vs v24, v1, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                ExpectedOp::Reduction(Box::new(expected_op_sum)),
                sum,
                WideningCategory::None,
                "vredsum.vs",
            );
        }
    }
}

fn expected_op_and(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            let res = u64::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 { l & r } else { l & res };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = U256::from_little_endian(rhs);
            let res = U256::from_little_endian(result);
            let res2 = if index == 0 { l & r } else { l & res };
            res2.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vredop_and_vv() {
    fn sum(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vredop_vs(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vredand.vs v24, v1, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                ExpectedOp::Reduction(Box::new(expected_op_and)),
                sum,
                WideningCategory::None,
                "vredand.vs",
            );
        }
    }
}

fn expected_op_or(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            let res = u64::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 { l | r } else { l | res };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = U256::from_little_endian(rhs);
            let res = U256::from_little_endian(result);
            let res2 = if index == 0 { l | r } else { l | res };
            res2.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vredop_or_vv() {
    fn sum(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vredop_vs(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vredor.vs v24, v1, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                ExpectedOp::Reduction(Box::new(expected_op_or)),
                sum,
                WideningCategory::None,
                "vredor.vs",
            );
        }
    }
}

fn expected_op_xor(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            let res = u64::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 { l ^ r } else { l ^ res };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = U256::from_little_endian(rhs);
            let res = U256::from_little_endian(result);
            let res2 = if index == 0 { l ^ r } else { l ^ res };
            res2.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vredop_xor_vv() {
    fn sum(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vredop_vs(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vredxor.vs v24, v1, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                ExpectedOp::Reduction(Box::new(expected_op_xor)),
                sum,
                WideningCategory::None,
                "vredxor.vs",
            );
        }
    }
}

fn expected_op_minu(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            let res = u64::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 {
                if l < r {
                    l
                } else {
                    r
                }
            } else {
                if l < res {
                    l
                } else {
                    res
                }
            };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = U256::from_little_endian(rhs);
            let res = U256::from_little_endian(result);
            let res2 = if index == 0 {
                if l < r {
                    l
                } else {
                    r
                }
            } else {
                if l < res {
                    l
                } else {
                    res
                }
            };
            res2.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vredop_minu_vv() {
    fn sum(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vredop_vs(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vredminu.vs v24, v1, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                ExpectedOp::Reduction(Box::new(expected_op_minu)),
                sum,
                WideningCategory::None,
                "vredminu.vs",
            );
        }
    }
}

fn expected_op_min(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap());
            let res = i64::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 {
                if l < r {
                    l
                } else {
                    r
                }
            } else {
                if l < res {
                    l
                } else {
                    res
                }
            };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        32 => {
            if index == 0 {
                if less_i256(lhs, rhs) {
                    result.copy_from_slice(lhs);
                } else {
                    result.copy_from_slice(rhs);
                }
            } else {
                if less_i256(lhs, result) {
                    result.copy_from_slice(lhs);
                }
            };
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vredop_min_vv() {
    fn sum(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vredop_vs(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vredmin.vs v24, v1, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                ExpectedOp::Reduction(Box::new(expected_op_min)),
                sum,
                WideningCategory::None,
                "vredmin.vs",
            );
        }
    }
}

fn expected_op_maxu(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            let res = u64::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 {
                if l > r {
                    l
                } else {
                    r
                }
            } else {
                if l > res {
                    l
                } else {
                    res
                }
            };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = U256::from_little_endian(rhs);
            let res = U256::from_little_endian(result);
            let res2 = if index == 0 {
                if l > r {
                    l
                } else {
                    r
                }
            } else {
                if l > res {
                    l
                } else {
                    res
                }
            };
            res2.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vredop_maxu_vv() {
    fn sum(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vredop_vs(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vredmaxu.vs v24, v1, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                ExpectedOp::Reduction(Box::new(expected_op_maxu)),
                sum,
                WideningCategory::None,
                "vredmaxu.vs",
            );
        }
    }
}

fn expected_op_max(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap());
            let res = i64::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 {
                if l > r {
                    l
                } else {
                    r
                }
            } else {
                if l > res {
                    l
                } else {
                    res
                }
            };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        32 => {
            if index == 0 {
                if greater_i256(lhs, rhs) {
                    result.copy_from_slice(lhs);
                } else {
                    result.copy_from_slice(rhs);
                }
            } else {
                if greater_i256(lhs, result) {
                    result.copy_from_slice(lhs);
                }
            };
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vredop_max_vv() {
    fn sum(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vredop_vs(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vredmax.vs v24, v1, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                ExpectedOp::Reduction(Box::new(expected_op_max)),
                sum,
                WideningCategory::None,
                "vredmax.vs",
            );
        }
    }
}
