use core::arch::asm;
use core::cmp::Ordering::{Greater, Less};
use core::convert::TryInto;
use eint::{Eint, E128, E256};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::{Widening, U256},
    runner::{run_template_v_vx, MaskType},
};

fn expected_op_add(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            result[0] = lhs[0].wrapping_add(x as u8);
        }
        2 => {
            let r = u16::from_le_bytes(lhs.try_into().unwrap()).wrapping_add(x as u16);
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = u32::from_le_bytes(lhs.try_into().unwrap()).wrapping_add(x as u32);
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = u64::from_le_bytes(lhs.try_into().unwrap()).wrapping_add(x as u64);
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = u128::from_le_bytes(lhs.try_into().unwrap()).wrapping_add(x as u128);
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let (r, _) = U256::from_little_endian(lhs).overflowing_add(x.sign_extend());
            r.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vadd_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vadd.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vadd.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_add, op, true, "vadd.vx");
}

fn expected_op_sub(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            let (r, _) = lhs[0].overflowing_sub(x as u8);
            result[0] = r;
        }
        2 => {
            let (r, _) = u16::from_le_bytes(lhs.try_into().unwrap()).overflowing_sub(x as u16);
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let (r, _) = u32::from_le_bytes(lhs.try_into().unwrap()).overflowing_sub(x as u32);
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let (r, _) = u64::from_le_bytes(lhs.try_into().unwrap()).overflowing_sub(x as u64);
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let (r, _) = u128::from_le_bytes(lhs.try_into().unwrap()).overflowing_sub(x as u128);
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let (r, _) = U256::from_little_endian(lhs).overflowing_sub(x.sign_extend());
            r.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vsub_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vsub.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vsub.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_sub, op, true, "vsub.vx");
}

fn expected_op_rsub(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            result[0] = (x as u8).wrapping_sub(lhs[0]);
        }
        2 => {
            let r = (x as u16).wrapping_sub(u16::from_le_bytes(lhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = (x as u32).wrapping_sub(u32::from_le_bytes(lhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = (x as u64).wrapping_sub(u64::from_le_bytes(lhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = (x as u128).wrapping_sub(u128::from_le_bytes(lhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let (r, _) = x
                .sign_extend()
                .overflowing_sub(U256::from_little_endian(lhs));
            r.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vrsub_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vrsub.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vrsub.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_rsub, op, true, "vrsub.vx");
}

fn expected_op_and(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            result[0] = x as u8 & lhs[0];
        }
        2 => {
            let r = x as u16 & u16::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = x as u32 & u32::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = x as u64 & u64::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = x as u128 & u128::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let r = x.sign_extend() & U256::from_little_endian(lhs);
            r.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vand_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vand.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vand.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_and, op, true, "vand.vx");
}

fn expected_op_or(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            result[0] = x as u8 - lhs[0];
        }
        2 => {
            let r = x as u16 | u16::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = x as u32 | u32::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = x as u64 | u64::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = x as u128 | u128::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let r = x.sign_extend() | U256::from_little_endian(lhs);
            r.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vor_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vor.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vor.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_or, op, true, "vor.vx");
}

fn expected_op_xor(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            result[0] = x as u8 - lhs[0];
        }
        2 => {
            let r = x as u16 ^ u16::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = x as u32 ^ u32::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = x as u64 ^ u64::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = x as u128 ^ u128::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let r = x.sign_extend() ^ U256::from_little_endian(lhs);
            r.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vxor_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vxor.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vxor.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_xor, op, true, "vxor.vx");
}

fn expected_op_mul(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            result[0] = (x as u8).wrapping_mul(lhs[0]);
        }
        2 => {
            let r = i16::from_le_bytes(lhs.try_into().unwrap()).wrapping_mul(x as i16);
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = i32::from_le_bytes(lhs.try_into().unwrap()).wrapping_mul(x as i32);
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = i64::from_le_bytes(lhs.try_into().unwrap()).wrapping_mul(x as i64);
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = i128::from_le_bytes(lhs.try_into().unwrap()).wrapping_mul(x as i128);
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
            let r = l.wrapping_mul(E256::from(x as i64));
            r.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmul_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmul.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmul.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_mul, op, true, "vmul.vx");
}

fn expected_op_mulh(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            let r = ((lhs[0] as i8) as i16) * x as i16;
            let r = (r >> 8) as i8;
            result.copy_from_slice(&r.to_le_bytes());
        }
        2 => {
            let r = i16::from_le_bytes(lhs.try_into().unwrap()) as i32 * x as i32;
            let r = (r >> 16) as i16;
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = i32::from_le_bytes(lhs.try_into().unwrap()) as i64 * x as i64;
            let r = (r >> 32) as i32;
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r1 = i64::from_le_bytes(lhs.try_into().unwrap()) as i128 * ((x as i64) as i128);
            let r = (r1 >> 64) as i64;

            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
            let (_, res) = l.widening_mul_s(E256::from(x as i64));
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmulh_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmulh.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmulh.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_mulh, op, true, "vmulh.vx");
}

fn expected_op_mulhu(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            let r = lhs[0] as u32 * x as u32;
            let r = (r >> 8) as u8;
            result.copy_from_slice(&r.to_le_bytes());
        }
        2 => {
            let r = u16::from_le_bytes(lhs.try_into().unwrap()) as u32 * x as u32;
            let r = (r >> 16) as u16;
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = u32::from_le_bytes(lhs.try_into().unwrap()) as u64 * x as u64;
            let r = (r >> 32) as u32;
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = u64::from_le_bytes(lhs.try_into().unwrap()) as u128 * x as u128;
            let r = (r >> 64) as u64;
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
            let (_, res) = l.widening_mul_u(E256::from(x as u64));
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmulhu_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmulhu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmulhu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_mulhu, op, true, "vmulhu.vx");
}

fn expected_op_mulhsu(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            let r = ((lhs[0] as i8) as u32).wrapping_mul(x as u32);
            let r = (r >> 8) as u8;
            result.copy_from_slice(&r.to_le_bytes());
        }
        2 => {
            let r = (i16::from_le_bytes(lhs.try_into().unwrap()) as u32).wrapping_mul(x as u32);
            let r = (r >> 16) as u16;
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = (i32::from_le_bytes(lhs.try_into().unwrap()) as u64).wrapping_mul(x as u64);
            let r = (r >> 32) as u32;
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = (i64::from_le_bytes(lhs.try_into().unwrap()) as u128).wrapping_mul(x as u128);
            let r = (r >> 64) as u64;
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
            let (_, res) = l.widening_mul_su(E256::from(x as u64));
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmulhsu_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmulhsu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmulhsu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_mulhsu, op, true, "vmulhsu.vx");
}

