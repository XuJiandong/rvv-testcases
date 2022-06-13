use core::{arch::asm, convert::TryInto};
use eint::{Eint, E1024, E128, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::runner::{
    run_template_m_vi, run_template_m_vim, run_template_m_vv, run_template_m_vvm,
    run_template_m_vx, run_template_m_vxm, run_template_v_vim, run_template_v_vvm,
    run_template_v_vxm, MaskType,
};

// use ckb_std::syscalls::debug;
// use rvv_testcases::log;

fn expected_op_adc_vvm(lhs: &[u8], rhs: &[u8], result: &mut [u8], mask: bool) {
    assert_eq!(lhs.len(), rhs.len());
    assert_eq!(rhs.len(), result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());
            let r = i8::from_le_bytes(rhs.try_into().unwrap());

            let (res, _) = l.overflowing_add(r);
            let (res, _) = res.overflowing_add(mask as i8);

            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            let r = i16::from_le_bytes(rhs.try_into().unwrap());

            let (res, _) = l.overflowing_add(r);
            let (res, _) = res.overflowing_add(mask as i16);

            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            let r = i32::from_le_bytes(rhs.try_into().unwrap());

            let (res, _) = l.overflowing_add(r);
            let (res, _) = res.overflowing_add(mask as i32);

            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap());

            let (res, _) = l.overflowing_add(r);
            let (res, _) = res.overflowing_add(mask as i64);

            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let mask = E128::from(mask);
            let (res, _) = l.overflowing_add_u(r);
            let (res, _) = res.overflowing_add_u(mask);
            res.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let mask = E256::from(mask);
            let (res, _) = l.overflowing_add_u(r);
            let (res, _) = res.overflowing_add_u(mask);
            res.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let mask = E512::from(mask);
            let (res, _) = l.overflowing_add_u(r);
            let (res, _) = res.overflowing_add_u(mask);
            res.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let mask = E1024::from(mask);
            let (res, _) = l.overflowing_add_u(r);
            let (res, _) = res.overflowing_add_u(mask);
            res.put(result);
        }
        _ => {
            panic!("Unsupported sew = {}", lhs.len());
        }
    }
}
fn test_vadc_vvm() {
    fn rvv_op(_: &[u8], _: &[u8], _: MaskType) {
        unsafe {
            rvv_asm!("vadc.vvm v24, v8, v16, v0");
        }
    }

    run_template_v_vvm(expected_op_adc_vvm, rvv_op, "vadc.vvm")
}

fn expected_op_adc_vxm(lhs: &[u8], x: u64, result: &mut [u8], mask: bool) {
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());

            let (res, _) = l.overflowing_add(x as i8);
            let (res, _) = res.overflowing_add(mask as i8);

            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());

            let (res, _) = l.overflowing_add(x as i16);
            let (res, _) = res.overflowing_add(mask as i16);

            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());

            let (res, _) = l.overflowing_add(x as i32);
            let (res, _) = res.overflowing_add(mask as i32);

            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());

            let (res, _) = l.overflowing_add(x as i64);
            let (res, _) = res.overflowing_add(mask as i64);

            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::from(x as i64);
            let mask = E128::from(mask);
            let (res, _) = l.overflowing_add_u(r);
            let (res, _) = res.overflowing_add_u(mask);
            res.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            let mask = E256::from(mask);
            let (res, _) = l.overflowing_add_u(r);
            let (res, _) = res.overflowing_add_u(mask);
            res.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::from(x as i64);
            let mask = E512::from(mask);
            let (res, _) = l.overflowing_add_u(r);
            let (res, _) = res.overflowing_add_u(mask);
            res.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::from(x as i64);
            let mask = E1024::from(mask);
            let (res, _) = l.overflowing_add_u(r);
            let (res, _) = res.overflowing_add_u(mask);
            res.put(result);
        }
        _ => {
            panic!("Unsupported");
        }
    }
}
fn test_vadc_vxm() {
    fn rvv_op(_: &[u8], rhs: &[u8], _: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            rvv_asm!("mv t0, {}", "vadc.vxm v24, v8, t0, v0", in (reg) x);
        }
    }
    run_template_v_vxm(expected_op_adc_vxm, rvv_op, "vadc.vxm");
}

