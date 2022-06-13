use core::{arch::asm, convert::TryInto};
use eint::{Eint, E1024, E128, E2048, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::{conver_to_i1024, conver_to_i2048, conver_to_i256, conver_to_i512},
    runner::{
        run_template_v_vv, run_template_v_vx, run_template_w_vv, run_template_w_vx, MaskType,
    },
};

// use ckb_std::syscalls::debug;
// use rvv_testcases::log;

fn expected_op_macc_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());
            let r = i8::from_le_bytes(rhs.try_into().unwrap());

            let extra = i8::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(r);
            let (res2, _) = res.overflowing_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            let r = i16::from_le_bytes(rhs.try_into().unwrap());

            let extra = i16::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(r);
            let (res2, _) = res.overflowing_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            let r = i32::from_le_bytes(rhs.try_into().unwrap());

            let extra = i32::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(r);
            let (res2, _) = res.overflowing_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap());

            let extra = i64::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(r);
            let (res2, _) = res.overflowing_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);

            let extra = E128::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);

            let extra = E256::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);

            let extra = E512::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);

            let extra = E1024::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmacc_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vmacc.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vmacc.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_macc_vv, op, true, "vmacc.vv");
}

fn expected_op_macc_vx(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert!(lhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());

            let extra = i8::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(x as i8);
            let (res2, _) = res.overflowing_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());

            let extra = i16::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(x as i16);
            let (res2, _) = res.overflowing_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());

            let extra = i32::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(x as i32);
            let (res2, _) = res.overflowing_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());

            let extra = i64::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(x as i64);
            let (res2, _) = res.overflowing_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::from(x as i64);

            let extra = E128::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);

            let extra = E256::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::from(x as i64);

            let extra = E512::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::from(x as i64);

            let extra = E1024::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmacc_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmacc.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmacc.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_macc_vx, op, true, "vmacc.vx");
}

fn expected_op_nmsac_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());
            let r = u8::from_le_bytes(rhs.try_into().unwrap());

            let extra = u8::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(r);
            let (res2, _) = extra.overflowing_sub(res);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            let r = u16::from_le_bytes(rhs.try_into().unwrap());

            let extra = u16::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(r);
            let (res2, _) = extra.overflowing_sub(res);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            let r = u32::from_le_bytes(rhs.try_into().unwrap());

            let extra = u32::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(r);
            let (res2, _) = extra.overflowing_sub(res);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());

            let extra = u64::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(r);
            let (res2, _) = extra.overflowing_sub(res);
            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);

            let extra = E128::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = extra.overflowing_sub_s(res);
            res2.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);

            let extra = E256::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = extra.overflowing_sub_s(res);
            res2.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);

            let extra = E512::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = extra.overflowing_sub_s(res);
            res2.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);

            let extra = E1024::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = extra.overflowing_sub_s(res);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vnmsac_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vnmsac.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vnmsac.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_nmsac_vv, op, true, "vnmsac.vv");
}

fn expected_op_nmsac_vx(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert!(lhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());

            let extra = i8::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(x as i8);
            let (res2, _) = extra.overflowing_sub(res);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());

            let extra = i16::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(x as i16);
            let (res2, _) = extra.overflowing_sub(res);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());

            let extra = i32::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(x as i32);
            let (res2, _) = extra.overflowing_sub(res);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());

            let extra = i64::from_le_bytes(result.try_into().unwrap());
            let (res, _) = l.overflowing_mul(x as i64);
            let (res2, _) = extra.overflowing_sub(res);
            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::from(x as i64);

            let extra = E128::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = extra.overflowing_sub_s(res);
            res2.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);

            let extra = E256::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = extra.overflowing_sub_s(res);
            res2.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::from(x as i64);

            let extra = E512::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = extra.overflowing_sub_s(res);
            res2.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::from(x as i64);

            let extra = E1024::get(&result);
            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = extra.overflowing_sub_s(res);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vnmsac_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vnmsac.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vnmsac.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_nmsac_vx, op, true, "vnmsac.vx");
}

