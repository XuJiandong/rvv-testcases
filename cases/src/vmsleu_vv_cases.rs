#![allow(dead_code)]
use core::arch::asm;

use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vmsop_vv;

use rvv_testcases::misc::{avl_iterator, set_bit_in_slice, U256};
use rvv_testcases::runner::{run_vmsop_vv, WideningCategory};

fn expected_op(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert_eq!(lhs.len(), rhs.len());
    match lhs.len() {
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = U256::from_little_endian(rhs);
            let res = if l <= r { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vmsleu_vv() {
    fn eq(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmsleu.vv v24, v1, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-8, -2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vmsop_vv(
                sew,
                lmul,
                avl,
                expected_op,
                eq,
                WideningCategory::None,
                "vmsleu.vv",
            );
        }
    }
}
