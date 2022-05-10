use alloc::boxed::Box;
use core::{arch::asm, convert::TryInto};
use eint::{Eint, E128, E256};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    intrinsic::{vop_vi, vop_vv, vop_vx},
    misc::{avl_iterator, conver_to_i256},
    runner::{run_vop_vi, run_vop_vv, run_vop_vx, ExpectedOp, WideningCategory},
};

fn expected_op_saddu(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            let l = lhs[0] as u8;
            let r = x as u8;
            let (res, overflow) = l.overflowing_add(r);
            if overflow {
                result.copy_from_slice(&u8::MAX.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        2 => {
            let (res, overflow) =
                u16::from_le_bytes(lhs.try_into().unwrap()).overflowing_add(x as u16);
            if overflow {
                result.copy_from_slice(&u16::MAX.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        4 => {
            let (res, overflow) =
                u32::from_le_bytes(lhs.try_into().unwrap()).overflowing_add(x as u32);
            if overflow {
                result.copy_from_slice(&u32::MAX.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        8 => {
            let (res, overflow) =
                u64::from_le_bytes(lhs.try_into().unwrap()).overflowing_add(x as u64);
            if overflow {
                result.copy_from_slice(&u64::MAX.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let (res, overflow) =
                u128::from_le_bytes(lhs.try_into().unwrap()).overflowing_add(x as u128);
            if overflow {
                result.copy_from_slice(&u128::MAX.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            let (res, overflow) = E256::get(lhs).overflowing_add_u(E256::from(x));
            if overflow {
                E256::MAX_U.put(result);
            } else {
                res.put(result);
            }
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vsaddu_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", 
                     "vsaddu.vx v24, v8, t0",
                     in (reg) x);
        });
    }

    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_saddu,
        op,
        WideningCategory::None,
        "vsaddu.vx",
    );
}

fn expected_op_saddu_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let l = lhs[0] as u8;
            let r = rhs[0] as u8;
            let (res, overflow) = l.overflowing_add(r);
            if overflow {
                result.copy_from_slice(&u8::MAX.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        2 => {
            let (res, overflow) = u16::from_le_bytes(lhs.try_into().unwrap())
                .overflowing_add(u16::from_le_bytes(rhs.try_into().unwrap()));
            if overflow {
                result.copy_from_slice(&u16::MAX.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        4 => {
            let (res, overflow) = u32::from_le_bytes(lhs.try_into().unwrap())
                .overflowing_add(u32::from_le_bytes(rhs.try_into().unwrap()));
            if overflow {
                result.copy_from_slice(&u32::MAX.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        8 => {
            let (res, overflow) = u64::from_le_bytes(lhs.try_into().unwrap())
                .overflowing_add(u64::from_le_bytes(rhs.try_into().unwrap()));
            if overflow {
                result.copy_from_slice(&u64::MAX.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let (res, overflow) = u128::from_le_bytes(lhs.try_into().unwrap())
                .overflowing_add(u128::from_le_bytes(rhs.try_into().unwrap()));
            if overflow {
                result.copy_from_slice(&u128::MAX.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            let (res, overflow) = E256::get(lhs).overflowing_add_u(E256::get(rhs));
            if overflow {
                E256::MAX_U.put(result);
            } else {
                res.put(result);
            }
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vsaddu_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vsaddu.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_saddu_vv)),
        op,
        WideningCategory::None,
        "vsaddu.vv",
    );
}

fn expected_op_saddu_vi(lhs: &[u8], imm: i64, result: &mut [u8]) {
    match lhs.len() {
        32 => {
            let (res, overflow) =
                E256::get(lhs).overflowing_add_u(conver_to_i256(E128::from(imm as i128)));
            if overflow {
                E256::MAX_U.put(result);
            } else {
                res.put(result);
            }
        }
        _ => {
            expected_op_saddu(lhs, imm as u64, result);
        }
    }
}
fn test_vsaddu_vi(sew: u64, lmul: i64, avl: u64, imm: i64) {
    fn op(lhs: &[u8], imm: i64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vi(lhs, imm, result, sew, avl, lmul, |imm| unsafe {
            match imm {
                16 => {
                    rvv_asm!("vsaddu.vi v24, v8, 16");
                }
                15 => {
                    rvv_asm!("vsaddu.vi v24, v8, 15");
                }
                14 => {
                    rvv_asm!("vsaddu.vi v24, v8, 14");
                }
                13 => {
                    rvv_asm!("vsaddu.vi v24, v8, 13");
                }
                12 => {
                    rvv_asm!("vsaddu.vi v24, v8, 12");
                }
                11 => {
                    rvv_asm!("vsaddu.vi v24, v8, 11");
                }
                10 => {
                    rvv_asm!("vsaddu.vi v24, v8, 10");
                }
                9 => {
                    rvv_asm!("vsaddu.vi v24, v8, 9");
                }
                8 => {
                    rvv_asm!("vsaddu.vi v24, v8, 8");
                }
                7 => {
                    rvv_asm!("vsaddu.vi v24, v8, 7");
                }
                6 => {
                    rvv_asm!("vsaddu.vi v24, v8, 6");
                }
                5 => {
                    rvv_asm!("vsaddu.vi v24, v8, 5");
                }
                4 => {
                    rvv_asm!("vsaddu.vi v24, v8, 4");
                }
                3 => {
                    rvv_asm!("vsaddu.vi v24, v8, 3");
                }
                2 => {
                    rvv_asm!("vsaddu.vi v24, v8, 2");
                }
                1 => {
                    rvv_asm!("vsaddu.vi v24, v8, 1");
                }
                0 => {
                    rvv_asm!("vsaddu.vi v24, v8, 0");
                }
                -1 => {
                    rvv_asm!("vsaddu.vi v24, v8, -1");
                }
                -2 => {
                    rvv_asm!("vsaddu.vi v24, v8, -2");
                }
                -3 => {
                    rvv_asm!("vsaddu.vi v24, v8, -3");
                }
                -4 => {
                    rvv_asm!("vsaddu.vi v24, v8, -4");
                }
                -5 => {
                    rvv_asm!("vsaddu.vi v24, v8, -5");
                }
                -6 => {
                    rvv_asm!("vsaddu.vi v24, v8, -6");
                }
                -7 => {
                    rvv_asm!("vsaddu.vi v24, v8, -7");
                }
                -8 => {
                    rvv_asm!("vsaddu.vi v24, v8, -8");
                }
                -9 => {
                    rvv_asm!("vsaddu.vi v24, v8, -9");
                }
                -10 => {
                    rvv_asm!("vsaddu.vi v24, v8, -10");
                }
                -11 => {
                    rvv_asm!("vsaddu.vi v24, v8, -11");
                }
                -12 => {
                    rvv_asm!("vsaddu.vi v24, v8, -12");
                }
                -13 => {
                    rvv_asm!("vsaddu.vi v24, v8, -13");
                }
                -14 => {
                    rvv_asm!("vsaddu.vi v24, v8, -14");
                }
                -15 => {
                    rvv_asm!("vsaddu.vi v24, v8, -15");
                }
                -16 => {
                    rvv_asm!("vsaddu.vi v24, v8, -16");
                }
                _ => {
                    panic!("can't support this immediate: {}", imm);
                }
            }
        });
    }
    run_vop_vi(
        sew,
        lmul,
        avl,
        imm,
        expected_op_saddu_vi,
        op,
        WideningCategory::None,
        "vsaddu.vi",
    );
}

fn expected_op_sadd(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            let l = lhs[0] as i16;
            let res = l.wrapping_add((x as i8) as i16);
            if res > i8::MAX as i16 {
                result[0] = i8::MAX as u8;
            } else if res < i8::MIN as i16 {
                result[0] = i8::MIN as u8;
            } else {
                result[0] = res as u8;
            }
        }
        2 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap()) as i32;
            let res = l.wrapping_add((x as i16) as i32);
            if res > i16::MAX as i32 {
                result.copy_from_slice(&i16::MAX.to_le_bytes());
            } else if res < i16::MIN as i32 {
                result.copy_from_slice(&i16::MIN.to_le_bytes());
            } else {
                let res = res as i16;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        4 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap()) as i64;
            let res = l.wrapping_add((x as i32) as i64);
            if res > i32::MAX as i64 {
                result.copy_from_slice(&i32::MAX.to_le_bytes());
            } else if res < i32::MIN as i64 {
                result.copy_from_slice(&i32::MIN.to_le_bytes());
            } else {
                let res = res as i32;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let res = l.wrapping_add((x as i64) as i128);
            if res > i64::MAX as i128 {
                result.copy_from_slice(&i64::MAX.to_le_bytes());
            } else if res < i64::MIN as i128 {
                result.copy_from_slice(&i64::MIN.to_le_bytes());
            } else {
                let res = res as i64;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let l = E128::get(lhs);
            let r = E128::from(x as i64);
            let (res, _) = l.saturating_add_s(r);
            res.put(result);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            let (res, _) = l.saturating_add_s(r);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vsadd_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", 
                     "vsadd.vx v24, v8, t0",
                     in (reg) x);
        });
    }

    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_sadd,
        op,
        WideningCategory::None,
        "vsadd.vx",
    );
}

fn expected_op_sadd_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let l = lhs[0] as i16;
            let res = l.wrapping_add((rhs[0] as i8) as i16);
            if res > i8::MAX as i16 {
                result[0] = i8::MAX as u8;
            } else if res < i8::MIN as i16 {
                result[0] = i8::MIN as u8;
            } else {
                result[0] = res as u8;
            }
        }
        2 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap()) as i32;
            let res = l.wrapping_add(i16::from_le_bytes(rhs.try_into().unwrap()) as i32);
            if res > i16::MAX as i32 {
                result.copy_from_slice(&i16::MAX.to_le_bytes());
            } else if res < i16::MIN as i32 {
                result.copy_from_slice(&i16::MIN.to_le_bytes());
            } else {
                let res = res as i16;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        4 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap()) as i64;
            let res = l.wrapping_add(i32::from_le_bytes(rhs.try_into().unwrap()) as i64);
            if res > i32::MAX as i64 {
                result.copy_from_slice(&i32::MAX.to_le_bytes());
            } else if res < i32::MIN as i64 {
                result.copy_from_slice(&i32::MIN.to_le_bytes());
            } else {
                let res = res as i32;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let res = l.wrapping_add(i64::from_le_bytes(rhs.try_into().unwrap()) as i128);
            if res > i64::MAX as i128 {
                result.copy_from_slice(&i64::MAX.to_le_bytes());
            } else if res < i64::MIN as i128 {
                result.copy_from_slice(&i64::MIN.to_le_bytes());
            } else {
                let res = res as i64;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let (res, _) = l.saturating_add_s(r);
            res.put(result);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let (res, _) = l.saturating_add_s(r);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vsadd_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vsadd.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_sadd_vv)),
        op,
        WideningCategory::None,
        "vsadd.vv",
    );
}

fn expected_op_sadd_vi(lhs: &[u8], imm: i64, result: &mut [u8]) {
    expected_op_sadd(lhs, imm as u64, result);
}
fn test_vsadd_vi(sew: u64, lmul: i64, avl: u64, imm: i64) {
    fn op(lhs: &[u8], imm: i64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vi(lhs, imm, result, sew, avl, lmul, |imm| unsafe {
            match imm {
                15 => {
                    rvv_asm!("vsadd.vi v24, v8, 15");
                }
                14 => {
                    rvv_asm!("vsadd.vi v24, v8, 14");
                }
                13 => {
                    rvv_asm!("vsadd.vi v24, v8, 13");
                }
                12 => {
                    rvv_asm!("vsadd.vi v24, v8, 12");
                }
                11 => {
                    rvv_asm!("vsadd.vi v24, v8, 11");
                }
                10 => {
                    rvv_asm!("vsadd.vi v24, v8, 10");
                }
                9 => {
                    rvv_asm!("vsadd.vi v24, v8, 9");
                }
                8 => {
                    rvv_asm!("vsadd.vi v24, v8, 8");
                }
                7 => {
                    rvv_asm!("vsadd.vi v24, v8, 7");
                }
                6 => {
                    rvv_asm!("vsadd.vi v24, v8, 6");
                }
                5 => {
                    rvv_asm!("vsadd.vi v24, v8, 5");
                }
                4 => {
                    rvv_asm!("vsadd.vi v24, v8, 4");
                }
                3 => {
                    rvv_asm!("vsadd.vi v24, v8, 3");
                }
                2 => {
                    rvv_asm!("vsadd.vi v24, v8, 2");
                }
                1 => {
                    rvv_asm!("vsadd.vi v24, v8, 1");
                }
                0 => {
                    rvv_asm!("vsadd.vi v24, v8, 0");
                }
                -1 => {
                    rvv_asm!("vsadd.vi v24, v8, -1");
                }
                -2 => {
                    rvv_asm!("vsadd.vi v24, v8, -2");
                }
                -3 => {
                    rvv_asm!("vsadd.vi v24, v8, -3");
                }
                -4 => {
                    rvv_asm!("vsadd.vi v24, v8, -4");
                }
                -5 => {
                    rvv_asm!("vsadd.vi v24, v8, -5");
                }
                -6 => {
                    rvv_asm!("vsadd.vi v24, v8, -6");
                }
                -7 => {
                    rvv_asm!("vsadd.vi v24, v8, -7");
                }
                -8 => {
                    rvv_asm!("vsadd.vi v24, v8, -8");
                }
                -9 => {
                    rvv_asm!("vsadd.vi v24, v8, -9");
                }
                -10 => {
                    rvv_asm!("vsadd.vi v24, v8, -10");
                }
                -11 => {
                    rvv_asm!("vsadd.vi v24, v8, -11");
                }
                -12 => {
                    rvv_asm!("vsadd.vi v24, v8, -12");
                }
                -13 => {
                    rvv_asm!("vsadd.vi v24, v8, -13");
                }
                -14 => {
                    rvv_asm!("vsadd.vi v24, v8, -14");
                }
                -15 => {
                    rvv_asm!("vsadd.vi v24, v8, -15");
                }
                -16 => {
                    rvv_asm!("vsadd.vi v24, v8, -16");
                }
                _ => {
                    panic!("can't support this immediate: {}", imm);
                }
            }
        });
    }
    run_vop_vi(
        sew,
        lmul,
        avl,
        imm,
        expected_op_sadd_vi,
        op,
        WideningCategory::None,
        "vsadd.vi",
    );
}

fn expected_op_ssubu(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            let l = lhs[0] as u8;
            let r = x as u8;
            let (res, overflow) = l.overflowing_sub(r);
            if overflow {
                result.copy_from_slice(&u8::MIN.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        2 => {
            let (res, overflow) =
                u16::from_le_bytes(lhs.try_into().unwrap()).overflowing_sub(x as u16);
            if overflow {
                result.copy_from_slice(&u16::MIN.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        4 => {
            let (res, overflow) =
                u32::from_le_bytes(lhs.try_into().unwrap()).overflowing_sub(x as u32);
            if overflow {
                result.copy_from_slice(&u32::MIN.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        8 => {
            let (res, overflow) =
                u64::from_le_bytes(lhs.try_into().unwrap()).overflowing_sub(x as u64);
            if overflow {
                result.copy_from_slice(&u64::MIN.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let (res, overflow) =
                u128::from_le_bytes(lhs.try_into().unwrap()).overflowing_sub(x as u128);
            if overflow {
                result.copy_from_slice(&u128::MIN.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            let (res, overflow) = E256::get(lhs).overflowing_sub_u(E256::from(x));
            if overflow {
                E256::MIN_U.put(result);
            } else {
                res.put(result);
            }
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vssubu_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", 
                     "vssubu.vx v24, v8, t0",
                     in (reg) x);
        });
    }

    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_ssubu,
        op,
        WideningCategory::None,
        "vssubu.vx",
    );
}

fn expected_op_ssubu_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let l = lhs[0] as u8;
            let r = rhs[0] as u8;
            let (res, overflow) = l.overflowing_sub(r);
            if overflow {
                result.copy_from_slice(&u8::MIN.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        2 => {
            let (res, overflow) = u16::from_le_bytes(lhs.try_into().unwrap())
                .overflowing_sub(u16::from_le_bytes(rhs.try_into().unwrap()));
            if overflow {
                result.copy_from_slice(&u16::MIN.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        4 => {
            let (res, overflow) = u32::from_le_bytes(lhs.try_into().unwrap())
                .overflowing_sub(u32::from_le_bytes(rhs.try_into().unwrap()));
            if overflow {
                result.copy_from_slice(&u32::MIN.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        8 => {
            let (res, overflow) = u64::from_le_bytes(lhs.try_into().unwrap())
                .overflowing_sub(u64::from_le_bytes(rhs.try_into().unwrap()));
            if overflow {
                result.copy_from_slice(&u64::MIN.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let (res, overflow) = u128::from_le_bytes(lhs.try_into().unwrap())
                .overflowing_sub(u128::from_le_bytes(rhs.try_into().unwrap()));
            if overflow {
                result.copy_from_slice(&u128::MIN.to_le_bytes());
            } else {
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        32 => {
            let (res, overflow) = E256::get(lhs).overflowing_sub_u(E256::get(rhs));
            if overflow {
                E256::MIN_U.put(result);
            } else {
                res.put(result);
            }
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vssubu_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vssubu.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_ssubu_vv)),
        op,
        WideningCategory::None,
        "vssubu.vv",
    );
}

fn expected_op_ssub(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            let l = lhs[0] as i16;
            let res = l.wrapping_sub((x as i8) as i16);
            if res > i8::MAX as i16 {
                result[0] = i8::MAX as u8;
            } else if res < i8::MIN as i16 {
                result[0] = i8::MIN as u8;
            } else {
                result[0] = res as u8;
            }
        }
        2 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap()) as i32;
            let res = l.wrapping_sub((x as i16) as i32);
            if res > i16::MAX as i32 {
                result.copy_from_slice(&i16::MAX.to_le_bytes());
            } else if res < i16::MIN as i32 {
                result.copy_from_slice(&i16::MIN.to_le_bytes());
            } else {
                let res = res as i16;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        4 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap()) as i64;
            let res = l.wrapping_sub((x as i32) as i64);
            if res > i32::MAX as i64 {
                result.copy_from_slice(&i32::MAX.to_le_bytes());
            } else if res < i32::MIN as i64 {
                result.copy_from_slice(&i32::MIN.to_le_bytes());
            } else {
                let res = res as i32;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let res = l.wrapping_sub((x as i64) as i128);
            if res > i64::MAX as i128 {
                result.copy_from_slice(&i64::MAX.to_le_bytes());
            } else if res < i64::MIN as i128 {
                result.copy_from_slice(&i64::MIN.to_le_bytes());
            } else {
                let res = res as i64;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let l = E128::get(lhs);
            let r = E128::from(x as i64);
            let (res, _) = l.saturating_sub_s(r);
            res.put(result);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            let (res, _) = l.saturating_sub_s(r);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vssub_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", 
                     "vssub.vx v24, v8, t0",
                     in (reg) x);
        });
    }

    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_ssub,
        op,
        WideningCategory::None,
        "vssub.vx",
    );
}

fn expected_op_ssub_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let l = lhs[0] as i16;
            let res = l.wrapping_sub((rhs[0] as i8) as i16);
            if res > i8::MAX as i16 {
                result[0] = i8::MAX as u8;
            } else if res < i8::MIN as i16 {
                result[0] = i8::MIN as u8;
            } else {
                result[0] = res as u8;
            }
        }
        2 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap()) as i32;
            let res = l.wrapping_sub(i16::from_le_bytes(rhs.try_into().unwrap()) as i32);
            if res > i16::MAX as i32 {
                result.copy_from_slice(&i16::MAX.to_le_bytes());
            } else if res < i16::MIN as i32 {
                result.copy_from_slice(&i16::MIN.to_le_bytes());
            } else {
                let res = res as i16;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        4 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap()) as i64;
            let res = l.wrapping_sub(i32::from_le_bytes(rhs.try_into().unwrap()) as i64);
            if res > i32::MAX as i64 {
                result.copy_from_slice(&i32::MAX.to_le_bytes());
            } else if res < i32::MIN as i64 {
                result.copy_from_slice(&i32::MIN.to_le_bytes());
            } else {
                let res = res as i32;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let res = l.wrapping_sub(i64::from_le_bytes(rhs.try_into().unwrap()) as i128);
            if res > i64::MAX as i128 {
                result.copy_from_slice(&i64::MAX.to_le_bytes());
            } else if res < i64::MIN as i128 {
                result.copy_from_slice(&i64::MIN.to_le_bytes());
            } else {
                let res = res as i64;
                result.copy_from_slice(&res.to_le_bytes());
            }
        }
        16 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let (res, _) = l.saturating_sub_s(r);
            res.put(result);
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let (res, _) = l.saturating_sub_s(r);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vssub_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vssub.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_ssub_vv)),
        op,
        WideningCategory::None,
        "vssub.vv",
    );
}

pub fn test_single_saturating_add_subtract() {
    let mut imm = -16;
    for sew in [8, 16, 32, 64, 256] {
        for lmul in [-2, 1, 4, 8] {
            for avl in avl_iterator(sew, 4) {
                test_vsaddu_vx(sew, lmul, avl);
                test_vsaddu_vv(sew, lmul, avl);
                test_vsaddu_vi(sew, lmul, avl, imm);

                test_vsadd_vx(sew, lmul, avl);
                test_vsadd_vv(sew, lmul, avl);
                test_vsadd_vi(sew, lmul, avl, imm);

                test_vssubu_vx(sew, lmul, avl);
                test_vssubu_vv(sew, lmul, avl);

                test_vssub_vx(sew, lmul, avl);
                test_vssub_vv(sew, lmul, avl);

                imm += 1;
                if imm > 15 {
                    imm = -16;
                }
            }
        }
    }
}
