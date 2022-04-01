use alloc::boxed::Box;
use alloc::vec::Vec;
use core::iter::zip;
use core::{arch::asm, ops::Not};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    intrinsic::vop_vv,
    runner::{run_vmop_mm, ExpectedOp},
};

fn expected_op_and(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let mut res = Vec::new();
    for (l, r) in zip(lhs, rhs) {
        res.push(l & r);
    }
    result.copy_from_slice(res.as_slice());
}

fn test_vmand_mm(sew: u64) {
    fn and(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmand.mm v24, v8, v16");
        });
    }
    // vm<op>.mm instructions can only operate on whole V register.
    run_vmop_mm(
        sew,
        1,
        256,
        ExpectedOp::Normal(Box::new(expected_op_and)),
        and,
        "vmand.mm",
    );
}

fn expected_op_or(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let mut res = Vec::new();
    for (l, r) in zip(lhs, rhs) {
        res.push(l | r);
    }
    result.copy_from_slice(res.as_slice());
}

fn test_vmor_mm(sew: u64) {
    fn or(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmor.mm v24, v8, v16");
        });
    }
    run_vmop_mm(
        sew,
        1,
        256,
        ExpectedOp::Normal(Box::new(expected_op_or)),
        or,
        "vmor.mm",
    );
}

fn expected_op_nor(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let mut res = Vec::new();
    for (l, r) in zip(lhs, rhs) {
        res.push((l | r).not());
    }
    result.copy_from_slice(res.as_slice());
}

fn test_vmnor_mm(sew: u64) {
    fn or(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmnor.mm v24, v8, v16");
        });
    }
    run_vmop_mm(
        sew,
        1,
        256,
        ExpectedOp::Normal(Box::new(expected_op_nor)),
        or,
        "vmnor.mm",
    );
}

fn expected_op_orn(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let mut res = Vec::new();
    for (l, r) in zip(lhs, rhs) {
        res.push(l | r.not());
    }
    result.copy_from_slice(res.as_slice());
}

fn test_vmorn_mm(sew: u64) {
    fn or(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmornot.mm v24, v8, v16");
        });
    }
    run_vmop_mm(
        sew,
        1,
        256,
        ExpectedOp::Normal(Box::new(expected_op_orn)),
        or,
        "vmornot.mm",
    );
}

fn expected_op_nand(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let mut res = Vec::new();
    for (l, r) in zip(lhs, rhs) {
        res.push((l & r).not());
    }
    result.copy_from_slice(res.as_slice());
}

fn test_vmnand_mm(sew: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmnand.mm v24, v8, v16");
        });
    }
    run_vmop_mm(
        sew,
        1,
        256,
        ExpectedOp::Normal(Box::new(expected_op_nand)),
        op,
        "vmnand.mm",
    );
}

fn expected_op_andn(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let mut res = Vec::new();
    for (l, r) in zip(lhs, rhs) {
        res.push(l & r.not());
    }
    result.copy_from_slice(res.as_slice());
}

fn test_vmandn_mm(sew: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmandnot.mm v24, v8, v16");
        });
    }
    run_vmop_mm(
        sew,
        1,
        256,
        ExpectedOp::Normal(Box::new(expected_op_andn)),
        op,
        "vmandnot.mm",
    );
}

fn expected_op_xor(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let mut res = Vec::new();
    for (l, r) in zip(lhs, rhs) {
        res.push(l ^ r);
    }
    result.copy_from_slice(res.as_slice());
}

fn test_vmxor_mm(sew: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmxor.mm v24, v8, v16");
        });
    }
    run_vmop_mm(
        sew,
        1,
        256,
        ExpectedOp::Normal(Box::new(expected_op_xor)),
        op,
        "vmxor.mm",
    );
}

fn expected_op_xnor(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let mut res = Vec::new();
    for (l, r) in zip(lhs, rhs) {
        res.push((l ^ r).not());
    }
    result.copy_from_slice(res.as_slice());
}

fn test_vmxnor_mm(sew: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmxnor.mm v24, v8, v16");
        });
    }
    run_vmop_mm(
        sew,
        1,
        256,
        ExpectedOp::Normal(Box::new(expected_op_xnor)),
        op,
        "vmxnor.mm",
    );
}

pub fn test_mask_register_logical() {
    let sew = 8u64;

    test_vmand_mm(sew);
    test_vmnand_mm(sew);
    test_vmandn_mm(sew);

    test_vmor_mm(sew);
    test_vmnor_mm(sew);
    test_vmorn_mm(sew);

    test_vmxor_mm(sew);
    test_vmxnor_mm(sew);
}
