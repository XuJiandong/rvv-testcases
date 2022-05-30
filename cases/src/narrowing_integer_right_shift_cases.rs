use core::{arch::asm, convert::TryInto};

use eint::{Eint, E128, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::{U256, U512},
    runner::{run_template_v_wi, run_template_v_wv, run_template_v_wx, MaskType},
};

fn expected_op_srl(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() * 2);

    match result.len() {
        8 => {
            let shift_amount = x as usize & (lhs.len() * 8 - 1);
            let l = u128::from_le_bytes(lhs.try_into().unwrap());
            let r = (l >> shift_amount) as u64;
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            // The low lg2(2*SEW) bits of the shift-amount value are used
            let shift_amount = x & 0b111111111;
            let l = U512::from_little_endian(lhs);
            let r = l >> shift_amount;
            let r2: U256 = r.into();
            r2.to_little_endian(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}

fn test_vnsrl_wv() {
    fn exp_op(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        let x = match rhs.len() {
            8 => u64::from_le_bytes(rhs.try_into().unwrap()),
            32 => E256::get(rhs).u64(),
            _ => panic!("Abort"),
        };
        expected_op_srl(lhs, x, result);
    }
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vnsrl.wv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vnsrl.wv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }

    run_template_v_wv(exp_op, op, true, "vnsrl.wv");
}

fn test_vnsrl_wx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vnsrl.wx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vnsrl.wx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }

    run_template_v_wx(expected_op_srl, op, true, "vnsrl.wx");
}

fn test_vnsrl_wi() {
    fn exp_op(lhs: &[u8], x: i64, result: &mut [u8]) {
        expected_op_srl(lhs, x as u64, result);
    }
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                17 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 17, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 17");
                    }
                    _ => panic!("Abort"),
                },
                18 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 18, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 18");
                    }
                    _ => panic!("Abort"),
                },
                19 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 19, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 19");
                    }
                    _ => panic!("Abort"),
                },
                20 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 20, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 20");
                    }
                    _ => panic!("Abort"),
                },
                21 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 21, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 21");
                    }
                    _ => panic!("Abort"),
                },
                22 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 22, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 22");
                    }
                    _ => panic!("Abort"),
                },
                23 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 23, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 23");
                    }
                    _ => panic!("Abort"),
                },
                24 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 24, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 24");
                    }
                    _ => panic!("Abort"),
                },
                25 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 25, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 25");
                    }
                    _ => panic!("Abort"),
                },
                26 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 26, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 26");
                    }
                    _ => panic!("Abort"),
                },
                27 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 27, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 27");
                    }
                    _ => panic!("Abort"),
                },
                28 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 28, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 28");
                    }
                    _ => panic!("Abort"),
                },
                29 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 29, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 29");
                    }
                    _ => panic!("Abort"),
                },
                30 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 30, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 30");
                    }
                    _ => panic!("Abort"),
                },
                31 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsrl.wi v24, v8, 31, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsrl.wi v24, v8, 31");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_wi(exp_op, op, "vnsrl.wi")
}

pub fn test_narrowing_integer_right_shift() {
    test_vnsrl_wv();
    test_vnsrl_wx();
    test_vnsrl_wi();
}

fn expected_op_arithmetic(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() * 2);

    match result.len() {
        4 => {
            let shamt = x % 64;
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let hi = if l < 0 {
                //i64::MAX.wrapping_shr(shamt as u32)
                l.wrapping_shr(shamt as u32)
            } else {
                0
            };
            let res = (l.wrapping_shr(x as u32) | hi) as i32;
            result.copy_from_slice(&res.to_le_bytes());
        }
        8 => {
            let res = E128::get(lhs).wrapping_sra(x as u32).u64();
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let res = E512::get(lhs).wrapping_sra(x as u32);
            res.0.put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}

fn test_vnsra_wv() {
    fn exp_op(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        let x = match rhs.len() {
            8 => u64::from_le_bytes(rhs.try_into().unwrap()),
            32 => E256::get(rhs).u64(),
            _ => panic!("Abort"),
        };
        expected_op_arithmetic(lhs, x, result);
    }
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vnsra.wv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vnsra.wv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }

    run_template_v_wv(exp_op, op, true, "vnsra.wv");
}

fn test_vnsra_wx() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vnsra.wx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vnsra.wx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            }
        }
    }

    run_template_v_wx(expected_op_arithmetic, op, true, "vnsra.wx");
}

fn test_vnsra_wi() {
    fn exp_op(lhs: &[u8], x: i64, result: &mut [u8]) {
        expected_op_arithmetic(lhs, x as u64, result);
    }
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                17 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 17, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 17");
                    }
                    _ => panic!("Abort"),
                },
                18 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 18, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 18");
                    }
                    _ => panic!("Abort"),
                },
                19 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 19, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 19");
                    }
                    _ => panic!("Abort"),
                },
                20 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 20, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 20");
                    }
                    _ => panic!("Abort"),
                },
                21 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 21, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 21");
                    }
                    _ => panic!("Abort"),
                },
                22 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 22, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 22");
                    }
                    _ => panic!("Abort"),
                },
                23 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 23, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 23");
                    }
                    _ => panic!("Abort"),
                },
                24 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 24, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 24");
                    }
                    _ => panic!("Abort"),
                },
                25 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 25, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 25");
                    }
                    _ => panic!("Abort"),
                },
                26 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 26, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 26");
                    }
                    _ => panic!("Abort"),
                },
                27 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 27, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 27");
                    }
                    _ => panic!("Abort"),
                },
                28 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 28, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 28");
                    }
                    _ => panic!("Abort"),
                },
                29 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 29, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 29");
                    }
                    _ => panic!("Abort"),
                },
                30 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 30, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 30");
                    }
                    _ => panic!("Abort"),
                },
                31 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnsra.wi v24, v8, 31, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnsra.wi v24, v8, 31");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_wi(exp_op, op, "vnsra.wi")
}

pub fn test_narrowing_integer_right_shift_arithmetic() {
    test_vnsra_wv();
    test_vnsra_wx();
    test_vnsra_wi();
}
