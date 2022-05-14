use alloc::boxed::Box;
use alloc::vec::Vec;
use core::convert::TryInto;
use core::ops::Range;

use ckb_std::syscalls::debug;
use rand::{Rng, RngCore};

use crate::intrinsic::{vl1r_v0, vle_v16, vle_v24, vle_v8, vse_v24, vsetvl};
use crate::misc::{avl_iterator, VLEN};
use crate::rng::{new_random01_vec, new_random_vec};

use super::log;
use super::misc::{ceiling, get_bit_in_slice, is_verbose, set_bit_in_slice};
use super::rng::BestNumberRng;

pub enum WideningCategory {
    None,
    VdOnly,
    Vs2Only,
    VdVs1,
    VdVs2,
    // EEW, 1/2, 1/4, or 1/8 of SEW
    NarrowVs2(usize),
}

pub enum ExpectedOp {
    Normal(Box<dyn FnMut(&[u8], &[u8], &mut [u8])>),
    Reduction(Box<dyn FnMut(&[u8], &[u8], &mut [u8], usize)>),
    EnableMask(Box<dyn FnMut(&[u8], &[u8], &mut [u8], bool, usize)>),
    WithMask(Box<dyn FnMut(&[u8], &[u8], &mut [u8], u8)>),
}

pub fn run_vop_vv<T>(
    sew: u64,
    lmul: i64,
    avl: u64,
    mut expected_op: ExpectedOp,
    mut v_op: T,
    cat: WideningCategory,
    desc: &str,
) where
    T: FnMut(&[u8], &[u8], &mut [u8], u64, i64, u64),
{
    if is_verbose() {
        log!(
            "run with sew = {}, lmul = {}, avl = {}, desc = {}",
            sew,
            lmul,
            avl,
            desc
        );
    }
    let vl = vsetvl(avl as u64, sew, lmul);
    if vl == 0 {
        return;
    }

    let avl_bytes = (sew / 8 * avl) as usize;
    let sew_bytes = (sew / 8) as usize;

    let mut rhs = Vec::<u8>::new();
    rhs.resize(avl_bytes, 0u8);

    let mut lhs = Vec::<u8>::new();
    // some destructive instructions, like vmacc.vv, require `expected` filled before executing.
    // `expected_before` is the place to put random values.
    let mut expected_before = Vec::<u8>::new();
    let mut expected = Vec::<u8>::new();
    let mut result = Vec::<u8>::new();

    match cat {
        WideningCategory::VdVs1 => {
            expected.resize(avl_bytes * 2, 0);
            result.resize(avl_bytes * 2, 0);
            lhs.resize(avl_bytes, 0);
            rhs.resize(avl_bytes * 2, 0u8);
        }
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
    expected_before.resize(expected.len(), 0);

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
        let rhs_range = match cat {
            WideningCategory::VdVs1 => i * sew_bytes * 2..(i + 1) * sew_bytes * 2,
            _ => range.clone(),
        };

        rng.fill(&mut lhs[lhs_range.clone()]);
        rng.fill(&mut rhs[rhs_range.clone()]);
        rng.fill(&mut expected_before[expected_range.clone()]);
        expected[expected_range.clone()].copy_from_slice(&expected_before[expected_range.clone()]);

        if result.len() == expected_before.len() {
            result[expected_range.clone()]
                .copy_from_slice(&expected_before[expected_range.clone()]);
        }

        match expected_op {
            ExpectedOp::Normal(ref mut op) => {
                op(
                    &lhs[lhs_range.clone()],
                    &rhs[rhs_range.clone()],
                    &mut expected[expected_range.clone()],
                );
            }
            ExpectedOp::Reduction(ref mut op) => {
                let expected_range = match cat {
                    WideningCategory::VdVs1 => 0..sew_bytes * 2,
                    WideningCategory::VdVs2 => 0..sew_bytes * 2,
                    WideningCategory::VdOnly => 0..sew_bytes * 2,
                    _ => 0..sew_bytes,
                };
                // vs2: lhs, vs1: rhs
                //  # vd[0] =  sum(vs2[*], vs1[0])
                let index = i % vl as usize;
                if index == 0 {
                    rhs[rhs_range.clone()].copy_from_slice(&expected[expected_range.clone()]);
                }
                op(
                    &lhs[lhs_range.clone()],
                    &rhs[rhs_range.clone()],
                    &mut expected[expected_range.clone()],
                    index,
                );
            }
            _ => {
                panic!("unexpected op")
            }
        }
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
        let rhs_range = match cat {
            WideningCategory::VdVs1 => i * sew_bytes * 2..(i + 1) * sew_bytes * 2,
            _ => range.clone(),
        };

        let left = &lhs[lhs_range.clone()];
        let right = &rhs[rhs_range.clone()];

        let res = &result[expected_range.clone()];
        let exp = &expected[expected_range.clone()];
        let exp_before = if result.len() == expected_before.len() {
            &expected_before[expected_range.clone()]
        } else {
            &[]
        };
        if res != exp {
            log!(
                "[sew = {}, describe = {}] unexpected values found at index {}: {:0>2X?} (result) {:0>2X?} (expected)",
                sew, desc, i, res, exp
            );
            log!(
                "more information, lhs = {:0>2X?}, rhs = {:0>2X?}, expected_before = {:0>2X?}, lmul = {}, avl = {}",
                left,
                right,
                exp_before,
                lmul,
                avl
            );
            panic!("Abort");
        }
        // for reduction operations, it only checks the first element
        if let ExpectedOp::Reduction(_) = expected_op {
            break;
        }
    }
    if is_verbose() {
        log!("finished");
    }
}

