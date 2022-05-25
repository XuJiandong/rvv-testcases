use core::{arch::asm, convert::TryInto};
use eint::{Eint, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::conver_to_i512,
    runner::{run_template_w_wv, MaskType},
};

fn expected_op_addu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), 2 * rhs.len());
    assert_eq!(lhs.len(), result.len());

    let sew_byte = lhs.len() / 2;
    match sew_byte {
        8 => {
            let l = u128::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap()) as u128;

            let (res, _) = l.overflowing_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::get(lhs);
            let r = E512::from(E256::get(rhs));

            let (res, _) = l.overflowing_add_u(r);
            res.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_addu_wv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vwaddu.wv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vwaddu.wv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_wv(expected_op_addu, op, true, "vwaddu.wv");
}

fn expected_op_add(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), 2 * rhs.len());
    assert_eq!(lhs.len(), result.len());

    let sew_byte = lhs.len() / 2;
    match sew_byte {
        8 => {
            let l = i128::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap()) as i128;

            let (res, _) = l.overflowing_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::get(lhs);
            let r = conver_to_i512(E256::get(rhs));

            let (res, _) = l.overflowing_add_s(r);
            res.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_add_wv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vwadd.wv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vwadd.wv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_wv(expected_op_add, op, true, "vwadd.wv");
}

fn expected_op_subu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), 2 * rhs.len());
    assert_eq!(lhs.len(), result.len());

    let sew_byte = lhs.len() / 2;
    match sew_byte {
        8 => {
            let l = u128::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap()) as u128;

            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::get(lhs);
            let r = E512::from(E256::get(rhs));

            let (res, _) = l.overflowing_sub_u(r);
            res.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_subu_wv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vwsubu.wv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vwsubu.wv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_wv(expected_op_subu, op, true, "vwsubu.wv");
}

fn expected_op_sub(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), 2 * rhs.len());
    assert_eq!(lhs.len(), result.len());

    let sew_byte = lhs.len() / 2;
    match sew_byte {
        8 => {
            let l = i128::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap()) as i128;

            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::get(lhs);
            let r = conver_to_i512(E256::get(rhs));

            let (res, _) = l.overflowing_sub_s(r);
            res.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vw_sub_wv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vwsub.wv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vwsub.wv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_wv(expected_op_sub, op, true, "vwsub.wv");
}

pub fn test_vwop_wv() {
    test_vw_addu_wv();
    test_vw_add_wv();

    test_vw_subu_wv();
    test_vw_sub_wv();
}
