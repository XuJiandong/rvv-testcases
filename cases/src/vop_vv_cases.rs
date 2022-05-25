use core::{
    arch::asm,
    cmp::Ordering::{Greater, Less},
    convert::TryInto,
};
use eint::{Eint, E256};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::{U1024, U256, U512},
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
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            if r == 0 {
                result.copy_from_slice(&u64::MAX.to_le_bytes());
            } else {
                let res = l.wrapping_div(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
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
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap());
            if r == 0 {
                result.copy_from_slice(&u64::MAX.to_le_bytes());
            } else {
                let res = l.wrapping_div(r);
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
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
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
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
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap());
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
        8 => {
            result.copy_from_slice(
                if u64::from_le_bytes(lhs.try_into().unwrap())
                    < u64::from_le_bytes(rhs.try_into().unwrap())
                {
                    lhs
                } else {
                    rhs
                },
            );
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);

            result.copy_from_slice(if l.cmp_u(&r) == Less { lhs } else { rhs });
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
        8 => {
            result.copy_from_slice(
                if i64::from_le_bytes(lhs.try_into().unwrap())
                    < i64::from_le_bytes(rhs.try_into().unwrap())
                {
                    lhs
                } else {
                    rhs
                },
            );
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);

            result.copy_from_slice(if l.cmp_s(&r) == Less { lhs } else { rhs });
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
        8 => {
            result.copy_from_slice(
                if u64::from_le_bytes(lhs.try_into().unwrap())
                    > u64::from_le_bytes(rhs.try_into().unwrap())
                {
                    lhs
                } else {
                    rhs
                },
            );
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);

            result.copy_from_slice(if l.cmp_u(&r) == Greater { lhs } else { rhs });
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
        8 => {
            result.copy_from_slice(
                if i64::from_le_bytes(lhs.try_into().unwrap())
                    > i64::from_le_bytes(rhs.try_into().unwrap())
                {
                    lhs
                } else {
                    rhs
                },
            );
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);

            result.copy_from_slice(if l.cmp_s(&r) == Greater { lhs } else { rhs });
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
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = i64::from_le_bytes(rhs.try_into().unwrap()) as i128;
            let res = (l.wrapping_mul(r) >> 63) as i64;

            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let (res_l, res_h) = l.widening_mul_s(r);
            let res = res_l.wrapping_shr(255) | res_h.wrapping_shl(1);
            res.put(result);
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
