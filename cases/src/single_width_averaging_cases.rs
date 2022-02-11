use alloc::boxed::Box;
use core::arch::asm;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vop_vv;

use rvv_testcases::misc::{Widening, U256, U512};
use rvv_testcases::runner::{run_vop_vv, ExpectedOp, WideningCategory};

pub fn test_single_width_averaging_add_and_subtract() {
    // vaaddu.vv
    fn vaaddu_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vaaddu.vv v21, v1, v11");
        });
    }
    pub fn expected_vaaddu_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
        let l = U256::from_little_endian(lhs);
        let r = U256::from_little_endian(rhs);
        match lhs.len() {
            32 => {
                // widening
                let (r, _) = U512::from(l).overflowing_add(U512::from(r));
                // narrow again
                let r2: U256 = (r >> 1).into();
                r2.to_little_endian(result);
            }
            _ => {
                panic!("expected_op_aaddu");
            }
        }
    }
    run_vop_vv(
        256,
        1,
        100,
        ExpectedOp::Normal(Box::new(expected_vaaddu_vv)),
        vaaddu_vv,
        WideningCategory::None,
        "vaaddu.vv",
    );

    // vaadd.vv
    fn vaadd_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vaadd.vv v21, v1, v11");
        });
    }
    pub fn expected_vaadd_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
        let l = U256::from_little_endian(lhs);
        let r = U256::from_little_endian(rhs);
        match lhs.len() {
            32 => {
                let (r, _) = l.sign_extend().overflowing_add(r.sign_extend());
                let r2 = r >> 1;
                let r3: U256 = r2.into();
                r3.to_little_endian(result)
            }
            _ => {
                panic!("expected_op_aadd");
            }
        }
    }
    run_vop_vv(
        256,
        1,
        100,
        ExpectedOp::Normal(Box::new(expected_vaadd_vv)),
        vaadd_vv,
        WideningCategory::None,
        "vaadd.vv",
    );

    // vasubu.vv
    fn vasubu_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vasubu.vv v21, v1, v11");
        });
    }
    pub fn expected_vasubu_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
        let l = U256::from_little_endian(lhs);
        let r = U256::from_little_endian(rhs);
        match lhs.len() {
            32 => {
                let l: U512 = l.into();
                let r: U512 = r.into();
                let r = l.wrapping_sub(r);
                let r2: U256 = (r >> 1).into();
                r2.to_little_endian(result);
            }
            _ => {
                panic!("expected_op_asubu");
            }
        }
    }
    run_vop_vv(
        256,
        1,
        100,
        ExpectedOp::Normal(Box::new(expected_vasubu_vv)),
        vasubu_vv,
        WideningCategory::None,
        "vasubu.vv",
    );

    // vasub.vv
    fn vasub_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vasub.vv v21, v1, v11");
        });
    }
    pub fn expected_vasub_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
        let l = U256::from_little_endian(lhs);
        let r = U256::from_little_endian(rhs);
        match lhs.len() {
            32 => {
                let l: U512 = l.sign_extend();
                let r: U512 = r.sign_extend();
                let (r, _) = l.overflowing_sub(r);
                let r2: U256 = (r >> 1).into();
                r2.to_little_endian(result)
            }
            _ => {
                panic!("expected_op_asub");
            }
        }
    }
    run_vop_vv(
        256,
        1,
        100,
        ExpectedOp::Normal(Box::new(expected_vasub_vv)),
        vasub_vv,
        WideningCategory::None,
        "vasub.vv",
    );
}
