use core::{arch::asm, convert::TryInto};
use eint::{Eint, E1024, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::{
        to_1024, to_128, to_256, to_512, to_i128, to_i16, to_i32, to_i64, to_i8, to_u128, to_u16,
        to_u32, to_u64, to_u8,
    },
    runner::{run_template_v_vv, MaskType},
};

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
            let (res, _) = to_256(lhs).overflowing_add_u(to_256(rhs));
            res.put(result);
        }
        64 => {
            let (res, _) = to_512(lhs).overflowing_add_u(to_512(rhs));
            res.put(result);
        }
        128 => {
            let (res, _) = to_1024(lhs).overflowing_add_u(to_1024(rhs));
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vadd_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vadd.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vadd.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_add, op, true, "vadd.vv");
}

fn expected_op_mul(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let res = to_u8(lhs).wrapping_mul(to_u8(rhs));
            result[0] = res;
        }
        2 => {
            let res = to_u16(lhs).wrapping_mul(to_u16(rhs));
            result.copy_from_slice(&res.to_le_bytes());
        }
        4 => {
            let res = to_u32(lhs).wrapping_mul(to_u32(rhs));
            result.copy_from_slice(&res.to_le_bytes());
        }
        8 => {
            let res = to_u64(lhs).wrapping_mul(to_u64(rhs));
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let res = to_u128(lhs).wrapping_mul(to_u128(rhs));
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let (res, _) = to_256(lhs).overflowing_mul_u(to_256(rhs));
            res.put(result);
        }
        64 => {
            let (res, _) = to_512(lhs).overflowing_mul_u(to_512(rhs));
            res.put(result);
        }
        128 => {
            let (res, _) = to_1024(lhs).overflowing_mul_u(to_1024(rhs));
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmul_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vmul.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vmul.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_mul, op, true, "vmul.vv");
}

fn expected_op_and(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let res = to_u8(lhs) & to_u8(rhs);
            result.copy_from_slice(&res.to_le_bytes());
        }
        2 => {
            let res = to_u16(lhs) & to_u16(rhs);
            result.copy_from_slice(&res.to_le_bytes());
        }
        4 => {
            let res = to_u32(lhs) & to_u32(rhs);
            result.copy_from_slice(&res.to_le_bytes());
        }
        8 => {
            let res = u64::from_le_bytes(lhs.try_into().unwrap())
                & u64::from_le_bytes(rhs.try_into().unwrap());
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let res = to_u128(lhs) & to_u128(rhs);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let res = to_256(lhs) & to_256(rhs);
            res.put(result);
        }
        64 => {
            let res = to_512(lhs) & to_512(rhs);
            res.put(result);
        }
        128 => {
            let res = to_1024(lhs) & to_1024(rhs);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vand_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vand.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vand.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_and, op, true, "vand.vv");
}

fn expected_op_or(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let res = to_u8(lhs) | to_u8(rhs);
            result.copy_from_slice(&res.to_le_bytes());
        }
        2 => {
            let res = to_u16(lhs) | to_u16(rhs);
            result.copy_from_slice(&res.to_le_bytes());
        }
        4 => {
            let res = to_u32(lhs) | to_u32(rhs);
            result.copy_from_slice(&res.to_le_bytes());
        }
        8 => {
            let res = to_u64(lhs) | to_u64(rhs);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let res = to_u128(lhs) | to_u128(rhs);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let res = to_256(lhs) | to_256(rhs);
            res.put(result);
        }
        64 => {
            let res = to_512(lhs) | to_512(rhs);
            res.put(result);
        }
        128 => {
            let res = to_1024(lhs) | to_1024(rhs);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vor_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vor.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vor.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_or, op, true, "vor.vv");
}

fn expected_op_xor(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let res = to_u8(lhs) ^ to_u8(rhs);
            result.copy_from_slice(&res.to_le_bytes());
        }
        2 => {
            let res = to_u16(lhs) ^ to_u16(rhs);
            result.copy_from_slice(&res.to_le_bytes());
        }
        4 => {
            let res = to_u32(lhs) ^ to_u32(rhs);
            result.copy_from_slice(&res.to_le_bytes());
        }
        8 => {
            let res = to_u64(lhs) ^ to_u64(rhs);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let res = to_u128(lhs) ^ to_u128(rhs);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let res = to_256(lhs) ^ to_256(rhs);
            res.put(result);
        }
        64 => {
            let res = to_512(lhs) ^ to_512(rhs);
            res.put(result);
        }
        128 => {
            let res = to_1024(lhs) ^ to_1024(rhs);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vxor_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vxor.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vxor.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_xor, op, true, "vxor.vv");
}

