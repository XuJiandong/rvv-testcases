use core::arch::asm;

use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vop_wx;
use rvv_testcases::misc::{avl_iterator, U256, U512};
use rvv_testcases::runner::{run_vop_vx, WideningCategory};

fn expected_op(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() * 2);

    match result.len() {
        32 => {
            let l = U512::from_little_endian(lhs);
            let r = l >> x;
            let r2: U256 = r.into();
            r2.to_little_endian(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}

pub fn test_narrowing_integer_right_shift() {
    fn srl(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_wx(lhs, x, result, sew, avl, lmul, |_| unsafe {
            rvv_asm!("mv t0, {}", "vnsrl.wx v21, v1, t0", in (reg) x);
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
                srl,
                WideningCategory::Vs2Only,
                "vnsrl.wx",
            );
        }
    }
}
