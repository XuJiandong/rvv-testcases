use alloc::boxed::Box;
use alloc::vec::Vec;

use ckb_std::syscalls::debug;
use rand::{Rng, RngCore};

use crate::intrinsic::vsetvl;
use crate::misc::VLEN;
use crate::rng::{new_random01_vec, new_random_vec};

use super::log;
use super::misc::{ceiling, get_bit_in_slice, is_verbose};
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
