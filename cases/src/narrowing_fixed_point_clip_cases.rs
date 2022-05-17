use core::{arch::asm, convert::TryInto};
use eint::{Eint, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::conver_to_i512,
    runner::{run_template_wi, run_template_wv, run_template_wx, MaskType},
};

fn expected_op_vnclipu(result: &mut [u8], lhs: &[u8], x: u64) {
    assert_eq!(lhs.len(), result.len() * 2);

    match result.len() {
        8 => {
            let l = u128::from_le_bytes(lhs.try_into().unwrap());

            let res = l.wrapping_shr(x as u32);
            let res2 = if (res >> 64) != 0 {
                u64::MAX
            } else {
                res as u64
            };
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = E512::get(lhs);
            let res = l.wrapping_shr(x as u32);

            let res2 = if res > E512::from(E256::MAX_U) {
                E256::MAX_U
            } else {
                res.0
            };
            res2.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vnclipu_wv() {
    fn exp_op(result: &mut [u8], lhs: &[u8], rhs: &[u8]) {
        let x = match rhs.len() {
            8 => u64::from_le_bytes(rhs.try_into().unwrap()),
            32 => E256::get(rhs).u64(),
            _ => {
                panic!("unsupported length: {}", rhs.len());
            }
        };
        expected_op_vnclipu(result, lhs, x)
    }
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vnclipu.wv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vnclipu.wv v24, v8, v16");
                }
                _ => panic!("Abort"),
            };
        }
    }

    run_template_wv(exp_op, op, &[64, 256], &[-2, 1, 2], true, "vnclipu.wv");
}
fn test_vnclipu_wx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vnclipu.wx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vnclipu.wx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }

    run_template_wx(
        expected_op_vnclipu,
        op,
        &[64, 256],
        &[-2, 1, 2],
        true,
        "vnclipu.wx",
    );
}
fn test_vnclipu_wi() {
    fn exp_op(result: &mut [u8], lhs: &[u8], x: i64) {
        expected_op_vnclipu(result, lhs, x as u64);
    }
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                0 => {
                    rvv_asm!("vnclipu.wi v24, v8, 0");
                }
                1 => {
                    rvv_asm!("vnclipu.wi v24, v8, 1");
                }
                2 => {
                    rvv_asm!("vnclipu.wi v24, v8, 2");
                }
                3 => {
                    rvv_asm!("vnclipu.wi v24, v8, 3");
                }
                4 => {
                    rvv_asm!("vnclipu.wi v24, v8, 4");
                }
                5 => {
                    rvv_asm!("vnclipu.wi v24, v8, 5");
                }
                6 => {
                    rvv_asm!("vnclipu.wi v24, v8, 6");
                }
                7 => {
                    rvv_asm!("vnclipu.wi v24, v8, 7");
                }
                8 => {
                    rvv_asm!("vnclipu.wi v24, v8, 8");
                }
                9 => {
                    rvv_asm!("vnclipu.wi v24, v8, 9");
                }
                10 => {
                    rvv_asm!("vnclipu.wi v24, v8, 10");
                }
                11 => {
                    rvv_asm!("vnclipu.wi v24, v8, 11");
                }
                12 => {
                    rvv_asm!("vnclipu.wi v24, v8, 12");
                }
                13 => {
                    rvv_asm!("vnclipu.wi v24, v8, 13");
                }
                14 => {
                    rvv_asm!("vnclipu.wi v24, v8, 14");
                }
                15 => {
                    rvv_asm!("vnclipu.wi v24, v8, 15");
                }
                16 => {
                    rvv_asm!("vnclipu.wi v24, v8, 16");
                }
                17 => {
                    rvv_asm!("vnclipu.wi v24, v8, 17");
                }
                18 => {
                    rvv_asm!("vnclipu.wi v24, v8, 18");
                }
                19 => {
                    rvv_asm!("vnclipu.wi v24, v8, 19");
                }
                20 => {
                    rvv_asm!("vnclipu.wi v24, v8, 20");
                }
                21 => {
                    rvv_asm!("vnclipu.wi v24, v8, 21");
                }
                22 => {
                    rvv_asm!("vnclipu.wi v24, v8, 22");
                }
                23 => {
                    rvv_asm!("vnclipu.wi v24, v8, 23");
                }
                24 => {
                    rvv_asm!("vnclipu.wi v24, v8, 24");
                }
                25 => {
                    rvv_asm!("vnclipu.wi v24, v8, 25");
                }
                26 => {
                    rvv_asm!("vnclipu.wi v24, v8, 26");
                }
                27 => {
                    rvv_asm!("vnclipu.wi v24, v8, 27");
                }
                28 => {
                    rvv_asm!("vnclipu.wi v24, v8, 28");
                }
                29 => {
                    rvv_asm!("vnclipu.wi v24, v8, 29");
                }
                30 => {
                    rvv_asm!("vnclipu.wi v24, v8, 30");
                }
                31 => {
                    rvv_asm!("vnclipu.wi v24, v8, 31");
                }
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_wi(exp_op, op, &[64, 256], &[-2, 1, 2], "vnclipu.wi");
}