fn expected_op_adc_vim(lhs: &[u8], x: i64, result: &mut [u8], mask: bool) {
    expected_op_adc_vxm(lhs, x as u64, result, mask);
}
fn test_vadc_vim() {
    fn rvv_op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => {
                    rvv_asm!("vadc.vim v24, v8, -16, v0");
                }
                -15 => {
                    rvv_asm!("vadc.vim v24, v8, -15, v0");
                }
                -14 => {
                    rvv_asm!("vadc.vim v24, v8, -14, v0");
                }
                -13 => {
                    rvv_asm!("vadc.vim v24, v8, -13, v0");
                }
                -12 => {
                    rvv_asm!("vadc.vim v24, v8, -12, v0");
                }
                -11 => {
                    rvv_asm!("vadc.vim v24, v8, -11, v0");
                }
                -10 => {
                    rvv_asm!("vadc.vim v24, v8, -10, v0");
                }
                -9 => {
                    rvv_asm!("vadc.vim v24, v8, -9, v0");
                }
                -8 => {
                    rvv_asm!("vadc.vim v24, v8, -8, v0");
                }
                -7 => {
                    rvv_asm!("vadc.vim v24, v8, -7, v0");
                }
                -6 => {
                    rvv_asm!("vadc.vim v24, v8, -6, v0");
                }
                -5 => {
                    rvv_asm!("vadc.vim v24, v8, -5, v0");
                }
                -4 => {
                    rvv_asm!("vadc.vim v24, v8, -4, v0");
                }
                -3 => {
                    rvv_asm!("vadc.vim v24, v8, -3, v0");
                }
                -2 => {
                    rvv_asm!("vadc.vim v24, v8, -2, v0");
                }
                -1 => {
                    rvv_asm!("vadc.vim v24, v8, -1, v0");
                }
                0 => {
                    rvv_asm!("vadc.vim v24, v8, 0, v0");
                }
                1 => {
                    rvv_asm!("vadc.vim v24, v8, 1, v0");
                }
                2 => {
                    rvv_asm!("vadc.vim v24, v8, 2, v0");
                }
                3 => {
                    rvv_asm!("vadc.vim v24, v8, 3, v0");
                }
                4 => {
                    rvv_asm!("vadc.vim v24, v8, 4, v0");
                }
                5 => {
                    rvv_asm!("vadc.vim v24, v8, 5, v0");
                }
                6 => {
                    rvv_asm!("vadc.vim v24, v8, 6, v0");
                }
                7 => {
                    rvv_asm!("vadc.vim v24, v8, 7, v0");
                }
                8 => {
                    rvv_asm!("vadc.vim v24, v8, 8, v0");
                }
                9 => {
                    rvv_asm!("vadc.vim v24, v8, 9, v0");
                }
                10 => {
                    rvv_asm!("vadc.vim v24, v8, 10, v0");
                }
                11 => {
                    rvv_asm!("vadc.vim v24, v8, 11, v0");
                }
                12 => {
                    rvv_asm!("vadc.vim v24, v8, 12, v0");
                }
                13 => {
                    rvv_asm!("vadc.vim v24, v8, 13, v0");
                }
                14 => {
                    rvv_asm!("vadc.vim v24, v8, 14, v0");
                }
                15 => {
                    rvv_asm!("vadc.vim v24, v8, 15, v0");
                }
                _ => {
                    panic!("Abort");
                }
            }
        }
    }
    run_template_v_vim(expected_op_adc_vim, rvv_op, "vadc.vim");
}

