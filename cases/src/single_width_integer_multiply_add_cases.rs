use core::arch::asm;

use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vop_vv;

use rvv_testcases::misc::{avl_iterator, U256};
use rvv_testcases::runner::{run_vop_vv, WideningCategory};

fn expected_op(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = U256::from_little_endian(rhs);
            let extra = U256::from_little_endian(result);
            let (res, _) = l.overflowing_mul(r);
            let (res2, _) = res.overflowing_add(extra);
            res2.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_single_width_integer_multiply_add() {
    fn macc(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmacc.vv v21, v1, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-8, -2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                expected_op,
                macc,
                WideningCategory::None,
                "vmacc.vv",
            );
        }
    }
}
