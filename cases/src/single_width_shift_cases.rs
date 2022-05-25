use core::{arch::asm, convert::TryInto};

use eint::{Eint, E256, E8};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::{to_i64, to_u64},
    runner::{run_template_v_vi, run_template_v_vv, run_template_v_vx, MaskType},
};

fn expected_op_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());

    match result.len() {
        1 => {
            let l = E8::get(lhs);
            let r = E8::get(rhs);
            let res = l.wrapping_shl(r.u32());
            res.put(result);
        }
        8 => {
            let l = to_i64(lhs);
            let r = to_i64(rhs);

            let res = l.wrapping_shl(r as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
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

    match result.len() {
        8 => {
            let l = to_i64(lhs);
            let r = to_i64(rhs);

            let res = l.wrapping_shr(r as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
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

    match result.len() {
        1 => {
            let l = E8::get(lhs);
            let r = E8::get(rhs);
            let res = l.wrapping_shr(r.u32());
            res.put(result);
        }
        8 => {
            let l = to_u64(lhs);
            let r = to_u64(rhs);

            let res = l.wrapping_shr(r as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
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
    match lhs.len() {
        1 => {
            result[0] = lhs[0] + x as u8;
        }
        8 => {
            let l = to_i64(lhs);
            let res = l.wrapping_shl(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
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
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                0 => {
                    rvv_asm!("vsll.vi v24, v8, 0");
                }
                1 => {
                    rvv_asm!("vsll.vi v24, v8, 1");
                }
                2 => {
                    rvv_asm!("vsll.vi v24, v8, 2");
                }
                3 => {
                    rvv_asm!("vsll.vi v24, v8, 3");
                }
                4 => {
                    rvv_asm!("vsll.vi v24, v8, 4");
                }
                5 => {
                    rvv_asm!("vsll.vi v24, v8, 5");
                }
                6 => {
                    rvv_asm!("vsll.vi v24, v8, 6");
                }
                7 => {
                    rvv_asm!("vsll.vi v24, v8, 7");
                }
                8 => {
                    rvv_asm!("vsll.vi v24, v8, 8");
                }
                9 => {
                    rvv_asm!("vsll.vi v24, v8, 9");
                }
                10 => {
                    rvv_asm!("vsll.vi v24, v8, 10");
                }
                11 => {
                    rvv_asm!("vsll.vi v24, v8, 11");
                }
                12 => {
                    rvv_asm!("vsll.vi v24, v8, 12");
                }
                13 => {
                    rvv_asm!("vsll.vi v24, v8, 13");
                }
                14 => {
                    rvv_asm!("vsll.vi v24, v8, 14");
                }
                15 => {
                    rvv_asm!("vsll.vi v24, v8, 15");
                }
                16 => {
                    rvv_asm!("vsll.vi v24, v8, 16");
                }
                17 => {
                    rvv_asm!("vsll.vi v24, v8, 17");
                }
                18 => {
                    rvv_asm!("vsll.vi v24, v8, 18");
                }
                19 => {
                    rvv_asm!("vsll.vi v24, v8, 19");
                }
                20 => {
                    rvv_asm!("vsll.vi v24, v8, 20");
                }
                21 => {
                    rvv_asm!("vsll.vi v24, v8, 21");
                }
                22 => {
                    rvv_asm!("vsll.vi v24, v8, 22");
                }
                23 => {
                    rvv_asm!("vsll.vi v24, v8, 23");
                }
                24 => {
                    rvv_asm!("vsll.vi v24, v8, 24");
                }
                25 => {
                    rvv_asm!("vsll.vi v24, v8, 25");
                }
                26 => {
                    rvv_asm!("vsll.vi v24, v8, 26");
                }
                27 => {
                    rvv_asm!("vsll.vi v24, v8, 27");
                }
                28 => {
                    rvv_asm!("vsll.vi v24, v8, 28");
                }
                29 => {
                    rvv_asm!("vsll.vi v24, v8, 29");
                }
                30 => {
                    rvv_asm!("vsll.vi v24, v8, 30");
                }
                31 => {
                    rvv_asm!("vsll.vi v24, v8, 31");
                }
                _ => {
                    panic!("Abort");
                }
            }
        }
    }
    run_template_v_vi(exp_op, op, false, "vsll.vi");
}

fn expected_op_vsrl_vx(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        8 => {
            let l = to_u64(lhs);
            let res = l.wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
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
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                0 => {
                    rvv_asm!("vsrl.vi v24, v8, 0");
                }
                1 => {
                    rvv_asm!("vsrl.vi v24, v8, 1");
                }
                2 => {
                    rvv_asm!("vsrl.vi v24, v8, 2");
                }
                3 => {
                    rvv_asm!("vsrl.vi v24, v8, 3");
                }
                4 => {
                    rvv_asm!("vsrl.vi v24, v8, 4");
                }
                5 => {
                    rvv_asm!("vsrl.vi v24, v8, 5");
                }
                6 => {
                    rvv_asm!("vsrl.vi v24, v8, 6");
                }
                7 => {
                    rvv_asm!("vsrl.vi v24, v8, 7");
                }
                8 => {
                    rvv_asm!("vsrl.vi v24, v8, 8");
                }
                9 => {
                    rvv_asm!("vsrl.vi v24, v8, 9");
                }
                10 => {
                    rvv_asm!("vsrl.vi v24, v8, 10");
                }
                11 => {
                    rvv_asm!("vsrl.vi v24, v8, 11");
                }
                12 => {
                    rvv_asm!("vsrl.vi v24, v8, 12");
                }
                13 => {
                    rvv_asm!("vsrl.vi v24, v8, 13");
                }
                14 => {
                    rvv_asm!("vsrl.vi v24, v8, 14");
                }
                15 => {
                    rvv_asm!("vsrl.vi v24, v8, 15");
                }
                16 => {
                    rvv_asm!("vsrl.vi v24, v8, 16");
                }
                17 => {
                    rvv_asm!("vsrl.vi v24, v8, 17");
                }
                18 => {
                    rvv_asm!("vsrl.vi v24, v8, 18");
                }
                19 => {
                    rvv_asm!("vsrl.vi v24, v8, 19");
                }
                20 => {
                    rvv_asm!("vsrl.vi v24, v8, 20");
                }
                21 => {
                    rvv_asm!("vsrl.vi v24, v8, 21");
                }
                22 => {
                    rvv_asm!("vsrl.vi v24, v8, 22");
                }
                23 => {
                    rvv_asm!("vsrl.vi v24, v8, 23");
                }
                24 => {
                    rvv_asm!("vsrl.vi v24, v8, 24");
                }
                25 => {
                    rvv_asm!("vsrl.vi v24, v8, 25");
                }
                26 => {
                    rvv_asm!("vsrl.vi v24, v8, 26");
                }
                27 => {
                    rvv_asm!("vsrl.vi v24, v8, 27");
                }
                28 => {
                    rvv_asm!("vsrl.vi v24, v8, 28");
                }
                29 => {
                    rvv_asm!("vsrl.vi v24, v8, 29");
                }
                30 => {
                    rvv_asm!("vsrl.vi v24, v8, 30");
                }
                31 => {
                    rvv_asm!("vsrl.vi v24, v8, 31");
                }
                _ => {
                    panic!("Abort");
                }
            }
        }
    }
    run_template_v_vi(exp_op, op, false, "vsrl.vi");
}

fn expected_op_vsra_vx(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        8 => {
            let l = to_i64(lhs);
            let res = l.wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = E256::get(lhs);
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
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                0 => {
                    rvv_asm!("vsra.vi v24, v8, 0");
                }
                1 => {
                    rvv_asm!("vsra.vi v24, v8, 1");
                }
                2 => {
                    rvv_asm!("vsra.vi v24, v8, 2");
                }
                3 => {
                    rvv_asm!("vsra.vi v24, v8, 3");
                }
                4 => {
                    rvv_asm!("vsra.vi v24, v8, 4");
                }
                5 => {
                    rvv_asm!("vsra.vi v24, v8, 5");
                }
                6 => {
                    rvv_asm!("vsra.vi v24, v8, 6");
                }
                7 => {
                    rvv_asm!("vsra.vi v24, v8, 7");
                }
                8 => {
                    rvv_asm!("vsra.vi v24, v8, 8");
                }
                9 => {
                    rvv_asm!("vsra.vi v24, v8, 9");
                }
                10 => {
                    rvv_asm!("vsra.vi v24, v8, 10");
                }
                11 => {
                    rvv_asm!("vsra.vi v24, v8, 11");
                }
                12 => {
                    rvv_asm!("vsra.vi v24, v8, 12");
                }
                13 => {
                    rvv_asm!("vsra.vi v24, v8, 13");
                }
                14 => {
                    rvv_asm!("vsra.vi v24, v8, 14");
                }
                15 => {
                    rvv_asm!("vsra.vi v24, v8, 15");
                }
                16 => {
                    rvv_asm!("vsra.vi v24, v8, 16");
                }
                17 => {
                    rvv_asm!("vsra.vi v24, v8, 17");
                }
                18 => {
                    rvv_asm!("vsra.vi v24, v8, 18");
                }
                19 => {
                    rvv_asm!("vsra.vi v24, v8, 19");
                }
                20 => {
                    rvv_asm!("vsra.vi v24, v8, 20");
                }
                21 => {
                    rvv_asm!("vsra.vi v24, v8, 21");
                }
                22 => {
                    rvv_asm!("vsra.vi v24, v8, 22");
                }
                23 => {
                    rvv_asm!("vsra.vi v24, v8, 23");
                }
                24 => {
                    rvv_asm!("vsra.vi v24, v8, 24");
                }
                25 => {
                    rvv_asm!("vsra.vi v24, v8, 25");
                }
                26 => {
                    rvv_asm!("vsra.vi v24, v8, 26");
                }
                27 => {
                    rvv_asm!("vsra.vi v24, v8, 27");
                }
                28 => {
                    rvv_asm!("vsra.vi v24, v8, 28");
                }
                29 => {
                    rvv_asm!("vsra.vi v24, v8, 29");
                }
                30 => {
                    rvv_asm!("vsra.vi v24, v8, 30");
                }
                31 => {
                    rvv_asm!("vsra.vi v24, v8, 31");
                }
                _ => {
                    panic!("Abort");
                }
            }
        }
    }
    run_template_v_vi(exp_op, op, false, "vsra.vi");
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