pub fn run_vop_vvm<T>(
    sew: u64,
    lmul: i64,
    avl: u64,
    mut expected_op: ExpectedOp,
    mut v_op: T,
    desc: &str,
) where
    T: FnMut(&[u8], &[u8], &mut [u8], &[u8], u64, i64, u64),
{
    if is_verbose() {
        log!(
            "run with sew = {}, lmul = {}, avl = {}, desc = {}",
            sew,
            lmul,
            avl,
            desc
        );
    }

    let avl_bytes = (sew / 8 * avl) as usize;
    let sew_bytes = (sew / 8) as usize;

    let lhs = new_random_vec(avl_bytes);
    let rhs = new_random_vec(avl_bytes);
    let mut expected = new_random_vec(avl_bytes);
    let mut result = new_random_vec(avl_bytes);
    let masks = new_random01_vec(avl as usize);

    for i in 0..avl as usize {
        let range = i * sew_bytes..(i + 1) * sew_bytes;
        if let ExpectedOp::WithMask(ref mut op) = expected_op {
            op(
                &lhs[range.clone()],
                &rhs[range.clone()],
                &mut expected[range.clone()],
                masks[i],
            );
        }
    }
    v_op(
        lhs.as_slice(),
        rhs.as_slice(),
        result.as_mut_slice(),
        masks.as_slice(),
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
                "[sew = {}, describe = {}] unexpected values found at index {}: {:?} (result) {:?} (expected)",
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
    if is_verbose() {
        log!("finished");
    }
}

pub fn run_vxop_m<T>(mut expected_op: ExpectedOp, mut v_op: T, enable_mask: bool, desc: &str)
where
    T: FnMut(&[u8], &[u8], &mut [u8], bool),
{
    if is_verbose() {
        log!("run with desc = {}", desc);
    }
    let mut mask_v0 = [0u8; VLEN / 8];
    let mut vs2 = [0u8; VLEN / 8];

    let mut rng = BestNumberRng::default();
    rng.fill(&mut mask_v0[..]);
    rng.fill(&mut vs2[..]);

    let vl = vsetvl(8, 256, 1) as usize;
    assert_eq!(vl, 8);

    let expected = if let ExpectedOp::EnableMask(ref mut op) = expected_op {
        let mut temp = [0u8; 8];
        op(&mask_v0[..], &vs2[..], &mut temp[..], enable_mask, vl);
        u64::from_le_bytes(temp)
    } else {
        panic!("Unexpected op")
    };

    let mut temp = [0u8; 8];
    v_op(
        mask_v0.as_slice(),
        vs2.as_slice(),
        temp.as_mut_slice(),
        enable_mask,
    );
    let result = u64::from_le_bytes(temp);

    if result != expected {
        log!(
            "[describe = {}] unexpected values found: {:?} (result) {:?} (expected)",
            desc,
            result,
            expected
        );
        panic!("Abort");
    }
    if is_verbose() {
        log!("finished");
    }
}

pub fn run_vmop_mm<T>(
    sew: u64,
    lmul: i64,
    avl: u64,
    mut expected_op: ExpectedOp,
    mut v_op: T,
    desc: &str,
) where
    T: FnMut(&[u8], &[u8], &mut [u8], u64, i64, u64),
{
    if is_verbose() {
        log!(
            "run with sew = {}, lmul = {}, avl = {}, desc = {}",
            sew,
            lmul,
            avl,
            desc
        );
    }
    let avl_bytes = (avl / 8) as usize;
    assert!(avl_bytes <= VLEN / 8);

    let mut rhs = Vec::<u8>::new();
    rhs.resize(avl_bytes, 0u8);

    let mut lhs = Vec::<u8>::new();
    let mut expected = Vec::<u8>::new();
    let mut result = Vec::<u8>::new();

    expected.resize(avl_bytes, 0);
    result.resize(avl_bytes, 0);
    lhs.resize(avl_bytes, 0);

    let mut rng = BestNumberRng::default();
    for i in 0..avl_bytes as usize {
        rng.fill(&mut lhs[i..i + 1]);
        rng.fill(&mut rhs[i..i + 1]);

        if let ExpectedOp::Normal(ref mut op) = expected_op {
            op(&lhs[i..i + 1], &rhs[i..i + 1], &mut expected[i..i + 1]);
        } else {
            panic!("Unexpected op")
        }
    }

    // why?
    let expected = expected.clone();

    v_op(
        lhs.as_slice(),
        rhs.as_slice(),
        result.as_mut_slice(),
        sew,
        lmul,
        avl,
    );

    for i in 0..avl_bytes as usize {
        let res = result[i];
        let exp = expected[i];

        if res != exp {
            log!(
                "[sew = {}, describe = {}] unexpected values found at index {}: {:0>2X?} (result) {:0>2X?} (expected)",
                sew, desc, i, res, exp
            );
            panic!("Abort");
        }
    }
    if is_verbose() {
        log!("finished");
    }
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
    if is_verbose() {
        log!(
            "run with sew = {}, lmul = {}, avl = {}, desc = {}",
            sew,
            lmul,
            avl,
            desc
        );
    }
    let vl = vsetvl(avl as u64, sew, lmul);
    if vl == 0 {
        return;
    }

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

        let res = get_bit_in_slice(result.as_slice(), i);
        let exp = get_bit_in_slice(expected.as_slice(), i);
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
    if is_verbose() {
        log!("finished");
    }
}

pub fn run_vmsop_vx<T1, T2>(
    sew: u64,
    lmul: i64,
    avl: u64,
    mut expected_op: T1,
    mut v_op: T2,
    _: WideningCategory,
    desc: &str,
) where
    T1: FnMut(&[u8], u64, &mut [u8], usize),
    T2: FnMut(&[u8], u64, &mut [u8], u64, i64, u64),
{
    if is_verbose() {
        log!(
            "run with sew = {}, lmul = {}, avl = {}, desc = {}",
            sew,
            lmul,
            avl,
            desc
        );
    }
    let vl = vsetvl(avl as u64, sew, lmul);
    if vl == 0 {
        return;
    }

    let avl_bytes = (sew / 8 * avl) as usize;
    let sew_bytes = (sew / 8) as usize;

    let mut lhs = Vec::<u8>::new();
    lhs.resize(avl_bytes, 0);

    let mut expected = Vec::<u8>::new();
    let mut result = Vec::<u8>::new();

    let result_avl_bytes = ceiling(avl as usize, 8);
    result.resize(result_avl_bytes, 0);
    expected.resize(result_avl_bytes, 0);

    let mut rng = BestNumberRng::default();
    let x = rng.next_u64();
    for i in 0..avl as usize {
        let range = i * sew_bytes..(i + 1) * sew_bytes;

        rng.fill(&mut lhs[range.clone()]);

        expected_op(&lhs[range.clone()], x, &mut expected, i);
    }
    v_op(lhs.as_slice(), x, result.as_mut_slice(), sew, lmul, avl);

    for i in 0..avl as usize {
        let range = i * sew_bytes..(i + 1) * sew_bytes;
        let left = &lhs[range.clone()];
        let right = x;

        let res = get_bit_in_slice(result.as_slice(), i);
        let exp = get_bit_in_slice(expected.as_slice(), i);
        if res != exp {
            log!(
                "[sew = {}, describe = {}] unexpected values found at index {} (nth-element): {:?} (result) {:?} (expected)",
                sew, desc, i, res, exp
            );
            log!(
                "more information, lhs = {:0>2X?}, rhs = {:?}, lmul = {}, avl = {}",
                left,
                right,
                lmul,
                avl
            );
            panic!("Abort");
        }
    }
    if is_verbose() {
        log!("finished");
    }
}

pub fn run_vmsop_vi<T1, T2>(
    sew: u64,
    lmul: i64,
    avl: u64,
    imm: i64,
    mut expected_op: T1,
    mut v_op: T2,
    _: WideningCategory,
    desc: &str,
) where
    T1: FnMut(&[u8], i64, &mut [u8], usize),
    T2: FnMut(&[u8], i64, &mut [u8], u64, i64, u64),
{
    if is_verbose() {
        log!(
            "run with sew = {}, lmul = {}, avl = {}, desc = {}",
            sew,
            lmul,
            avl,
            desc
        );
    }
    let vl = vsetvl(avl as u64, sew, lmul);
    if vl == 0 {
        return;
    }

    let avl_bytes = (sew / 8 * avl) as usize;
    let sew_bytes = (sew / 8) as usize;

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

        expected_op(&lhs[range.clone()], imm, &mut expected, i);
    }
    v_op(lhs.as_slice(), imm, result.as_mut_slice(), sew, lmul, avl);

    for i in 0..avl as usize {
        let range = i * sew_bytes..(i + 1) * sew_bytes;
        let left = &lhs[range.clone()];

        let res = get_bit_in_slice(result.as_slice(), i);
        let exp = get_bit_in_slice(expected.as_slice(), i);
        if res != exp {
            log!(
                "[sew = {}, describe = {}] unexpected values found at index {} (nth-element): {:?} (result) {:?} (expected)",
                sew, desc, i, res, exp
            );
            log!(
                "more information, lhs = {:0>2X?}, rhs = {:?}, lmul = {}, avl = {}",
                left,
                imm,
                lmul,
                avl
            );
            panic!("Abort");
        }
    }
    if is_verbose() {
        log!("finished");
    }
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
    if is_verbose() {
        log!(
            "run with sew = {}, lmul = {}, avl = {}, desc = {}",
            sew,
            lmul,
            avl,
            desc
        );
    }

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
                "[sew = {}, describe = {}] unexpected values found at index {} (nth-element): {:0>2X?} (result) {:0>2X?} (expected)",
                sew, desc, i, res, exp
            );
            log!(
                "more information, lhs = {:0>2X?}, rhs = {:0>2X?}, lmul = {}, avl = {}",
                left,
                right,
                lmul,
                avl
            );
            panic!("Abort");
        }
    }
    if is_verbose() {
        log!("finished");
    }
}

