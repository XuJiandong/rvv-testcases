use core::arch::asm;

use alloc::boxed::Box;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vop_vv;

use rvv_testcases::misc::avl_iterator;
use rvv_testcases::runner::{run_vop_vv, ExpectedOp, WideningCategory};

fn expected_op(_: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(rhs.len() == result.len());
    match result.len() {
        32 => {
            result.copy_from_slice(rhs);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_integer_move() {
    fn mv(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmv.v.v v24, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                ExpectedOp::Normal(Box::new(expected_op)),
                mv,
                WideningCategory::None,
                "vmv.v.v",
            );
        }
    }
}
