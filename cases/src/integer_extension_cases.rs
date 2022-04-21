use core::arch::asm;
use core::convert::TryInto;

use alloc::boxed::Box;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vop_nv;
use rvv_testcases::misc::{avl_iterator};
use rvv_testcases::runner::{run_vop_vv, ExpectedOp, WideningCategory};

fn expected_op(lhs: &[u8], _: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() / 2);

    match result.len() {
        16 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r: u128 = l.into();
            result.copy_from_slice(&r.to_le_bytes());
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}

pub fn test_integer_extension() {
    fn add(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_nv(
            lhs,
            rhs,
            result,
            sew,
            avl,
            lmul,
            || unsafe {
                rvv_asm!("vzext.vf2 v24, v8");
            },
            WideningCategory::NarrowVs2(2),
        );
    }
    let sew = 128u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                ExpectedOp::Normal(Box::new(expected_op)),
                add,
                WideningCategory::NarrowVs2(2),
                "vzext.vf2",
            );
        }
    }
}