pub fn run_vop_vi<T1, T2>(
    sew: u64,
    lmul: i64,
    avl: u64,
    imm: i64,
    mut expected_op: T1,
    mut v_op: T2,
    cat: WideningCategory,
    desc: &str,
) where
    T1: FnMut(&[u8], i64, &mut [u8]),
    T2: FnMut(&[u8], i64, &mut [u8], u64, i64, u64),
{
    if is_verbose() {
        log!(
            "run with sew = {}, lmul = {}, avl = {}, desc = {}",
            sew,
            lmul,
            avl,
            desc
        );
    }

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
            imm,
            &mut expected[expected_range.clone()],
        );
    }

    v_op(lhs.as_slice(), imm, result.as_mut_slice(), sew, lmul, avl);

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

        let res = &result[expected_range.clone()];
        let exp = &expected[expected_range.clone()];
        if res != exp {
            log!(
                "[sew = {}, describe = {}] unexpected values found at index {} (nth-element): {:0>2X?} (result) {:0>2X?} (expected)",
                sew, desc, i, res, exp
            );
            log!(
                "more information, lhs = {:0>2X?}, rhs = {:0>2X?}, lmul = {}, avl = {}",
                left,
                imm,
                lmul,
                avl
            );
            panic!("Abort");
        }
    }
    if is_verbose() {
        log!("finished");
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum InstructionArgsType {
    Vector,
    Vector2,
    VectorBit,
    Scalar,
    Immediate,
    UImmediate,
    None,
}

#[derive(Clone, Copy, PartialEq)]
pub enum MaskType {
    Disable,
    Enable,
    AsParam,
}

pub struct ExpectedParam {
    pub lhs: Vec<u8>,
    pub lhs_type: InstructionArgsType,
    pub rhs: Vec<u8>,
    pub rhs_type: InstructionArgsType,
    pub mask: Vec<u8>,
    pub mask_type: MaskType,

    pub res: Vec<u8>,
    pub res_type: InstructionArgsType,

    pub sew_bytes: usize,
    pub index: usize,

    pub sew: u64,
    pub lmul: i64,
    pub avl: u64,

    pub theoretically_vl: usize,
    pub count: usize,
}

#[derive(Clone, Copy)]
enum VectorCallbackType {
    None(fn(&mut ExpectedParam)),
    VV(fn(&mut [u8], &[u8], &[u8])),
    VX(fn(&mut [u8], &[u8], u64)),
    VI(fn(&mut [u8], &[u8], i64)),
    VVM(fn(&mut [u8], &[u8], &[u8], bool)),
    VXM(fn(&mut [u8], &[u8], u64, bool)),
    VIM(fn(&mut [u8], &[u8], i64, bool)),
    MVVM(fn(&mut bool, &[u8], &[u8], bool)),
    MVXM(fn(&mut bool, &[u8], u64, bool)),
    MVIM(fn(&mut bool, &[u8], i64, bool)),
    MVV(fn(&mut bool, &[u8], &[u8])),
    MVX(fn(&mut bool, &[u8], u64)),
    MVI(fn(&mut bool, &[u8], i64)),
}

struct RunOpConfig {
    pub sew: u64,
    pub lmul: i64,
    pub avl: u64,
    pub mask_type: MaskType,
    pub expected_op_ext: VectorCallbackType,
    pub v_op: fn(&[u8], &[u8], MaskType),
    pub vd_type: InstructionArgsType,
    pub args1_type: InstructionArgsType,
    pub args2_type: InstructionArgsType,
    pub immediate_val: i64,
}

impl RunOpConfig {
    fn get_args_len(t: InstructionArgsType, vl: usize) -> usize {
        match t {
            InstructionArgsType::None => 1,
            InstructionArgsType::Vector => vl,
            InstructionArgsType::Vector2 => vl * 2,
            InstructionArgsType::VectorBit => vl,
            _ => 8,
        }
    }

    pub fn get_vd_len(&self, vl: usize) -> usize {
        Self::get_args_len(self.vd_type, vl)
    }

    pub fn get_left_len(&self, vl: usize) -> usize {
        Self::get_args_len(self.args1_type, vl)
    }

    pub fn get_right_len(&self, vl: usize) -> usize {
        Self::get_args_len(self.args2_type, vl)
    }
}

impl ExpectedParam {
    fn new(config: &RunOpConfig, avl_bytes: usize) -> Self {
        let mut d = ExpectedParam {
            lhs: Vec::new(),
            lhs_type: config.args1_type,
            rhs: Vec::new(),
            rhs_type: config.args2_type,
            mask: Vec::new(),
            mask_type: config.mask_type,
            res: Vec::new(),
            res_type: config.vd_type,

            sew_bytes: config.sew as usize / 8,

            index: 0,

            sew: config.sew,
            lmul: config.lmul,
            avl: config.avl,
            theoretically_vl: 0,

            count: 0,
        };

        let mut rng = BestNumberRng::default();
        d.mask.resize(VLEN / 8, 0xFF);
        rng.fill(d.mask.as_mut_slice());

        d.lhs.resize(config.get_left_len(avl_bytes), 0);
        if config.args1_type == InstructionArgsType::Immediate
            || config.args1_type == InstructionArgsType::UImmediate
        {
            d.lhs.copy_from_slice(&config.immediate_val.to_le_bytes());
        } else {
            rng.fill(d.lhs.as_mut_slice());
        }

        d.rhs.resize(config.get_right_len(avl_bytes), 0);
        if config.args2_type == InstructionArgsType::Immediate
            || config.args2_type == InstructionArgsType::UImmediate
        {
            d.rhs.copy_from_slice(&config.immediate_val.to_le_bytes());
        } else {
            rng.fill(d.rhs.as_mut_slice());
        }

        d.res.resize(config.get_vd_len(avl_bytes), 0);
        rng.fill(d.res.as_mut_slice());

        d
    }

    pub fn get_data_by_sclice(
        d: &[u8],
        t: InstructionArgsType,
        sew_bytes: usize,
        index: usize,
    ) -> Vec<u8> {
        if t == InstructionArgsType::VectorBit {
            let mut res = Vec::<u8>::new();
            res.resize(1, 0);
            res[0] = get_bit_in_slice(d, index);
            res
        } else {
            d[get_args_range(t, sew_bytes, index)].to_vec()
        }
    }

    pub fn get_left(&self) -> Vec<u8> {
        Self::get_data_by_sclice(&self.lhs, self.lhs_type, self.sew_bytes, self.index)
    }

    pub fn get_right(&self) -> Vec<u8> {
        Self::get_data_by_sclice(&self.rhs, self.rhs_type, self.sew_bytes, self.index)
    }

    pub fn get_right_u64(&self) -> u64 {
        let d = self.get_right();
        if d.len() != 8 {
            panic!("get_right_u64, right len is: {}", d.len())
        }
        u64::from_le_bytes(self.get_right().try_into().unwrap())
    }

    pub fn get_result(&self) -> Vec<u8> {
        Self::get_data_by_sclice(&self.res, self.res_type, self.sew_bytes, self.index)
    }

    pub fn set_result(&mut self, data: &[u8]) {
        let r = get_args_range(self.res_type, self.sew_bytes, self.index);
        if data.len() != r.len() {
            panic!("set_result data len is ne: {}, {}", data.len(), r.len());
        }
        self.res[r].copy_from_slice(data);
    }

    pub fn get_mask(&self) -> bool {
        match self.mask_type {
            MaskType::Enable => {
                get_bit_in_slice(&self.mask, self.index % self.theoretically_vl) == 1
            }
            MaskType::AsParam => {
                get_bit_in_slice(&self.mask, self.index % self.theoretically_vl) == 1
            }
            MaskType::Disable => true,
        }
    }

    pub fn get_rvv_left(&self) -> &[u8] {
        if self.lhs_type == InstructionArgsType::Immediate
            || self.lhs_type == InstructionArgsType::UImmediate
            || self.lhs_type == InstructionArgsType::Scalar
        {
            self.lhs.as_slice()
        } else {
            let vl =
                RunOpConfig::get_args_len(self.lhs_type, self.theoretically_vl) * self.sew_bytes;

            let begin = vl * self.count;
            let mut end = vl * (self.count + 1);
            if end > self.lhs.len() {
                end = self.lhs.len();
            }

            &self.lhs[begin..end]
        }
    }

    pub fn get_rvv_right(&self) -> &[u8] {
        if self.rhs_type == InstructionArgsType::Immediate
            || self.rhs_type == InstructionArgsType::UImmediate
            || self.rhs_type == InstructionArgsType::Scalar
        {
            self.rhs.as_slice()
        } else {
            let vl =
                RunOpConfig::get_args_len(self.rhs_type, self.theoretically_vl) * self.sew_bytes;

            let begin = vl * self.count;
            let mut end = vl * (self.count + 1);
            if end > self.rhs.len() {
                end = self.rhs.len();
            }

            &self.rhs[begin..end]
        }
    }

    pub fn get_rvv_result_range(&self) -> Range<usize> {
        if self.res_type == InstructionArgsType::Immediate
            || self.res_type == InstructionArgsType::UImmediate
            || self.res_type == InstructionArgsType::Scalar
        {
            0..self.res.len()
        } else {
            let vl =
                RunOpConfig::get_args_len(self.res_type, self.theoretically_vl) * self.sew_bytes;

            let begin = vl * self.count;
            let mut end = vl * (self.count + 1);
            if end > self.res.len() {
                end = self.res.len();
            }

            begin..end
        }
    }

    fn get_sew(sew: u64, t: InstructionArgsType) -> u64 {
        match t {
            InstructionArgsType::Vector2 => sew * 2,
            _ => sew,
        }
    }

    fn get_left_sew(&self, sew: u64) -> u64 {
        ExpectedParam::get_sew(sew, self.lhs_type)
    }

    fn get_right_sew(&self, sew: u64) -> u64 {
        ExpectedParam::get_sew(sew, self.rhs_type)
    }

    fn get_result_sew(&self, sew: u64) -> u64 {
        ExpectedParam::get_sew(sew, self.res_type)
    }
}

fn get_args_range(t: InstructionArgsType, sew_bytes: usize, index: usize) -> Range<usize> {
    match t {
        InstructionArgsType::None => {
            panic!("Can not get range for here")
        }
        InstructionArgsType::Vector => index * sew_bytes..(index + 1) * sew_bytes,
        InstructionArgsType::Vector2 => index * sew_bytes * 2..(index + 1) * sew_bytes * 2,
        InstructionArgsType::VectorBit => {
            panic!("VectorBit can not get range for here")
        }
        _ => 0..8,
    }
}

fn run_rvv_op(exp_param: &mut ExpectedParam, res: &mut [u8], op: fn(&[u8], &[u8], MaskType)) {
    let empty_buf = [0u8; 1];

    let mut avl = exp_param.avl as i64;
    let sew = exp_param.sew;
    let mask_type = exp_param.mask_type;

    exp_param.count = 0;
    while avl > 0 {
        let vl = vsetvl(avl as u64, exp_param.sew, exp_param.lmul) as usize;
        if vl == 0 {
            panic!("Abort")
        }
        avl -= vl as i64;

        let l = if exp_param.lhs_type == InstructionArgsType::Immediate
            || exp_param.lhs_type == InstructionArgsType::UImmediate
            || exp_param.lhs_type == InstructionArgsType::Scalar
        {
            exp_param.lhs.as_slice()
        } else {
            vle_v8(exp_param.get_left_sew(sew), &exp_param.get_rvv_left());
            &empty_buf
        };

        let r = if exp_param.rhs_type == InstructionArgsType::Immediate
            || exp_param.rhs_type == InstructionArgsType::UImmediate
            || exp_param.rhs_type == InstructionArgsType::Scalar
        {
            exp_param.rhs.as_slice()
        } else {
            vle_v16(exp_param.get_right_sew(sew), exp_param.get_rvv_right());
            &empty_buf
        };

        if mask_type == MaskType::Enable || mask_type == MaskType::AsParam {
            vl1r_v0(&exp_param.mask);
        }

        let res_range = exp_param.get_rvv_result_range();
        vle_v24(exp_param.get_result_sew(sew), &res[res_range.clone()]);
        op.clone()(l, r, mask_type);
        vse_v24(exp_param.get_result_sew(sew), &mut res[res_range.clone()]);

        exp_param.count += 1;
    }
}

fn run_op(config: &RunOpConfig, desc: &str) {
    if is_verbose() {
        log!(
            "run with sew = {}, lmul = {}, avl = {}, desc = {}",
            config.sew,
            config.lmul,
            config.avl,
            desc
        );
    }
    let vl = vsetvl(config.avl as u64, config.sew, config.lmul);
    if vl == 0 {
        return;
    }

    let avl_bytes = (config.sew / 8 * config.avl) as usize;
    let sew_bytes = (config.sew / 8) as usize;

    let mut exp_param = ExpectedParam::new(config, avl_bytes);

    exp_param.theoretically_vl = vl as usize;
    let expected_before = exp_param.res.clone();
    let mut result = exp_param.res.clone();
    for i in 0..config.avl as usize {
        exp_param.index = i;
        if config.mask_type == MaskType::Enable && !exp_param.get_mask() {
            continue;
        }
        match config.expected_op_ext {
            VectorCallbackType::None(op) => {
                op(&mut exp_param);
            }
            VectorCallbackType::VV(op) => {
                let mut res = exp_param.get_result();
                op(
                    res.as_mut_slice(),
                    exp_param.get_left().as_slice(),
                    exp_param.get_right().as_slice(),
                );
                exp_param.set_result(&res);
            }
            VectorCallbackType::VX(op) => {
                let mut res = exp_param.get_result();
                op(
                    res.as_mut_slice(),
                    exp_param.get_left().as_slice(),
                    exp_param.get_right_u64(),
                );
                exp_param.set_result(&res);
            }
            VectorCallbackType::VI(op) => {
                let mut res = exp_param.get_result();
                op(
                    res.as_mut_slice(),
                    exp_param.get_left().as_slice(),
                    exp_param.get_right_u64() as i64,
                );
                exp_param.set_result(&res);
            }
            VectorCallbackType::VVM(op) => {
                let mut res = exp_param.get_result();
                op(
                    res.as_mut_slice(),
                    exp_param.get_left().as_slice(),
                    exp_param.get_right().as_slice(),
                    exp_param.get_mask(),
                );
                exp_param.set_result(&res);
            }
            VectorCallbackType::VXM(op) => {
                let mut res = exp_param.get_result();
                op(
                    res.as_mut_slice(),
                    exp_param.get_left().as_slice(),
                    exp_param.get_right_u64(),
                    exp_param.get_mask(),
                );
                exp_param.set_result(&res);
            }
            VectorCallbackType::VIM(op) => {
                let mut res = exp_param.get_result();
                op(
                    res.as_mut_slice(),
                    exp_param.get_left().as_slice(),
                    exp_param.get_right_u64() as i64,
                    exp_param.get_mask(),
                );
                exp_param.set_result(&res);
            }
            VectorCallbackType::MVVM(op) => {
                let index = exp_param.index as u64;
                let pos = (config.sew * vl * (index / vl) + index % vl) as usize;

                let mut res = get_bit_in_slice(&exp_param.res, pos) == 1;
                op(
                    &mut res,
                    exp_param.get_left().as_slice(),
                    exp_param.get_right().as_slice(),
                    exp_param.get_mask(),
                );
                set_bit_in_slice(&mut exp_param.res, pos, res as u8);
            }
            VectorCallbackType::MVXM(op) => {
                let index = exp_param.index as u64;
                let pos = (config.sew * vl * (index / vl) + index % vl) as usize;

                let mut res = get_bit_in_slice(&exp_param.res, pos) == 1;
                op(
                    &mut res,
                    exp_param.get_left().as_slice(),
                    exp_param.get_right_u64(),
                    exp_param.get_mask(),
                );
                set_bit_in_slice(&mut exp_param.res, pos, res as u8);
            }
            VectorCallbackType::MVIM(op) => {
                let index = exp_param.index as u64;
                let pos = (config.sew * vl * (index / vl) + index % vl) as usize;

                let mut res = get_bit_in_slice(&exp_param.res, pos) == 1;
                op(
                    &mut res,
                    exp_param.get_left().as_slice(),
                    exp_param.get_right_u64() as i64,
                    exp_param.get_mask(),
                );
                set_bit_in_slice(&mut exp_param.res, pos, res as u8);
            }
            VectorCallbackType::MVV(op) => {
                let index = exp_param.index as u64;
                let pos = (config.sew * vl * (index / vl) + index % vl) as usize;

                let mut res = get_bit_in_slice(&exp_param.res, pos) == 1;
                op(
                    &mut res,
                    exp_param.get_left().as_slice(),
                    exp_param.get_right().as_slice(),
                );
                set_bit_in_slice(&mut exp_param.res, pos, res as u8);
            }
            VectorCallbackType::MVX(op) => {
                let index = exp_param.index as u64;
                let pos = (config.sew * vl * (index / vl) + index % vl) as usize;

                let mut res = get_bit_in_slice(&exp_param.res, pos) == 1;
                op(
                    &mut res,
                    exp_param.get_left().as_slice(),
                    exp_param.get_right_u64(),
                );
                set_bit_in_slice(&mut exp_param.res, pos, res as u8);
            }
            VectorCallbackType::MVI(op) => {
                let index = exp_param.index as u64;
                let pos = (config.sew * vl * (index / vl) + index % vl) as usize;

                let mut res = get_bit_in_slice(&exp_param.res, pos) == 1;
                op(
                    &mut res,
                    exp_param.get_left().as_slice(),
                    exp_param.get_right_u64() as i64,
                );
                set_bit_in_slice(&mut exp_param.res, pos, res as u8);
            }
        }
    }

    run_rvv_op(&mut exp_param, result.as_mut_slice(), config.v_op);

    for i in 0..config.avl as usize {
        exp_param.index = i;
        let exp = exp_param.get_result();
        let res = ExpectedParam::get_data_by_sclice(&result, config.vd_type, sew_bytes, i);
        let exp_befor =
            ExpectedParam::get_data_by_sclice(&expected_before, config.vd_type, sew_bytes, i);

        if exp != res {
            log!(
                "[sew = {}, describe = {}] unexpected values found at index {} \nresult = {:0>2X?} \nexpected = {:0>2X?}",
                config.sew, desc, i, res, exp
            );
            log!(
                "more information, \nlhs = {:0>2X?} \nrhs = {:0>2X?} \nexpected_before = {:0>2X?}, \nlmul = {}, avl = {}",
                exp_param.get_left(),
                exp_param.get_right(),
                exp_befor,
                config.lmul,
                config.avl
            );
            //log!("more infomation, expected: {:0>2X?} \nresult: {:0>2X?} \nmask: {:0>2X?}", exp_param.res, result, exp_param.mask);
            panic!("Abort");
        }
    }
    if is_verbose() {
        log!("finished");
    }
}

fn get_imm_begin(l: InstructionArgsType, r: InstructionArgsType) -> i64 {
    if l == InstructionArgsType::Immediate || r == InstructionArgsType::Immediate {
        -16
    } else if l == InstructionArgsType::UImmediate || r == InstructionArgsType::UImmediate {
        0
    } else {
        0
    }
}

fn run_template_ext(
    vd_type: InstructionArgsType,
    left_type: InstructionArgsType,
    right_type: InstructionArgsType,
    mask_type: MaskType,
    rvv_op: fn(&[u8], &[u8], MaskType),
    callback: VectorCallbackType,
    sews: &[u64],
    lmuls: &[i64],
    desc: &str,
) {
    let mut enable_mask = true;
    let imm_begin = get_imm_begin(left_type, right_type);
    let mut imm = imm_begin;
    for sew in sews {
        for lmul in lmuls {
            for avl in avl_iterator(sew.clone(), 2) {
                let mut config = RunOpConfig {
                    sew: sew.clone(),
                    lmul: lmul.clone(),
                    avl,
                    mask_type,
                    expected_op_ext: callback,
                    v_op: rvv_op,
                    vd_type: vd_type,
                    args1_type: left_type,
                    args2_type: right_type,
                    immediate_val: imm,
                };

                if mask_type == MaskType::Enable {
                    config.mask_type = if enable_mask {
                        MaskType::Enable
                    } else {
                        MaskType::Disable
                    };
                    enable_mask = !enable_mask;
                }
                run_op(&config, desc);

                if left_type == InstructionArgsType::Immediate
                    || right_type == InstructionArgsType::Immediate
                    || left_type == InstructionArgsType::UImmediate
                    || right_type == InstructionArgsType::UImmediate
                {
                    imm += 1;
                    if imm == imm_begin + 32 {
                        imm = imm_begin
                    }
                }
            }
        }
    }
}

pub fn run_template(
    vd_type: InstructionArgsType,
    left_type: InstructionArgsType,
    right_type: InstructionArgsType,
    mask_type: MaskType,
    expected_op: fn(&mut ExpectedParam),
    rvv_op: fn(&[u8], &[u8], MaskType),
    sews: &[u64],
    lmuls: &[i64],
    desc: &str,
) {
    run_template_ext(
        vd_type,
        left_type,
        right_type,
        mask_type,
        rvv_op,
        VectorCallbackType::None(expected_op),
        sews,
        lmuls,
        desc,
    );
}

pub fn run_template_vvm(
    expected_op: fn(&mut [u8], &[u8], &[u8], bool),
    rvv_op: fn(&[u8], &[u8], MaskType),
    sews: &[u64],
    lmuls: &[i64],
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        MaskType::AsParam,
        rvv_op,
        VectorCallbackType::VVM(expected_op),
        sews,
        lmuls,
        desc,
    );
}

