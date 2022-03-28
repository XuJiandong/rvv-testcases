use alloc::boxed::Box;
use core::arch::asm;
use core::convert::TryInto;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vwredop_vs;
use rvv_testcases::misc::{add_i512, avl_iterator, i256_to_i512, U512};
use rvv_testcases::runner::{run_vop_vv, ExpectedOp, WideningCategory};

fn expected_op_sumu(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    let len = lhs.len();

    let mut lhs = lhs.to_vec();
    lhs.resize(len * 2, 0);

    let rhs = if index == 0 {
        let mut r = rhs.to_vec();
        r.resize(len * 2, 0);
        r
    } else {
        result.to_vec()
    };

    match len {
        4 => {
            let l = u64::from_le_bytes(lhs.as_slice().try_into().unwrap());
            let r = u64::from_le_bytes(rhs.as_slice().try_into().unwrap());
            let res2 = { l.wrapping_add(r) };

            result.copy_from_slice(&res2.to_le_bytes());
        }
        8 => {
            let l = u128::from_le_bytes(lhs.as_slice().try_into().unwrap());
            let r = u128::from_le_bytes(rhs.as_slice().try_into().unwrap());
            let res2 = { l.wrapping_add(r) };

            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = U512::from_little_endian(lhs.as_slice());
            let r = U512::from_little_endian(rhs.as_slice());

            let res2: U512 = l.wrapping_add(r).into();
            res2.to_little_endian(result);

            // log!("Test\nindex: {}\nl: {:0>2X?}\nr: {:0>2X?}\nres: {:0>2X?}", index, lhs, rhs, result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vwredsumu_vs() {
    fn add(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        //log!("res: {:0>2X?}", result);
        vwredop_vs(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwredsumu.vs v21, v1, v11");
        });
    }
    //let sew = 256u64;
    //let sew = 64u64;
    //let sew = 32u64;

    for sew in [32, 64, 256] {
        for lmul in [-2, 1, 4, 8] {
            for avl in avl_iterator(sew, 4) {
                run_vop_vv(
                    sew,
                    lmul,
                    avl,
                    ExpectedOp::Reduction(Box::new(expected_op_sumu)),
                    add,
                    WideningCategory::VdOnly,
                    "vwredsumu.vs",
                );
            }
        }
    }
}

fn expected_op_sum(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() == rhs.len() && rhs.len() * 2 == result.len());
    let len = lhs.len();

    match len {
        4 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap()) as i64;
            let r = if index == 0 {
                i32::from_le_bytes(rhs.try_into().unwrap()) as i64
            } else {
                i64::from_le_bytes(result.try_into().unwrap())
            };
            let res2 = l.wrapping_add(r);

            result.copy_from_slice(&res2.to_le_bytes());
        }
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = if index == 0 {
                i64::from_le_bytes(rhs.try_into().unwrap()) as i128
            } else {
                i128::from_le_bytes(result.try_into().unwrap())
            };
            let res2 = l.wrapping_add(r);

            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = i256_to_i512(lhs.try_into().unwrap());
            let r = if index == 0 {
                i256_to_i512(rhs.try_into().unwrap())
            } else {
                result.try_into().unwrap()
            };

            result.copy_from_slice(add_i512(&l, &r).as_slice());
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vwredsum_vs() {
    fn add(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwredop_vs(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwredsum.vs v21, v1, v11");
        });
    }

    for sew in [256] {
        for lmul in [-2, 1, 4, 8] {
            for avl in avl_iterator(sew, 4) {
                run_vop_vv(
                    sew,
                    lmul,
                    avl,
                    ExpectedOp::Reduction(Box::new(expected_op_sum)),
                    add,
                    WideningCategory::VdOnly,
                    "vwredsum.vs",
                );
            }
        }
    }
}
