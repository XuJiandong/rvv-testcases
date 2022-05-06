use core::{arch::asm, convert::TryInto};
use eint::{Eint, E256};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    intrinsic::vmsop_vx,
    misc::{avl_iterator, set_bit_in_slice},
    runner::{run_vmsop_vx, WideningCategory},
};

fn expected_op_eq(lhs: &[u8], x: u64, result: &mut [u8], index: usize) {
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let res = if l == x as i64 { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            let res = if l.cmp_s(&r).is_eq() { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmseq(sew: u64, lmul: i64, avl: u64) {
    fn eq(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vx(lhs, x, result, sew, avl, lmul, |x| unsafe {
            rvv_asm!("mv t0, {}", "vmseq.vx v24, v8, t0", in (reg) x);
        });
    }
    run_vmsop_vx(
        sew,
        lmul,
        avl,
        expected_op_eq,
        eq,
        WideningCategory::None,
        "vmseq.vx",
    );
}

fn expected_op_ne(lhs: &[u8], x: u64, result: &mut [u8], index: usize) {
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let res = if l != x as i64 { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            let res = if l.cmp_s(&r).is_ne() { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsne(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vx(lhs, x, result, sew, avl, lmul, |x| unsafe {
            rvv_asm!("mv t0, {}", "vmsne.vx v24, v8, t0", in (reg) x);
        });
    }
    run_vmsop_vx(
        sew,
        lmul,
        avl,
        expected_op_ne,
        op,
        WideningCategory::None,
        "vmsne.vx",
    );
}

fn expected_op_ltu(lhs: &[u8], x: u64, result: &mut [u8], index: usize) {
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let res = if l < x { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x);
            let res = if l.cmp_u(&r).is_lt() { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsltu(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vx(lhs, x, result, sew, avl, lmul, |x| unsafe {
            rvv_asm!("mv t0, {}", "vmsltu.vx v24, v8, t0", in (reg) x);
        });
    }

    run_vmsop_vx(
        sew,
        lmul,
        avl,
        expected_op_ltu,
        op,
        WideningCategory::None,
        "vmsltu.vx",
    );
}

fn expected_op_lt(lhs: &[u8], x: u64, result: &mut [u8], index: usize) {
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = x as i64;
            let res = if l < r { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            let res = if l.cmp_s(&r).is_lt() { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmslt(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vx(lhs, x, result, sew, avl, lmul, |x| unsafe {
            rvv_asm!("mv t0, {}", "vmslt.vx v24, v8, t0", in (reg) x);
        });
    }
    run_vmsop_vx(
        sew,
        lmul,
        avl,
        expected_op_lt,
        op,
        WideningCategory::None,
        "vmslt.vx",
    );
}

fn expected_op_leu(lhs: &[u8], x: u64, result: &mut [u8], index: usize) {
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let res = if l <= x { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x);
            let res = if l.cmp_u(&r).is_le() { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsleu(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vx(lhs, x, result, sew, avl, lmul, |x| unsafe {
            rvv_asm!("mv t0, {}", "vmsleu.vx v24, v8, t0", in (reg) x);
        });
    }
    run_vmsop_vx(
        sew,
        lmul,
        avl,
        expected_op_leu,
        op,
        WideningCategory::None,
        "vmsleu.vx",
    );
}

fn expected_op_le(lhs: &[u8], x: u64, result: &mut [u8], index: usize) {
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let res = if l <= x as i64 { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            let res = if l.cmp_s(&r).is_le() { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsle(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vx(lhs, x, result, sew, avl, lmul, |x| unsafe {
            rvv_asm!("mv t0, {}", "vmsle.vx v24, v8, t0", in (reg) x);
        });
    }
    run_vmsop_vx(
        sew,
        lmul,
        avl,
        expected_op_le,
        op,
        WideningCategory::None,
        "vmsle.vx",
    );
}

fn expected_op_gtu(lhs: &[u8], x: u64, result: &mut [u8], index: usize) {
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let res = if l > x { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x);
            let res = if l.cmp_u(&r).is_gt() { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsgtu(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vx(lhs, x, result, sew, avl, lmul, |x| unsafe {
            rvv_asm!("mv t0, {}", "vmsgtu.vx v24, v8, t0", in (reg) x);
        });
    }
    run_vmsop_vx(
        sew,
        lmul,
        avl,
        expected_op_gtu,
        op,
        WideningCategory::None,
        "vmsgtu.vx",
    );
}

fn expected_op_gt(lhs: &[u8], x: u64, result: &mut [u8], index: usize) {
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let res = if l > x as i64 { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            let res = if l.cmp_s(&r).is_gt() { 1 } else { 0 };
            set_bit_in_slice(result, index, res);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsgt(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vx(lhs, x, result, sew, avl, lmul, |x| unsafe {
            rvv_asm!("mv t0, {}", "vmsgt.vx v24, v8, t0", in (reg) x);
        });
    }
    run_vmsop_vx(
        sew,
        lmul,
        avl,
        expected_op_gt,
        op,
        WideningCategory::None,
        "vmsgt.vx",
    );
}

pub fn test_vmsop_vx() {
    for sew in [64, 256] {
        for lmul in [-8, -2, 1, 4, 8] {
            for avl in avl_iterator(sew, 4) {
                test_vmseq(sew, lmul, avl);
                test_vmsne(sew, lmul, avl);
                test_vmsltu(sew, lmul, avl);
                test_vmslt(sew, lmul, avl);
                test_vmsleu(sew, lmul, avl);
                test_vmsle(sew, lmul, avl);
                test_vmsgtu(sew, lmul, avl);
                test_vmsgt(sew, lmul, avl);
            }
        }
    }
}
