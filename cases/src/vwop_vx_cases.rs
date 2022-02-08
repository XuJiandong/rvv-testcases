use core::arch::asm;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vwop_vx;
use rvv_testcases::misc::{U256, U512};
use rvv_testcases::runner::{run_vop_vx, WideningCategory};

fn expected_op_add(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len() * 2, result.len());

    match lhs.len() {
        32 => {
            let l: U512 = U256::from_little_endian(lhs).into();
            let r: U512 = rhs.into();

            let (r, _) = l.overflowing_add(r);
            r.to_little_endian(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}

pub fn test_vwop_vx() {
    fn add(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwaddu.vx v21, v1, t0", in (reg) x);
        });
    }
    for lmul in [1, 2, 4] {
        for avl in 99..=100 {
            run_vop_vx(
                256,
                lmul,
                avl,
                expected_op_add,
                add,
                WideningCategory::VdOnly,
                "vwaddu.vx",
            );
        }
    }
}
