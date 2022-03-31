use core::arch::asm;

use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vmsop_vx;

use rvv_testcases::misc::{avl_iterator, set_bit_in_slice, U256};
use rvv_testcases::runner::{run_vmsop_vx, WideningCategory};

fn expected_op(lhs: &[u8], x: u64, result: &mut [u8], index: usize) {
    match lhs.len() {
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = U256::from(x);
            let res = if l == r { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vmsop_vx() {
    fn eq(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vx(lhs, x, result, sew, avl, lmul, |x| unsafe {
            rvv_asm!("mv t0, {}", "vmseq.vx v24, v1, t0", in (reg) x);
        });
    }
    let sew = 256u64;
    for lmul in [-8, -2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vmsop_vx(
                sew,
                lmul,
                avl,
                expected_op,
                eq,
                WideningCategory::None,
                "vmseq.vx",
            );
        }
    }
}
