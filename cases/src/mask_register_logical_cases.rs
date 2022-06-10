use core::arch::asm;
use rvv_asm::rvv_asm;
use rvv_testcases::runner::{run_template_m_mm, MaskType};

fn test_vmand_mm() {
    fn expected_op(lhs: bool, rhs: bool, result: &mut bool) {
        *result = lhs & rhs;
    }
    fn op(_: &[u8], _: &[u8], _: MaskType) {
        unsafe {
            rvv_asm!("vmand.mm v24, v8, v16");
        }
    }
    run_template_m_mm(expected_op, op, false, "vmand.mm");
}

fn test_vmor_mm() {
    fn expected_op(lhs: bool, rhs: bool, result: &mut bool) {
        *result = lhs | rhs;
    }
    fn op(_: &[u8], _: &[u8], _: MaskType) {
        unsafe {
            rvv_asm!("vmor.mm v24, v8, v16");
        }
    }
    run_template_m_mm(expected_op, op, false, "vmor.mm");
}

fn test_vmnor_mm() {
    fn expected_op(lhs: bool, rhs: bool, result: &mut bool) {
        *result = !(lhs | rhs);
    }
    fn op(_: &[u8], _: &[u8], _: MaskType) {
        unsafe {
            rvv_asm!("vmnor.mm v24, v8, v16");
        }
    }
    run_template_m_mm(expected_op, op, false, "vmnor.mm");
}

fn test_vmorn_mm() {
    fn expected_op(lhs: bool, rhs: bool, result: &mut bool) {
        *result = lhs | !rhs;
    }
    fn op(_: &[u8], _: &[u8], _: MaskType) {
        unsafe {
            rvv_asm!("vmornot.mm v24, v8, v16");
        }
    }
    run_template_m_mm(expected_op, op, false, "vmornot.mm");
}

fn test_vmnand_mm() {
    fn expected_op(lhs: bool, rhs: bool, result: &mut bool) {
        *result = !(lhs & rhs);
    }
    fn op(_: &[u8], _: &[u8], _: MaskType) {
        unsafe {
            rvv_asm!("vmnand.mm v24, v8, v16");
        }
    }
    run_template_m_mm(expected_op, op, false, "vmnand.mm");
}

fn test_vmandn_mm() {
    fn expected_op(lhs: bool, rhs: bool, result: &mut bool) {
        *result = lhs & !rhs;
    }
    fn op(_: &[u8], _: &[u8], _: MaskType) {
        unsafe {
            rvv_asm!("vmandnot.mm v24, v8, v16");
        }
    }
    run_template_m_mm(expected_op, op, false, "vmandnot.mm");
}

fn test_vmxor_mm() {
    fn expected_op(lhs: bool, rhs: bool, result: &mut bool) {
        *result = lhs ^ rhs;
    }
    fn op(_: &[u8], _: &[u8], _: MaskType) {
        unsafe {
            rvv_asm!("vmxor.mm v24, v8, v16");
        }
    }
    run_template_m_mm(expected_op, op, false, "vmxor.mm");
}

fn test_vmxnor_mm() {
    fn expected_op(lhs: bool, rhs: bool, result: &mut bool) {
        *result = !(lhs ^ rhs);
    }
    fn op(_: &[u8], _: &[u8], _: MaskType) {
        unsafe {
            rvv_asm!("vmxnor.mm v24, v8, v16");
        }
    }
    run_template_m_mm(expected_op, op, false, "vmxnor.mm");
}

pub fn test_mask_register_logical() {
    test_vmand_mm();
    test_vmnand_mm();
    test_vmandn_mm();

    test_vmor_mm();
    test_vmnor_mm();
    test_vmorn_mm();

    test_vmxor_mm();
    test_vmxnor_mm();
}
