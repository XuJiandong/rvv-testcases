use core::{arch::asm, convert::TryInto};
use eint::{Eint, E128, E256};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::conver_to_i256,
    runner::{run_template_m_vi, MaskType},
};

fn expected_eq(lhs: &[u8], imm: i64, result: &mut bool) {
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            *result = l == imm as i64;
        }
        32 => {
            let l = E256::get(lhs);
            let r = conver_to_i256(E128::from(imm as i128));
            *result = l == r;
        }
        _ => {
            panic!("Invalid sew");
        }
    };
}
fn test_vmseq() {
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => {
                    rvv_asm!("vmseq.vi v24, v8, -16");
                }
                -15 => {
                    rvv_asm!("vmseq.vi v24, v8, -15");
                }
                -14 => {
                    rvv_asm!("vmseq.vi v24, v8, -14");
                }
                -13 => {
                    rvv_asm!("vmseq.vi v24, v8, -13");
                }
                -12 => {
                    rvv_asm!("vmseq.vi v24, v8, -12");
                }
                -11 => {
                    rvv_asm!("vmseq.vi v24, v8, -11");
                }
                -10 => {
                    rvv_asm!("vmseq.vi v24, v8, -10");
                }
                -9 => {
                    rvv_asm!("vmseq.vi v24, v8, -9");
                }
                -8 => {
                    rvv_asm!("vmseq.vi v24, v8, -8");
                }
                -7 => {
                    rvv_asm!("vmseq.vi v24, v8, -7");
                }
                -6 => {
                    rvv_asm!("vmseq.vi v24, v8, -6");
                }
                -5 => {
                    rvv_asm!("vmseq.vi v24, v8, -5");
                }
                -4 => {
                    rvv_asm!("vmseq.vi v24, v8, -4");
                }
                -3 => {
                    rvv_asm!("vmseq.vi v24, v8, -3");
                }
                -2 => {
                    rvv_asm!("vmseq.vi v24, v8, -2");
                }
                -1 => {
                    rvv_asm!("vmseq.vi v24, v8, -1");
                }
                0 => {
                    rvv_asm!("vmseq.vi v24, v8, 0");
                }
                1 => {
                    rvv_asm!("vmseq.vi v24, v8, 1");
                }
                2 => {
                    rvv_asm!("vmseq.vi v24, v8, 2");
                }
                3 => {
                    rvv_asm!("vmseq.vi v24, v8, 3");
                }
                4 => {
                    rvv_asm!("vmseq.vi v24, v8, 4");
                }
                5 => {
                    rvv_asm!("vmseq.vi v24, v8, 5");
                }
                6 => {
                    rvv_asm!("vmseq.vi v24, v8, 6");
                }
                7 => {
                    rvv_asm!("vmseq.vi v24, v8, 7");
                }
                8 => {
                    rvv_asm!("vmseq.vi v24, v8, 8");
                }
                9 => {
                    rvv_asm!("vmseq.vi v24, v8, 9");
                }
                10 => {
                    rvv_asm!("vmseq.vi v24, v8, 10");
                }
                11 => {
                    rvv_asm!("vmseq.vi v24, v8, 11");
                }
                12 => {
                    rvv_asm!("vmseq.vi v24, v8, 12");
                }
                13 => {
                    rvv_asm!("vmseq.vi v24, v8, 13");
                }
                14 => {
                    rvv_asm!("vmseq.vi v24, v8, 14");
                }
                15 => {
                    rvv_asm!("vmseq.vi v24, v8, 15");
                }
                _ => {
                    panic!("Invalid immediate: {}", imm);
                }
            }
        }
    }
    run_template_m_vi(expected_eq, op, false, "vmseq.vi");
}