pub fn run_template_vxm(
    expected_op: fn(&mut [u8], &[u8], u64, bool),
    rvv_op: fn(&[u8], &[u8], MaskType),
    sews: &[u64],
    lmuls: &[i64],
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        InstructionArgsType::Scalar,
        MaskType::AsParam,
        rvv_op,
        VectorCallbackType::VXM(expected_op),
        sews,
        lmuls,
        desc,
    );
}

pub fn run_template_vim(
    expected_op: fn(&mut [u8], &[u8], i64, bool),
    rvv_op: fn(&[u8], &[u8], MaskType),
    sews: &[u64],
    lmuls: &[i64],
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        InstructionArgsType::Immediate,
        MaskType::AsParam,
        rvv_op,
        VectorCallbackType::VIM(expected_op),
        sews,
        lmuls,
        desc,
    );
}

pub fn run_template_mvvm(
    expected_op: fn(&mut bool, &[u8], &[u8], bool),
    rvv_op: fn(&[u8], &[u8], MaskType),
    sews: &[u64],
    lmuls: &[i64],
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::VectorBit,
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        MaskType::AsParam,
        rvv_op,
        VectorCallbackType::MVVM(expected_op),
        sews,
        lmuls,
        desc,
    );
}

pub fn run_template_mvxm(
    expected_op: fn(&mut bool, &[u8], u64, bool),
    rvv_op: fn(&[u8], &[u8], MaskType),
    sews: &[u64],
    lmuls: &[i64],
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::VectorBit,
        InstructionArgsType::Vector,
        InstructionArgsType::Scalar,
        MaskType::AsParam,
        rvv_op,
        VectorCallbackType::MVXM(expected_op),
        sews,
        lmuls,
        desc,
    );
}

