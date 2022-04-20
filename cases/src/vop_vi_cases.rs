use core::arch::asm;
use core::convert::TryInto;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vop_vx;
use rvv_testcases::misc::{avl_iterator, shrink_to_imm, Widening, U256};
use rvv_testcases::runner::{run_vop_vx, WideningCategory};

fn expected_op_add(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert!(lhs.len() == result.len());
    let imm = shrink_to_imm(x);
    match lhs.len() {
        1 => {
            result[0] = lhs[0] + imm as u8;
        }
        2 => {
            let r = u16::from_le_bytes(lhs.try_into().unwrap()) + imm as u16;
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = u32::from_le_bytes(lhs.try_into().unwrap()) + imm as u32;
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = u64::from_le_bytes(lhs.try_into().unwrap()) + imm as u64;
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = u128::from_le_bytes(lhs.try_into().unwrap()) + imm as u128;
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let x = imm as u64;
            let (r, _) = U256::from_little_endian(lhs).overflowing_add(x.sign_extend());
            r.to_little_endian(result);
        }
        // 64 => {
        //     let (r, _) = U512::from_little_endian(lhs).overflowing_add(U512::from(imm as i64));
        //     r.to_little_endian(result);
        // }
        // 128 => {
        //     let (r, _) = U1024::from_little_endian(lhs).overflowing_add(U1024::from(imm as i64));
        //     r.to_little_endian(result);
        // }
        _ => {
            panic!("Invalid sew");
        }
    }
}

fn test_vadd_vi() {
    // test combinations of lmul, sew, avl, etc
    fn add(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            let imm = shrink_to_imm(x);
            match imm {
                15 => {
                    rvv_asm!("vadd.vi v24, v8, 15");
                }
                14 => {
                    rvv_asm!("vadd.vi v24, v8, 14");
                }
                13 => {
                    rvv_asm!("vadd.vi v24, v8, 13");
                }
                12 => {
                    rvv_asm!("vadd.vi v24, v8, 12");
                }
                11 => {
                    rvv_asm!("vadd.vi v24, v8, 11");
                }
                10 => {
                    rvv_asm!("vadd.vi v24, v8, 10");
                }
                9 => {
                    rvv_asm!("vadd.vi v24, v8, 9");
                }
                8 => {
                    rvv_asm!("vadd.vi v24, v8, 8");
                }
                7 => {
                    rvv_asm!("vadd.vi v24, v8, 7");
                }
                6 => {
                    rvv_asm!("vadd.vi v24, v8, 6");
                }
                5 => {
                    rvv_asm!("vadd.vi v24, v8, 5");
                }
                4 => {
                    rvv_asm!("vadd.vi v24, v8, 4");
                }
                3 => {
                    rvv_asm!("vadd.vi v24, v8, 3");
                }
                2 => {
                    rvv_asm!("vadd.vi v24, v8, 2");
                }
                1 => {
                    rvv_asm!("vadd.vi v24, v8, 1");
                }
                0 => {
                    rvv_asm!("vadd.vi v24, v8, 0");
                }
                -1 => {
                    rvv_asm!("vadd.vi v24, v8, -1");
                }
                -2 => {
                    rvv_asm!("vadd.vi v24, v8, -2");
                }
                -3 => {
                    rvv_asm!("vadd.vi v24, v8, -3");
                }
                -4 => {
                    rvv_asm!("vadd.vi v24, v8, -4");
                }
                -5 => {
                    rvv_asm!("vadd.vi v24, v8, -5");
                }
                -6 => {
                    rvv_asm!("vadd.vi v24, v8, -6");
                }
                -7 => {
                    rvv_asm!("vadd.vi v24, v8, -7");
                }
                -8 => {
                    rvv_asm!("vadd.vi v24, v8, -8");
                }
                -9 => {
                    rvv_asm!("vadd.vi v24, v8, -9");
                }
                -10 => {
                    rvv_asm!("vadd.vi v24, v8, -10");
                }
                -11 => {
                    rvv_asm!("vadd.vi v24, v8, -11");
                }
                -12 => {
                    rvv_asm!("vadd.vi v24, v8, -12");
                }
                -13 => {
                    rvv_asm!("vadd.vi v24, v8, -13");
                }
                -14 => {
                    rvv_asm!("vadd.vi v24, v8, -14");
                }
                -15 => {
                    rvv_asm!("vadd.vi v24, v8, -15");
                }
                -16 => {
                    rvv_asm!("vadd.vi v24, v8, -16");
                }
                _ => {
                    panic!("can't support this immediate: {}", imm);
                }
            }
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vx(
                sew,
                lmul,
                avl,
                expected_op_add,
                add,
                WideningCategory::None,
                "vadd.vi",
            );
        }
    }
}

