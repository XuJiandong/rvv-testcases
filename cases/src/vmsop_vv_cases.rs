use core::{arch::asm, convert::TryInto};
use eint::{Eint, E256};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    intrinsic::vmsop_vv,
    misc::{avl_iterator, set_bit_in_slice},
    runner::{run_vmsop_vv, WideningCategory},
};

fn expected_eq(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert_eq!(lhs.len(), rhs.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            let res = if l == r { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = if l == r { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmseq(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmseq.vv v24, v8, v16");
        });
    }

    run_vmsop_vv(
        sew,
        lmul,
        avl,
        expected_eq,
        op,
        WideningCategory::None,
        "vmseq.vv",
    );
}

fn expected_ne(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert_eq!(lhs.len(), rhs.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            let res = if l != r { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = if l != r { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsne(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmsne.vv v24, v8, v16");
        });
    }

    run_vmsop_vv(
        sew,
        lmul,
        avl,
        expected_ne,
        op,
        WideningCategory::None,
        "vmsne.vv",
    );
}

fn expected_ltu(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert_eq!(lhs.len(), rhs.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            let res = if l < r { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = if l < r { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsltu(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmsltu.vv v24, v8, v16");
        });
    }

    run_vmsop_vv(
        sew,
        lmul,
        avl,
        expected_ltu,
        op,
        WideningCategory::None,
        "vmsltu.vv",
    );
}

fn expected_lt(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert_eq!(lhs.len(), rhs.len());
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap());
            let res = if l < r { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = if l.cmp_s(&r).is_le() { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmslt(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmslt.vv v24, v8, v16");
        });
    }

    run_vmsop_vv(
        sew,
        lmul,
        avl,
        expected_lt,
        op,
        WideningCategory::None,
        "vmslt.vv",
    );
}

fn expected_leu(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert_eq!(lhs.len(), rhs.len());
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            let res = if l <= r { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = if l <= r { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsleu(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmsleu.vv v24, v8, v16");
        });
    }

    run_vmsop_vv(
        sew,
        lmul,
        avl,
        expected_leu,
        op,
        WideningCategory::None,
        "vmsleu.vv",
    );
}

fn expected_le(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert_eq!(lhs.len(), rhs.len());
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap());
            let res = if l <= r { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let ord = l.cmp_s(&r);
            let res = if ord.is_eq() || ord.is_le() { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsle(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vmsle.vv v24, v8, v16");
        });
    }

    run_vmsop_vv(
        sew,
        lmul,
        avl,
        expected_le,
        op,
        WideningCategory::None,
        "vmsle.vv",
    );
}

pub fn test_vmsop_vv() {
    for sew in [64, 256] {
        for lmul in [-8, -2, 1, 4, 8] {
            for avl in avl_iterator(sew, 4) {
                test_vmseq(sew, lmul, avl);
                test_vmsne(sew, lmul, avl);
                test_vmsltu(sew, lmul, avl);
                test_vmslt(sew, lmul, avl);
                test_vmsleu(sew, lmul, avl);
                test_vmsle(sew, lmul, avl);
            }
        }
    }
}
