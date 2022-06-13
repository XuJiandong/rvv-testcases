use core::{arch::asm, convert::TryInto};
use eint::{Eint, E1024, E128, E2048, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::misc::{
    conver_to_i1024, conver_to_i2048, conver_to_i256, conver_to_i512, to_i16, to_i32, to_i64,
    to_i8, to_u16, to_u32, to_u64, to_u8,
};
use rvv_testcases::runner::{run_template_v_vv, run_template_v_vx, MaskType};

fn test_vaaddu_vv() {
    fn expected_op(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
        let sew = lhs.len() * 8;
        match sew {
            8 => {
                let (r, _) = (to_u8(lhs) as u16).overflowing_add(to_u8(rhs) as u16);
                let r2 = r.wrapping_shr(1) as u8;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            16 => {
                let (r, _) = (to_u16(lhs) as u32).overflowing_add(to_u16(rhs) as u32);
                let r2 = r.wrapping_shr(1) as u16;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            32 => {
                let (r, _) = (to_u32(lhs) as u64).overflowing_add(to_u32(rhs) as u64);
                let r2 = r.wrapping_shr(1) as u32;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            64 => {
                let (r, _) = (to_u64(lhs) as u128).overflowing_add(to_u64(rhs) as u128);
                let r2 = r.wrapping_shr(1) as u64;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            128 => {
                let (res, _) =
                    E256::from(E128::get(lhs)).overflowing_add_u(E256::from(E128::get(rhs)));
                let res = res.wrapping_shr(1).0;
                res.put(result);
            }

            256 => {
                let (res, _) =
                    E512::from(E256::get(lhs)).overflowing_add_u(E512::from(E256::get(rhs)));
                let res = res.wrapping_shr(1).0;
                res.put(result);
            }

            512 => {
                let (res, _) =
                    E1024::from(E512::get(lhs)).overflowing_add_u(E1024::from(E512::get(rhs)));
                let res = res.wrapping_shr(1).0;
                res.put(result);
            }

            1024 => {
                let (res, _) =
                    E2048::from(E1024::get(lhs)).overflowing_add_u(E2048::from(E1024::get(rhs)));
                let res = res.wrapping_shr(1).0;
                res.put(result);
            }

            _ => {
                panic!("expected_op_aaddu");
            }
        }
    }
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vaaddu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vaaddu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op, op, true, "vaaddu.vv");
}

fn test_vaaddu_vx() {
    fn expected_op(lhs: &[u8], x: u64, result: &mut [u8]) {
        assert!(lhs.len() == result.len());
        let sew = lhs.len() * 8;
        match sew {
            8 => {
                let (r, _) = (to_u8(lhs) as u16).overflowing_add(x as u8 as u16);
                let r2 = (r >> 1) as u8;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            16 => {
                let (r, _) = (to_u16(lhs) as u32).overflowing_add(x as u16 as u32);
                let r2 = (r >> 1) as u16;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            32 => {
                let (r, _) = (to_u32(lhs) as u64).overflowing_add(x as u32 as u64);
                let r2 = (r >> 1) as u32;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            64 => {
                let (r, _) = (to_u64(lhs) as u128).overflowing_add(x as u64 as u128);
                let r2 = (r >> 1) as u64;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            128 => {
                let (r, _) =
                    E256::from(E128::get(lhs)).overflowing_add_u(E256::from(E128::from(x)));
                r.wrapping_shr(1).0.put(result);
            }

            256 => {
                let (r, _) =
                    E512::from(E256::get(lhs)).overflowing_add_u(E512::from(E256::from(x)));
                r.wrapping_shr(1).0.put(result);
            }

            512 => {
                let (r, _) =
                    E1024::from(E512::get(lhs)).overflowing_add_u(E1024::from(E512::from(x)));
                r.wrapping_shr(1).0.put(result);
            }

            1024 => {
                let (r, _) =
                    E2048::from(E1024::get(lhs)).overflowing_add_u(E2048::from(E1024::from(x)));
                r.wrapping_shr(1).0.put(result);
            }

            _ => {
                panic!("expected_op_aaddu");
            }
        }
    }
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vaaddu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vaaddu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op, op, true, "vaaddu.vx");
}

fn test_vaadd_vv() {
    fn expected_op(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
        let sew = lhs.len() * 8;
        match sew {
            8 => {
                let (r, _) = (to_i8(lhs) as i16).overflowing_add(to_i8(rhs) as i8 as i16);
                let r2 = (r >> 1) as i8;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            16 => {
                let (r, _) = (to_i16(lhs) as i32).overflowing_add(to_i16(rhs) as i16 as i32);
                let r2 = (r >> 1) as i16;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            32 => {
                let (r, _) = (to_i32(lhs) as i64).overflowing_add(to_i32(rhs) as i32 as i64);
                let r2 = (r >> 1) as i32;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            64 => {
                let (r, _) = (to_i64(lhs) as i128).overflowing_add(to_i64(rhs) as i64 as i128);
                let r2 = (r >> 1) as i64;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            128 => {
                let (r, _) = conver_to_i256(E128::get(lhs))
                    .overflowing_add_s(conver_to_i256(E128::get(rhs)));
                r.wrapping_shr(1).0.put(result);
            }

            256 => {
                let (r, _) = conver_to_i512(E256::get(lhs))
                    .overflowing_add_s(conver_to_i512(E256::get(rhs)));
                r.wrapping_shr(1).0.put(result);
            }

            512 => {
                let (r, _) = conver_to_i1024(E512::get(lhs))
                    .overflowing_add_s(conver_to_i1024(E512::get(rhs)));
                r.wrapping_shr(1).0.put(result);
            }

            1024 => {
                let (r, _) = conver_to_i2048(E1024::get(lhs))
                    .overflowing_add_s(conver_to_i2048(E1024::get(rhs)));
                r.wrapping_shr(1).0.put(result);
            }

            _ => {
                panic!("expected_op_aadd");
            }
        }
    }
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vaadd.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vaadd.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op, op, true, "vaadd.vv");
}

fn test_vaadd_vx() {
    fn expected_op(lhs: &[u8], x: u64, result: &mut [u8]) {
        assert!(lhs.len() == result.len());
        let sew = lhs.len() * 8;
        match sew {
            8 => {
                let (r, _) = (to_i8(lhs) as i16).overflowing_add((x as i8) as i16);
                let r2 = (r >> 1) as i8;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            16 => {
                let (r, _) = (to_i16(lhs) as i32).overflowing_add((x as i16) as i32);
                let r2 = (r >> 1) as i16;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            32 => {
                let (r, _) = (to_i32(lhs) as i64).overflowing_add((x as i32) as i64);
                let r2 = (r >> 1) as i32;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            64 => {
                let (r, _) = (to_i64(lhs) as i128).overflowing_add((x as i64) as i128);
                let r2 = (r >> 1) as i64;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            128 => {
                let (r, _) = conver_to_i256(E128::get(lhs)).overflowing_add_s(E256::from(x as i64));
                r.wrapping_shr(1).0.put(result);
            }

            256 => {
                let (r, _) = conver_to_i512(E256::get(lhs)).overflowing_add_s(E512::from(x as i64));
                r.wrapping_shr(1).0.put(result);
            }

            512 => {
                let (r, _) =
                    conver_to_i1024(E512::get(lhs)).overflowing_add_s(E1024::from(x as i64));
                r.wrapping_shr(1).0.put(result);
            }

            1024 => {
                let (r, _) =
                    conver_to_i2048(E1024::get(lhs)).overflowing_add_s(E2048::from(x as i64));
                r.wrapping_shr(1).0.put(result);
            }

            _ => {
                panic!("expected_op_aadd");
            }
        }
    }
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vaadd.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vaadd.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op, op, true, "vaadd.vx");
}

fn test_vasubu_vv() {
    fn expected_op(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
        let sew = lhs.len() * 8;
        match sew {
            8 => {
                let r = (to_u8(lhs) as u16).wrapping_sub(to_u8(rhs) as u16);
                let r2 = (r >> 1) as u8;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            16 => {
                let r = (to_u16(lhs) as u32).wrapping_sub(to_u16(rhs) as u32);
                let r2 = (r >> 1) as u16;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            32 => {
                let r = (to_u32(lhs) as u64).wrapping_sub(to_u32(rhs) as u64);
                let r2 = (r >> 1) as u32;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            64 => {
                let r = (to_u64(lhs) as u128).wrapping_sub(to_u64(rhs) as u128);
                let r2 = (r >> 1) as u64;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            128 => {
                let (r, _) =
                    E256::from(E128::get(lhs)).overflowing_sub_u(E256::from(E128::get(rhs)));
                r.wrapping_shr(1).0.put(result);
            }

            256 => {
                let (r, _) =
                    E512::from(E256::get(lhs)).overflowing_sub_u(E512::from(E256::get(rhs)));
                r.wrapping_shr(1).0.put(result);
            }

            512 => {
                let (r, _) =
                    E1024::from(E512::get(lhs)).overflowing_sub_u(E1024::from(E512::get(rhs)));
                r.wrapping_shr(1).0.put(result);
            }

            1024 => {
                let (r, _) =
                    E2048::from(E1024::get(lhs)).overflowing_sub_u(E2048::from(E1024::get(rhs)));
                r.wrapping_shr(1).0.put(result);
            }

            _ => {
                panic!("expected_op_aadd");
            }
        }
    }
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vasubu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vasubu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op, op, true, "vasubu.vv");
}

fn test_vasubu_vx() {
    fn expected_op(lhs: &[u8], x: u64, result: &mut [u8]) {
        assert!(lhs.len() == result.len());
        let sew = lhs.len() * 8;
        match sew {
            8 => {
                let (r, _) = (to_u8(lhs) as u16).overflowing_sub(x as u8 as u16);
                let r2 = (r >> 1) as u8;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            16 => {
                let (r, _) = (to_u16(lhs) as u32).overflowing_sub(x as u16 as u32);
                let r2 = (r >> 1) as u16;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            32 => {
                let (r, _) = (to_u32(lhs) as u64).overflowing_sub(x as u32 as u64);
                let r2 = (r >> 1) as u32;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            64 => {
                let (r, _) = (to_u64(lhs) as u128).overflowing_sub(x as u64 as u128);
                let r2 = (r >> 1) as u64;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            128 => {
                let (r, _) = E256::from(E128::get(lhs)).overflowing_sub_u(E256::from(x));
                r.wrapping_shr(1).0.put(result);
            }

            256 => {
                let (r, _) = E512::from(E256::get(lhs)).overflowing_sub_u(E512::from(x));
                r.wrapping_shr(1).0.put(result);
            }

            512 => {
                let (r, _) = E1024::from(E512::get(lhs)).overflowing_sub_u(E1024::from(x));
                r.wrapping_shr(1).0.put(result);
            }

            1024 => {
                let (r, _) = E2048::from(E1024::get(lhs)).overflowing_sub_u(E2048::from(x));
                r.wrapping_shr(1).0.put(result);
            }

            _ => {
                panic!("expected_op_aaddu");
            }
        }
    }
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vasubu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vasubu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op, op, true, "vasubu.vx");
}

fn test_vasub_vv() {
    fn expected_op(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
        let sew = lhs.len() * 8;
        match sew {
            8 => {
                let r = (to_i8(lhs) as i16).wrapping_sub(to_i8(rhs) as i16);
                let r2 = (r >> 1) as i8;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            16 => {
                let r = (to_i16(lhs) as i32).wrapping_sub(to_i16(rhs) as i32);
                let r2 = (r >> 1) as i16;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            32 => {
                let r = (to_i32(lhs) as i64).wrapping_sub(to_i32(rhs) as i64);
                let r2 = (r >> 1) as i32;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            64 => {
                let r = (to_i64(lhs) as i128).wrapping_sub(to_i64(rhs) as i128);
                let r2 = (r >> 1) as i64;
                result.copy_from_slice(&r2.to_le_bytes());
            }

            128 => {
                let (r, _) = conver_to_i256(E128::get(lhs))
                    .overflowing_sub_s(conver_to_i256(E128::get(rhs)));
                r.wrapping_shr(1).0.put(result);
            }

            256 => {
                let (r, _) = conver_to_i512(E256::get(lhs))
                    .overflowing_sub_s(conver_to_i512(E256::get(rhs)));
                r.wrapping_shr(1).0.put(result);
            }

            512 => {
                let (r, _) = conver_to_i1024(E512::get(lhs))
                    .overflowing_sub_s(conver_to_i1024(E512::get(rhs)));
                r.wrapping_shr(1).0.put(result);
            }

            1024 => {
                let (r, _) = conver_to_i2048(E1024::get(lhs))
                    .overflowing_sub_s(conver_to_i2048(E1024::get(rhs)));
                r.wrapping_shr(1).0.put(result);
            }

            _ => {
                panic!("expected_op_aadd");
            }
        }
    }
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vasub.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vasub.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op, op, true, "vasub.vv");
}

fn test_vasub_vx() {
    fn expected_op(lhs: &[u8], x: u64, result: &mut [u8]) {
        assert!(lhs.len() == result.len());
        let sew = lhs.len() * 8;
        match sew {
            
            8 => {
                let (r, _) = (to_i8(lhs) as i16).overflowing_sub((x as i8) as i16);
                let r2 = (r >> 1) as i8;
                result.copy_from_slice(&r2.to_le_bytes());
            }
            
            16 => {
                let (r, _) = (to_i16(lhs) as i32).overflowing_sub((x as i16) as i32);
                let r2 = (r >> 1) as i16;
                result.copy_from_slice(&r2.to_le_bytes());
            }
            
            32 => {
                let (r, _) = (to_i32(lhs) as i64).overflowing_sub((x as i32) as i64);
                let r2 = (r >> 1) as i32;
                result.copy_from_slice(&r2.to_le_bytes());
            }
            
            64 => {
                let (r, _) = (to_i64(lhs) as i128).overflowing_sub((x as i64) as i128);
                let r2 = (r >> 1) as i64;
                result.copy_from_slice(&r2.to_le_bytes());
            }
            
            
            128 => {
                let (r, _) = conver_to_i256(E128::get(lhs)).overflowing_sub_s(E256::from(x as i64));
                r.wrapping_shr(1).0.put(result);
            }
            
            256 => {
                let (r, _) = conver_to_i512(E256::get(lhs)).overflowing_sub_s(E512::from(x as i64));
                r.wrapping_shr(1).0.put(result);
            }
            
            512 => {
                let (r, _) = conver_to_i1024(E512::get(lhs)).overflowing_sub_s(E1024::from(x as i64));
                r.wrapping_shr(1).0.put(result);
            }
            
            1024 => {
                let (r, _) = conver_to_i2048(E1024::get(lhs)).overflowing_sub_s(E2048::from(x as i64));
                r.wrapping_shr(1).0.put(result);
            }
            
            _ => {
                panic!("expected_op_aadd");
            }
        }
    }
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vasub.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vasub.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op, op, true, "vasub.vx");
}

pub fn test_single_width_averaging_add_and_subtract() {
    test_vaaddu_vv();
    test_vaaddu_vx();
    test_vaadd_vv();
    test_vaadd_vx();
    test_vasubu_vv();
    test_vasubu_vx();
    test_vasub_vv();
    test_vasub_vx();
}
