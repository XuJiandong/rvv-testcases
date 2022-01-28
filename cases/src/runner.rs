use alloc::vec::Vec;

use ckb_std::syscalls::debug;
use core::convert::TryInto;
use rand::Rng;

use super::log;
use super::misc::{U1024, U256, U512};
use super::rng::BestNumberRng;

pub fn expected_op_add(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            result[0] = lhs[0] + rhs[0];
        }
        2 => {
            let r = u16::from_le_bytes(lhs.try_into().unwrap())
                + u16::from_le_bytes(rhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = u32::from_le_bytes(lhs.try_into().unwrap())
                + u32::from_le_bytes(rhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = u64::from_le_bytes(lhs.try_into().unwrap())
                + u64::from_le_bytes(rhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = u128::from_le_bytes(lhs.try_into().unwrap())
                + u128::from_le_bytes(rhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let (r, _) =
                U256::from_little_endian(lhs).overflowing_add(U256::from_little_endian(rhs));
            r.to_little_endian(result);
        }
        64 => {
            let (r, _) =
                U512::from_little_endian(lhs).overflowing_add(U512::from_little_endian(rhs));
            r.to_little_endian(result);
        }
        128 => {
            let (r, _) =
                U1024::from_little_endian(lhs).overflowing_add(U1024::from_little_endian(rhs));
            r.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

// like vadd.vv
pub fn run<T1, T2>(sew: u64, lmul: i64, avl: u64, mut expected_op: T1, mut v_op: T2, desc: &str)
where
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
