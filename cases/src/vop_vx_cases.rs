use alloc::vec::Vec;
use core::arch::asm;
use core::cmp::Ordering::{Greater, Less};
use core::convert::TryInto;
use eint::{Eint, E1024, E128, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::runner::{run_template_v_vx, MaskType};

// use ckb_std::syscalls::debug;
// use rvv_testcases::log;

fn expected_op_add(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let r = u8::from_le_bytes(lhs.try_into().unwrap()).wrapping_add(x as u8);
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = u16::from_le_bytes(lhs.try_into().unwrap()).wrapping_add(x as u16);
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let r = u32::from_le_bytes(lhs.try_into().unwrap()).wrapping_add(x as u32);
            result.copy_from_slice(&r.to_le_bytes());
        }
        64 => {
            let r = u64::from_le_bytes(lhs.try_into().unwrap()).wrapping_add(x as u64);
            result.copy_from_slice(&r.to_le_bytes());
        }
        128 => {
            let (r, _) = E128::get(lhs).overflowing_add_u(E128::from(x as i64));
            r.put(result);
        }
        256 => {
            let (r, _) = E256::get(lhs).overflowing_add_u(E256::from(x as i64));
            r.put(result);
        }
        512 => {
            let (r, _) = E512::get(lhs).overflowing_add_u(E512::from(x as i64));
            r.put(result);
        }
        1024 => {
            let (r, _) = E1024::get(lhs).overflowing_add_u(E1024::from(x as i64));
            r.put(result);
        }
        _ => {
            panic!("Unknow sew: {}", sew);
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let (r, _) = u8::from_le_bytes(lhs.try_into().unwrap()).overflowing_sub(x as u8);
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let (r, _) = u16::from_le_bytes(lhs.try_into().unwrap()).overflowing_sub(x as u16);
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let (r, _) = u32::from_le_bytes(lhs.try_into().unwrap()).overflowing_sub(x as u32);
            result.copy_from_slice(&r.to_le_bytes());
        }
        64 => {
            let (r, _) = u64::from_le_bytes(lhs.try_into().unwrap()).overflowing_sub(x as u64);
            result.copy_from_slice(&r.to_le_bytes());
        }

        128 => {
            let (r, _) = E128::get(lhs).overflowing_sub_u(E128::from(x as i64));
            r.put(result);
        }
        256 => {
            let (r, _) = E256::get(lhs).overflowing_sub_u(E256::from(x as i64));
            r.put(result);
        }
        512 => {
            let (r, _) = E512::get(lhs).overflowing_sub_u(E512::from(x as i64));
            r.put(result);
        }
        1024 => {
            let (r, _) = E1024::get(lhs).overflowing_sub_u(E1024::from(x as i64));
            r.put(result);
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let r = (x as u8).wrapping_sub(u8::from_le_bytes(lhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = (x as u16).wrapping_sub(u16::from_le_bytes(lhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let r = (x as u32).wrapping_sub(u32::from_le_bytes(lhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        64 => {
            let r = (x as u64).wrapping_sub(u64::from_le_bytes(lhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }

        128 => {
            let (r, _) = E128::from(x as i64).overflowing_sub_u(E128::get(lhs));
            r.put(result);
        }
        256 => {
            let (r, _) = E256::from(x as i64).overflowing_sub_u(E256::get(lhs));
            r.put(result);
        }
        512 => {
            let (r, _) = E512::from(x as i64).overflowing_sub_u(E512::get(lhs));
            r.put(result);
        }
        1024 => {
            let (r, _) = E1024::from(x as i64).overflowing_sub_u(E1024::get(lhs));
            r.put(result);
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let r = x as u8 & u8::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = x as u16 & u16::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let r = x as u32 & u32::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        64 => {
            let r = x as u64 & u64::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }

        128 => {
            let r = E128::from(x as i64) & E128::get(lhs);
            r.put(result);
        }
        256 => {
            let r = E256::from(x as i64) & E256::get(lhs);
            r.put(result);
        }
        512 => {
            let r = E512::from(x as i64) & E512::get(lhs);
            r.put(result);
        }
        1024 => {
            let r = E1024::from(x as i64) & E1024::get(lhs);
            r.put(result);
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let r = x as u8 | u8::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = x as u16 | u16::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let r = x as u32 | u32::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        64 => {
            let r = x as u64 | u64::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }

        128 => {
            let r = E128::from(x as i64) | E128::get(lhs);
            r.put(result);
        }
        256 => {
            let r = E256::from(x as i64) | E256::get(lhs);
            r.put(result);
        }
        512 => {
            let r = E512::from(x as i64) | E512::get(lhs);
            r.put(result);
        }
        1024 => {
            let r = E1024::from(x as i64) | E1024::get(lhs);
            r.put(result);
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let r = x as u8 ^ u8::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = x as u16 ^ u16::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let r = x as u32 ^ u32::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        64 => {
            let r = x as u64 ^ u64::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }

        128 => {
            let r = E128::from(x as i64) ^ E128::get(lhs);
            r.put(result);
        }
        256 => {
            let r = E256::from(x as i64) ^ E256::get(lhs);
            r.put(result);
        }
        512 => {
            let r = E512::from(x as i64) ^ E512::get(lhs);
            r.put(result);
        }
        1024 => {
            let r = E1024::from(x as i64) ^ E1024::get(lhs);
            r.put(result);
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let r = i8::from_le_bytes(lhs.try_into().unwrap()).wrapping_mul(x as i8);
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = i16::from_le_bytes(lhs.try_into().unwrap()).wrapping_mul(x as i16);
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let r = i32::from_le_bytes(lhs.try_into().unwrap()).wrapping_mul(x as i32);
            result.copy_from_slice(&r.to_le_bytes());
        }
        64 => {
            let r = i64::from_le_bytes(lhs.try_into().unwrap()).wrapping_mul(x as i64);
            result.copy_from_slice(&r.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let r = l.wrapping_mul(E128::from(x as i64));
            r.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = l.wrapping_mul(E256::from(x as i64));
            r.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = l.wrapping_mul(E512::from(x as i64));
            r.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = l.wrapping_mul(E1024::from(x as i64));
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let r1 = i8::from_le_bytes(lhs.try_into().unwrap()) as i16 * ((x as i8) as i16);
            let r = (r1 >> 8) as i8;

            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r1 = i16::from_le_bytes(lhs.try_into().unwrap()) as i32 * ((x as i16) as i32);
            let r = (r1 >> 16) as i16;

            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let r1 = i32::from_le_bytes(lhs.try_into().unwrap()) as i64 * ((x as i32) as i64);
            let r = (r1 >> 32) as i32;

            result.copy_from_slice(&r.to_le_bytes());
        }
        64 => {
            let r1 = i64::from_le_bytes(lhs.try_into().unwrap()) as i128 * ((x as i64) as i128);
            let r = (r1 >> 64) as i64;

            result.copy_from_slice(&r.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let (_, res) = l.widening_mul_s(E128::from(x as i64));
            res.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let (_, res) = l.widening_mul_s(E256::from(x as i64));
            res.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let (_, res) = l.widening_mul_s(E512::from(x as i64));
            res.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let (_, res) = l.widening_mul_s(E1024::from(x as i64));
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let (r, _) =
                (u8::from_le_bytes(lhs.try_into().unwrap()) as u16).overflowing_mul(x as u8 as u16);
            let r = (r >> 8) as u8;
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let (r, _) = (u16::from_le_bytes(lhs.try_into().unwrap()) as u32)
                .overflowing_mul(x as u16 as u32);
            let r = (r >> 16) as u16;
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let (r, _) = (u32::from_le_bytes(lhs.try_into().unwrap()) as u64)
                .overflowing_mul(x as u32 as u64);
            let r = (r >> 32) as u32;
            result.copy_from_slice(&r.to_le_bytes());
        }
        64 => {
            let (r, _) = (u64::from_le_bytes(lhs.try_into().unwrap()) as u128)
                .overflowing_mul(x as u64 as u128);
            let r = (r >> 64) as u64;
            result.copy_from_slice(&r.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let (_, res) = l.widening_mul_u(E128::from(x as u64));
            res.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let (_, res) = l.widening_mul_u(E256::from(x as u64));
            res.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let (_, res) = l.widening_mul_u(E512::from(x as u64));
            res.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let (_, res) = l.widening_mul_u(E1024::from(x as u64));
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let r =
                (i8::from_le_bytes(lhs.try_into().unwrap()) as u16).wrapping_mul(x as u8 as u16);
            let r = (r >> 8) as u8;
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r =
                (i16::from_le_bytes(lhs.try_into().unwrap()) as u32).wrapping_mul(x as u16 as u32);
            let r = (r >> 16) as u16;
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let r =
                (i32::from_le_bytes(lhs.try_into().unwrap()) as u64).wrapping_mul(x as u32 as u64);
            let r = (r >> 32) as u32;
            result.copy_from_slice(&r.to_le_bytes());
        }
        64 => {
            let r = (i64::from_le_bytes(lhs.try_into().unwrap()) as u128)
                .wrapping_mul(x as u64 as u128);
            let r = (r >> 64) as u64;
            result.copy_from_slice(&r.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let (_, res) = l.widening_mul_su(E128::from(x as u64));
            res.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let (_, res) = l.widening_mul_su(E256::from(x as u64));
            res.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let (_, res) = l.widening_mul_su(E512::from(x as u64));
            res.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let (_, res) = l.widening_mul_su(E1024::from(x as u64));
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            if x as u8 == 0 {
                result.copy_from_slice(&u8::MAX.to_le_bytes());
            } else {
                let res = u8::from_le_bytes(lhs.try_into().unwrap()).wrapping_div(x as u8);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            if x as u16 == 0 {
                result.copy_from_slice(&u16::MAX.to_le_bytes());
            } else {
                let res = u16::from_le_bytes(lhs.try_into().unwrap()).wrapping_div(x as u16);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            if x as u32 == 0 {
                result.copy_from_slice(&u32::MAX.to_le_bytes());
            } else {
                let res = u32::from_le_bytes(lhs.try_into().unwrap()).wrapping_div(x as u32);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        64 => {
            if x == 0 {
                result.copy_from_slice(&u64::MAX.to_le_bytes());
            } else {
                let res = u64::from_le_bytes(lhs.try_into().unwrap()).wrapping_div(x as u64);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }

        128 => {
            if x == 0 {
                E128::MAX_U.put(result)
            } else {
                let l = E128::get(lhs);
                let res = l.wrapping_div_u(E128::from(x));
                res.put(result);
            }
        }
        256 => {
            if x == 0 {
                E256::MAX_U.put(result)
            } else {
                let l = E256::get(lhs);
                let res = l.wrapping_div_u(E256::from(x));
                res.put(result);
            }
        }
        512 => {
            if x == 0 {
                E512::MAX_U.put(result)
            } else {
                let l = E512::get(lhs);
                let res = l.wrapping_div_u(E512::from(x));
                res.put(result);
            }
        }
        1024 => {
            if x == 0 {
                E1024::MAX_U.put(result)
            } else {
                let l = E1024::get(lhs);
                let res = l.wrapping_div_u(E1024::from(x));
                res.put(result);
            }
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            if x as i8 == 0 {
                result.copy_from_slice(&u8::MAX.to_le_bytes());
            } else {
                let res = i8::from_le_bytes(lhs.try_into().unwrap()).wrapping_div(x as i8);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            if x as i16 == 0 {
                result.copy_from_slice(&u16::MAX.to_le_bytes());
            } else {
                let res = i16::from_le_bytes(lhs.try_into().unwrap()).wrapping_div(x as i16);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            if x as i32 == 0 {
                result.copy_from_slice(&u32::MAX.to_le_bytes());
            } else {
                let res = i32::from_le_bytes(lhs.try_into().unwrap()).wrapping_div(x as i32);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        64 => {
            if x == 0 {
                result.copy_from_slice(&u64::MAX.to_le_bytes());
            } else {
                let res = i64::from_le_bytes(lhs.try_into().unwrap()).wrapping_div(x as i64);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }

        128 => {
            if x == 0 {
                E128::MAX_U.put(result)
            } else {
                let l = E128::get(lhs);
                let res = l.wrapping_div_s(E128::from(x as i64));
                res.put(result);
            }
        }
        256 => {
            if x == 0 {
                E256::MAX_U.put(result)
            } else {
                let l = E256::get(lhs);
                let res = l.wrapping_div_s(E256::from(x as i64));
                res.put(result);
            }
        }
        512 => {
            if x == 0 {
                E512::MAX_U.put(result)
            } else {
                let l = E512::get(lhs);
                let res = l.wrapping_div_s(E512::from(x as i64));
                res.put(result);
            }
        }
        1024 => {
            if x == 0 {
                E1024::MAX_U.put(result)
            } else {
                let l = E1024::get(lhs);
                let res = l.wrapping_div_s(E1024::from(x as i64));
                res.put(result);
            }
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            if x as u8 == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = u8::from_le_bytes(lhs.try_into().unwrap()) % (x as u8);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            if x as u16 == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = u16::from_le_bytes(lhs.try_into().unwrap()) % (x as u16);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            if x as u32 == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = u32::from_le_bytes(lhs.try_into().unwrap()) % (x as u32);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        64 => {
            if x == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = u64::from_le_bytes(lhs.try_into().unwrap()) % (x as u64);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }

        128 => {
            let l = E128::get(lhs);
            let res = l.wrapping_rem_u(E128::from(x as u64));
            res.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let res = l.wrapping_rem_u(E256::from(x as u64));
            res.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let res = l.wrapping_rem_u(E512::from(x as u64));
            res.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let res = l.wrapping_rem_u(E1024::from(x as u64));
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let x = x as i8;
            if x == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = i8::from_le_bytes(lhs.try_into().unwrap()).wrapping_rem(x);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let x = x as i16;
            if x == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = i16::from_le_bytes(lhs.try_into().unwrap()).wrapping_rem(x);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            let x = x as i32;
            if x == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = i32::from_le_bytes(lhs.try_into().unwrap()).wrapping_rem(x);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        64 => {
            let x = x as i64;
            if x == 0 {
                result.copy_from_slice(lhs);
            } else {
                let res = i64::from_le_bytes(lhs.try_into().unwrap()).wrapping_rem(x);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }

        128 => {
            let l = E128::get(lhs);
            let res = l.wrapping_rem_s(E128::from(x as i64));
            res.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let res = l.wrapping_rem_s(E256::from(x as i64));
            res.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let res = l.wrapping_rem_s(E512::from(x as i64));
            res.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let res = l.wrapping_rem_s(E1024::from(x as i64));
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            if u8::from_le_bytes(lhs.try_into().unwrap()) < x as u8 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as u8).to_le_bytes().as_slice())
            }
        }
        16 => {
            if u16::from_le_bytes(lhs.try_into().unwrap()) < x as u16 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as u16).to_le_bytes().as_slice())
            }
        }
        32 => {
            if u32::from_le_bytes(lhs.try_into().unwrap()) < x as u32 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as u32).to_le_bytes().as_slice())
            }
        }
        64 => {
            if u64::from_le_bytes(lhs.try_into().unwrap()) < x as u64 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as u64).to_le_bytes().as_slice())
            }
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::from(x);
            if l.cmp_u(&r) == Less {
                result.copy_from_slice(lhs);
            } else {
                r.put(result);
            }
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::from(x);
            if l.cmp_u(&r) == Less {
                result.copy_from_slice(lhs);
            } else {
                r.put(result);
            }
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::from(x);
            if l.cmp_u(&r) == Less {
                result.copy_from_slice(lhs);
            } else {
                r.put(result);
            }
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::from(x);
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            if i8::from_le_bytes(lhs.try_into().unwrap()) < x as i8 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as i8).to_le_bytes().as_slice())
            }
        }
        16 => {
            if i16::from_le_bytes(lhs.try_into().unwrap()) < x as i16 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as i16).to_le_bytes().as_slice())
            }
        }
        32 => {
            if i32::from_le_bytes(lhs.try_into().unwrap()) < x as i32 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as i32).to_le_bytes().as_slice())
            }
        }
        64 => {
            if i64::from_le_bytes(lhs.try_into().unwrap()) < x as i64 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as i64).to_le_bytes().as_slice())
            }
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::from(x as i64);
            if l.cmp_s(&r) == Less {
                result.copy_from_slice(lhs);
            } else {
                E128::from(x as i64).put(result);
            }
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::from(x);
            if l.cmp_s(&r) == Less {
                result.copy_from_slice(lhs);
            } else {
                E256::from(x as i64).put(result);
            }
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::from(x);
            if l.cmp_s(&r) == Less {
                result.copy_from_slice(lhs);
            } else {
                E512::from(x as i64).put(result);
            }
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::from(x);
            if l.cmp_s(&r) == Less {
                result.copy_from_slice(lhs);
            } else {
                E1024::from(x as i64).put(result);
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
    let sew = lhs.len() * 8;
    let mut zero_buf = Vec::<u8>::new();
    zero_buf.resize(128, 0);

    match sew {
        8 => {
            if u8::from_le_bytes(lhs.try_into().unwrap()) > x as u8 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as u8).to_le_bytes().as_slice())
            }
        }
        16 => {
            if u16::from_le_bytes(lhs.try_into().unwrap()) > x as u16 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as u16).to_le_bytes().as_slice())
            }
        }
        32 => {
            if u32::from_le_bytes(lhs.try_into().unwrap()) > x as u32 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as u32).to_le_bytes().as_slice())
            }
        }
        64 => {
            if u64::from_le_bytes(lhs.try_into().unwrap()) > x as u64 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as u64).to_le_bytes().as_slice())
            }
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::from(x);
            if l.cmp_u(&r).is_ge() {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice(&zero_buf[..result.len()]);
                result[..8].copy_from_slice(x.to_le_bytes().as_slice());
            }
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::from(x);
            if l.cmp_u(&r) == Greater {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice(&zero_buf[..result.len()]);
                result[..8].copy_from_slice(x.to_le_bytes().as_slice());
            }
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::from(x);
            if l.cmp_u(&r) == Greater {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice(&zero_buf[..result.len()]);
                result[..8].copy_from_slice(x.to_le_bytes().as_slice());
            }
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::from(x);
            if l.cmp_u(&r) == Greater {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice(&zero_buf[..result.len()]);
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            if i8::from_le_bytes(lhs.try_into().unwrap()) > x as i8 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as i8).to_le_bytes().as_slice())
            }
        }
        16 => {
            if i16::from_le_bytes(lhs.try_into().unwrap()) > x as i16 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as i16).to_le_bytes().as_slice())
            }
        }
        32 => {
            if i32::from_le_bytes(lhs.try_into().unwrap()) > x as i32 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as i32).to_le_bytes().as_slice())
            }
        }
        64 => {
            if i64::from_le_bytes(lhs.try_into().unwrap()) > x as i64 {
                result.copy_from_slice(lhs);
            } else {
                result.copy_from_slice((x as i64).to_le_bytes().as_slice())
            }
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::from(x as i64);
            if l.cmp_s(&r) == Greater {
                result.copy_from_slice(lhs);
            } else {
                E128::from(x as i64).put(result);
            }
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            if l.cmp_s(&r) == Greater {
                result.copy_from_slice(lhs);
            } else {
                E256::from(x as i64).put(result);
            }
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::from(x as i64);
            if l.cmp_s(&r) == Greater {
                result.copy_from_slice(lhs);
            } else {
                E512::from(x as i64).put(result);
            }
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::from(x as i64);
            if l.cmp_s(&r) == Greater {
                result.copy_from_slice(lhs);
            } else {
                E1024::from(x as i64).put(result);
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
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());
            let res = if l == (x as i8) && l == i8::MIN {
                i8::MAX
            } else {
                ((l as i16).wrapping_mul((x as i8) as i16).wrapping_shr(7)) as i8
            };
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            let res = if l == (x as i16) && l == i16::MIN {
                i16::MAX
            } else {
                ((l as i32).wrapping_mul((x as i16) as i32).wrapping_shr(15)) as i16
            };
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            let res = if l == (x as i32) && l == i32::MIN {
                i32::MAX
            } else {
                ((l as i64).wrapping_mul((x as i32) as i64).wrapping_shr(31)) as i32
            };
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let res = if l == (x as i64) && l == i64::MIN {
                i64::MAX
            } else {
                ((l as i128)
                    .wrapping_mul((x as i64) as i128)
                    .wrapping_shr(63)) as i64
            };
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let (res_l, res_h) = E128::get(lhs).widening_mul_s(E128::from(x as i64));
            let res = res_l.wrapping_shr(127) | res_h.wrapping_shl(1);
            res.put(result);
        }
        256 => {
            let (res_l, res_h) = E256::get(lhs).widening_mul_s(E256::from(x as i64));
            let res = res_l.wrapping_shr(255) | res_h.wrapping_shl(1);
            res.put(result);
        }
        512 => {
            let (res_l, res_h) = E512::get(lhs).widening_mul_s(E512::from(x as i64));
            let res = res_l.wrapping_shr(511) | res_h.wrapping_shl(1);
            res.put(result);
        }
        1024 => {
            let (res_l, res_h) = E1024::get(lhs).widening_mul_s(E1024::from(x as i64));
            let res = res_l.wrapping_shr(1023) | res_h.wrapping_shl(1);
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
