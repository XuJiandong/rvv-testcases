use alloc::vec::Vec;

use ckb_std::syscalls::debug;
use rand::{Rng, RngCore};

use super::log;
use super::rng::BestNumberRng;

// like vadd.vv
pub fn run_vop_vv<T1, T2>(
    sew: u64,
    lmul: i64,
    avl: u64,
    mut expected_op: T1,
    mut v_op: T2,
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
    let mut lhs = Vec::<u8>::new();
    lhs.resize(avl_bytes, 0u8);
    let mut rhs = Vec::<u8>::new();
    rhs.resize(avl_bytes, 0u8);
    let mut expected = Vec::<u8>::new();
    expected.resize(avl_bytes, 0u8);
    let mut result = Vec::<u8>::new();
    result.resize(avl_bytes, 0u8);

    let mut rng = BestNumberRng::default();
    for i in 0..avl as usize {
        let range = i * sew_bytes..(i + 1) * sew_bytes;
        rng.fill(&mut lhs[range.clone()]);
        rng.fill(&mut rhs[range.clone()]);

        expected_op(
            &lhs[range.clone()],
            &rhs[range.clone()],
            &mut expected[range.clone()],
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

        let left = &lhs[range.clone()];
        let right = &rhs[range.clone()];

        let res = &result[range.clone()];
        let exp = &expected[range.clone()];
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
    lhs.resize(avl_bytes, 0u8);
    let mut expected = Vec::<u8>::new();
    expected.resize(avl_bytes, 0u8);
    let mut result = Vec::<u8>::new();
    result.resize(avl_bytes, 0u8);

    let mut rng = BestNumberRng::default();
    let x = rng.next_u64();

    for i in 0..avl as usize {
        let range = i * sew_bytes..(i + 1) * sew_bytes;
        rng.fill(&mut lhs[range.clone()]);

        expected_op(&lhs[range.clone()], x, &mut expected[range.clone()]);
    }
    v_op(lhs.as_slice(), x, result.as_mut_slice(), sew, lmul, avl);

    for i in 0..avl as usize {
        let range = i * sew_bytes..(i + 1) * sew_bytes;

        let left = &lhs[range.clone()];
        let right = x;

        let res = &result[range.clone()];
        let exp = &expected[range.clone()];
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