fn expected_ne(lhs: &[u8], imm: i64, result: &mut bool) {
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            *result = l != imm as i64;
        }
        32 => {
            let l = E256::get(lhs);
            let r = conver_to_i256(E128::from(imm as i128));
            *result = l != r;
        }
        _ => {
            panic!("Invalid sew");
        }
    };
}
fn test_vmsne() {
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => {
                    rvv_asm!("vmsne.vi v24, v8, -16");
                }
                -15 => {
                    rvv_asm!("vmsne.vi v24, v8, -15");
                }
                -14 => {
                    rvv_asm!("vmsne.vi v24, v8, -14");
                }
                -13 => {
                    rvv_asm!("vmsne.vi v24, v8, -13");
                }
                -12 => {
                    rvv_asm!("vmsne.vi v24, v8, -12");
                }
                -11 => {
                    rvv_asm!("vmsne.vi v24, v8, -11");
                }
                -10 => {
                    rvv_asm!("vmsne.vi v24, v8, -10");
                }
                -9 => {
                    rvv_asm!("vmsne.vi v24, v8, -9");
                }
                -8 => {
                    rvv_asm!("vmsne.vi v24, v8, -8");
                }
                -7 => {
                    rvv_asm!("vmsne.vi v24, v8, -7");
                }
                -6 => {
                    rvv_asm!("vmsne.vi v24, v8, -6");
                }
                -5 => {
                    rvv_asm!("vmsne.vi v24, v8, -5");
                }
                -4 => {
                    rvv_asm!("vmsne.vi v24, v8, -4");
                }
                -3 => {
                    rvv_asm!("vmsne.vi v24, v8, -3");
                }
                -2 => {
                    rvv_asm!("vmsne.vi v24, v8, -2");
                }
                -1 => {
                    rvv_asm!("vmsne.vi v24, v8, -1");
                }
                0 => {
                    rvv_asm!("vmsne.vi v24, v8, 0");
                }
                1 => {
                    rvv_asm!("vmsne.vi v24, v8, 1");
                }
                2 => {
                    rvv_asm!("vmsne.vi v24, v8, 2");
                }
                3 => {
                    rvv_asm!("vmsne.vi v24, v8, 3");
                }
                4 => {
                    rvv_asm!("vmsne.vi v24, v8, 4");
                }
                5 => {
                    rvv_asm!("vmsne.vi v24, v8, 5");
                }
                6 => {
                    rvv_asm!("vmsne.vi v24, v8, 6");
                }
                7 => {
                    rvv_asm!("vmsne.vi v24, v8, 7");
                }
                8 => {
                    rvv_asm!("vmsne.vi v24, v8, 8");
                }
                9 => {
                    rvv_asm!("vmsne.vi v24, v8, 9");
                }
                10 => {
                    rvv_asm!("vmsne.vi v24, v8, 10");
                }
                11 => {
                    rvv_asm!("vmsne.vi v24, v8, 11");
                }
                12 => {
                    rvv_asm!("vmsne.vi v24, v8, 12");
                }
                13 => {
                    rvv_asm!("vmsne.vi v24, v8, 13");
                }
                14 => {
                    rvv_asm!("vmsne.vi v24, v8, 14");
                }
                15 => {
                    rvv_asm!("vmsne.vi v24, v8, 15");
                }
                _ => {
                    panic!("Invalid immediate: {}", imm);
                }
            }
        }
    }
    run_template_m_vi(expected_ne, op, false, "vmsne.vi");
}

