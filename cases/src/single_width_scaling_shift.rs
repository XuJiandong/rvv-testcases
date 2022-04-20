use alloc::boxed::Box;
use core::{arch::asm, convert::TryInto};
use eint::{Eint, E256};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    intrinsic::{vop_vv, vop_vx},
    misc::{avl_iterator, shrink_to_imm},
    runner::{run_vop_vv, run_vop_vx, ExpectedOp, WideningCategory},
};

fn expected_op_vssrl_vx(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            let res = lhs[0].wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        2 => {
            let res = u16::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        4 => {
            let res = u32::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        8 => {
            let res = u64::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let res = u128::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
            let r = l.wrapping_shr(x as u32);
            r.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vssrl_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", 
                     "vssrl.vx v24, v8, t0",
                     in (reg) x);
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_vssrl_vx,
        op,
        WideningCategory::None,
        "vssrl.vx",
    );
}

fn expected_op_ssrl_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let res = lhs[0].wrapping_shr(rhs[0] as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        2 => {
            let x = u16::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = u16::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x);
            result.copy_from_slice(&res.to_le_bytes());
        }
        4 => {
            let x = u32::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = u32::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x);
            result.copy_from_slice(&res.to_le_bytes());
        }
        8 => {
            let x = u64::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = u64::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let x = u128::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = u128::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let x = E256::get(rhs).u32();
            let l = E256::get(lhs);
            let r = l.wrapping_shr(x);
            r.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vssrl_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vssrl.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_ssrl_vv)),
        op,
        WideningCategory::None,
        "vssrl.vv",
    );
}

fn expected_op_vssrl_vi(lhs: &[u8], x: u64, result: &mut [u8]) {
    let mut imm = shrink_to_imm(x);
    if imm < 0 {
        imm = 0;
    }
    expected_op_vssrl_vx(lhs, imm as u64, result);
}
fn test_vssrl_vi(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            let mut imm = shrink_to_imm(x);
            if imm < 0 {
                imm = 0;
            }
            match imm {
                15 => {
                    rvv_asm!("vssrl.vi v24, v8, 15");
                }
                14 => {
                    rvv_asm!("vssrl.vi v24, v8, 14");
                }
                13 => {
                    rvv_asm!("vssrl.vi v24, v8, 13");
                }
                12 => {
                    rvv_asm!("vssrl.vi v24, v8, 12");
                }
                11 => {
                    rvv_asm!("vssrl.vi v24, v8, 11");
                }
                10 => {
                    rvv_asm!("vssrl.vi v24, v8, 10");
                }
                9 => {
                    rvv_asm!("vssrl.vi v24, v8, 9");
                }
                8 => {
                    rvv_asm!("vssrl.vi v24, v8, 8");
                }
                7 => {
                    rvv_asm!("vssrl.vi v24, v8, 7");
                }
                6 => {
                    rvv_asm!("vssrl.vi v24, v8, 6");
                }
                5 => {
                    rvv_asm!("vssrl.vi v24, v8, 5");
                }
                4 => {
                    rvv_asm!("vssrl.vi v24, v8, 4");
                }
                3 => {
                    rvv_asm!("vssrl.vi v24, v8, 3");
                }
                2 => {
                    rvv_asm!("vssrl.vi v24, v8, 2");
                }
                1 => {
                    rvv_asm!("vssrl.vi v24, v8, 1");
                }
                0 => {
                    rvv_asm!("vssrl.vi v24, v8, 0");
                }
                _ => {
                    panic!("can't support this immediate: {}", imm);
                }
            }
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_vssrl_vi,
        op,
        WideningCategory::None,
        "vssrl.vi",
    );
}

fn expected_op_vssra_vx(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            let res = (lhs[0] as i8).wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        2 => {
            let res = i16::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        4 => {
            let res = i32::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        8 => {
            let res = i64::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let res = i128::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
            let r = l.wrapping_sra(x as u32);
            r.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vssra_vx(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", 
                     "vssra.vx v24, v8, t0",
                     in (reg) x);
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_vssra_vx,
        op,
        WideningCategory::None,
        "vssra.vx",
    );
}

fn expected_op_ssra_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            let res = (lhs[0] as i8).wrapping_shr(rhs[0] as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        2 => {
            let x = u16::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = i16::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x);
            result.copy_from_slice(&res.to_le_bytes());
        }
        4 => {
            let x = u32::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = i32::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x);
            result.copy_from_slice(&res.to_le_bytes());
        }
        8 => {
            let x = u64::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = i64::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let x = u128::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = i128::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let x = E256::get(rhs).u32();
            let l = E256::get(lhs);
            let r = l.wrapping_sra(x);
            r.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vssra_vv(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vssra.vv v24, v8, v16");
        });
    }
    run_vop_vv(
        sew,
        lmul,
        avl,
        ExpectedOp::Normal(Box::new(expected_op_ssra_vv)),
        op,
        WideningCategory::None,
        "vssra.vv",
    );
}

fn expected_op_vssra_vi(lhs: &[u8], x: u64, result: &mut [u8]) {
    let mut imm = shrink_to_imm(x);
    if imm < 0 {
        imm = 0;
    }
    expected_op_vssra_vx(lhs, imm as u64, result);
}
fn test_vssra_vi(sew: u64, lmul: i64, avl: u64) {
    fn op(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            let mut imm = shrink_to_imm(x);
            if imm < 0 {
                imm = 0;
            }
            match imm {
                15 => {
                    rvv_asm!("vssra.vi v24, v8, 15");
                }
                14 => {
                    rvv_asm!("vssra.vi v24, v8, 14");
                }
                13 => {
                    rvv_asm!("vssra.vi v24, v8, 13");
                }
                12 => {
                    rvv_asm!("vssra.vi v24, v8, 12");
                }
                11 => {
                    rvv_asm!("vssra.vi v24, v8, 11");
                }
                10 => {
                    rvv_asm!("vssra.vi v24, v8, 10");
                }
                9 => {
                    rvv_asm!("vssra.vi v24, v8, 9");
                }
                8 => {
                    rvv_asm!("vssra.vi v24, v8, 8");
                }
                7 => {
                    rvv_asm!("vssra.vi v24, v8, 7");
                }
                6 => {
                    rvv_asm!("vssra.vi v24, v8, 6");
                }
                5 => {
                    rvv_asm!("vssra.vi v24, v8, 5");
                }
                4 => {
                    rvv_asm!("vssra.vi v24, v8, 4");
                }
                3 => {
                    rvv_asm!("vssra.vi v24, v8, 3");
                }
                2 => {
                    rvv_asm!("vssra.vi v24, v8, 2");
                }
                1 => {
                    rvv_asm!("vssra.vi v24, v8, 1");
                }
                0 => {
                    rvv_asm!("vssra.vi v24, v8, 0");
                }
                _ => {
                    panic!("can't support this immediate: {}", imm);
                }
            }
        });
    }
    run_vop_vx(
        sew,
        lmul,
        avl,
        expected_op_vssra_vi,
        op,
        WideningCategory::None,
        "vssra.vi",
    );
}

pub fn test_single_with_scaling_shift() {
    for sew in [8, 16, 32, 64, 128, 256] {
        for lmul in [-2, 1, 4, 8] {
            for avl in avl_iterator(sew, 4) {
                test_vssrl_vx(sew, lmul, avl);
                test_vssrl_vv(sew, lmul, avl);
                test_vssrl_vi(sew, lmul, avl);

                test_vssra_vx(sew, lmul, avl);
                test_vssra_vv(sew, lmul, avl);
                test_vssra_vi(sew, lmul, avl);
            }
        }
    }
}
