use alloc::boxed::Box;
use core::arch::asm;
use core::convert::TryInto;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vwredop_vs;
use rvv_testcases::misc::{add_i512, avl_iterator, U512};
use rvv_testcases::runner::{run_vop_vv, ExpectedOp, WideningCategory};

// use ckb_std::syscalls::debug;
// use rvv_testcases::log;

fn expected_op_sumu(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() * 2 == rhs.len() && rhs.len() == result.len());
    let len = lhs.len();

    let lhs = {
        let mut l = lhs.to_vec();
        l.resize(len * 2, 0);
        l
    };

    let rhs = if index == 0 {
        rhs.to_vec()
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
            rvv_asm!("vwredsumu.vs v24, v8, v16");
        });
    }

    for sew in [32, 64, 256] {
        for lmul in [-2, 1, 4] {
            for avl in avl_iterator(sew, 4) {
                run_vop_vv(
                    sew,
                    lmul,
                    avl,
                    ExpectedOp::Reduction(Box::new(expected_op_sumu)),
                    add,
                    WideningCategory::VdVs1,
                    "vwredsumu.vs",
                );
            }
        }
    }
}

fn expected_op_sum(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() * 2 == rhs.len() && rhs.len() == result.len());
    let len = lhs.len();

    let lhs = {
        let mut l = lhs.to_vec();
        l.resize(len * 2, 0);
        l
    };

    let rhs = if index == 0 {
        rhs.to_vec()
    } else {
        result.to_vec()
    };

    match len {
        4 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap());
            let res2 = l.wrapping_add(r);

            result.copy_from_slice(&res2.to_le_bytes());
        }
        8 => {
            let l = i128::from_le_bytes(lhs.try_into().unwrap());
            let r = i128::from_le_bytes(rhs.try_into().unwrap());
            let res2 = l.wrapping_add(r);

            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            result.copy_from_slice(
                add_i512(&lhs.try_into().unwrap(), &rhs.try_into().unwrap()).as_slice(),
            );
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vwredsum_vs() {
    fn add(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwredop_vs(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwredsum.vs v24, v8, v16");
        });
    }

    for sew in [32, 64, 256] {
        for lmul in [-2, 1, 4] {
            for avl in avl_iterator(sew, 4) {
                run_vop_vv(
                    sew,
                    lmul,
                    avl,
                    ExpectedOp::Reduction(Box::new(expected_op_sum)),
                    add,
                    WideningCategory::VdVs1,
                    "vwredsum.vs",
                );
            }
        }
    }
}