fn expected_op_mulh(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let l = to_i8(lhs) as i16;
            let r = to_i8(rhs) as i16;
            let res = ((l * r) >> 8) as i8;
            result[0] = res as u8;
        }
        2 => {
            let l = to_i16(lhs) as i32;
            let r = to_i16(rhs) as i32;
            let res = ((l * r) >> 16) as i16;
            result.copy_from_slice(&res.to_le_bytes());
        }
        4 => {
            let l = to_i32(lhs) as i64;
            let r = to_i32(rhs) as i64;
            let res = ((l * r) >> 32) as i32;
            result.copy_from_slice(&res.to_le_bytes());
        }
        8 => {
            let l = to_i64(lhs) as i128;
            let r = to_i64(rhs) as i128;
            let res = ((l * r) >> 64) as i64;
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = E256::from(to_i128(lhs));
            let r = E256::from(to_i128(rhs));
            let (res, _) = l.overflowing_mul_s(r);
            res.1.put(result);
        }
        32 => {
            let l = to_256(lhs);
            let r = to_256(rhs);
            let (_, res) = l.widening_mul_s(r);
            res.put(result);
        }
        64 => {
            let l = to_512(lhs);
            let r = to_512(rhs);
            let (_, res) = l.widening_mul_s(r);
            res.put(result);
        }
        128 => {
            let l = to_1024(lhs);
            let r = to_1024(rhs);
            let (_, res) = l.widening_mul_s(r);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmulh_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vmulh.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vmulh.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_mulh, op, true, "vmulh.vv");
}

fn expected_op_mulhu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let l = to_u8(lhs) as u16;
            let r = to_u8(rhs) as u16;
            let res = ((l * r) >> 8) as u8;
            result.copy_from_slice(&res.to_le_bytes());
        }
        2 => {
            let l = to_u16(lhs) as u32;
            let r = to_u16(rhs) as u32;
            let res = ((l * r) >> 16) as u16;
            result.copy_from_slice(&res.to_le_bytes());
        }
        4 => {
            let l = to_u32(lhs) as u64;
            let r = to_u32(rhs) as u64;
            let res = ((l * r) >> 32) as u32;
            result.copy_from_slice(&res.to_le_bytes());
        }
        8 => {
            let l = to_u64(lhs) as u128;
            let r = to_u64(rhs) as u128;
            let res = ((l * r) >> 64) as u64;
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = E256::from(to_u128(lhs));
            let r = E256::from(to_u128(rhs));
            let (res, _) = l.overflowing_mul_u(r);
            res.1.put(result);
        }
        32 => {
            let l = to_256(lhs);
            let r = to_256(rhs);
            let (_, res) = l.widening_mul_u(r);
            res.put(result);
        }
        64 => {
            let l = to_512(lhs);
            let r = to_512(rhs);
            let (_, res) = l.widening_mul_u(r);
            res.put(result);
        }
        128 => {
            let l = to_1024(lhs);
            let r = to_1024(rhs);
            let (_, res) = l.widening_mul_u(r);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmulhu_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vmulhu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vmulhu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_mulhu, op, true, "vmulhu.vv");
}

fn expected_op_mulhsu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let l = to_i8(lhs) as u16;
            let r = to_u8(rhs) as u16;
            let (res, _) = l.overflowing_mul(r);
            let res = (res >> 8) as u8;
            result[0] = res;
        }
        2 => {
            let l = to_i16(lhs) as u32;
            let r = to_u16(rhs) as u32;
            let (res, _) = l.overflowing_mul(r);
            let res = (res >> 16) as u16;
            result.copy_from_slice(&res.to_le_bytes());
        }
        4 => {
            let l = to_i32(lhs) as u64;
            let r = to_u32(rhs) as u64;
            let (res, _) = l.overflowing_mul(r);
            let res = (res >> 32) as u32;
            result.copy_from_slice(&res.to_le_bytes());
        }
        8 => {
            let l = to_i64(lhs) as u128;
            let r = to_u64(rhs) as u128;
            let (res, _) = l.overflowing_mul(r);
            let res = (res >> 64) as u64;
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = E256::from(to_i128(lhs));
            let r = E256::from(to_u128(rhs));
            let (res, _) = l.overflowing_mul_u(r);
            res.1.put(result);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let (_, res) = l.widening_mul_su(r);
            res.put(result);
        }
        64 => {
            let l = to_512(lhs);
            let r = to_512(rhs);
            let (_, res) = l.widening_mul_su(r);
            res.put(result);
        }
        128 => {
            let l = to_1024(lhs);
            let r = to_1024(rhs);
            let (_, res) = l.widening_mul_su(r);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmulhsu_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vmulhsu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vmulhsu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_mulhsu, op, true, "vmulhsu.vv");
}

