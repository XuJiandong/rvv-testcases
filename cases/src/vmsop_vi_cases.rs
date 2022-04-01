use core::arch::asm;

use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vmsop_vx;

use rvv_testcases::misc::{avl_iterator, set_bit_in_slice, shrink_to_imm, Widening, U256};
use rvv_testcases::runner::{run_vmsop_vx, WideningCategory};

fn expected_op(lhs: &[u8], x: u64, result: &mut [u8], index: usize) {
    let imm = shrink_to_imm(x);
    let x = imm as u64;
    match lhs.len() {
        32 => {
            let l = U256::from_little_endian(lhs);
            let r = x.sign_extend();
            let res = if l == r { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vmsop_vi() {
    fn eq(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vx(lhs, x, result, sew, avl, lmul, |x| unsafe {
            let imm = shrink_to_imm(x);
            match imm {
                -16 => {
                    rvv_asm!("vmseq.vi v24, v8, -16");
                }
                -15 => {
                    rvv_asm!("vmseq.vi v24, v8, -15");
                }
                -14 => {
                    rvv_asm!("vmseq.vi v24, v8, -14");
                }
                -13 => {
                    rvv_asm!("vmseq.vi v24, v8, -13");
                }
                -12 => {
                    rvv_asm!("vmseq.vi v24, v8, -12");
                }
                -11 => {
                    rvv_asm!("vmseq.vi v24, v8, -11");
                }
                -10 => {
                    rvv_asm!("vmseq.vi v24, v8, -10");
                }
                -9 => {
                    rvv_asm!("vmseq.vi v24, v8, -9");
                }
                -8 => {
                    rvv_asm!("vmseq.vi v24, v8, -8");
                }
                -7 => {
                    rvv_asm!("vmseq.vi v24, v8, -7");
                }
                -6 => {
                    rvv_asm!("vmseq.vi v24, v8, -6");
                }
                -5 => {
                    rvv_asm!("vmseq.vi v24, v8, -5");
                }
                -4 => {
                    rvv_asm!("vmseq.vi v24, v8, -4");
                }
                -3 => {
                    rvv_asm!("vmseq.vi v24, v8, -3");
                }
                -2 => {
                    rvv_asm!("vmseq.vi v24, v8, -2");
                }
                -1 => {
                    rvv_asm!("vmseq.vi v24, v8, -1");
                }
                0 => {
                    rvv_asm!("vmseq.vi v24, v8, 0");
                }
                1 => {
                    rvv_asm!("vmseq.vi v24, v8, 1");
                }
                2 => {
                    rvv_asm!("vmseq.vi v24, v8, 2");
                }
                3 => {
                    rvv_asm!("vmseq.vi v24, v8, 3");
                }
                4 => {
                    rvv_asm!("vmseq.vi v24, v8, 4");
                }
                5 => {
                    rvv_asm!("vmseq.vi v24, v8, 5");
                }
                6 => {
                    rvv_asm!("vmseq.vi v24, v8, 6");
                }
                7 => {
                    rvv_asm!("vmseq.vi v24, v8, 7");
                }
                8 => {
                    rvv_asm!("vmseq.vi v24, v8, 8");
                }
                9 => {
                    rvv_asm!("vmseq.vi v24, v8, 9");
                }
                10 => {
                    rvv_asm!("vmseq.vi v24, v8, 10");
                }
                11 => {
                    rvv_asm!("vmseq.vi v24, v8, 11");
                }
                12 => {
                    rvv_asm!("vmseq.vi v24, v8, 12");
                }
                13 => {
                    rvv_asm!("vmseq.vi v24, v8, 13");
                }
                14 => {
                    rvv_asm!("vmseq.vi v24, v8, 14");
                }
                15 => {
                    rvv_asm!("vmseq.vi v24, v8, 15");
                }
                _ => {
                    panic!("Invalid immediate: {}", imm);
                }
            }
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
                "vmseq.vi",
            );
        }
    }
}