fn expected_op_madc_vvm(lhs: &[u8], rhs: &[u8], result: &mut bool, mask: bool) {
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());
            let r = u8::from_le_bytes(rhs.try_into().unwrap());

            let (res, carry1) = l.overflowing_add(r);
            let (_, carry2) = res.overflowing_add(mask as u8);

            *result = carry1 | carry2;
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            let r = u16::from_le_bytes(rhs.try_into().unwrap());

            let (res, carry1) = l.overflowing_add(r);
            let (_, carry2) = res.overflowing_add(mask as u16);

            *result = carry1 | carry2;
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            let r = u32::from_le_bytes(rhs.try_into().unwrap());

            let (res, carry1) = l.overflowing_add(r);
            let (_, carry2) = res.overflowing_add(mask as u32);

            *result = carry1 | carry2;
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());

            let (res, carry1) = l.overflowing_add(r);
            let (_, carry2) = res.overflowing_add(mask as u64);

            *result = carry1 | carry2;
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let mask = E128::from(mask);
            let (res, carry1) = l.overflowing_add_u(r);
            let (_, carry2) = res.overflowing_add_u(mask);

            *result = carry1 | carry2;
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let mask = E256::from(mask);
            let (res, carry1) = l.overflowing_add_u(r);
            let (_, carry2) = res.overflowing_add_u(mask);

            *result = carry1 | carry2;
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let mask = E512::from(mask);
            let (res, carry1) = l.overflowing_add_u(r);
            let (_, carry2) = res.overflowing_add_u(mask);

            *result = carry1 | carry2;
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let mask = E1024::from(mask);
            let (res, carry1) = l.overflowing_add_u(r);
            let (_, carry2) = res.overflowing_add_u(mask);

            *result = carry1 | carry2;
        }
        _ => {
            panic!("Unsupported sew = {}", lhs.len());
        }
    }
}
fn test_vmadc_vvm() {
    fn rvv_op(_: &[u8], _: &[u8], _: MaskType) {
        unsafe {
            rvv_asm!("vmadc.vvm v24, v8, v16, v0");
        }
    }

    run_template_m_vvm(expected_op_madc_vvm, rvv_op, "vmadc.vvm")
}

fn expected_op_madc_vxm(lhs: &[u8], x: u64, result: &mut bool, mask: bool) {
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());

            let (res, carry1) = l.overflowing_add(x as u8);
            let (_, carry2) = res.overflowing_add(mask as u8);

            *result = carry1 | carry2;
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());

            let (res, carry1) = l.overflowing_add(x as u16);
            let (_, carry2) = res.overflowing_add(mask as u16);

            *result = carry1 | carry2;
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());

            let (res, carry1) = l.overflowing_add(x as u32);
            let (_, carry2) = res.overflowing_add(mask as u32);

            *result = carry1 | carry2;
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());

            let (res, carry1) = l.overflowing_add(x as u64);
            let (_, carry2) = res.overflowing_add(mask as u64);

            *result = carry1 | carry2;
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::from(x as i64);
            let mask = E128::from(mask);
            let (res, carry1) = l.overflowing_add_u(r);
            let (_, carry2) = res.overflowing_add_u(mask);

            *result = carry1 | carry2;
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            let mask = E256::from(mask);
            let (res, carry1) = l.overflowing_add_u(r);
            let (_, carry2) = res.overflowing_add_u(mask);

            *result = carry1 | carry2;
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::from(x as i64);
            let mask = E512::from(mask);
            let (res, carry1) = l.overflowing_add_u(r);
            let (_, carry2) = res.overflowing_add_u(mask);

            *result = carry1 | carry2;
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::from(x as i64);
            let mask = E1024::from(mask);
            let (res, carry1) = l.overflowing_add_u(r);
            let (_, carry2) = res.overflowing_add_u(mask);

            *result = carry1 | carry2;
        }
        _ => {
            panic!("Unsupported sew = {}", lhs.len());
        }
    }
}
fn test_vmadc_vxm() {
    fn rvv_op(_: &[u8], rhs: &[u8], _: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            rvv_asm!("mv t0, {}", "vmadc.vxm v24, v8, t0, v0", in (reg) x);
        }
    }

    run_template_m_vxm(expected_op_madc_vxm, rvv_op, "vmadc.vxm")
}

