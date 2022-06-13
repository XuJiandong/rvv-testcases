use core::{arch::asm, convert::TryInto};

use eint::{Eint, E1024, E128, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::{to_i16, to_i32, to_i64, to_i8, to_u16, to_u32, to_u64, to_u8},
    runner::{run_template_v_vi, run_template_v_vv, run_template_v_vx, MaskType},
};

fn expected_op_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = to_i8(lhs);
            let r = to_i8(rhs);

            let res = l.wrapping_shl(r as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = to_i16(lhs);
            let r = to_i16(rhs);

            let res = l.wrapping_shl(r as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = to_i32(lhs);
            let r = to_i32(rhs);

            let res = l.wrapping_shl(r as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = to_i64(lhs);
            let r = to_i64(rhs);

            let res = l.wrapping_shl(r as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let res = l.wrapping_shl(r.u32());
            res.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = l.wrapping_shl(r.u32());
            res.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let res = l.wrapping_shl(r.u32());
            res.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let res = l.wrapping_shl(r.u32());
            res.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vsll_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vsll.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vsll.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_vv, op, true, "vsll.vv");
}

fn expected_op_sra_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = to_i8(lhs);
            let r = to_i8(rhs);

            let res = l.wrapping_shr(r as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = to_i16(lhs);
            let r = to_i16(rhs);

            let res = l.wrapping_shr(r as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = to_i32(lhs);
            let r = to_i32(rhs);

            let res = l.wrapping_shr(r as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = to_i64(lhs);
            let r = to_i64(rhs);

            let res = l.wrapping_shr(r as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let res = l.wrapping_sra(r.u32());
            res.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = l.wrapping_sra(r.u32());
            res.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let res = l.wrapping_sra(r.u32());
            res.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let res = l.wrapping_sra(r.u32());
            res.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vsra_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vsra.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vsra.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_sra_vv, op, true, "vsra.vv");
}

fn expected_op_srl_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = to_u8(lhs);
            let r = to_u8(rhs);

            let res = l.wrapping_shr(r as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = to_u16(lhs);
            let r = to_u16(rhs);

            let res = l.wrapping_shr(r as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = to_u32(lhs);
            let r = to_u32(rhs);

            let res = l.wrapping_shr(r as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = to_u64(lhs);
            let r = to_u64(rhs);

            let res = l.wrapping_shr(r as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let res = l.wrapping_shr(r.u32());
            res.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let res = l.wrapping_shr(r.u32());
            res.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let res = l.wrapping_shr(r.u32());
            res.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let res = l.wrapping_shr(r.u32());
            res.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vsrl_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vsrl.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vsrl.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_srl_vv, op, true, "vsrl.vv");
}

fn expected_op_vsll_vx(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = to_i8(lhs);
            let res = l.wrapping_shl(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = to_i16(lhs);
            let res = l.wrapping_shl(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = to_i32(lhs);
            let res = l.wrapping_shl(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = to_i64(lhs);
            let res = l.wrapping_shl(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let res = l.wrapping_shl(x as u32);
            res.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let res = l.wrapping_shl(x as u32);
            res.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let res = l.wrapping_shl(x as u32);
            res.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let res = l.wrapping_shl(x as u32);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vsll_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vsll.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vsll.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_vsll_vx, op, true, "vsll.vx");
}
fn test_vsll_vi() {
    fn exp_op(lhs: &[u8], x: i64, result: &mut [u8]) {
        expected_op_vsll_vx(lhs, x as u64, result);
    }
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                17 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 17, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 17");
                    }
                    _ => panic!("Abort"),
                },
                18 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 18, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 18");
                    }
                    _ => panic!("Abort"),
                },
                19 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 19, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 19");
                    }
                    _ => panic!("Abort"),
                },
                20 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 20, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 20");
                    }
                    _ => panic!("Abort"),
                },
                21 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 21, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 21");
                    }
                    _ => panic!("Abort"),
                },
                22 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 22, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 22");
                    }
                    _ => panic!("Abort"),
                },
                23 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 23, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 23");
                    }
                    _ => panic!("Abort"),
                },
                24 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 24, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 24");
                    }
                    _ => panic!("Abort"),
                },
                25 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 25, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 25");
                    }
                    _ => panic!("Abort"),
                },
                26 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 26, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 26");
                    }
                    _ => panic!("Abort"),
                },
                27 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 27, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 27");
                    }
                    _ => panic!("Abort"),
                },
                28 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 28, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 28");
                    }
                    _ => panic!("Abort"),
                },
                29 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 29, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 29");
                    }
                    _ => panic!("Abort"),
                },
                30 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 30, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 30");
                    }
                    _ => panic!("Abort"),
                },
                31 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsll.vi v24, v8, 31, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsll.vi v24, v8, 31");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_vi(exp_op, op, true, false, "vsll.vi");
}

