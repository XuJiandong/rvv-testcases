use core::arch::asm;

use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vop_vx;
use rvv_testcases::misc::{avl_iterator, U256};
use rvv_testcases::runner::{run_vop_vx, WideningCategory};

fn expected_op(lhs: &[u8], _: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    match result.len() {
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = l << 10; // fixed
            r.to_little_endian(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}

pub fn test_single_width_shift() {
    fn sll(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |_| unsafe {
            // fixed: 10
            rvv_asm!("vsll.vi v24, v8, 10");
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vx(
                256,
                lmul,
                avl,
                expected_op,
                sll,
                WideningCategory::None,
                "vsll.vi",
            );
        }
    }
}