fn expected_leu(lhs: &[u8], imm: i64, result: &mut bool) {
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            *result = l <= imm as u64;
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(imm);
            *result = l.cmp_u(&r).is_le();
        }
        _ => {
            panic!("Invalid sew");
        }
    };
}
fn test_vmsleu() {
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => {
                    rvv_asm!("vmsleu.vi v24, v8, -16");
                }
                -15 => {
                    rvv_asm!("vmsleu.vi v24, v8, -15");
                }
                -14 => {
                    rvv_asm!("vmsleu.vi v24, v8, -14");
                }
                -13 => {
                    rvv_asm!("vmsleu.vi v24, v8, -13");
                }
                -12 => {
                    rvv_asm!("vmsleu.vi v24, v8, -12");
                }
                -11 => {
                    rvv_asm!("vmsleu.vi v24, v8, -11");
                }
                -10 => {
                    rvv_asm!("vmsleu.vi v24, v8, -10");
                }
                -9 => {
                    rvv_asm!("vmsleu.vi v24, v8, -9");
                }
                -8 => {
                    rvv_asm!("vmsleu.vi v24, v8, -8");
                }
                -7 => {
                    rvv_asm!("vmsleu.vi v24, v8, -7");
                }
                -6 => {
                    rvv_asm!("vmsleu.vi v24, v8, -6");
                }
                -5 => {
                    rvv_asm!("vmsleu.vi v24, v8, -5");
                }
                -4 => {
                    rvv_asm!("vmsleu.vi v24, v8, -4");
                }
                -3 => {
                    rvv_asm!("vmsleu.vi v24, v8, -3");
                }
                -2 => {
                    rvv_asm!("vmsleu.vi v24, v8, -2");
                }
                -1 => {
                    rvv_asm!("vmsleu.vi v24, v8, -1");
                }
                0 => {
                    rvv_asm!("vmsleu.vi v24, v8, 0");
                }
                1 => {
                    rvv_asm!("vmsleu.vi v24, v8, 1");
                }
                2 => {
                    rvv_asm!("vmsleu.vi v24, v8, 2");
                }
                3 => {
                    rvv_asm!("vmsleu.vi v24, v8, 3");
                }
                4 => {
                    rvv_asm!("vmsleu.vi v24, v8, 4");
                }
                5 => {
                    rvv_asm!("vmsleu.vi v24, v8, 5");
                }
                6 => {
                    rvv_asm!("vmsleu.vi v24, v8, 6");
                }
                7 => {
                    rvv_asm!("vmsleu.vi v24, v8, 7");
                }
                8 => {
                    rvv_asm!("vmsleu.vi v24, v8, 8");
                }
                9 => {
                    rvv_asm!("vmsleu.vi v24, v8, 9");
                }
                10 => {
                    rvv_asm!("vmsleu.vi v24, v8, 10");
                }
                11 => {
                    rvv_asm!("vmsleu.vi v24, v8, 11");
                }
                12 => {
                    rvv_asm!("vmsleu.vi v24, v8, 12");
                }
                13 => {
                    rvv_asm!("vmsleu.vi v24, v8, 13");
                }
                14 => {
                    rvv_asm!("vmsleu.vi v24, v8, 14");
                }
                15 => {
                    rvv_asm!("vmsleu.vi v24, v8, 15");
                }
                _ => {
                    panic!("Invalid immediate: {}", imm);
                }
            }
        }
    }
    run_template_m_vi(expected_leu, op, false, "vmsleu.vi");
}

fn expected_le(lhs: &[u8], imm: i64, result: &mut bool) {
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            *result = l <= imm as i64;
        }
        32 => {
            let l = E256::get(lhs);
            let r = conver_to_i256(E128::from(imm as i128));
            *result = l.cmp_s(&r).is_le();
        }
        _ => {
            panic!("Invalid sew");
        }
    };
}
fn test_vmsle() {
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => {
                    rvv_asm!("vmsle.vi v24, v8, -16");
                }
                -15 => {
                    rvv_asm!("vmsle.vi v24, v8, -15");
                }
                -14 => {
                    rvv_asm!("vmsle.vi v24, v8, -14");
                }
                -13 => {
                    rvv_asm!("vmsle.vi v24, v8, -13");
                }
                -12 => {
                    rvv_asm!("vmsle.vi v24, v8, -12");
                }
                -11 => {
                    rvv_asm!("vmsle.vi v24, v8, -11");
                }
                -10 => {
                    rvv_asm!("vmsle.vi v24, v8, -10");
                }
                -9 => {
                    rvv_asm!("vmsle.vi v24, v8, -9");
                }
                -8 => {
                    rvv_asm!("vmsle.vi v24, v8, -8");
                }
                -7 => {
                    rvv_asm!("vmsle.vi v24, v8, -7");
                }
                -6 => {
                    rvv_asm!("vmsle.vi v24, v8, -6");
                }
                -5 => {
                    rvv_asm!("vmsle.vi v24, v8, -5");
                }
                -4 => {
                    rvv_asm!("vmsle.vi v24, v8, -4");
                }
                -3 => {
                    rvv_asm!("vmsle.vi v24, v8, -3");
                }
                -2 => {
                    rvv_asm!("vmsle.vi v24, v8, -2");
                }
                -1 => {
                    rvv_asm!("vmsle.vi v24, v8, -1");
                }
                0 => {
                    rvv_asm!("vmsle.vi v24, v8, 0");
                }
                1 => {
                    rvv_asm!("vmsle.vi v24, v8, 1");
                }
                2 => {
                    rvv_asm!("vmsle.vi v24, v8, 2");
                }
                3 => {
                    rvv_asm!("vmsle.vi v24, v8, 3");
                }
                4 => {
                    rvv_asm!("vmsle.vi v24, v8, 4");
                }
                5 => {
                    rvv_asm!("vmsle.vi v24, v8, 5");
                }
                6 => {
                    rvv_asm!("vmsle.vi v24, v8, 6");
                }
                7 => {
                    rvv_asm!("vmsle.vi v24, v8, 7");
                }
                8 => {
                    rvv_asm!("vmsle.vi v24, v8, 8");
                }
                9 => {
                    rvv_asm!("vmsle.vi v24, v8, 9");
                }
                10 => {
                    rvv_asm!("vmsle.vi v24, v8, 10");
                }
                11 => {
                    rvv_asm!("vmsle.vi v24, v8, 11");
                }
                12 => {
                    rvv_asm!("vmsle.vi v24, v8, 12");
                }
                13 => {
                    rvv_asm!("vmsle.vi v24, v8, 13");
                }
                14 => {
                    rvv_asm!("vmsle.vi v24, v8, 14");
                }
                15 => {
                    rvv_asm!("vmsle.vi v24, v8, 15");
                }
                _ => {
                    panic!("Invalid immediate: {}", imm);
                }
            }
        }
    }
    run_template_m_vi(expected_le, op, false, "vmsle.vi");
}