fn expected_op_sub(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert!(lhs.len() == result.len());
    let imm = shrink_to_imm(x);
    match lhs.len() {
        1 => {
            result[0] = lhs[0] + imm as u8;
        }
        2 => {
            let r = imm as u16 - u16::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = imm as u32 - u32::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = imm as u64 - u64::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = imm as u128 - u128::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let x = imm as u64;
            let (r, _) = x
                .sign_extend()
                .overflowing_sub(U256::from_little_endian(lhs));
            r.to_little_endian(result);
        }
        // 64 => {
        //     let (r, _) = U512::from_little_endian(lhs).overflowing_add(U512::from(imm as i64));
        //     r.to_little_endian(result);
        // }
        // 128 => {
        //     let (r, _) = U1024::from_little_endian(lhs).overflowing_add(U1024::from(imm as i64));
        //     r.to_little_endian(result);
        // }
        _ => {
            panic!("Invalid sew");
        }
    }
}

fn test_vrsub_vi() {
    fn sub(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            let imm = shrink_to_imm(x);
            match imm {
                15 => {
                    rvv_asm!("vrsub.vi v24, v8, 15");
                }
                14 => {
                    rvv_asm!("vrsub.vi v24, v8, 14");
                }
                13 => {
                    rvv_asm!("vrsub.vi v24, v8, 13");
                }
                12 => {
                    rvv_asm!("vrsub.vi v24, v8, 12");
                }
                11 => {
                    rvv_asm!("vrsub.vi v24, v8, 11");
                }
                10 => {
                    rvv_asm!("vrsub.vi v24, v8, 10");
                }
                9 => {
                    rvv_asm!("vrsub.vi v24, v8, 9");
                }
                8 => {
                    rvv_asm!("vrsub.vi v24, v8, 8");
                }
                7 => {
                    rvv_asm!("vrsub.vi v24, v8, 7");
                }
                6 => {
                    rvv_asm!("vrsub.vi v24, v8, 6");
                }
                5 => {
                    rvv_asm!("vrsub.vi v24, v8, 5");
                }
                4 => {
                    rvv_asm!("vrsub.vi v24, v8, 4");
                }
                3 => {
                    rvv_asm!("vrsub.vi v24, v8, 3");
                }
                2 => {
                    rvv_asm!("vrsub.vi v24, v8, 2");
                }
                1 => {
                    rvv_asm!("vrsub.vi v24, v8, 1");
                }
                0 => {
                    rvv_asm!("vrsub.vi v24, v8, 0");
                }
                -1 => {
                    rvv_asm!("vrsub.vi v24, v8, -1");
                }
                -2 => {
                    rvv_asm!("vrsub.vi v24, v8, -2");
                }
                -3 => {
                    rvv_asm!("vrsub.vi v24, v8, -3");
                }
                -4 => {
                    rvv_asm!("vrsub.vi v24, v8, -4");
                }
                -5 => {
                    rvv_asm!("vrsub.vi v24, v8, -5");
                }
                -6 => {
                    rvv_asm!("vrsub.vi v24, v8, -6");
                }
                -7 => {
                    rvv_asm!("vrsub.vi v24, v8, -7");
                }
                -8 => {
                    rvv_asm!("vrsub.vi v24, v8, -8");
                }
                -9 => {
                    rvv_asm!("vrsub.vi v24, v8, -9");
                }
                -10 => {
                    rvv_asm!("vrsub.vi v24, v8, -10");
                }
                -11 => {
                    rvv_asm!("vrsub.vi v24, v8, -11");
                }
                -12 => {
                    rvv_asm!("vrsub.vi v24, v8, -12");
                }
                -13 => {
                    rvv_asm!("vrsub.vi v24, v8, -13");
                }
                -14 => {
                    rvv_asm!("vrsub.vi v24, v8, -14");
                }
                -15 => {
                    rvv_asm!("vrsub.vi v24, v8, -15");
                }
                -16 => {
                    rvv_asm!("vrsub.vi v24, v8, -16");
                }
                _ => {
                    panic!("can't support this immediate: {}", imm);
                }
            }
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vx(
                sew,
                lmul,
                avl,
                expected_op_sub,
                sub,
                WideningCategory::None,
                "vrsub.vi",
            );
        }
    }
}

pub fn test_vop_vi() {
    test_vadd_vi();
    test_vrsub_vi();
}
