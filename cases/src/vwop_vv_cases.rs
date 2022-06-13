use core::{arch::asm, convert::TryInto};
use eint::{Eint, E1024, E128, E2048, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::{conver_to_i1024, conver_to_i2048, conver_to_i256, conver_to_i512},
    runner::{run_template_w_vv, MaskType},
};

fn expected_op_addu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap()) as u16;
            let r = u8::from_le_bytes(rhs.try_into().unwrap()) as u16;
            let res = l.wrapping_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap()) as u32;
            let r = u16::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = l.wrapping_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap()) as u64;
            let r = u32::from_le_bytes(rhs.try_into().unwrap()) as u64;
            let res = l.wrapping_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = u64::from_le_bytes(rhs.try_into().unwrap()) as u128;
            let res = l.wrapping_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E256::from(E128::get(lhs));
            let r = E256::from(E128::get(rhs));
            l.overflowing_add_u(r).0.put(result);
        }
        256 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(E256::get(rhs));
            l.overflowing_add_u(r).0.put(result);
        }
        512 => {
            let l = E1024::from(E512::get(lhs));
            let r = E1024::from(E512::get(rhs));
            l.overflowing_add_u(r).0.put(result);
        }
        1024 => {
            let l = E2048::from(E1024::get(lhs));
            let r = E2048::from(E1024::get(rhs));
            l.overflowing_add_u(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_addu_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vwaddu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vwaddu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vv(expected_op_addu, op, true, "vwaddu.vv");
}

fn expected_op_add(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap()) as i16;
            let r = i8::from_le_bytes(rhs.try_into().unwrap()) as i16;
            let res = l.wrapping_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap()) as i32;
            let r = i16::from_le_bytes(rhs.try_into().unwrap()) as i32;
            let res = l.wrapping_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap()) as i64;
            let r = i32::from_le_bytes(rhs.try_into().unwrap()) as i64;
            let res = l.wrapping_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = i64::from_le_bytes(rhs.try_into().unwrap()) as i128;
            let res = l.wrapping_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = conver_to_i256(E128::get(lhs));
            let r = conver_to_i256(E128::get(rhs));
            l.overflowing_add_s(r).0.put(result);
        }
        256 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = conver_to_i512(E256::get(rhs));
            l.overflowing_add_s(r).0.put(result);
        }
        512 => {
            let l = conver_to_i1024(E512::get(lhs));
            let r = conver_to_i1024(E512::get(rhs));
            l.overflowing_add_s(r).0.put(result);
        }
        1024 => {
            let l = conver_to_i2048(E1024::get(lhs));
            let r = conver_to_i2048(E1024::get(rhs));
            l.overflowing_add_s(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_add_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vwadd.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vwadd.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vv(expected_op_add, op, true, "vwadd.vv");
}

fn expected_op_mulu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap()) as u16;
            let r = u8::from_le_bytes(rhs.try_into().unwrap()) as u16;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap()) as u32;
            let r = u16::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap()) as u64;
            let r = u32::from_le_bytes(rhs.try_into().unwrap()) as u64;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = u64::from_le_bytes(rhs.try_into().unwrap()) as u128;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E256::from(E128::get(lhs));
            let r = E256::from(E128::get(rhs));
            l.overflowing_mul_u(r).0.put(result);
        }
        256 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(E256::get(rhs));
            l.overflowing_mul_u(r).0.put(result);
        }
        512 => {
            let l = E1024::from(E512::get(lhs));
            let r = E1024::from(E512::get(rhs));
            l.overflowing_mul_u(r).0.put(result);
        }
        1024 => {
            let l = E2048::from(E1024::get(lhs));
            let r = E2048::from(E1024::get(rhs));
            l.overflowing_mul_u(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_mulu_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vwmulu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vwmulu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vv(expected_op_mulu, op, true, "vwmulu.vv");
}

fn expected_op_mul(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap()) as i16;
            let r = i8::from_le_bytes(rhs.try_into().unwrap()) as i16;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap()) as i32;
            let r = i16::from_le_bytes(rhs.try_into().unwrap()) as i32;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap()) as i64;
            let r = i32::from_le_bytes(rhs.try_into().unwrap()) as i64;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = i64::from_le_bytes(rhs.try_into().unwrap()) as i128;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = conver_to_i256(E128::get(lhs));
            let r = conver_to_i256(E128::get(rhs));
            l.overflowing_mul_s(r).0.put(result);
        }
        256 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = conver_to_i512(E256::get(rhs));
            l.overflowing_mul_s(r).0.put(result);
        }
        512 => {
            let l = conver_to_i1024(E512::get(lhs));
            let r = conver_to_i1024(E512::get(rhs));
            l.overflowing_mul_s(r).0.put(result);
        }
        1024 => {
            let l = conver_to_i2048(E1024::get(lhs));
            let r = conver_to_i2048(E1024::get(rhs));
            l.overflowing_mul_s(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_mul_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vwmul.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vwmul.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vv(expected_op_mul, op, true, "vwmul.vv");
}

fn expected_op_mulsu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap()) as i16;
            let r = u8::from_le_bytes(rhs.try_into().unwrap()) as i16;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap()) as i32;
            let r = u16::from_le_bytes(rhs.try_into().unwrap()) as i32;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap()) as i64;
            let r = u32::from_le_bytes(rhs.try_into().unwrap()) as i64;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = u64::from_le_bytes(rhs.try_into().unwrap()) as i128;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = conver_to_i256(E128::get(lhs));
            let r = E256::from(E128::get(rhs));
            l.overflowing_mul_u(r).0.put(result);
        }
        256 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = E512::from(E256::get(rhs));
            l.overflowing_mul_u(r).0.put(result);
        }
        512 => {
            let l = conver_to_i1024(E512::get(lhs));
            let r = E1024::from(E512::get(rhs));
            l.overflowing_mul_u(r).0.put(result);
        }
        1024 => {
            let l = conver_to_i2048(E1024::get(lhs));
            let r = E2048::from(E1024::get(rhs));
            l.overflowing_mul_u(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_mulsu_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vwmulsu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vwmulsu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vv(expected_op_mulsu, op, true, "vwmulsu.vv");
}

fn expected_op_subu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap()) as u16;
            let r = u8::from_le_bytes(rhs.try_into().unwrap()) as u16;
            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap()) as u32;
            let r = u16::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap()) as u64;
            let r = u32::from_le_bytes(rhs.try_into().unwrap()) as u64;
            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = u64::from_le_bytes(rhs.try_into().unwrap()) as u128;
            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E256::from(E128::get(lhs));
            let r = E256::from(E128::get(rhs));
            l.overflowing_sub_u(r).0.put(result);
        }
        256 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(E256::get(rhs));
            l.overflowing_sub_u(r).0.put(result);
        }
        512 => {
            let l = E1024::from(E512::get(lhs));
            let r = E1024::from(E512::get(rhs));
            l.overflowing_sub_u(r).0.put(result);
        }
        1024 => {
            let l = E2048::from(E1024::get(lhs));
            let r = E2048::from(E1024::get(rhs));
            l.overflowing_sub_u(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_subu_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vwsubu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vwsubu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vv(expected_op_subu, op, true, "vwsubu.vv");
}

fn expected_op_sub(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap()) as i16;
            let r = i8::from_le_bytes(rhs.try_into().unwrap()) as i16;
            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap()) as i32;
            let r = i16::from_le_bytes(rhs.try_into().unwrap()) as i32;
            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap()) as i64;
            let r = i32::from_le_bytes(rhs.try_into().unwrap()) as i64;
            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = i64::from_le_bytes(rhs.try_into().unwrap()) as i128;
            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        128 => {
            let l = conver_to_i256(E128::get(lhs));
            let r = conver_to_i256(E128::get(rhs));
            l.overflowing_sub_s(r).0.put(result);
        }
        256 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = conver_to_i512(E256::get(rhs));
            l.overflowing_sub_s(r).0.put(result);
        }
        512 => {
            let l = conver_to_i1024(E512::get(lhs));
            let r = conver_to_i1024(E512::get(rhs));
            l.overflowing_sub_s(r).0.put(result);
        }
        1024 => {
            let l = conver_to_i2048(E1024::get(lhs));
            let r = conver_to_i2048(E1024::get(rhs));
            l.overflowing_sub_s(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_sub_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vwsub.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vwsub.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vv(expected_op_sub, op, true, "vwsub.vv");
}

pub fn test_vwop_vv() {
    test_vw_addu_vv();
    test_vw_add_vv();
    test_vw_mulu_vv();
    test_vw_mul_vv();
    test_vw_mulsu_vv();
    test_vw_subu_vv();
    test_vw_sub_vv();
}