fn expected_op_madc_vim(lhs: &[u8], x: i64, result: &mut bool, mask: bool) {
    expected_op_madc_vxm(lhs, x as u64, result, mask);
}
fn test_vmadc_vim() {
    fn rvv_op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => {
                    rvv_asm!("vmadc.vim v24, v8, -16, v0");
                }
                -15 => {
                    rvv_asm!("vmadc.vim v24, v8, -15, v0");
                }
                -14 => {
                    rvv_asm!("vmadc.vim v24, v8, -14, v0");
                }
                -13 => {
                    rvv_asm!("vmadc.vim v24, v8, -13, v0");
                }
                -12 => {
                    rvv_asm!("vmadc.vim v24, v8, -12, v0");
                }
                -11 => {
                    rvv_asm!("vmadc.vim v24, v8, -11, v0");
                }
                -10 => {
                    rvv_asm!("vmadc.vim v24, v8, -10, v0");
                }
                -9 => {
                    rvv_asm!("vmadc.vim v24, v8, -9, v0");
                }
                -8 => {
                    rvv_asm!("vmadc.vim v24, v8, -8, v0");
                }
                -7 => {
                    rvv_asm!("vmadc.vim v24, v8, -7, v0");
                }
                -6 => {
                    rvv_asm!("vmadc.vim v24, v8, -6, v0");
                }
                -5 => {
                    rvv_asm!("vmadc.vim v24, v8, -5, v0");
                }
                -4 => {
                    rvv_asm!("vmadc.vim v24, v8, -4, v0");
                }
                -3 => {
                    rvv_asm!("vmadc.vim v24, v8, -3, v0");
                }
                -2 => {
                    rvv_asm!("vmadc.vim v24, v8, -2, v0");
                }
                -1 => {
                    rvv_asm!("vmadc.vim v24, v8, -1, v0");
                }
                0 => {
                    rvv_asm!("vmadc.vim v24, v8, 0, v0");
                }
                1 => {
                    rvv_asm!("vmadc.vim v24, v8, 1, v0");
                }
                2 => {
                    rvv_asm!("vmadc.vim v24, v8, 2, v0");
                }
                3 => {
                    rvv_asm!("vmadc.vim v24, v8, 3, v0");
                }
                4 => {
                    rvv_asm!("vmadc.vim v24, v8, 4, v0");
                }
                5 => {
                    rvv_asm!("vmadc.vim v24, v8, 5, v0");
                }
                6 => {
                    rvv_asm!("vmadc.vim v24, v8, 6, v0");
                }
                7 => {
                    rvv_asm!("vmadc.vim v24, v8, 7, v0");
                }
                8 => {
                    rvv_asm!("vmadc.vim v24, v8, 8, v0");
                }
                9 => {
                    rvv_asm!("vmadc.vim v24, v8, 9, v0");
                }
                10 => {
                    rvv_asm!("vmadc.vim v24, v8, 10, v0");
                }
                11 => {
                    rvv_asm!("vmadc.vim v24, v8, 11, v0");
                }
                12 => {
                    rvv_asm!("vmadc.vim v24, v8, 12, v0");
                }
                13 => {
                    rvv_asm!("vmadc.vim v24, v8, 13, v0");
                }
                14 => {
                    rvv_asm!("vmadc.vim v24, v8, 14, v0");
                }
                15 => {
                    rvv_asm!("vmadc.vim v24, v8, 15, v0");
                }
                _ => {
                    panic!("Abort");
                }
            }
        }
    }
    run_template_m_vim(expected_op_madc_vim, rvv_op, "vmadc.vim");
}

fn test_vmadc_vv() {
    fn exp_op(lhs: &[u8], rhs: &[u8], result: &mut bool) {
        expected_op_madc_vvm(lhs, rhs, result, false);
    }
    fn rvv_op(_: &[u8], _: &[u8], _: MaskType) {
        unsafe {
            rvv_asm!("vmadc.vv v24, v8, v16");
        }
    }

    run_template_m_vv(exp_op, rvv_op, false, "vmadc.vv")
}

fn test_vmadc_vx() {
    fn exp_op(lhs: &[u8], rhs: u64, result: &mut bool) {
        expected_op_madc_vxm(lhs, rhs, result, false);
    }
    fn rvv_op(_: &[u8], rhs: &[u8], _: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            rvv_asm!("mv t0, {}", "vmadc.vx v24, v8, t0", in (reg) x);
        }
    }

    run_template_m_vx(exp_op, rvv_op, false, "vmadc.vx")
}

