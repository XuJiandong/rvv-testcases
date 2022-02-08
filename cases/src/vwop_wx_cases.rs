use core::arch::asm;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vwop_wx;
use rvv_testcases::misc::U512;
use rvv_testcases::runner::{run_vop_vx, WideningCategory};

fn expected_op_add(lhs: &[u8], rhs: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    match lhs.len() {
        64 => {
            let l: U512 = U512::from_little_endian(lhs);
            let r: U512 = rhs.into();

            let (r, _) = l.overflowing_add(r);
            r.to_little_endian(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}

pub fn test_vwop_wx() {
    fn add(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_wx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", "vwaddu.wx v21, v1, t0", in (reg) x);
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
                WideningCategory::VdVs2,
                "vwaddu.wx",
            );
        }
    }
}
