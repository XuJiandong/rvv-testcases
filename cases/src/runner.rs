use alloc::vec::Vec;

use ckb_std::syscalls::debug;
use rand::{Rng, RngCore};

use super::log;
use super::misc::{ceiling, get_bit};
use super::rng::BestNumberRng;

pub enum WideningCategory {
    None,
    VdOnly,
    Vs2Only,
    VdVs2,
    // EEW, 1/2, 1/4, or 1/8 of SEW
    NarrowVs2(usize),
}

pub fn run_vop_vv<T1, T2>(
    sew: u64,
    lmul: i64,
    avl: u64,
    mut expected_op: T1,
    mut v_op: T2,
    cat: WideningCategory,
    desc: &str,
) where
    T1: FnMut(&[u8], &[u8], &mut [u8]),
    T2: FnMut(&[u8], &[u8], &mut [u8], u64, i64, u64),
{
    log!(
        "run with sew = {}, lmul = {}, avl = {}, desc = {}",
        sew,
        lmul,
        avl,
        desc
    );

    let avl_bytes = (sew / 8 * avl) as usize;
    let sew_bytes = (sew / 8) as usize;

    let mut rhs = Vec::<u8>::new();
    rhs.resize(avl_bytes, 0u8);

    let mut lhs = Vec::<u8>::new();
    let mut expected = Vec::<u8>::new();
    let mut result = Vec::<u8>::new();

    match cat {
        WideningCategory::VdVs2 => {
            expected.resize(avl_bytes * 2, 0);
            result.resize(avl_bytes * 2, 0);
            lhs.resize(avl_bytes * 2, 0);
        }
        WideningCategory::VdOnly => {
            expected.resize(avl_bytes * 2, 0);
            result.resize(avl_bytes * 2, 0);
            lhs.resize(avl_bytes, 0);
        }
        WideningCategory::NarrowVs2(n) => {
            expected.resize(avl_bytes, 0);
            result.resize(avl_bytes, 0);
            lhs.resize(avl_bytes / n, 0);
        }
        WideningCategory::Vs2Only => {
            expected.resize(avl_bytes, 0);
            result.resize(avl_bytes, 0);
            lhs.resize(avl_bytes * 2, 0);
        }
        _ => {
            expected.resize(avl_bytes, 0);
            result.resize(avl_bytes, 0);
            lhs.resize(avl_bytes, 0);
        }
    }

    let mut rng = BestNumberRng::default();
    for i in 0..avl as usize {
        let range = i * sew_bytes..(i + 1) * sew_bytes;
        let expected_range = match cat {
            WideningCategory::VdVs2 => i * sew_bytes * 2..(i + 1) * sew_bytes * 2,
            WideningCategory::VdOnly => i * sew_bytes * 2..(i + 1) * sew_bytes * 2,
            _ => range.clone(),
        };
        let lhs_range = match cat {
            WideningCategory::VdVs2 => i * sew_bytes * 2..(i + 1) * sew_bytes * 2,
            WideningCategory::Vs2Only => i * sew_bytes * 2..(i + 1) * sew_bytes * 2,
            WideningCategory::NarrowVs2(n) => i * sew_bytes / n..(i + 1) * sew_bytes / n,
            _ => range.clone(),
        };

        rng.fill(&mut lhs[lhs_range.clone()]);
        rng.fill(&mut rhs[range.clone()]);

        expected_op(
            &lhs[lhs_range.clone()],
            &rhs[range.clone()],
            &mut expected[expected_range.clone()],
        );
    }
    v_op(
        lhs.as_slice(),
        rhs.as_slice(),
        result.as_mut_slice(),
        sew,
        lmul,
        avl,
    );

    for i in 0..avl as usize {
        let range = i * sew_bytes..(i + 1) * sew_bytes;
        let expected_range = match cat {
            WideningCategory::VdVs2 | WideningCategory::VdOnly => {
                i * sew_bytes * 2..(i + 1) * sew_bytes * 2
            }
            _ => range.clone(),
        };
        let lhs_range = match cat {
            WideningCategory::VdVs2 => i * sew_bytes * 2..(i + 1) * sew_bytes * 2,
            WideningCategory::Vs2Only => i * sew_bytes * 2..(i + 1) * sew_bytes * 2,
            WideningCategory::NarrowVs2(n) => i * sew_bytes / n..(i + 1) * sew_bytes / n,
            _ => range.clone(),
        };

        let left = &lhs[lhs_range.clone()];
        let right = &rhs[range.clone()];

        let res = &result[expected_range.clone()];
        let exp = &expected[expected_range.clone()];
        if res != exp {
            log!(
                "[sew = {}, describe = {}] unexpected values found at index {} (nth-element): {:?} (result) {:?} (expected)",
                sew, desc, i, res, exp
            );
            log!(
                "more information, lhs = {:?}, rhs = {:?}, lmul = {}, avl = {}",
                left,
                right,
                lmul,
                avl
            );
            panic!("Abort");
        }
    }
    log!("finished");
}