fn expected_op_vsrl_vx(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = to_u8(lhs);
            let res = l.wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = to_u16(lhs);
            let res = l.wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = to_u32(lhs);
            let res = l.wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = to_u64(lhs);
            let res = l.wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let res = l.wrapping_shr(x as u32);
            res.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let res = l.wrapping_shr(x as u32);
            res.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let res = l.wrapping_shr(x as u32);
            res.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let res = l.wrapping_shr(x as u32);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vsrl_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vsrl.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vsrl.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_vsrl_vx, op, true, "vsrl.vx");
}
fn test_vsrl_vi() {
    fn exp_op(lhs: &[u8], x: i64, result: &mut [u8]) {
        expected_op_vsrl_vx(lhs, x as u64, result);
    }
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                17 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 17, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 17");
                    }
                    _ => panic!("Abort"),
                },
                18 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 18, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 18");
                    }
                    _ => panic!("Abort"),
                },
                19 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 19, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 19");
                    }
                    _ => panic!("Abort"),
                },
                20 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 20, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 20");
                    }
                    _ => panic!("Abort"),
                },
                21 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 21, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 21");
                    }
                    _ => panic!("Abort"),
                },
                22 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 22, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 22");
                    }
                    _ => panic!("Abort"),
                },
                23 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 23, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 23");
                    }
                    _ => panic!("Abort"),
                },
                24 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 24, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 24");
                    }
                    _ => panic!("Abort"),
                },
                25 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 25, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 25");
                    }
                    _ => panic!("Abort"),
                },
                26 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 26, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 26");
                    }
                    _ => panic!("Abort"),
                },
                27 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 27, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 27");
                    }
                    _ => panic!("Abort"),
                },
                28 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 28, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 28");
                    }
                    _ => panic!("Abort"),
                },
                29 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 29, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 29");
                    }
                    _ => panic!("Abort"),
                },
                30 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 30, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 30");
                    }
                    _ => panic!("Abort"),
                },
                31 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsrl.vi v24, v8, 31, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsrl.vi v24, v8, 31");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }
    run_template_v_vi(exp_op, op, true, false, "vsrl.vi");
}

fn expected_op_vsra_vx(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = to_i8(lhs);
            let res = l.wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = to_i16(lhs);
            let res = l.wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = to_i32(lhs);
            let res = l.wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = to_i64(lhs);
            let res = l.wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let res = l.wrapping_sra(x as u32);
            res.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let res = l.wrapping_sra(x as u32);
            res.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let res = l.wrapping_sra(x as u32);
            res.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let res = l.wrapping_sra(x as u32);
            res.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vsra_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vsra.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vsra.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_vsra_vx, op, true, "vsra.vx");
}
fn test_vsra_vi() {
    fn exp_op(lhs: &[u8], x: i64, result: &mut [u8]) {
        expected_op_vsra_vx(lhs, x as u64, result);
    }
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                17 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 17, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 17");
                    }
                    _ => panic!("Abort"),
                },
                18 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 18, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 18");
                    }
                    _ => panic!("Abort"),
                },
                19 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 19, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 19");
                    }
                    _ => panic!("Abort"),
                },
                20 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 20, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 20");
                    }
                    _ => panic!("Abort"),
                },
                21 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 21, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 21");
                    }
                    _ => panic!("Abort"),
                },
                22 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 22, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 22");
                    }
                    _ => panic!("Abort"),
                },
                23 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 23, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 23");
                    }
                    _ => panic!("Abort"),
                },
                24 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 24, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 24");
                    }
                    _ => panic!("Abort"),
                },
                25 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 25, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 25");
                    }
                    _ => panic!("Abort"),
                },
                26 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 26, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 26");
                    }
                    _ => panic!("Abort"),
                },
                27 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 27, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 27");
                    }
                    _ => panic!("Abort"),
                },
                28 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 28, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 28");
                    }
                    _ => panic!("Abort"),
                },
                29 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 29, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 29");
                    }
                    _ => panic!("Abort"),
                },
                30 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 30, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 30");
                    }
                    _ => panic!("Abort"),
                },
                31 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vsra.vi v24, v8, 31, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vsra.vi v24, v8, 31");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_vi(exp_op, op, true, false, "vsra.vi");
}

pub fn test_single_width_shift() {
    test_vsll_vv();
    test_vsll_vx();
    test_vsll_vi();

    test_vsrl_vv();
    test_vsrl_vx();
    test_vsrl_vi();

    test_vsra_vv();
    test_vsra_vx();
    test_vsra_vi();
}
