use core::{arch::asm, convert::TryInto};
use eint::{Eint, E1024, E2048, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::runner::{run_template_w_wx, MaskType};

fn expected_op_addu(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    let sew_byte = lhs.len() / 2 * 8;
    match sew_byte {
        8 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as u8) as u16;

            let (res, _) = l.overflowing_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as u16) as u32;

            let (res, _) = l.overflowing_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as u32) as u64;

            let (res, _) = l.overflowing_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = u128::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as u64) as u128;

            let (res, _) = l.overflowing_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E256::get(lhs);
            let r = E256::from(rhs);

            let (r, _) = l.overflowing_add_u(r);
            r.put(result);
        }
        256 => {
            let l = E512::get(lhs);
            let r = E512::from(rhs);

            let (r, _) = l.overflowing_add_u(r);
            r.put(result);
        }
        512 => {
            let l = E1024::get(lhs);
            let r = E1024::from(rhs);

            let (r, _) = l.overflowing_add_u(r);
            r.put(result);
        }
        1024 => {
            let l = E2048::get(lhs);
            let r = E2048::from(rhs);

            let (r, _) = l.overflowing_add_u(r);
            r.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_addu_wx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vwaddu.wx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vwaddu.wx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_wx(expected_op_addu, op, true, "vwaddu.wx");
}

fn expected_op_add(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    let sew_byte = lhs.len() / 2 * 8;
    match sew_byte {
        8 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as i8) as i16;

            let (res, _) = l.overflowing_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as i16) as i32;

            let (res, _) = l.overflowing_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as i32) as i64;

            let (res, _) = l.overflowing_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = i128::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as i64) as i128;

            let (res, _) = l.overflowing_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E256::get(lhs);
            let r = E256::from(rhs as i64);

            let (r, _) = l.overflowing_add_s(r);
            r.put(result);
        }
        256 => {
            let l = E512::get(lhs);
            let r = E512::from(rhs as i64);

            let (r, _) = l.overflowing_add_s(r);
            r.put(result);
        }
        512 => {
            let l = E1024::get(lhs);
            let r = E1024::from(rhs as i64);

            let (r, _) = l.overflowing_add_s(r);
            r.put(result);
        }
        1024 => {
            let l = E2048::get(lhs);
            let r = E2048::from(rhs as i64);

            let (r, _) = l.overflowing_add_s(r);
            r.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_add_wx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vwadd.wx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vwadd.wx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_wx(expected_op_add, op, true, "vwadd.wx");
}

fn expected_op_subu(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    let sew_byte = lhs.len() / 2 * 8;
    match sew_byte {
        8 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as u8) as u16;

            let (res, _) = l.overflowing_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as u16) as u32;

            let (res, _) = l.overflowing_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as u32) as u64;

            let (res, _) = l.overflowing_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = u128::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as u64) as u128;

            let (res, _) = l.overflowing_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E256::get(lhs);
            let r = E256::from(rhs);

            let (r, _) = l.overflowing_sub_u(r);
            r.put(result);
        }
        256 => {
            let l = E512::get(lhs);
            let r = E512::from(rhs);

            let (r, _) = l.overflowing_sub_u(r);
            r.put(result);
        }
        512 => {
            let l = E1024::get(lhs);
            let r = E1024::from(rhs);

            let (r, _) = l.overflowing_sub_u(r);
            r.put(result);
        }
        1024 => {
            let l = E2048::get(lhs);
            let r = E2048::from(rhs);

            let (r, _) = l.overflowing_sub_u(r);
            r.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_subu_wx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vwsubu.wx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vwsubu.wx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_wx(expected_op_subu, op, true, "vwsubu.wx");
}

fn expected_op_sub(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    let sew_byte = lhs.len() / 2 * 8;
    match sew_byte {
        8 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as i8) as i16;

            let (res, _) = l.overflowing_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as i16) as i32;

            let (res, _) = l.overflowing_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as i32) as i64;

            let (res, _) = l.overflowing_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = i128::from_le_bytes(lhs.try_into().unwrap());
            let r = (rhs as i64) as i128;

            let (res, _) = l.overflowing_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E256::get(lhs);
            let r = E256::from(rhs as i64);

            let (r, _) = l.overflowing_sub_s(r);
            r.put(result);
        }
        256 => {
            let l = E512::get(lhs);
            let r = E512::from(rhs as i64);

            let (r, _) = l.overflowing_sub_s(r);
            r.put(result);
        }
        512 => {
            let l = E1024::get(lhs);
            let r = E1024::from(rhs as i64);

            let (r, _) = l.overflowing_sub_s(r);
            r.put(result);
        }
        1024 => {
            let l = E2048::get(lhs);
            let r = E2048::from(rhs as i64);

            let (r, _) = l.overflowing_sub_s(r);
            r.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_sub_wx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vwsub.wx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vwsub.wx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_wx(expected_op_sub, op, true, "vwsub.wx");
}

pub fn test_vwop_wx() {
    test_vw_addu_wx();
    test_vw_add_wx();
    test_vw_subu_wx();
    test_vw_sub_wx();
}