fn expected_gtu(lhs: &[u8], imm: i64, result: &mut bool) {
    match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            *result = l > imm as u64;
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(imm);
            *result = l.cmp_u(&r).is_gt();
        }
        _ => {
            panic!("Invalid sew");
        }
    };
}
fn test_vmsgtu() {
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => {
                    rvv_asm!("vmsgtu.vi v24, v8, -16");
                }
                -15 => {
                    rvv_asm!("vmsgtu.vi v24, v8, -15");
                }
                -14 => {
                    rvv_asm!("vmsgtu.vi v24, v8, -14");
                }
                -13 => {
                    rvv_asm!("vmsgtu.vi v24, v8, -13");
                }
                -12 => {
                    rvv_asm!("vmsgtu.vi v24, v8, -12");
                }
                -11 => {
                    rvv_asm!("vmsgtu.vi v24, v8, -11");
                }
                -10 => {
                    rvv_asm!("vmsgtu.vi v24, v8, -10");
                }
                -9 => {
                    rvv_asm!("vmsgtu.vi v24, v8, -9");
                }
                -8 => {
                    rvv_asm!("vmsgtu.vi v24, v8, -8");
                }
                -7 => {
                    rvv_asm!("vmsgtu.vi v24, v8, -7");
                }
                -6 => {
                    rvv_asm!("vmsgtu.vi v24, v8, -6");
                }
                -5 => {
                    rvv_asm!("vmsgtu.vi v24, v8, -5");
                }
                -4 => {
                    rvv_asm!("vmsgtu.vi v24, v8, -4");
                }
                -3 => {
                    rvv_asm!("vmsgtu.vi v24, v8, -3");
                }
                -2 => {
                    rvv_asm!("vmsgtu.vi v24, v8, -2");
                }
                -1 => {
                    rvv_asm!("vmsgtu.vi v24, v8, -1");
                }
                0 => {
                    rvv_asm!("vmsgtu.vi v24, v8, 0");
                }
                1 => {
                    rvv_asm!("vmsgtu.vi v24, v8, 1");
                }
                2 => {
                    rvv_asm!("vmsgtu.vi v24, v8, 2");
                }
                3 => {
                    rvv_asm!("vmsgtu.vi v24, v8, 3");
                }
                4 => {
                    rvv_asm!("vmsgtu.vi v24, v8, 4");
                }
                5 => {
                    rvv_asm!("vmsgtu.vi v24, v8, 5");
                }
                6 => {
                    rvv_asm!("vmsgtu.vi v24, v8, 6");
                }
                7 => {
                    rvv_asm!("vmsgtu.vi v24, v8, 7");
                }
                8 => {
                    rvv_asm!("vmsgtu.vi v24, v8, 8");
                }
                9 => {
                    rvv_asm!("vmsgtu.vi v24, v8, 9");
                }
                10 => {
                    rvv_asm!("vmsgtu.vi v24, v8, 10");
                }
                11 => {
                    rvv_asm!("vmsgtu.vi v24, v8, 11");
                }
                12 => {
                    rvv_asm!("vmsgtu.vi v24, v8, 12");
                }
                13 => {
                    rvv_asm!("vmsgtu.vi v24, v8, 13");
                }
                14 => {
                    rvv_asm!("vmsgtu.vi v24, v8, 14");
                }
                15 => {
                    rvv_asm!("vmsgtu.vi v24, v8, 15");
                }
                _ => {
                    panic!("Invalid immediate: {}", imm);
                }
            }
        }
    }
    run_template_m_vi(expected_gtu, op, false, "vmsgtu.vi");
}