fn expected_op_madd_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());
            let r = i8::from_le_bytes(rhs.try_into().unwrap());

            let extra = i8::from_le_bytes(result.try_into().unwrap());
            let (res, _) = extra.overflowing_mul(r);
            let (res2, _) = res.overflowing_add(l);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            let r = i16::from_le_bytes(rhs.try_into().unwrap());

            let extra = i16::from_le_bytes(result.try_into().unwrap());
            let (res, _) = extra.overflowing_mul(r);
            let (res2, _) = res.overflowing_add(l);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            let r = i32::from_le_bytes(rhs.try_into().unwrap());

            let extra = i32::from_le_bytes(result.try_into().unwrap());
            let (res, _) = extra.overflowing_mul(r);
            let (res2, _) = res.overflowing_add(l);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap());

            let extra = i64::from_le_bytes(result.try_into().unwrap());
            let (res, _) = extra.overflowing_mul(r);
            let (res2, _) = res.overflowing_add(l);
            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);

            let extra = E128::get(&result);
            let (res, _) = extra.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(l);
            res2.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);

            let extra = E256::get(&result);
            let (res, _) = extra.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(l);
            res2.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);

            let extra = E512::get(&result);
            let (res, _) = extra.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(l);
            res2.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);

            let extra = E1024::get(&result);
            let (res, _) = extra.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(l);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmadd_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vmadd.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vmadd.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_madd_vv, op, true, "vmadd.vv");
}

fn expected_op_madd_vx(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert!(lhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());

            let extra = i8::from_le_bytes(result.try_into().unwrap());
            let (res, _) = extra.overflowing_mul(x as i8);
            let (res2, _) = res.overflowing_add(l);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());

            let extra = i16::from_le_bytes(result.try_into().unwrap());
            let (res, _) = extra.overflowing_mul(x as i16);
            let (res2, _) = res.overflowing_add(l);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());

            let extra = i32::from_le_bytes(result.try_into().unwrap());
            let (res, _) = extra.overflowing_mul(x as i32);
            let (res2, _) = res.overflowing_add(l);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());

            let extra = i64::from_le_bytes(result.try_into().unwrap());
            let (res, _) = extra.overflowing_mul(x as i64);
            let (res2, _) = res.overflowing_add(l);
            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::from(x as i64);

            let extra = E128::get(&result);
            let (res, _) = extra.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(l);
            res2.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);

            let extra = E256::get(&result);
            let (res, _) = extra.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(l);
            res2.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::from(x as i64);

            let extra = E512::get(&result);
            let (res, _) = extra.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(l);
            res2.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::from(x as i64);

            let extra = E1024::get(&result);
            let (res, _) = extra.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(l);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmadd_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmadd.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmadd.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_madd_vx, op, true, "vmadd.vx");
}

fn expected_op_nmsub_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());
            let r = i8::from_le_bytes(rhs.try_into().unwrap());
            let extra = i8::from_le_bytes(result.try_into().unwrap());

            let (res, _) = r.overflowing_mul(extra);
            let (res2, _) = l.overflowing_sub(res);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            let r = i16::from_le_bytes(rhs.try_into().unwrap());
            let extra = i16::from_le_bytes(result.try_into().unwrap());

            let (res, _) = r.overflowing_mul(extra);
            let (res2, _) = l.overflowing_sub(res);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            let r = i32::from_le_bytes(rhs.try_into().unwrap());
            let extra = i32::from_le_bytes(result.try_into().unwrap());

            let (res, _) = r.overflowing_mul(extra);
            let (res2, _) = l.overflowing_sub(res);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap());
            let extra = i64::from_le_bytes(result.try_into().unwrap());

            let (res, _) = r.overflowing_mul(extra);
            let (res2, _) = l.overflowing_sub(res);
            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let extra = E128::get(&result);

            let (res, _) = r.overflowing_mul_s(extra);
            let (res2, _) = l.overflowing_sub_s(res);
            res2.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let extra = E256::get(&result);

            let (res, _) = r.overflowing_mul_s(extra);
            let (res2, _) = l.overflowing_sub_s(res);
            res2.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let extra = E512::get(&result);

            let (res, _) = r.overflowing_mul_s(extra);
            let (res2, _) = l.overflowing_sub_s(res);
            res2.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let extra = E1024::get(&result);

            let (res, _) = r.overflowing_mul_s(extra);
            let (res2, _) = l.overflowing_sub_s(res);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vnmsub_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vnmsub.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vnmsub.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_nmsub_vv, op, true, "vnmsub.vv");
}

