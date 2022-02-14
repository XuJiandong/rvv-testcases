use alloc::boxed::Box;
use alloc::vec::Vec;
use core::arch::asm;
use core::iter::zip;
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

fn expected_op_or(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    let mut res = Vec::new();
    for (l, r) in zip(lhs, rhs) {
        res.push(l | r);
    }
    result.copy_from_slice(res.as_slice());
}

pub fn test_mask_register_logical() {
    fn and(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmand.mm v21, v1, v11");
        });
    }
    let sew = 8u64;
    // vm<op>.mm instructions can only operate on whole V register.
    run_vmop_mm(
        sew,
        1,
        256,
        ExpectedOp::Normal(Box::new(expected_op_and)),
        and,
        "vmand.mm",
    );

    fn or(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmor.mm v21, v1, v11");
        });
    }
    let sew = 8u64;
    run_vmop_mm(
        sew,
        1,
        256,
        ExpectedOp::Normal(Box::new(expected_op_or)),
        or,
        "vmor.mm",
    );
}
