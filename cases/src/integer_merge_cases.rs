use alloc::boxed::Box;
use core::arch::asm;
use rvv_asm::rvv_asm;
use rvv_testcases::{
    intrinsic::vop_vvm,
    runner::{run_vop_vvm, ExpectedOp},
};

fn expected_op(lhs: &[u8], rhs: &[u8], result: &mut [u8], mask: u8) {
    if mask == 0 {
        result.copy_from_slice(lhs);
    } else {
        result.copy_from_slice(rhs);
    }
}

fn v_op(lhs: &[u8], rhs: &[u8], result: &mut [u8], masks: &[u8], sew: u64, lmul: i64, avl: u64) {
    vop_vvm(lhs, rhs, result, masks, sew, avl, lmul, || unsafe {
        rvv_asm!("vmerge.vvm v24, v1, v11, v0");
    })
}

pub fn test_integer_merge() {
    run_vop_vvm(
        256,
        1,
        8,
        ExpectedOp::WithMask(Box::new(expected_op)),
        v_op,
        "vmerge.vvm",
    );
}
