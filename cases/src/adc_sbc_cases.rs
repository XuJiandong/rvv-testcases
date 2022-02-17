use alloc::boxed::Box;
use core::arch::asm;
use rvv_asm::rvv_asm;
use rvv_testcases::misc::{avl_iterator, U256};
use rvv_testcases::{
    intrinsic::vop_vvm,
    runner::{run_vop_vvm, ExpectedOp},
};

fn expected_op(lhs: &[u8], rhs: &[u8], result: &mut [u8], mask: u8) {
    assert_eq!(lhs.len(), rhs.len());
    assert_eq!(rhs.len(), result.len());
    match lhs.len() {
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = U256::from_little_endian(rhs);
            let mask = U256::from(mask as u64);
            let (res, _) = l.overflowing_add(r);
            let (res, _) = res.overflowing_add(mask);
            res.to_little_endian(result);
        }
        _ => {
            panic!("Unsupported sew = {}", lhs.len());
        }
    }
}

fn v_op(lhs: &[u8], rhs: &[u8], result: &mut [u8], masks: &[u8], sew: u64, lmul: i64, avl: u64) {
    vop_vvm(lhs, rhs, result, masks, sew, avl, lmul, || unsafe {
        rvv_asm!("vadc.vvm v21, v1, v11, v0");
    })
}

pub fn test_adc_sbc() {
    for avl in avl_iterator(256, 2) {
        run_vop_vvm(
            256,
            1,
            avl,
            ExpectedOp::WithMask(Box::new(expected_op)),
            v_op,
            "vadc.vvm",
        );
    }
}