pub fn run_vmsop_vv<T1, T2>(
    sew: u64,
    lmul: i64,
    avl: u64,
    mut expected_op: T1,
    mut v_op: T2,
    _: WideningCategory,
    desc: &str,
) where
    T1: FnMut(&[u8], &[u8], &mut [u8], usize),
    T2: FnMut(&[u8], &[u8], &mut [u8], u64, i64, u64),
{
    log!(
        "run with sew = {}, lmul = {}, avl = {}, desc = {}",
        sew,
        lmul,
        avl,
        desc
    );

    let avl_bytes = (sew / 8 * avl) as usize;
    let sew_bytes = (sew / 8) as usize;

    let mut rhs = Vec::<u8>::new();
    rhs.resize(avl_bytes, 0u8);
    let mut lhs = Vec::<u8>::new();
    lhs.resize(avl_bytes, 0);

    let mut expected = Vec::<u8>::new();
    let mut result = Vec::<u8>::new();

    let result_avl_bytes = ceiling(avl as usize, 8);
    result.resize(result_avl_bytes, 0);
    expected.resize(result_avl_bytes, 0);

    let mut rng = BestNumberRng::default();
    for i in 0..avl as usize {
        let range = i * sew_bytes..(i + 1) * sew_bytes;

        rng.fill(&mut lhs[range.clone()]);
        rng.fill(&mut rhs[range.clone()]);

        expected_op(&lhs[range.clone()], &rhs[range.clone()], &mut expected, i);
    }
    v_op(
        lhs.as_slice(),
        rhs.as_slice(),
        result.as_mut_slice(),
        sew,
        lmul,
        avl,
    );

    for i in 0..avl as usize {
        let range = i * sew_bytes..(i + 1) * sew_bytes;
        let left = &lhs[range.clone()];
        let right = &rhs[range.clone()];

        let byte_index = i / 8;
        let bit_index = i - byte_index * 8;

        let res = get_bit(result[byte_index], bit_index);
        let exp = get_bit(expected[byte_index], bit_index);
        if res != exp {
            log!(
                "[sew = {}, describe = {}] unexpected values found at index {} (nth-element): {:?} (result) {:?} (expected)",
                sew, desc, i, res, exp
            );
            log!(
                "more information, lhs = {:?}, rhs = {:?}, lmul = {}, avl = {}",
                left,
                right,
                lmul,
                avl
            );
            panic!("Abort");
        }
    }
    log!("finished");
}

pub fn run_vop_vx<T1, T2>(
    sew: u64,
    lmul: i64,
    avl: u64,
    mut expected_op: T1,
    mut v_op: T2,
    cat: WideningCategory,
    desc: &str,
) where
    T1: FnMut(&[u8], u64, &mut [u8]),
    T2: FnMut(&[u8], u64, &mut [u8], u64, i64, u64),
{
    log!(
        "run with sew = {}, lmul = {}, avl = {}, desc = {}",
        sew,
        lmul,
        avl,
        desc
    );

    let avl_bytes = (sew / 8 * avl) as usize;
    let sew_bytes = (sew / 8) as usize;
    let mut lhs = Vec::<u8>::new();
    let mut expected = Vec::<u8>::new();
    let mut result = Vec::<u8>::new();

    match cat {
        WideningCategory::VdVs2 => {
            expected.resize(avl_bytes * 2, 0);
            result.resize(avl_bytes * 2, 0);
            lhs.resize(avl_bytes * 2, 0);
        }
        WideningCategory::VdOnly => {
            expected.resize(avl_bytes * 2, 0);
            result.resize(avl_bytes * 2, 0);
            lhs.resize(avl_bytes, 0);
        }
        WideningCategory::Vs2Only => {
            expected.resize(avl_bytes, 0);
            result.resize(avl_bytes, 0);
            lhs.resize(avl_bytes * 2, 0);
        }
        _ => {
            expected.resize(avl_bytes, 0);
            result.resize(avl_bytes, 0);
            lhs.resize(avl_bytes, 0);
        }
    }

    let mut rng = BestNumberRng::default();
    let x = rng.next_u64();

    for i in 0..avl as usize {
        let range = i * sew_bytes..(i + 1) * sew_bytes;
        let lhs_range = match cat {
            WideningCategory::VdVs2 => i * sew_bytes * 2..(i + 1) * sew_bytes * 2,
            WideningCategory::Vs2Only => i * sew_bytes * 2..(i + 1) * sew_bytes * 2,
            _ => range.clone(),
        };
        let expected_range = match cat {
            WideningCategory::VdVs2 | WideningCategory::VdOnly => {
                i * sew_bytes * 2..(i + 1) * sew_bytes * 2
            }
            _ => range.clone(),
        };
        rng.fill(&mut lhs[lhs_range.clone()]);
        expected_op(
            &lhs[lhs_range.clone()],
            x,
            &mut expected[expected_range.clone()],
        );
    }

    v_op(lhs.as_slice(), x, result.as_mut_slice(), sew, lmul, avl);

    for i in 0..avl as usize {
        let range = i * sew_bytes..(i + 1) * sew_bytes;

        let expected_range = match cat {
            WideningCategory::VdVs2 | WideningCategory::VdOnly => {
                i * sew_bytes * 2..(i + 1) * sew_bytes * 2
            }
            _ => range.clone(),
        };
        let lhs_range = match cat {
            WideningCategory::VdVs2 => i * sew_bytes * 2..(i + 1) * sew_bytes * 2,
            WideningCategory::Vs2Only => i * sew_bytes * 2..(i + 1) * sew_bytes * 2,
            _ => range.clone(),
        };

        let left = &lhs[lhs_range.clone()];
        let right = x;

        let res = &result[expected_range.clone()];
        let exp = &expected[expected_range.clone()];
        if res != exp {
            log!(
                "[sew = {}, describe = {}] unexpected values found at index {} (nth-element): {:?} (result) {:?} (expected)",
                sew, desc, i, res, exp
            );
            log!(
                "more information, lhs = {:?}, rhs = {:?}, lmul = {}, avl = {}",
                left,
                right,
                lmul,
                avl
            );
            panic!("Abort");
        }
    }
    log!("finished");
}