fn test_vmadc_vi() {
    fn exp_op(lhs: &[u8], rhs: i64, result: &mut bool) {
        expected_op_madc_vxm(lhs, rhs as u64, result, false);
    }
    fn rvv_op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => {
                    rvv_asm!("vmadc.vi v24, v8, -16");
                }
                -15 => {
                    rvv_asm!("vmadc.vi v24, v8, -15");
                }
                -14 => {
                    rvv_asm!("vmadc.vi v24, v8, -14");
                }
                -13 => {
                    rvv_asm!("vmadc.vi v24, v8, -13");
                }
                -12 => {
                    rvv_asm!("vmadc.vi v24, v8, -12");
                }
                -11 => {
                    rvv_asm!("vmadc.vi v24, v8, -11");
                }
                -10 => {
                    rvv_asm!("vmadc.vi v24, v8, -10");
                }
                -9 => {
                    rvv_asm!("vmadc.vi v24, v8, -9");
                }
                -8 => {
                    rvv_asm!("vmadc.vi v24, v8, -8");
                }
                -7 => {
                    rvv_asm!("vmadc.vi v24, v8, -7");
                }
                -6 => {
                    rvv_asm!("vmadc.vi v24, v8, -6");
                }
                -5 => {
                    rvv_asm!("vmadc.vi v24, v8, -5");
                }
                -4 => {
                    rvv_asm!("vmadc.vi v24, v8, -4");
                }
                -3 => {
                    rvv_asm!("vmadc.vi v24, v8, -3");
                }
                -2 => {
                    rvv_asm!("vmadc.vi v24, v8, -2");
                }
                -1 => {
                    rvv_asm!("vmadc.vi v24, v8, -1");
                }
                0 => {
                    rvv_asm!("vmadc.vi v24, v8, 0");
                }
                1 => {
                    rvv_asm!("vmadc.vi v24, v8, 1");
                }
                2 => {
                    rvv_asm!("vmadc.vi v24, v8, 2");
                }
                3 => {
                    rvv_asm!("vmadc.vi v24, v8, 3");
                }
                4 => {
                    rvv_asm!("vmadc.vi v24, v8, 4");
                }
                5 => {
                    rvv_asm!("vmadc.vi v24, v8, 5");
                }
                6 => {
                    rvv_asm!("vmadc.vi v24, v8, 6");
                }
                7 => {
                    rvv_asm!("vmadc.vi v24, v8, 7");
                }
                8 => {
                    rvv_asm!("vmadc.vi v24, v8, 8");
                }
                9 => {
                    rvv_asm!("vmadc.vi v24, v8, 9");
                }
                10 => {
                    rvv_asm!("vmadc.vi v24, v8, 10");
                }
                11 => {
                    rvv_asm!("vmadc.vi v24, v8, 11");
                }
                12 => {
                    rvv_asm!("vmadc.vi v24, v8, 12");
                }
                13 => {
                    rvv_asm!("vmadc.vi v24, v8, 13");
                }
                14 => {
                    rvv_asm!("vmadc.vi v24, v8, 14");
                }
                15 => {
                    rvv_asm!("vmadc.vi v24, v8, 15");
                }
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_m_vi(exp_op, rvv_op, false, "vmadc.vi")
}

fn expected_op_sbc_vvm(lhs: &[u8], rhs: &[u8], result: &mut [u8], mask: bool) {
    assert_eq!(lhs.len(), rhs.len());
    assert_eq!(rhs.len(), result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());
            let r = i8::from_le_bytes(rhs.try_into().unwrap());

            let (res, _) = l.overflowing_sub(r);
            let (res, _) = res.overflowing_sub(mask as i8);

            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            let r = i16::from_le_bytes(rhs.try_into().unwrap());

            let (res, _) = l.overflowing_sub(r);
            let (res, _) = res.overflowing_sub(mask as i16);

            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            let r = i32::from_le_bytes(rhs.try_into().unwrap());

            let (res, _) = l.overflowing_sub(r);
            let (res, _) = res.overflowing_sub(mask as i32);

            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap());

            let (res, _) = l.overflowing_sub(r);
            let (res, _) = res.overflowing_sub(mask as i64);

            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let mask = E128::from(mask);
            let (res, _) = l.overflowing_sub_u(r);
            let (res, _) = res.overflowing_sub_u(mask);
            res.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let mask = E256::from(mask);
            let (res, _) = l.overflowing_sub_u(r);
            let (res, _) = res.overflowing_sub_u(mask);
            res.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let mask = E512::from(mask);
            let (res, _) = l.overflowing_sub_u(r);
            let (res, _) = res.overflowing_sub_u(mask);
            res.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let mask = E1024::from(mask);
            let (res, _) = l.overflowing_sub_u(r);
            let (res, _) = res.overflowing_sub_u(mask);
            res.put(result);
        }
        _ => {
            panic!("Unsupported sew = {}", lhs.len());
        }
    }
}
fn test_vsbc_vvm() {
    fn rvv_op(_: &[u8], _: &[u8], _: MaskType) {
        unsafe {
            rvv_asm!("vsbc.vvm v24, v8, v16, v0");
        }
    }

    run_template_v_vvm(expected_op_sbc_vvm, rvv_op, "vsbc.vvm")
}