pub fn run_template_mvim(
    expected_op: fn(&mut bool, &[u8], i64, bool),
    rvv_op: fn(&[u8], &[u8], MaskType),
    sews: &[u64],
    lmuls: &[i64],
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::VectorBit,
        InstructionArgsType::Vector,
        InstructionArgsType::Immediate,
        MaskType::AsParam,
        rvv_op,
        VectorCallbackType::MVIM(expected_op),
        sews,
        lmuls,
        desc,
    );
}

pub fn run_template_mvv(
    expected_op: fn(&mut bool, &[u8], &[u8]),
    rvv_op: fn(&[u8], &[u8], MaskType),
    sews: &[u64],
    lmuls: &[i64],
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::VectorBit,
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        MaskType::Disable,
        rvv_op,
        VectorCallbackType::MVV(expected_op),
        sews,
        lmuls,
        desc,
    );
}

pub fn run_template_mvx(
    expected_op: fn(&mut bool, &[u8], u64),
    rvv_op: fn(&[u8], &[u8], MaskType),
    sews: &[u64],
    lmuls: &[i64],
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::VectorBit,
        InstructionArgsType::Vector,
        InstructionArgsType::Scalar,
        MaskType::Disable,
        rvv_op,
        VectorCallbackType::MVX(expected_op),
        sews,
        lmuls,
        desc,
    );
}