fn expected_op_divu(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            let x = x as u8;
            if x == 0 {
                result.copy_from_slice(&u8::MAX.to_le_bytes());
            } else {
                let res = lhs[0].wrapping_div(x);
                result[0] = res;
            }
        }
        2 => {
            let x = x as u16;
            if x == 0 {
                result.copy_from_slice(&u16::MAX.to_le_bytes());
            } else {
                let res = u16::from_le_bytes(lhs.try_into().unwrap()).wrapping_div(x as u16);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        4 => {
            let x = x as u32;
            if x == 0 {
                result.copy_from_slice(&u32::MAX.to_le_bytes());
            } else {
                let res = u32::from_le_bytes(lhs.try_into().unwrap()).wrapping_div(x as u32);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        8 => {
            if x == 0 {
                result.copy_from_slice(&u64::MAX.to_le_bytes());
            } else {
                let res = u64::from_le_bytes(lhs.try_into().unwrap()).wrapping_div(x);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let x = x as u128;
            if x == 0 {
                result.copy_from_slice(&u128::MAX.to_le_bytes());
            } else {
                let res = u128::from_le_bytes(lhs.try_into().unwrap()).wrapping_div(x as u128);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            let l = E256::get(lhs);
            let res = l.wrapping_div_u(E256::from(x));
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vdivu_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vdivu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vdivu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_divu, op, true, "vdivu.vx");
}

fn expected_op_div(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            let x = x as i8;
            if x == 0 {
                result.copy_from_slice(&u8::MAX.to_le_bytes());
            } else {
                let res = (lhs[0] as i8).wrapping_div(x);
                result[0] = res as u8;
            }
        }
        2 => {
            let x = x as i16;
            if x == 0 {
                result.copy_from_slice(&u16::MAX.to_le_bytes());
            } else {
                let res = i16::from_le_bytes(lhs.try_into().unwrap()).wrapping_div(x as i16);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        4 => {
            let x = x as i32;
            if x == 0 {
                result.copy_from_slice(&u32::MAX.to_le_bytes());
            } else {
                let res = i32::from_le_bytes(lhs.try_into().unwrap()).wrapping_div(x as i32);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        8 => {
            if x == 0 {
                result.copy_from_slice(&u64::MAX.to_le_bytes());
            } else {
                let res = i64::from_le_bytes(lhs.try_into().unwrap()).wrapping_div(x as i64);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let x = x as i128;
            if x == 0 {
                result.copy_from_slice(&u128::MAX.to_le_bytes());
            } else {
                let res = i128::from_le_bytes(lhs.try_into().unwrap()).wrapping_div(x as i128);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            let l = E256::get(lhs);
            let res = l.wrapping_div_s(E256::from(x as i64));
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vdiv_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vdiv.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vdiv.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_div, op, true, "vdiv.vx");
}

fn expected_op_remu(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            let x = x as u8;
            if x == 0 {
                result[0] = lhs[0];
            } else {
                let res = lhs[0] as u8 % x;
                result[0] = res;
            }
        }
        2 => {
            let x = x as u16;
            if x == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = u16::from_le_bytes(lhs.try_into().unwrap()) % (x as u16);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        4 => {
            let x = x as u32;
            if x == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = u32::from_le_bytes(lhs.try_into().unwrap()) % (x as u32);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        8 => {
            if x == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = u64::from_le_bytes(lhs.try_into().unwrap()) % (x as u64);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let x = x as u128;
            if x == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = u128::from_le_bytes(lhs.try_into().unwrap()) % (x as u128);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            let l = E256::get(lhs);
            let res = l.wrapping_rem_u(E256::from(x as u64));
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vremu_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vremu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vremu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_remu, op, true, "vremu.vx");
}

fn expected_op_rem(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            let x = x as i8;
            if x == 0 {
                result[0] = lhs[0];
            } else {
                let res = lhs[0] as i8 % x;
                result[0] = res as u8;
            }
        }
        2 => {
            let x = x as i16;
            if x == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = i16::from_le_bytes(lhs.try_into().unwrap()) % x;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        4 => {
            let x = x as i32;
            if x == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = i32::from_le_bytes(lhs.try_into().unwrap()) % x;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        8 => {
            let x = x as i64;
            if x == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = i64::from_le_bytes(lhs.try_into().unwrap()) % x;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let x = x as i128;
            if x == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = i128::from_le_bytes(lhs.try_into().unwrap()) % x;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            let l = E256::get(lhs);
            let res = l.wrapping_rem_s(E256::from(x as i64));
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vrem_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vrem.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vrem.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_rem, op, true, "vrem.vx");
}

fn expected_op_minu(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            result[0] = if lhs[0] < x as u8 { lhs[0] } else { x as u8 };
        }
        2 => {
            if u16::from_le_bytes(lhs.try_into().unwrap()) < x as u16 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as u16).to_le_bytes().as_slice())
            }
        }
        4 => {
            if u32::from_le_bytes(lhs.try_into().unwrap()) < x as u32 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as u32).to_le_bytes().as_slice())
            }
        }
        8 => {
            if u64::from_le_bytes(lhs.try_into().unwrap()) < x as u64 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as u64).to_le_bytes().as_slice())
            }
        }
        16 => {
            if u128::from_le_bytes(lhs.try_into().unwrap()) < x as u128 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as u128).to_le_bytes().as_slice())
            }
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x);
            if l.cmp_u(&r) == Less {
                result.copy_from_slice(lhs);
            } else {
                r.put(result);
            }
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vminu_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vminu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vminu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_minu, op, true, "vminu.vx");
}