fn expected_op_nmsub_vx(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert!(lhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());
            let extra = i8::from_le_bytes(result.try_into().unwrap());

            let (res, _) = (x as i8).overflowing_mul(extra);
            let (res2, _) = l.overflowing_sub(res);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            let extra = i16::from_le_bytes(result.try_into().unwrap());

            let (res, _) = (x as i16).overflowing_mul(extra);
            let (res2, _) = l.overflowing_sub(res);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            let extra = i32::from_le_bytes(result.try_into().unwrap());

            let (res, _) = (x as i32).overflowing_mul(extra);
            let (res2, _) = l.overflowing_sub(res);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let extra = i64::from_le_bytes(result.try_into().unwrap());

            let (res, _) = (x as i64).overflowing_mul(extra);
            let (res2, _) = l.overflowing_sub(res);
            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::from(x as i64);
            let extra = E128::get(&result);

            let (res, _) = r.overflowing_mul_s(extra);
            let (res2, _) = l.overflowing_sub_s(res);
            res2.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            let extra = E256::get(&result);

            let (res, _) = r.overflowing_mul_s(extra);
            let (res2, _) = l.overflowing_sub_s(res);
            res2.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::from(x as i64);
            let extra = E512::get(&result);

            let (res, _) = r.overflowing_mul_s(extra);
            let (res2, _) = l.overflowing_sub_s(res);
            res2.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::from(x as i64);
            let extra = E1024::get(&result);

            let (res, _) = r.overflowing_mul_s(extra);
            let (res2, _) = l.overflowing_sub_s(res);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vnmsub_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vnmsub.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vnmsub.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_nmsub_vx, op, true, "vnmsub.vx");
}

