use core::{arch::asm, convert::TryInto};
use eint::{Eint, E128, E256};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    intrinsic::vmsop_vi,
    misc::{avl_iterator, conver_to_i256, set_bit_in_slice},
    runner::{run_vmsop_vi, WideningCategory},
};

fn expected_eq(lhs: &[u8], imm: i64, result: &mut [u8], index: usize) {
    let res = match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            if l == imm as i64 {
                1
            } else {
                0
            }
        }
        32 => {
            let l = E256::get(lhs);
            let r = conver_to_i256(E128::from(imm as i128));
            if l == r {
                1
            } else {
                0
            }
        }
        _ => {
            panic!("Invalid sew");
        }
    };
    set_bit_in_slice(result, index, res);
}
fn test_vmseq(sew: u64, lmul: i64, avl: u64, imm: i64) {
    fn op(lhs: &[u8], imm: i64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vi(lhs, imm, result, sew, avl, lmul, |imm| unsafe {
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
        });
    }
    run_vmsop_vi(
        sew,
        lmul,
        avl,
        imm,
        expected_eq,
        op,
        WideningCategory::None,
        "vmseq.vi",
    );
}

fn expected_ne(lhs: &[u8], imm: i64, result: &mut [u8], index: usize) {
    let res = match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            if l != imm as i64 {
                1
            } else {
                0
            }
        }
        32 => {
            let l = E256::get(lhs);
            let r = conver_to_i256(E128::from(imm as i128));
            if l != r {
                1
            } else {
                0
            }
        }
        _ => {
            panic!("Invalid sew");
        }
    };
    set_bit_in_slice(result, index, res);
}
fn test_vmsne(sew: u64, lmul: i64, avl: u64, imm: i64) {
    fn op(lhs: &[u8], imm: i64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vi(lhs, imm, result, sew, avl, lmul, |imm| unsafe {
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
        });
    }
    run_vmsop_vi(
        sew,
        lmul,
        avl,
        imm,
        expected_ne,
        op,
        WideningCategory::None,
        "vmsne.vi",
    );
}

fn expected_leu(lhs: &[u8], imm: i64, result: &mut [u8], index: usize) {
    let res = match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            if l <= imm as u64 {
                1
            } else {
                0
            }
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(imm);
            if l.cmp_u(&r).is_le() {
                1
            } else {
                0
            }
        }
        _ => {
            panic!("Invalid sew");
        }
    };
    set_bit_in_slice(result, index, res);
}
fn test_vmsleu(sew: u64, lmul: i64, avl: u64, imm: i64) {
    fn op(lhs: &[u8], imm: i64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vi(lhs, imm, result, sew, avl, lmul, |imm| unsafe {
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
        });
    }
    run_vmsop_vi(
        sew,
        lmul,
        avl,
        imm,
        expected_leu,
        op,
        WideningCategory::None,
        "vmsleu.vi",
    );
}

fn expected_le(lhs: &[u8], imm: i64, result: &mut [u8], index: usize) {
    let res = match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            if l <= imm as i64 {
                1
            } else {
                0
            }
        }
        32 => {
            let l = E256::get(lhs);
            let r = conver_to_i256(E128::from(imm as i128));
            if l.cmp_s(&r).is_le() {
                1
            } else {
                0
            }
        }
        _ => {
            panic!("Invalid sew");
        }
    };
    set_bit_in_slice(result, index, res);
}
fn test_vmsle(sew: u64, lmul: i64, avl: u64, imm: i64) {
    fn op(lhs: &[u8], imm: i64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vi(lhs, imm, result, sew, avl, lmul, |imm| unsafe {
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
        });
    }
    run_vmsop_vi(
        sew,
        lmul,
        avl,
        imm,
        expected_le,
        op,
        WideningCategory::None,
        "vmsle.vi",
    );
}

fn expected_gtu(lhs: &[u8], imm: i64, result: &mut [u8], index: usize) {
    let res = match lhs.len() {
        8 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            if l > imm as u64 {
                1
            } else {
                0
            }
        }
        32 => {
            let l = E256::get(lhs);
            let r = E256::from(imm);
            if l.cmp_u(&r).is_gt() {
                1
            } else {
                0
            }
        }
        _ => {
            panic!("Invalid sew");
        }
    };
    set_bit_in_slice(result, index, res);
}
fn test_vmsgtu(sew: u64, lmul: i64, avl: u64, imm: i64) {
    fn op(lhs: &[u8], imm: i64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vi(lhs, imm, result, sew, avl, lmul, |imm| unsafe {
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
        });
    }
    run_vmsop_vi(
        sew,
        lmul,
        avl,
        imm,
        expected_gtu,
        op,
        WideningCategory::None,
        "vmsgtu.vi",
    );
}

fn expected_gt(lhs: &[u8], imm: i64, result: &mut [u8], index: usize) {
    let res = match lhs.len() {
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            if l > imm {
                1
            } else {
                0
            }
        }
        32 => {
            let l = E256::get(lhs);
            let r = conver_to_i256(E128::from(imm as i128));
            if l.cmp_s(&r).is_gt() {
                1
            } else {
                0
            }
        }
        _ => {
            panic!("Invalid sew");
        }
    };
    set_bit_in_slice(result, index, res);
}
fn test_vmsgt(sew: u64, lmul: i64, avl: u64, imm: i64) {
    fn op(lhs: &[u8], imm: i64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vmsop_vi(lhs, imm, result, sew, avl, lmul, |imm| unsafe {
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
        });
    }
    run_vmsop_vi(
        sew,
        lmul,
        avl,
        imm,
        expected_gt,
        op,
        WideningCategory::None,
        "vmsgt.vi",
    );
}

pub fn test_vmsop_vi() {
    let mut imm = -16;
    for sew in [64, 256] {
        for lmul in [-8, -2, 1, 4, 8] {
            for avl in avl_iterator(sew, 4) {
                test_vmseq(sew, lmul, avl, imm);
                test_vmsne(sew, lmul, avl, imm);
                test_vmsleu(sew, lmul, avl, imm);
                test_vmsle(sew, lmul, avl, imm);
                test_vmsgtu(sew, lmul, avl, imm);
                test_vmsgt(sew, lmul, avl, imm);
                imm += 1;
                if imm > 15 {
                    imm = -16;
                }
            }
        }
    }
}