fn expected_op_min(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            result[0] = if (lhs[0] as i8) < (x as i8) {
                lhs[0]
            } else {
                x as u8
            };
        }
        2 => {
            if i16::from_le_bytes(lhs.try_into().unwrap()) < x as i16 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as i16).to_le_bytes().as_slice())
            }
        }
        4 => {
            if i32::from_le_bytes(lhs.try_into().unwrap()) < x as i32 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as i32).to_le_bytes().as_slice())
            }
        }
        8 => {
            if i64::from_le_bytes(lhs.try_into().unwrap()) < x as i64 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as i64).to_le_bytes().as_slice())
            }
        }
        16 => {
            if i128::from_le_bytes(lhs.try_into().unwrap()) < x as i128 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as i128).to_le_bytes().as_slice())
            }
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x);
            if l.cmp_s(&r) == Less {
                result.copy_from_slice(lhs);
            } else {
                E256::from(x as i64).put(result);
            }
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmin_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmin.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmin.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_min, op, true, "vmin.vx");
}

fn expected_op_maxu(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            result[0] = if lhs[0] > x as u8 { lhs[0] } else { x as u8 };
        }
        2 => {
            if u16::from_le_bytes(lhs.try_into().unwrap()) > x as u16 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as u16).to_le_bytes().as_slice())
            }
        }
        4 => {
            if u32::from_le_bytes(lhs.try_into().unwrap()) > x as u32 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as u32).to_le_bytes().as_slice())
            }
        }
        8 => {
            if u64::from_le_bytes(lhs.try_into().unwrap()) > x as u64 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as u64).to_le_bytes().as_slice())
            }
        }
        16 => {
            if u128::from_le_bytes(lhs.try_into().unwrap()) > x as u128 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as u128).to_le_bytes().as_slice())
            }
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x);
            if l.cmp_u(&r) == Greater {
                result.copy_from_slice(lhs);
            } else {
                result[..8].copy_from_slice(x.to_le_bytes().as_slice());
            }
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmaxu_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmaxu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmaxu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_maxu, op, true, "vmaxu.vx");
}

