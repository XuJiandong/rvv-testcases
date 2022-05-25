use core::{arch::asm, convert::TryInto};
use eint::{Eint, E256};
use rvv_asm::rvv_asm;
use rvv_testcases::runner::{run_template_m_vx, MaskType};

fn expected_op_eq(lhs: &[u8], x: u64, result: &mut bool) {
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            *result = l == x as i64;
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            *result = l.cmp_s(&r).is_eq();
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmseq() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmseq.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmseq.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_m_vx(expected_op_eq, op, true, "vmseq.vx");
}

fn expected_op_ne(lhs: &[u8], x: u64, result: &mut bool) {
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            *result = l != x as i64;
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            *result = l.cmp_s(&r).is_ne();
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsne() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmsne.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmsne.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_m_vx(expected_op_ne, op, true, "vmsne.vx");
}

fn expected_op_ltu(lhs: &[u8], x: u64, result: &mut bool) {
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            *result = l < x;
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            *result = l.cmp_u(&r).is_lt();
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsltu() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmsltu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmsltu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_m_vx(expected_op_ltu, op, true, "vmsltu.vx");
}

fn expected_op_lt(lhs: &[u8], x: u64, result: &mut bool) {
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = x as i64;
            *result = l < r;
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            *result = l.cmp_s(&r).is_lt();
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmslt() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmslt.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmslt.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_m_vx(expected_op_lt, op, true, "vmslt.vx");
}

fn expected_op_leu(lhs: &[u8], x: u64, result: &mut bool) {
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            *result = l <= x;
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            *result = l.cmp_u(&r).is_le();
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsleu() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmsleu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmsleu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_m_vx(expected_op_leu, op, true, "vmsleu.vx");
}

fn expected_op_le(lhs: &[u8], x: u64, result: &mut bool) {
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            *result = l <= x as i64;
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            *result = l.cmp_s(&r).is_le();
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsle() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmsle.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmsle.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_m_vx(expected_op_le, op, true, "vmsle.vx");
}

fn expected_op_gtu(lhs: &[u8], x: u64, result: &mut bool) {
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            *result = l > x;
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            *result = l.cmp_u(&r).is_gt();
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsgtu() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmsgtu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmsgtu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_m_vx(expected_op_gtu, op, true, "vmsgtu.vx");
}

fn expected_op_gt(lhs: &[u8], x: u64, result: &mut bool) {
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            *result = l > x as i64;
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            *result = l.cmp_s(&r).is_gt();
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsgt() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vmsgt.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vmsgt.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_m_vx(expected_op_gt, op, true, "vmsgt.vx");
}

pub fn test_vmsop_vx() {
    test_vmseq();
    test_vmsne();
    test_vmsltu();
    test_vmslt();
    test_vmsleu();
    test_vmsle();
    test_vmsgtu();
    test_vmsgt();
}