pub fn run_template_mvi(
    expected_op: fn(&mut bool, &[u8], i64),
    rvv_op: fn(&[u8], &[u8], MaskType),
    sews: &[u64],
    lmuls: &[i64],
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::VectorBit,
        InstructionArgsType::Vector,
        InstructionArgsType::Immediate,
        MaskType::Disable,
        rvv_op,
        VectorCallbackType::MVI(expected_op),
        sews,
        lmuls,
        desc,
    );
}

pub fn run_template_wv(
    expected_op: fn(&mut [u8], &[u8], &[u8]),
    rvv_op: fn(&[u8], &[u8], MaskType),
    sews: &[u64],
    lmuls: &[i64],
    enable_mask: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector2,
        InstructionArgsType::Vector,
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::VV(expected_op),
        sews,
        lmuls,
        desc,
    );
}

pub fn run_template_wx(
    expected_op: fn(&mut [u8], &[u8], u64),
    rvv_op: fn(&[u8], &[u8], MaskType),
    sews: &[u64],
    lmuls: &[i64],
    enable_mask: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector2,
        InstructionArgsType::Scalar,
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::VX(expected_op),
        sews,
        lmuls,
        desc,
    );
}

pub fn run_template_wi(
    expected_op: fn(&mut [u8], &[u8], i64),
    rvv_op: fn(&[u8], &[u8], MaskType),
    sews: &[u64],
    lmuls: &[i64],
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector2,
        InstructionArgsType::UImmediate,
        MaskType::Disable,
        rvv_op,
        VectorCallbackType::VI(expected_op),
        sews,
        lmuls,
        desc,
    );
}