fn expected_op_sbc_vxm(lhs: &[u8], x: u64, result: &mut [u8], mask: bool) {
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());

            let (res, _) = l.overflowing_sub(x as i8);
            let (res, _) = res.overflowing_sub(mask as i8);

            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());

            let (res, _) = l.overflowing_sub(x as i16);
            let (res, _) = res.overflowing_sub(mask as i16);

            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());

            let (res, _) = l.overflowing_sub(x as i32);
            let (res, _) = res.overflowing_sub(mask as i32);

            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());

            let (res, _) = l.overflowing_sub(x as i64);
            let (res, _) = res.overflowing_sub(mask as i64);

            result.copy_from_slice(&res.to_le_bytes());
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::from(x as i64);
            let mask = E128::from(mask);
            let (res, _) = l.overflowing_sub_u(r);
            let (res, _) = res.overflowing_sub_u(mask);
            res.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            let mask = E256::from(mask);
            let (res, _) = l.overflowing_sub_u(r);
            let (res, _) = res.overflowing_sub_u(mask);
            res.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::from(x as i64);
            let mask = E512::from(mask);
            let (res, _) = l.overflowing_sub_u(r);
            let (res, _) = res.overflowing_sub_u(mask);
            res.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::from(x as i64);
            let mask = E1024::from(mask);
            let (res, _) = l.overflowing_sub_u(r);
            let (res, _) = res.overflowing_sub_u(mask);
            res.put(result);
        }
        _ => {
            panic!("Unsupported");
        }
    }
}
fn test_vsbc_vxm() {
    fn rvv_op(_: &[u8], rhs: &[u8], _: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            rvv_asm!("mv t0, {}", "vsbc.vxm v24, v8, t0, v0", in (reg) x);
        }
    }
    run_template_v_vxm(expected_op_sbc_vxm, rvv_op, "vsbc.vxm");
}

fn expected_op_msbc_vvm(lhs: &[u8], rhs: &[u8], result: &mut bool, mask: bool) {
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());
            let r = u8::from_le_bytes(rhs.try_into().unwrap());

            let (res, carry1) = l.overflowing_sub(r as u8);
            let (_, carry2) = res.overflowing_sub(mask as u8);

            *result = carry1 | carry2;
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            let r = u16::from_le_bytes(rhs.try_into().unwrap());

            let (res, carry1) = l.overflowing_sub(r as u16);
            let (_, carry2) = res.overflowing_sub(mask as u16);

            *result = carry1 | carry2;
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            let r = u32::from_le_bytes(rhs.try_into().unwrap());

            let (res, carry1) = l.overflowing_sub(r as u32);
            let (_, carry2) = res.overflowing_sub(mask as u32);

            *result = carry1 | carry2;
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());

            let (res, carry1) = l.overflowing_sub(r as u64);
            let (_, carry2) = res.overflowing_sub(mask as u64);

            *result = carry1 | carry2;
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let mask = E128::from(mask);
            let (res, carry1) = l.overflowing_sub_u(r);
            let (_, carry2) = res.overflowing_sub_u(mask);

            *result = carry1 | carry2;
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let mask = E256::from(mask);
            let (res, carry1) = l.overflowing_sub_u(r);
            let (_, carry2) = res.overflowing_sub_u(mask);

            *result = carry1 | carry2;
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let mask = E512::from(mask);
            let (res, carry1) = l.overflowing_sub_u(r);
            let (_, carry2) = res.overflowing_sub_u(mask);

            *result = carry1 | carry2;
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let mask = E1024::from(mask);
            let (res, carry1) = l.overflowing_sub_u(r);
            let (_, carry2) = res.overflowing_sub_u(mask);

            *result = carry1 | carry2;
        }
        _ => {
            panic!("Unsupported sew = {}", lhs.len());
        }
    }
}
fn test_vmsbc_vvm() {
    fn rvv_op(_: &[u8], _: &[u8], _: MaskType) {
        unsafe {
            rvv_asm!("vmsbc.vvm v24, v8, v16, v0");
        }
    }

    run_template_m_vvm(expected_op_msbc_vvm, rvv_op, "vmsbc.vvm")
}

