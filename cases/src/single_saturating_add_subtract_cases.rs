use core::{arch::asm, convert::TryInto};
use eint::{Eint, E128, E256};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::conver_to_i256,
    runner::{run_template_v_vi, run_template_v_vv, run_template_v_vx, MaskType},
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
fn test_vsaddu_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vsaddu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vsaddu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }

    run_template_v_vx(expected_op_saddu, op, true, "vsaddu.vx");
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
fn test_vsaddu_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vsaddu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vsaddu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }

    run_template_v_vv(expected_op_saddu_vv, op, true, "vsaddu.vv");
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
fn test_vsaddu_vi() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsaddu.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsaddu.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_vi(expected_op_saddu_vi, op, true, true, "vsaddu.vi");
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
fn test_vsadd_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vsadd.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vsadd.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }

    run_template_v_vx(expected_op_sadd, op, true, "vsadd.vx");
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
fn test_vsadd_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vsadd.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vsadd.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_sadd_vv, op, true, "vsadd.vv");
}

fn expected_op_sadd_vi(lhs: &[u8], imm: i64, result: &mut [u8]) {
    expected_op_sadd(lhs, imm as u64, result);
}
fn test_vsadd_vi() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsadd.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsadd.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_vi(expected_op_sadd_vi, op, true, true, "vsadd.vi");
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
fn test_vssubu_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vssubu.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vssubu.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_ssubu, op, true, "vssubu.vx");
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
fn test_vssubu_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vssubu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vssubu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_ssubu_vv, op, true, "vssubu.vv");
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
fn test_vssub_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vssub.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vssub.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_ssub, op, true, "vssub.vx");
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
fn test_vssub_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vssub.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vssub.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_ssub_vv, op, true, "vssub.vv");
}

pub fn test_single_saturating_add_subtract() {
    test_vsaddu_vx();
    test_vsaddu_vv();
    test_vsaddu_vi();

    test_vsadd_vx();
    test_vsadd_vv();
    test_vsadd_vi();

    test_vssubu_vx();
    test_vssubu_vv();

    test_vssub_vx();
    test_vssub_vv();
}
