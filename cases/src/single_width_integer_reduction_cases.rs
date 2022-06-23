#![allow(dead_code)]

use core::{arch::asm, convert::TryInto};
use eint::{Eint, E1024, E128, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::runner::{run_template_r_vv, MaskType};

// use ckb_std::syscalls::debug;
// use rvv_testcases::log;

fn expected_op_sum(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());
            let r = u8::from_le_bytes(rhs.try_into().unwrap());
            let res = u8::from_le_bytes(result.try_into().unwrap());

            let res2 = if index == 0 {
                l.wrapping_add(r)
            } else {
                l.wrapping_add(res)
            };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            let r = u16::from_le_bytes(rhs.try_into().unwrap());
            let res = u16::from_le_bytes(result.try_into().unwrap());

            let res2 = if index == 0 {
                l.wrapping_add(r)
            } else {
                l.wrapping_add(res)
            };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            let r = u32::from_le_bytes(rhs.try_into().unwrap());
            let res = u32::from_le_bytes(result.try_into().unwrap());

            let res2 = if index == 0 {
                l.wrapping_add(r)
            } else {
                l.wrapping_add(res)
            };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            let res = u64::from_le_bytes(result.try_into().unwrap());

            let res2 = if index == 0 {
                l.wrapping_add(r)
            } else {
                l.wrapping_add(res)
            };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let res = E128::get(result);
            let res2 = if index == 0 {
                let (res, _) = l.overflowing_add_u(r);
                res
            } else {
                let (res, _) = l.overflowing_add_u(res);
                res
            };
            res2.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = E256::get(result);
            let res2 = if index == 0 {
                let (res, _) = l.overflowing_add_u(r);
                res
            } else {
                let (res, _) = l.overflowing_add_u(res);
                res
            };
            res2.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let res = E512::get(result);
            let res2 = if index == 0 {
                let (res, _) = l.overflowing_add_u(r);
                res
            } else {
                let (res, _) = l.overflowing_add_u(res);
                res
            };
            res2.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let res = E1024::get(result);
            let res2 = if index == 0 {
                let (res, _) = l.overflowing_add_u(r);
                res
            } else {
                let (res, _) = l.overflowing_add_u(res);
                res
            };
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vredop_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vredsum.vs v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vredsum.vs v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_r_vv(expected_op_sum, op, true, "vredsum.vs");
}

fn expected_op_and(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());
            let r = u8::from_le_bytes(rhs.try_into().unwrap());
            let res = u8::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 { l & r } else { l & res };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            let r = u16::from_le_bytes(rhs.try_into().unwrap());
            let res = u16::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 { l & r } else { l & res };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            let r = u32::from_le_bytes(rhs.try_into().unwrap());
            let res = u32::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 { l & r } else { l & res };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            let res = u64::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 { l & r } else { l & res };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let res = E128::get(result);
            let res2 = if index == 0 { l & r } else { l & res };
            res2.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = E256::get(result);
            let res2 = if index == 0 { l & r } else { l & res };
            res2.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let res = E512::get(result);
            let res2 = if index == 0 { l & r } else { l & res };
            res2.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let res = E1024::get(result);
            let res2 = if index == 0 { l & r } else { l & res };
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vredop_and_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vredand.vs v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vredand.vs v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_r_vv(expected_op_and, op, true, "vredand.vs");
}

fn expected_op_or(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());
            let r = u8::from_le_bytes(rhs.try_into().unwrap());
            let res = u8::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 { l | r } else { l | res };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            let r = u16::from_le_bytes(rhs.try_into().unwrap());
            let res = u16::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 { l | r } else { l | res };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            let r = u32::from_le_bytes(rhs.try_into().unwrap());
            let res = u32::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 { l | r } else { l | res };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            let res = u64::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 { l | r } else { l | res };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let res = E128::get(result);
            let res2 = if index == 0 { l | r } else { l | res };
            res2.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = E256::get(result);
            let res2 = if index == 0 { l | r } else { l | res };
            res2.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let res = E512::get(result);
            let res2 = if index == 0 { l | r } else { l | res };
            res2.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let res = E1024::get(result);
            let res2 = if index == 0 { l | r } else { l | res };
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vredop_or_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vredor.vs v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vredor.vs v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_r_vv(expected_op_or, op, true, "vredor.vs");
}

fn expected_op_xor(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());
            let r = u8::from_le_bytes(rhs.try_into().unwrap());
            let res = u8::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 { l ^ r } else { l ^ res };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            let r = u16::from_le_bytes(rhs.try_into().unwrap());
            let res = u16::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 { l ^ r } else { l ^ res };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            let r = u32::from_le_bytes(rhs.try_into().unwrap());
            let res = u32::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 { l ^ r } else { l ^ res };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            let res = u64::from_le_bytes(result.try_into().unwrap());
            let res2 = if index == 0 { l ^ r } else { l ^ res };
            result.copy_from_slice(res2.to_le_bytes().as_slice());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let res = E128::get(result);
            let res2 = if index == 0 { l ^ r } else { l ^ res };
            res2.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = E256::get(result);
            let res2 = if index == 0 { l ^ r } else { l ^ res };
            res2.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let res = E512::get(result);
            let res2 = if index == 0 { l ^ r } else { l ^ res };
            res2.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let res = E1024::get(result);
            let res2 = if index == 0 { l ^ r } else { l ^ res };
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vredop_xor_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vredxor.vs v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vredxor.vs v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_r_vv(expected_op_xor, op, true, "vredxor.vs");
}