fn expected_op_vnclip(result: &mut [u8], lhs: &[u8], x: u64) {
    assert_eq!(lhs.len(), result.len() * 2);

    match result.len() {
        8 => {
            let l = i128::from_le_bytes(lhs.try_into().unwrap());
            let res = l.wrapping_shr(x as u32);

            let res2 = if res < i64::MIN as i128 {
                i64::MIN as u64
            } else if res > i64::MAX as i128 {
                i64::MAX as u64
            } else {
                res as u64
            };
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = E512::get(lhs);
            let res = l.wrapping_sra(x as u32);

            let res2 = if res.cmp_s(&conver_to_i512(E256::MIN_S)).is_lt() {
                E256::MIN_S
            } else if res.cmp_s(&conver_to_i512(E256::MAX_S)).is_gt() {
                E256::MAX_S
            } else {
                res.0
            };
            res2.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vnclip_wv() {
    fn exp_op(result: &mut [u8], lhs: &[u8], rhs: &[u8]) {
        let x = match rhs.len() {
            8 => u64::from_le_bytes(rhs.try_into().unwrap()),
            32 => E256::get(rhs).u64(),
            _ => {
                panic!("unsupported length: {}", rhs.len());
            }
        };
        expected_op_vnclip(result, lhs, x)
    }
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vnclip.wv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vnclip.wv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }

    run_template_wv(exp_op, op, &[64, 256], &[-2, 1, 2], true, "vnclip.wv");
}
fn test_vnclip_wx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vnclip.wx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vnclip.wx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }

    run_template_wx(
        expected_op_vnclip,
        op,
        &[64, 256],
        &[-2, 1, 2],
        true,
        "vnclip.wx",
    );
}
fn test_vnclip_wi() {
    fn exp_op(result: &mut [u8], lhs: &[u8], x: i64) {
        expected_op_vnclip(result, lhs, x as u64);
    }
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                0 => {
                    rvv_asm!("vnclip.wi v24, v8, 0");
                }
                1 => {
                    rvv_asm!("vnclip.wi v24, v8, 1");
                }
                2 => {
                    rvv_asm!("vnclip.wi v24, v8, 2");
                }
                3 => {
                    rvv_asm!("vnclip.wi v24, v8, 3");
                }
                4 => {
                    rvv_asm!("vnclip.wi v24, v8, 4");
                }
                5 => {
                    rvv_asm!("vnclip.wi v24, v8, 5");
                }
                6 => {
                    rvv_asm!("vnclip.wi v24, v8, 6");
                }
                7 => {
                    rvv_asm!("vnclip.wi v24, v8, 7");
                }
                8 => {
                    rvv_asm!("vnclip.wi v24, v8, 8");
                }
                9 => {
                    rvv_asm!("vnclip.wi v24, v8, 9");
                }
                10 => {
                    rvv_asm!("vnclip.wi v24, v8, 10");
                }
                11 => {
                    rvv_asm!("vnclip.wi v24, v8, 11");
                }
                12 => {
                    rvv_asm!("vnclip.wi v24, v8, 12");
                }
                13 => {
                    rvv_asm!("vnclip.wi v24, v8, 13");
                }
                14 => {
                    rvv_asm!("vnclip.wi v24, v8, 14");
                }
                15 => {
                    rvv_asm!("vnclip.wi v24, v8, 15");
                }
                16 => {
                    rvv_asm!("vnclip.wi v24, v8, 16");
                }
                17 => {
                    rvv_asm!("vnclip.wi v24, v8, 17");
                }
                18 => {
                    rvv_asm!("vnclip.wi v24, v8, 18");
                }
                19 => {
                    rvv_asm!("vnclip.wi v24, v8, 19");
                }
                20 => {
                    rvv_asm!("vnclip.wi v24, v8, 20");
                }
                21 => {
                    rvv_asm!("vnclip.wi v24, v8, 21");
                }
                22 => {
                    rvv_asm!("vnclip.wi v24, v8, 22");
                }
                23 => {
                    rvv_asm!("vnclip.wi v24, v8, 23");
                }
                24 => {
                    rvv_asm!("vnclip.wi v24, v8, 24");
                }
                25 => {
                    rvv_asm!("vnclip.wi v24, v8, 25");
                }
                26 => {
                    rvv_asm!("vnclip.wi v24, v8, 26");
                }
                27 => {
                    rvv_asm!("vnclip.wi v24, v8, 27");
                }
                28 => {
                    rvv_asm!("vnclip.wi v24, v8, 28");
                }
                29 => {
                    rvv_asm!("vnclip.wi v24, v8, 29");
                }
                30 => {
                    rvv_asm!("vnclip.wi v24, v8, 30");
                }
                31 => {
                    rvv_asm!("vnclip.wi v24, v8, 31");
                }
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_wi(exp_op, op, &[64, 256], &[-2, 1, 2], "vnclip.wi");
}

pub fn test_narrowing_fixed_point_clip() {
    test_vnclipu_wv();
    test_vnclipu_wx();
    test_vnclipu_wi();
    test_vnclip_wv();
    test_vnclip_wx();
    test_vnclip_wi();
}