fn expected_op_msbc_vxm(lhs: &[u8], x: u64, result: &mut bool, mask: bool) {
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());

            let (res, carry1) = l.overflowing_sub(x as u8);
            let (_, carry2) = res.overflowing_sub(mask as u8);

            *result = carry1 | carry2;
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());

            let (res, carry1) = l.overflowing_sub(x as u16);
            let (_, carry2) = res.overflowing_sub(mask as u16);

            *result = carry1 | carry2;
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());

            let (res, carry1) = l.overflowing_sub(x as u32);
            let (_, carry2) = res.overflowing_sub(mask as u32);

            *result = carry1 | carry2;
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());

            let (res, carry1) = l.overflowing_sub(x as u64);
            let (_, carry2) = res.overflowing_sub(mask as u64);

            *result = carry1 | carry2;
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::from(x as i64);
            let mask = E128::from(mask);
            let (res, carry1) = l.overflowing_sub_u(r);
            let (_, carry2) = res.overflowing_sub_u(mask);

            *result = carry1 | carry2;
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::from(x as i64);
            let mask = E256::from(mask);
            let (res, carry1) = l.overflowing_sub_u(r);
            let (_, carry2) = res.overflowing_sub_u(mask);

            *result = carry1 | carry2;
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::from(x as i64);
            let mask = E512::from(mask);
            let (res, carry1) = l.overflowing_sub_u(r);
            let (_, carry2) = res.overflowing_sub_u(mask);

            *result = carry1 | carry2;
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::from(x as i64);
            let mask = E1024::from(mask);
            let (res, carry1) = l.overflowing_sub_u(r);
            let (_, carry2) = res.overflowing_sub_u(mask);

            *result = carry1 | carry2;
        }
        _ => {
            panic!("Unsupported sew = {}", lhs.len());
        }
    }
}
fn test_vmsbc_vxm() {
    fn rvv_op(_: &[u8], rhs: &[u8], _: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            rvv_asm!("mv t0, {}", "vmsbc.vxm v24, v8, t0, v0", in (reg) x);
        }
    }

    run_template_m_vxm(expected_op_msbc_vxm, rvv_op, "vmsbc.vxm")
}

fn test_vmsbc_vv() {
    fn exp_op(lhs: &[u8], rhs: &[u8], result: &mut bool) {
        expected_op_msbc_vvm(lhs, rhs, result, false);
    }
    fn rvv_op(_: &[u8], _: &[u8], _: MaskType) {
        unsafe {
            rvv_asm!("vmsbc.vv v24, v8, v16");
        }
    }

    run_template_m_vv(exp_op, rvv_op, false, "vmsbc.vv")
}

fn test_vmsbc_vx() {
    fn exp_op(lhs: &[u8], rhs: u64, result: &mut bool) {
        expected_op_msbc_vxm(lhs, rhs, result, false);
    }
    fn rvv_op(_: &[u8], rhs: &[u8], _: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            rvv_asm!("mv t0, {}", "vmsbc.vx v24, v8, t0", in (reg) x);
        }
    }

    run_template_m_vx(exp_op, rvv_op, false, "vmsbc.vx")
}

pub fn test_adc_sbc() {
    test_vadc_vvm();
    test_vadc_vxm();
    test_vadc_vim();

    test_vmadc_vvm();
    test_vmadc_vxm();
    test_vmadc_vim();

    test_vmadc_vv();
    test_vmadc_vx();
    test_vmadc_vi();

    test_vsbc_vvm();
    test_vsbc_vxm();

    test_vmsbc_vvm();
    test_vmsbc_vxm();

    test_vmsbc_vv();
    test_vmsbc_vx();
}