fn expected_op_max(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            result[0] = if (lhs[0] as i8) > (x as i8) {
                lhs[0]
            } else {
                x as u8
            };
        }
        2 => {
            if i16::from_le_bytes(lhs.try_into().unwrap()) > x as i16 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as i16).to_le_bytes().as_slice())
            }
        }
        4 => {
            if i32::from_le_bytes(lhs.try_into().unwrap()) > x as i32 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as i32).to_le_bytes().as_slice())
            }
        }
        8 => {
            if i64::from_le_bytes(lhs.try_into().unwrap()) > x as i64 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as i64).to_le_bytes().as_slice())
            }
        }
        16 => {
            if i128::from_le_bytes(lhs.try_into().unwrap()) > x as i128 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as i128).to_le_bytes().as_slice())
            }
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x);
            if l.cmp_s(&r) == Greater {
                result.copy_from_slice(lhs);
            } else {
                E256::from(x as i64).put(result);
            }
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmax_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmax.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmax.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_max, op, true, "vmax.vx");
}

fn expected_op_smul(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            let l = lhs[0] as i16;
            let res = (l.wrapping_mul(x as i16) >> 7) as i8;

            result.copy_from_slice(&res.to_le_bytes());
        }
        2 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap()) as i32;
            let res = (l.wrapping_mul(x as i32) >> 15) as i16;

            result.copy_from_slice(&res.to_le_bytes());
        }
        4 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap()) as i64;
            let res = (l.wrapping_mul(x as i64) >> 31) as i32;

            result.copy_from_slice(&res.to_le_bytes());
        }
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let res = (l.wrapping_mul((x as i64) as i128) >> 63) as i64;

            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let (res_l, res_h) = E128::get(lhs).widening_mul_s(E128::from(x as i64));
            let res = res_l.wrapping_shr(127) | res_h.wrapping_shl(1);
            res.put(result);
        }
        32 => {
            let (res_l, res_h) = E256::get(lhs).widening_mul_s(E256::from(x as i64));
            let res = res_l.wrapping_shr(255) | res_h.wrapping_shl(1);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vsmul_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vsmul.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vsmul.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_smul, op, true, "vsmul.vx");
}

pub fn test_vop_vx() {
    test_vadd_vx();
    test_vsub_vx();
    test_vrsub_vx();
    test_vand_vx();
    test_vor_vx();
    test_vxor_vx();
    test_vmul_vx();
    test_vmulh_vx();
    test_vmulhu_vx();
    test_vmulhsu_vx();
    test_vdivu_vx();
    test_vdiv_vx();
    test_vremu_vx();
    test_vrem_vx();
    test_vminu_vx();
    test_vmin_vx();
    test_vmaxu_vx();
    test_vmax_vx();
    test_vsmul_vx();
}
