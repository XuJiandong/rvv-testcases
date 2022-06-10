use core::{arch::asm, convert::TryInto};
use eint::{Eint, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::conver_to_i512,
    runner::{run_template_w_vx, MaskType},
};

fn expected_op_addu(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len() * 2, result.len());

    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = rhs as u128;
            let res = l.wrapping_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(rhs);
            l.overflowing_add_u(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vwaddu_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vwaddu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vwaddu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vx(expected_op_addu, op, true, "vwaddu.vx");
}

fn expected_op_add(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len() * 2, result.len());

    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = (rhs as i64) as u128;
            let res = l.wrapping_add(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = E512::from(rhs as i64);
            l.overflowing_add_s(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vwadd_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vwadd.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vwadd.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vx(expected_op_add, op, true, "vwadd.vx");
}

fn expected_op_subu(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len() * 2, result.len());

    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = rhs as u128;
            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(rhs);
            l.overflowing_sub_u(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vwsubu_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vwsubu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vwsubu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vx(expected_op_subu, op, true, "vwsubu.vx");
}

fn expected_op_sub(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len() * 2, result.len());

    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = (rhs as i64) as u128;
            let res = l.wrapping_sub(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = E512::from(rhs as i64);
            l.overflowing_sub_s(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vwsub_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vwsub.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vwsub.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vx(expected_op_sub, op, true, "vwsub.vx");
}

fn expected_op_mulu(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len() * 2, result.len());

    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = rhs as u128;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E512::from(E256::get(lhs));
            let r = E512::from(rhs);
            l.overflowing_mul_u(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vwmulu_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vwmulu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vwmulu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vx(expected_op_mulu, op, true, "vwmulu.vx");
}

fn expected_op_mul(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len() * 2, result.len());

    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = (rhs as i64) as u128;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = E512::from(rhs as i64);
            l.overflowing_mul_s(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vwmul_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vwmul.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vwmul.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vx(expected_op_mul, op, true, "vwmul.vx");
}

fn expected_op_mulsu(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len() * 2, result.len());

    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as u128;
            let r = rhs as u128;
            let res = l.wrapping_mul(r);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = E512::from(rhs);
            l.overflowing_mul_u(r).0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vwmulsu_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vwmulsu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vwmulsu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_w_vx(expected_op_mulsu, op, true, "vwmulsu.vx");
}

pub fn test_vwop_vx() {
    test_vwaddu_vx();
    test_vwadd_vx();
    test_vwsubu_vx();
    test_vwsub_vx();
    test_vwmulu_vx();
    test_vwmul_vx();
    test_vwmulsu_vx();
}