fn expected_gt(lhs: &[u8], imm: i64, result: &mut bool) {
    match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            *result = l > imm;
        }
        32 => {
            let l = E256::get(lhs);
            let r = conver_to_i256(E128::from(imm as i128));
            *result = l.cmp_s(&r).is_gt();
        }
        _ => {
            panic!("Invalid sew");
        }
    };
}
fn test_vmsgt() {
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => {
                    rvv_asm!("vmsgt.vi v24, v8, -16");
                }
                -15 => {
                    rvv_asm!("vmsgt.vi v24, v8, -15");
                }
                -14 => {
                    rvv_asm!("vmsgt.vi v24, v8, -14");
                }
                -13 => {
                    rvv_asm!("vmsgt.vi v24, v8, -13");
                }
                -12 => {
                    rvv_asm!("vmsgt.vi v24, v8, -12");
                }
                -11 => {
                    rvv_asm!("vmsgt.vi v24, v8, -11");
                }
                -10 => {
                    rvv_asm!("vmsgt.vi v24, v8, -10");
                }
                -9 => {
                    rvv_asm!("vmsgt.vi v24, v8, -9");
                }
                -8 => {
                    rvv_asm!("vmsgt.vi v24, v8, -8");
                }
                -7 => {
                    rvv_asm!("vmsgt.vi v24, v8, -7");
                }
                -6 => {
                    rvv_asm!("vmsgt.vi v24, v8, -6");
                }
                -5 => {
                    rvv_asm!("vmsgt.vi v24, v8, -5");
                }
                -4 => {
                    rvv_asm!("vmsgt.vi v24, v8, -4");
                }
                -3 => {
                    rvv_asm!("vmsgt.vi v24, v8, -3");
                }
                -2 => {
                    rvv_asm!("vmsgt.vi v24, v8, -2");
                }
                -1 => {
                    rvv_asm!("vmsgt.vi v24, v8, -1");
                }
                0 => {
                    rvv_asm!("vmsgt.vi v24, v8, 0");
                }
                1 => {
                    rvv_asm!("vmsgt.vi v24, v8, 1");
                }
                2 => {
                    rvv_asm!("vmsgt.vi v24, v8, 2");
                }
                3 => {
                    rvv_asm!("vmsgt.vi v24, v8, 3");
                }
                4 => {
                    rvv_asm!("vmsgt.vi v24, v8, 4");
                }
                5 => {
                    rvv_asm!("vmsgt.vi v24, v8, 5");
                }
                6 => {
                    rvv_asm!("vmsgt.vi v24, v8, 6");
                }
                7 => {
                    rvv_asm!("vmsgt.vi v24, v8, 7");
                }
                8 => {
                    rvv_asm!("vmsgt.vi v24, v8, 8");
                }
                9 => {
                    rvv_asm!("vmsgt.vi v24, v8, 9");
                }
                10 => {
                    rvv_asm!("vmsgt.vi v24, v8, 10");
                }
                11 => {
                    rvv_asm!("vmsgt.vi v24, v8, 11");
                }
                12 => {
                    rvv_asm!("vmsgt.vi v24, v8, 12");
                }
                13 => {
                    rvv_asm!("vmsgt.vi v24, v8, 13");
                }
                14 => {
                    rvv_asm!("vmsgt.vi v24, v8, 14");
                }
                15 => {
                    rvv_asm!("vmsgt.vi v24, v8, 15");
                }
                _ => {
                    panic!("Invalid immediate: {}", imm);
                }
            }
        }
    }
    run_template_m_vi(expected_gt, op, false, "vmsgt.vi");
}

pub fn test_vmsop_vi() {
    test_vmseq();
    test_vmsne();
    test_vmsleu();
    test_vmsle();
    test_vmsgtu();
    test_vmsgt();
}
