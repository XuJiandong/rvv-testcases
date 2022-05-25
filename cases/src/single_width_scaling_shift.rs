use core::{arch::asm, convert::TryInto};
use eint::{Eint, E256};
use rvv_asm::rvv_asm;
use rvv_testcases::runner::{run_template_v_vi, run_template_v_vv, run_template_v_vx, MaskType};

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
fn test_vssrl_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vssrl.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vssrl.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_vssrl_vx, op, true, "vssrl.vx");
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
fn test_vssrl_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vssrl.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vssrl.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_ssrl_vv, op, true, "vssrl.vv");
}

fn expected_op_vssrl_vi(lhs: &[u8], imm: i64, result: &mut [u8]) {
    expected_op_vssrl_vx(lhs, imm as u64, result);
}
fn test_vssrl_vi() {
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                31 => {
                    rvv_asm!("vssrl.vi v24, v8, 31");
                }
                30 => {
                    rvv_asm!("vssrl.vi v24, v8, 30");
                }
                29 => {
                    rvv_asm!("vssrl.vi v24, v8, 29");
                }
                28 => {
                    rvv_asm!("vssrl.vi v24, v8, 28");
                }
                27 => {
                    rvv_asm!("vssrl.vi v24, v8, 27");
                }
                26 => {
                    rvv_asm!("vssrl.vi v24, v8, 26");
                }
                25 => {
                    rvv_asm!("vssrl.vi v24, v8, 25");
                }
                24 => {
                    rvv_asm!("vssrl.vi v24, v8, 24");
                }
                23 => {
                    rvv_asm!("vssrl.vi v24, v8, 23");
                }
                22 => {
                    rvv_asm!("vssrl.vi v24, v8, 22");
                }
                21 => {
                    rvv_asm!("vssrl.vi v24, v8, 21");
                }
                20 => {
                    rvv_asm!("vssrl.vi v24, v8, 20");
                }
                19 => {
                    rvv_asm!("vssrl.vi v24, v8, 19");
                }
                18 => {
                    rvv_asm!("vssrl.vi v24, v8, 18");
                }
                17 => {
                    rvv_asm!("vssrl.vi v24, v8, 17");
                }
                16 => {
                    rvv_asm!("vssrl.vi v24, v8, 16");
                }
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
        }
    }
    run_template_v_vi(expected_op_vssrl_vi, op, false, "vssrl.vi");
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
fn test_vssra_vx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vssra.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vssra.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vx(expected_op_vssra_vx, op, true, "vssra.vx");
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
fn test_vssra_vv() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vssra.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vssra.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_ssra_vv, op, true, "vssra.vv");
}

fn expected_op_vssra_vi(lhs: &[u8], imm: i64, result: &mut [u8]) {
    expected_op_vssra_vx(lhs, imm as u64, result);
}
fn test_vssra_vi() {
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                31 => {
                    rvv_asm!("vssra.vi v24, v8, 31");
                }
                30 => {
                    rvv_asm!("vssra.vi v24, v8, 30");
                }
                29 => {
                    rvv_asm!("vssra.vi v24, v8, 29");
                }
                28 => {
                    rvv_asm!("vssra.vi v24, v8, 28");
                }
                27 => {
                    rvv_asm!("vssra.vi v24, v8, 27");
                }
                26 => {
                    rvv_asm!("vssra.vi v24, v8, 26");
                }
                25 => {
                    rvv_asm!("vssra.vi v24, v8, 25");
                }
                24 => {
                    rvv_asm!("vssra.vi v24, v8, 24");
                }
                23 => {
                    rvv_asm!("vssra.vi v24, v8, 23");
                }
                22 => {
                    rvv_asm!("vssra.vi v24, v8, 22");
                }
                21 => {
                    rvv_asm!("vssra.vi v24, v8, 21");
                }
                20 => {
                    rvv_asm!("vssra.vi v24, v8, 20");
                }
                19 => {
                    rvv_asm!("vssra.vi v24, v8, 19");
                }
                18 => {
                    rvv_asm!("vssra.vi v24, v8, 18");
                }
                17 => {
                    rvv_asm!("vssra.vi v24, v8, 17");
                }
                16 => {
                    rvv_asm!("vssra.vi v24, v8, 16");
                }
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
        }
    }
    run_template_v_vi(expected_op_vssra_vi, op, false, "vssra.vi");
}

pub fn test_single_with_scaling_shift() {
    test_vssrl_vx();
    test_vssrl_vv();
    test_vssrl_vi();

    test_vssra_vx();
    test_vssra_vv();
    test_vssra_vi();
}