fn expected_op_divu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let l = to_u8(lhs);
            let r = to_u8(rhs);
            if r == 0 {
                result.copy_from_slice(&u8::MAX.to_le_bytes());
            } else {
                let res = l.wrapping_div(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        2 => {
            let l = to_u16(lhs);
            let r = to_u16(rhs);
            if r == 0 {
                result.copy_from_slice(&u16::MAX.to_le_bytes());
            } else {
                let res = l.wrapping_div(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        4 => {
            let l = to_u32(lhs);
            let r = to_u32(rhs);
            if r == 0 {
                result.copy_from_slice(&u32::MAX.to_le_bytes());
            } else {
                let res = l.wrapping_div(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        8 => {
            let l = to_u64(lhs);
            let r = to_u64(rhs);
            if r == 0 {
                result.copy_from_slice(&u64::MAX.to_le_bytes());
            } else {
                let res = l.wrapping_div(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let l = to_u128(lhs);
            let r = to_u128(rhs);
            if r == 0 {
                result.copy_from_slice(&u128::MAX.to_le_bytes());
            } else {
                let res = l.wrapping_div(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            let l = to_256(lhs);
            let r = to_256(rhs);
            let res = l.wrapping_div_u(r);
            res.put(result);
        }
        64 => {
            let l = to_512(lhs);
            let r = to_512(rhs);
            let res = l.wrapping_div_u(r);
            res.put(result);
        }
        128 => {
            let l = to_1024(lhs);
            let r = to_1024(rhs);
            let res = l.wrapping_div_u(r);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vdivu_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vdivu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vdivu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_divu, op, true, "vdivu.vv");
}

fn expected_op_div(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let l = to_i8(lhs);
            let r = to_i8(rhs);
            if r == 0 {
                result.copy_from_slice(&u8::MAX.to_le_bytes());
            } else {
                let res = l.wrapping_div(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        2 => {
            let l = to_i16(lhs);
            let r = to_i16(rhs);
            if r == 0 {
                result.copy_from_slice(&u16::MAX.to_le_bytes());
            } else {
                let res = l.wrapping_div(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        4 => {
            let l = to_i32(lhs);
            let r = to_i32(rhs);
            if r == 0 {
                result.copy_from_slice(&u32::MAX.to_le_bytes());
            } else {
                let res = l.wrapping_div(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        8 => {
            let l = to_i64(lhs);
            let r = to_i64(rhs);
            if r == 0 {
                result.copy_from_slice(&u64::MAX.to_le_bytes());
            } else {
                let res = l.wrapping_div(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let l = to_i128(lhs);
            let r = to_i128(rhs);
            if r == 0 {
                result.copy_from_slice(&u128::MAX.to_le_bytes());
            } else {
                let res = l.wrapping_div(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            let l = to_256(lhs);
            let r = to_256(rhs);
            let res = l.wrapping_div_s(r);
            res.put(result);
        }
        64 => {
            let l = to_512(lhs);
            let r = to_512(rhs);
            let res = l.wrapping_div_s(r);
            res.put(result);
        }
        128 => {
            let l = to_1024(lhs);
            let r = to_1024(rhs);
            let res = l.wrapping_div_s(r);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vdiv_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vdiv.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vdiv.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_div, op, true, "vdiv.vv");
}

fn expected_op_remu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let l = to_u8(lhs);
            let r = to_u8(rhs);
            if r == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = l % r;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        2 => {
            let l = to_u16(lhs);
            let r = to_u16(rhs);
            if r == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = l % r;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        4 => {
            let l = to_u32(lhs);
            let r = to_u32(rhs);
            if r == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = l % r;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        8 => {
            let l = to_u64(lhs);
            let r = to_u64(rhs);
            if r == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = l % r;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let l = to_u128(lhs);
            let r = to_u128(rhs);
            if r == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = l % r;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = l.wrapping_rem_u(r);
            res.put(result);
        }
        64 => {
            let l = to_512(lhs);
            let r = to_512(rhs);
            let res = l.wrapping_rem_u(r);
            res.put(result);
        }
        128 => {
            let l = to_1024(lhs);
            let r = to_1024(rhs);
            let res = l.wrapping_rem_u(r);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vremu_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vremu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vremu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_remu, op, true, "vremu.vv");
}

fn expected_op_rem(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let l = to_i8(lhs);
            let r = to_i8(rhs);
            if r == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = l.wrapping_rem(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        2 => {
            let l = to_i16(lhs);
            let r = to_i16(rhs);
            if r == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = l.wrapping_rem(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        4 => {
            let l = to_i32(lhs);
            let r = to_i32(rhs);
            if r == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = l.wrapping_rem(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        8 => {
            let l = to_i64(lhs);
            let r = to_i64(rhs);
            if r == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = l.wrapping_rem(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let l = to_i128(lhs);
            let r = to_i128(rhs);
            if r == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = l.wrapping_rem(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            let l = to_256(lhs);
            let r = to_256(rhs);
            let res = l.wrapping_rem_s(r);
            res.put(result);
        }
        64 => {
            let l = to_512(lhs);
            let r = to_512(rhs);
            let res = l.wrapping_rem_s(r);
            res.put(result);
        }
        128 => {
            let l = to_1024(lhs);
            let r = to_1024(rhs);
            let res = l.wrapping_rem_s(r);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vrem_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vrem.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vrem.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_rem, op, true, "vrem.vv");
}

fn expected_op_minu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            result.copy_from_slice(if to_u8(lhs) < to_u8(rhs) { lhs } else { rhs });
        }
        2 => {
            result.copy_from_slice(if to_u16(lhs) < to_u16(rhs) { lhs } else { rhs });
        }
        4 => {
            result.copy_from_slice(if to_u32(lhs) < to_u32(rhs) { lhs } else { rhs });
        }
        8 => {
            result.copy_from_slice(if to_u64(lhs) < to_u64(rhs) { lhs } else { rhs });
        }
        16 => {
            result.copy_from_slice(if to_u128(lhs) < to_u128(rhs) {
                lhs
            } else {
                rhs
            });
        }
        32 => {
            result.copy_from_slice(if to_256(lhs).cmp_u(&to_256(rhs)).is_le() {
                lhs
            } else {
                rhs
            });
        }
        64 => {
            result.copy_from_slice(if to_512(lhs).cmp_u(&to_512(rhs)).is_le() {
                lhs
            } else {
                rhs
            });
        }
        128 => {
            result.copy_from_slice(if to_1024(lhs).cmp_u(&to_1024(rhs)).is_le() {
                lhs
            } else {
                rhs
            });
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vminu_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vminu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vminu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_minu, op, true, "vminu.vv");
}

fn expected_op_min(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            result.copy_from_slice(if to_i8(lhs) < to_i8(rhs) { lhs } else { rhs });
        }
        2 => {
            result.copy_from_slice(if to_i16(lhs) < to_i16(rhs) { lhs } else { rhs });
        }
        4 => {
            result.copy_from_slice(if to_i32(lhs) < to_i32(rhs) { lhs } else { rhs });
        }
        8 => {
            result.copy_from_slice(if to_i64(lhs) < to_i64(rhs) { lhs } else { rhs });
        }
        16 => {
            result.copy_from_slice(if to_i128(lhs) < to_i128(rhs) {
                lhs
            } else {
                rhs
            });
        }
        32 => {
            result.copy_from_slice(if to_256(lhs).cmp_s(&to_256(rhs)).is_le() {
                lhs
            } else {
                rhs
            });
        }
        64 => {
            result.copy_from_slice(if to_512(lhs).cmp_s(&to_512(rhs)).is_le() {
                lhs
            } else {
                rhs
            });
        }
        128 => {
            result.copy_from_slice(if to_1024(lhs).cmp_s(&to_1024(rhs)).is_le() {
                lhs
            } else {
                rhs
            });
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmin_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vmin.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vmin.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_min, op, true, "vmin.vv");
}

fn expected_op_maxu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            result.copy_from_slice(if to_u8(lhs) > to_u8(rhs) { lhs } else { rhs });
        }
        2 => {
            result.copy_from_slice(if to_u16(lhs) > to_u16(rhs) { lhs } else { rhs });
        }
        4 => {
            result.copy_from_slice(if to_u32(lhs) > to_u32(rhs) { lhs } else { rhs });
        }
        8 => {
            result.copy_from_slice(if to_u64(lhs) > to_u64(rhs) { lhs } else { rhs });
        }
        16 => {
            result.copy_from_slice(if to_u128(lhs) > to_u128(rhs) {
                lhs
            } else {
                rhs
            });
        }
        32 => {
            result.copy_from_slice(if to_256(lhs).cmp_u(&to_256(rhs)).is_gt() {
                lhs
            } else {
                rhs
            });
        }
        64 => {
            result.copy_from_slice(if to_512(lhs).cmp_u(&to_512(rhs)).is_gt() {
                lhs
            } else {
                rhs
            });
        }
        128 => {
            result.copy_from_slice(if to_1024(lhs).cmp_u(&to_1024(rhs)).is_gt() {
                lhs
            } else {
                rhs
            });
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmaxu_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vmaxu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vmaxu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_maxu, op, true, "vmaxu.vv");
}

fn expected_op_max(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            result.copy_from_slice(if to_i8(lhs) > to_i8(rhs) { lhs } else { rhs });
        }
        2 => {
            result.copy_from_slice(if to_i16(lhs) > to_i16(rhs) { lhs } else { rhs });
        }
        4 => {
            result.copy_from_slice(if to_i32(lhs) > to_i32(rhs) { lhs } else { rhs });
        }
        8 => {
            result.copy_from_slice(if to_i64(lhs) > to_i64(rhs) { lhs } else { rhs });
        }
        16 => {
            result.copy_from_slice(if to_i128(lhs) > to_i128(rhs) {
                lhs
            } else {
                rhs
            });
        }
        32 => {
            result.copy_from_slice(if to_256(lhs).cmp_s(&to_256(rhs)).is_gt() {
                lhs
            } else {
                rhs
            });
        }
        64 => {
            result.copy_from_slice(if to_512(lhs).cmp_s(&to_512(rhs)).is_gt() {
                lhs
            } else {
                rhs
            });
        }
        128 => {
            result.copy_from_slice(if to_1024(lhs).cmp_s(&to_1024(rhs)).is_gt() {
                lhs
            } else {
                rhs
            });
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmax_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vmax.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vmax.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_max, op, true, "vmax.vv");
}

fn expected_op_smul(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let l = to_i8(lhs);
            let r = to_i8(rhs);

            let res = if l == r && l == i8::MIN {
                i8::MAX
            } else {
                ((l as i16).wrapping_mul(r as i16) >> 7) as i8
            };

            result.copy_from_slice(&res.to_le_bytes());
        }
        2 => {
            let l = to_i16(lhs);
            let r = to_i16(rhs);

            let res = if l == r && l == i16::MIN {
                i16::MAX
            } else {
                ((l as i32).wrapping_mul(r as i32) >> 15) as i16
            };

            result.copy_from_slice(&res.to_le_bytes());
        }
        4 => {
            let l = to_i32(lhs);
            let r = to_i32(rhs);

            let res = if l == r && l == i32::MIN {
                i32::MAX
            } else {
                ((l as i64).wrapping_mul(r as i64) >> 31) as i32
            };

            result.copy_from_slice(&res.to_le_bytes());
        }
        8 => {
            let l = to_i64(lhs);
            let r = to_i64(rhs);

            let res = if l == r && l == i64::MIN {
                i64::MAX
            } else {
                ((l as i128).wrapping_mul(r as i128) >> 63) as i64
            };

            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = to_128(lhs);
            let r = to_128(rhs);
            let (res_l, res_h) = l.widening_mul_s(r);
            let res = res_l.wrapping_shr(127) | res_h.wrapping_shl(1);
            res.put(result);
        }
        32 => {
            let l = to_256(lhs);
            let r = to_256(rhs);
            if l == r && l == E256::MIN_S {
                E256::MAX_S.put(result);
            } else {
                let (res_l, res_h) = l.widening_mul_s(r);
                let res = res_l.wrapping_shr(255) | res_h.wrapping_shl(1);
                res.put(result);
            }
        }
        64 => {
            let l = to_512(lhs);
            let r = to_512(rhs);

            if l == r && l == E512::MIN_S {
                E512::MAX_S.put(result);
            } else {
                let (res_l, res_h) = l.widening_mul_s(r);
                let res = res_l.wrapping_shr(511) | res_h.wrapping_shl(1);
                res.put(result);
            }
        }
        128 => {
            let l = to_1024(lhs);
            let r = to_1024(rhs);

            if l == r && l == E1024::MIN_S {
                E1024::MAX_S.put(result);
            } else {
                let (res_l, res_h) = l.widening_mul_s(r);
                let res = res_l.wrapping_shr(1023) | res_h.wrapping_shl(1);
                res.put(result);
            }
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vsmul_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vsmul.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vsmul.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_smul, op, true, "vsmul.vv");
}

pub fn test_vop_vv() {
    test_vadd_vv();
    test_vmul_vv();
    test_vand_vv();
    test_vor_vv();
    test_vxor_vv();
    test_vmulh_vv();
    test_vmulhu_vv();
    test_vmulhsu_vv();
    test_vdivu_vv();
    test_vdiv_vv();
    test_vremu_vv();
    test_vrem_vv();
    test_vminu_vv();
    test_vmin_vv();
    test_vmaxu_vv();
    test_vmax_vv();
    test_vsmul_vv();
}