fn expected_op_minu(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());
            let r = u8::from_le_bytes(rhs.try_into().unwrap());
            let res = u8::from_le_bytes(result.try_into().unwrap());
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
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            let r = u16::from_le_bytes(rhs.try_into().unwrap());
            let res = u16::from_le_bytes(result.try_into().unwrap());
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
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            let r = u32::from_le_bytes(rhs.try_into().unwrap());
            let res = u32::from_le_bytes(result.try_into().unwrap());
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
        64 => {
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

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let res = E128::get(result);
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
            res2.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = E256::get(result);
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
            res2.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let res = E512::get(result);
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
            res2.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let res = E1024::get(result);
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
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vredop_minu_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vredminu.vs v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vredminu.vs v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_r_vv(expected_op_minu, op, true, "vredminu.vs");
}

fn expected_op_min(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());
            let r = i8::from_le_bytes(rhs.try_into().unwrap());
            let res = i8::from_le_bytes(result.try_into().unwrap());
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
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            let r = i16::from_le_bytes(rhs.try_into().unwrap());
            let res = i16::from_le_bytes(result.try_into().unwrap());
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
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            let r = i32::from_le_bytes(rhs.try_into().unwrap());
            let res = i32::from_le_bytes(result.try_into().unwrap());
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
        64 => {
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

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let res = E128::get(&result);
            if index == 0 {
                if l.cmp_s(&r).is_le() {
                    result.copy_from_slice(lhs);
                } else {
                    result.copy_from_slice(rhs);
                }
            } else {
                if l.cmp_s(&res).is_le() {
                    result.copy_from_slice(lhs);
                }
            };
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = E256::get(&result);
            if index == 0 {
                if l.cmp_s(&r).is_le() {
                    result.copy_from_slice(lhs);
                } else {
                    result.copy_from_slice(rhs);
                }
            } else {
                if l.cmp_s(&res).is_le() {
                    result.copy_from_slice(lhs);
                }
            };
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let res = E512::get(&result);
            if index == 0 {
                if l.cmp_s(&r).is_le() {
                    result.copy_from_slice(lhs);
                } else {
                    result.copy_from_slice(rhs);
                }
            } else {
                if l.cmp_s(&res).is_le() {
                    result.copy_from_slice(lhs);
                }
            };
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let res = E1024::get(&result);
            if index == 0 {
                if l.cmp_s(&r).is_le() {
                    result.copy_from_slice(lhs);
                } else {
                    result.copy_from_slice(rhs);
                }
            } else {
                if l.cmp_s(&res).is_le() {
                    result.copy_from_slice(lhs);
                }
            };
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vredop_min_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vredmin.vs v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vredmin.vs v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_r_vv(expected_op_min, op, true, "vredmin.vs");
}

fn expected_op_maxu(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());
            let r = u8::from_le_bytes(rhs.try_into().unwrap());
            let res = u8::from_le_bytes(result.try_into().unwrap());
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
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            let r = u16::from_le_bytes(rhs.try_into().unwrap());
            let res = u16::from_le_bytes(result.try_into().unwrap());
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
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            let r = u32::from_le_bytes(rhs.try_into().unwrap());
            let res = u32::from_le_bytes(result.try_into().unwrap());
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
        64 => {
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

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let res = E128::get(result);
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
            res2.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = E256::get(result);
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
            res2.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let res = E512::get(result);
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
            res2.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let res = E1024::get(result);
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
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vredop_maxu_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vredmaxu.vs v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vredmaxu.vs v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_r_vv(expected_op_maxu, op, true, "vredmaxu.vs");
}

fn expected_op_max(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());
            let r = i8::from_le_bytes(rhs.try_into().unwrap());
            let res = i8::from_le_bytes(result.try_into().unwrap());
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
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            let r = i16::from_le_bytes(rhs.try_into().unwrap());
            let res = i16::from_le_bytes(result.try_into().unwrap());
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
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            let r = i32::from_le_bytes(rhs.try_into().unwrap());
            let res = i32::from_le_bytes(result.try_into().unwrap());
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
        64 => {
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

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let res = E128::get(&result);
            if index == 0 {
                if l.cmp_s(&r).is_ge() {
                    result.copy_from_slice(lhs);
                } else {
                    result.copy_from_slice(rhs);
                }
            } else {
                if l.cmp_s(&res).is_ge() {
                    result.copy_from_slice(lhs);
                }
            };
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = E256::get(&result);
            if index == 0 {
                if l.cmp_s(&r).is_ge() {
                    result.copy_from_slice(lhs);
                } else {
                    result.copy_from_slice(rhs);
                }
            } else {
                if l.cmp_s(&res).is_ge() {
                    result.copy_from_slice(lhs);
                }
            };
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let res = E512::get(&result);
            if index == 0 {
                if l.cmp_s(&r).is_ge() {
                    result.copy_from_slice(lhs);
                } else {
                    result.copy_from_slice(rhs);
                }
            } else {
                if l.cmp_s(&res).is_ge() {
                    result.copy_from_slice(lhs);
                }
            };
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let res = E1024::get(&result);
            if index == 0 {
                if l.cmp_s(&r).is_ge() {
                    result.copy_from_slice(lhs);
                } else {
                    result.copy_from_slice(rhs);
                }
            } else {
                if l.cmp_s(&res).is_ge() {
                    result.copy_from_slice(lhs);
                }
            };
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vredop_max_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vredmax.vs v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vredmax.vs v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_r_vv(expected_op_max, op, true, "vredmax.vs");
}

pub fn test_vred_op() {
    test_vredop_vv();
    test_vredop_and_vv();
    test_vredop_or_vv();
    test_vredop_xor_vv();
    test_vredop_minu_vv();
    test_vredop_min_vv();
    test_vredop_maxu_vv();
    test_vredop_max_vv();
}
