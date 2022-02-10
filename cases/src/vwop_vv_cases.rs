use core::arch::asm;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vwop_vv;
use rvv_testcases::misc::{avl_iterator, U256, U512};
use rvv_testcases::runner::{run_vop_vv, WideningCategory};

fn expected_op_add(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    match lhs.len() {
        32 => {
            let l: U512 = U256::from_little_endian(lhs).into();
            let r: U512 = U256::from_little_endian(rhs).into();

            let (r, _) = l.overflowing_add(r);
            r.to_little_endian(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}

fn expected_op_mulu(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    match lhs.len() {
        32 => {
            let l: U512 = U256::from_little_endian(lhs).into();
            let r: U512 = U256::from_little_endian(rhs).into();

            let (res, _) = l.overflowing_mul(r);
            let res2: U512 = res.into();
            res2.to_little_endian(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}

pub fn test_vwop_vv() {
    fn add(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwaddu.vv v21, v1, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                expected_op_add,
                add,
                WideningCategory::VdOnly,
                "vwaddu.vv",
            );
        }
    }

    fn mulu(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwmulu.vv v21, v1, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                expected_op_mulu,
                mulu,
                WideningCategory::VdOnly,
                "vwmulu.vv",
            );
        }
    }
}