fn expected_op_wmaccu_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l: u16 = u8::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: u16 = u8::from_le_bytes(rhs.try_into().unwrap()).into();

            let extra: u16 = u16::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l: u32 = u16::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: u32 = u16::from_le_bytes(rhs.try_into().unwrap()).into();

            let extra: u32 = u32::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l: u64 = u32::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: u64 = u32::from_le_bytes(rhs.try_into().unwrap()).into();

            let extra: u64 = u64::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l: u128 = u64::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: u128 = u64::from_le_bytes(rhs.try_into().unwrap()).into();

            let extra: u128 = u128::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = E256::from(E128::get(lhs));
            let r = E256::from(E128::get(rhs));

            let extra = E256::get(result);

            let (res, _) = l.overflowing_mul_u(r);
            let (res2, _) = res.overflowing_add_u(extra);
            res2.put(result);
        }
        256 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(E256::get(rhs));

            let extra = E512::get(result);

            let (res, _) = l.overflowing_mul_u(r);
            let (res2, _) = res.overflowing_add_u(extra);
            res2.put(result);
        }
        512 => {
            let l = E1024::from(E512::get(lhs));
            let r = E1024::from(E512::get(rhs));

            let extra = E1024::get(result);

            let (res, _) = l.overflowing_mul_u(r);
            let (res2, _) = res.overflowing_add_u(extra);
            res2.put(result);
        }
        1024 => {
            let l = E2048::from(E1024::get(lhs));
            let r = E2048::from(E1024::get(rhs));

            let extra = E2048::get(result);

            let (res, _) = l.overflowing_mul_u(r);
            let (res2, _) = res.overflowing_add_u(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vwmaccu_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vwmaccu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vwmaccu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vv(expected_op_wmaccu_vv, op, true, "vwmaccu.vv");
}

fn expected_op_wmaccu_vx(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert!(lhs.len() * 2 == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap()) as u16;
            let r = rhs as u8 as u16;
            let extra: u16 = u16::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap()) as u32;
            let r = rhs as u16 as u32;
            let extra: u32 = u32::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap()) as u64;
            let r = rhs as u32 as u64;
            let extra: u64 = u64::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = rhs as u64 as u128;
            let extra: u128 = u128::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = E256::from(E128::get(lhs));
            let r = E256::from(rhs);
            let extra = E256::get(result);

            let (res, _) = l.overflowing_mul_u(r);
            let (res2, _) = res.overflowing_add_u(extra);
            res2.put(result);
        }
        256 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(rhs);
            let extra = E512::get(result);

            let (res, _) = l.overflowing_mul_u(r);
            let (res2, _) = res.overflowing_add_u(extra);
            res2.put(result);
        }
        512 => {
            let l = E1024::from(E512::get(lhs));
            let r = E1024::from(rhs);
            let extra = E1024::get(result);

            let (res, _) = l.overflowing_mul_u(r);
            let (res2, _) = res.overflowing_add_u(extra);
            res2.put(result);
        }
        1024 => {
            let l = E2048::from(E1024::get(lhs));
            let r = E2048::from(rhs);
            let extra = E2048::get(result);

            let (res, _) = l.overflowing_mul_u(r);
            let (res2, _) = res.overflowing_add_u(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vwmaccu_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vwmaccu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vwmaccu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vx(expected_op_wmaccu_vx, op, true, "vwmaccu.vx");
}

fn expected_op_wmacc_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l: i16 = i8::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: i16 = i8::from_le_bytes(rhs.try_into().unwrap()).into();

            let extra = i16::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l: i32 = i16::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: i32 = i16::from_le_bytes(rhs.try_into().unwrap()).into();

            let extra = i32::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l: i64 = i32::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: i64 = i32::from_le_bytes(rhs.try_into().unwrap()).into();

            let extra = i64::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l: i128 = i64::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: i128 = i64::from_le_bytes(rhs.try_into().unwrap()).into();

            let extra = i128::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = conver_to_i256(E128::get(lhs));
            let r = conver_to_i256(E128::get(rhs));

            let extra = E256::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        256 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = conver_to_i512(E256::get(rhs));

            let extra = E512::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        512 => {
            let l = conver_to_i1024(E512::get(lhs));
            let r = conver_to_i1024(E512::get(rhs));

            let extra = E1024::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        1024 => {
            let l = conver_to_i2048(E1024::get(lhs));
            let r = conver_to_i2048(E1024::get(rhs));

            let extra = E2048::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vwmacc_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vwmacc.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vwmacc.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vv(expected_op_wmacc_vv, op, true, "vwmacc.vv");
}

fn expected_op_wmacc_vx(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert!(lhs.len() * 2 == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l: i16 = i8::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: i16 = (rhs as i8) as i16;

            let extra = i16::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l: i32 = i16::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: i32 = (rhs as i16) as i32;

            let extra = i32::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l: i64 = i32::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: i64 = (rhs as i32) as i64;

            let extra = i64::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l: i128 = i64::from_le_bytes(lhs.try_into().unwrap()).into();
            let r: i128 = (rhs as i64) as i128;

            let extra = i128::from_le_bytes(result.try_into().unwrap());

            let res2 = l.wrapping_mul(r).wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = conver_to_i256(E128::get(lhs));
            let r = E256::from(rhs as i64);

            let extra = E256::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        256 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = E512::from(rhs as i64);

            let extra = E512::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        512 => {
            let l = conver_to_i1024(E512::get(lhs));
            let r = E1024::from(rhs as i64);

            let extra = E1024::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        1024 => {
            let l = conver_to_i2048(E1024::get(lhs));
            let r = E2048::from(rhs as i64);

            let extra = E2048::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vwmacc_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vwmacc.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vwmacc.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vx(expected_op_wmacc_vx, op, true, "vwmacc.vx");
}

fn expected_op_wmaccsu_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap()) as i16;
            let r = i8::from_le_bytes(rhs.try_into().unwrap()) as i16;
            let extra = i16::from_le_bytes(result.try_into().unwrap());

            let res1 = l.wrapping_mul(r);
            let res2 = res1.wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap()) as i32;
            let r = i16::from_le_bytes(rhs.try_into().unwrap()) as i32;
            let extra = i32::from_le_bytes(result.try_into().unwrap());

            let res1 = l.wrapping_mul(r);
            let res2 = res1.wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap()) as i64;
            let r = i32::from_le_bytes(rhs.try_into().unwrap()) as i64;
            let extra = i64::from_le_bytes(result.try_into().unwrap());

            let res1 = l.wrapping_mul(r);
            let res2 = res1.wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = i64::from_le_bytes(rhs.try_into().unwrap()) as i128;
            let extra = i128::from_le_bytes(result.try_into().unwrap());

            let res1 = l.wrapping_mul(r);
            let res2 = res1.wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = E256::from(E128::get(lhs));
            let r = conver_to_i256(E128::get(rhs));
            let extra = E256::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        256 => {
            let l = E512::from(E256::get(lhs));
            let r = conver_to_i512(E256::get(rhs));
            let extra = E512::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        512 => {
            let l = E1024::from(E512::get(lhs));
            let r = conver_to_i1024(E512::get(rhs));
            let extra = E1024::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        1024 => {
            let l = E2048::from(E1024::get(lhs));
            let r = conver_to_i2048(E1024::get(rhs));
            let extra = E2048::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vwmaccsu_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vwmaccsu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vwmaccsu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vv(expected_op_wmaccsu_vv, op, true, "vwmaccsu.vv");
}

fn expected_op_wmaccsu_vx(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert!(lhs.len() * 2 == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap()) as i16;
            let r = (rhs as i8) as i16;
            let extra = i16::from_le_bytes(result.try_into().unwrap());

            let res1 = l.wrapping_mul(r);
            let res2 = res1.wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap()) as i32;
            let r = (rhs as i16) as i32;
            let extra = i32::from_le_bytes(result.try_into().unwrap());

            let res1 = l.wrapping_mul(r);
            let res2 = res1.wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap()) as i64;
            let r = (rhs as i32) as i64;
            let extra = i64::from_le_bytes(result.try_into().unwrap());

            let res1 = l.wrapping_mul(r);
            let res2 = res1.wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = (rhs as i64) as i128;
            let extra = i128::from_le_bytes(result.try_into().unwrap());

            let res1 = l.wrapping_mul(r);
            let res2 = res1.wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = E256::from(E128::get(lhs));
            let r = E256::from(rhs as i64);
            let extra = E256::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        256 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(rhs as i64);
            let extra = E512::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        512 => {
            let l = E1024::from(E512::get(lhs));
            let r = E1024::from(rhs as i64);
            let extra = E1024::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        1024 => {
            let l = E2048::from(E1024::get(lhs));
            let r = E2048::from(rhs as i64);
            let extra = E2048::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vwmaccsu_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vwmaccsu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vwmaccsu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vx(expected_op_wmaccsu_vx, op, true, "vwmaccsu.vx");
}

fn expected_op_wmaccus_vx(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert!(lhs.len() * 2 == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap()) as i16;
            let r = rhs as u8 as i16;
            let extra = i16::from_le_bytes(result.try_into().unwrap());
            
            let res1 = l.wrapping_mul(r);
            let res2 = res1.wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap()) as i32;
            let r = rhs as u16 as i32;
            let extra = i32::from_le_bytes(result.try_into().unwrap());

            let res1 = l.wrapping_mul(r);
            let res2 = res1.wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap()) as i64;
            let r = rhs as u32 as i64;
            let extra = i64::from_le_bytes(result.try_into().unwrap());

            let res1 = l.wrapping_mul(r);
            let res2 = res1.wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = rhs as i128;
            let extra = i128::from_le_bytes(result.try_into().unwrap());

            let res1 = l.wrapping_mul(r);
            let res2 = res1.wrapping_add(extra);
            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = conver_to_i256(E128::get(lhs));
            let r = E256::from(rhs);
            let extra = E256::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        256 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = E512::from(rhs);
            let extra = E512::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        512 => {
            let l = conver_to_i1024(E512::get(lhs));
            let r = E1024::from(rhs);
            let extra = E1024::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        1024 => {
            let l = conver_to_i2048(E1024::get(lhs));
            let r = E2048::from(rhs);
            let extra = E2048::get(result);

            let (res, _) = l.overflowing_mul_s(r);
            let (res2, _) = res.overflowing_add_s(extra);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vwmaccus_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vwmaccus.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vwmaccus.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vx(expected_op_wmaccus_vx, op, true, "vwmaccus.vx");
}

pub fn test_widening_width_multiply_add() {
    test_vmacc_vv();
    test_vmacc_vx();

    test_vnmsac_vv();
    test_vnmsac_vx();

    test_vmadd_vv();
    test_vmadd_vx();

    test_vnmsub_vv();
    test_vnmsub_vx();

    test_vwmaccu_vv();
    test_vwmaccu_vx();

    test_vwmacc_vv();
    test_vwmacc_vx();

    test_vwmaccsu_vv();
    test_vwmaccsu_vx();
    test_vwmaccus_vx();
}
