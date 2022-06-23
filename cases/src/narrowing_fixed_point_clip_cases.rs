use core::{arch::asm, convert::TryInto};
use eint::{Eint, E1024, E128, E2048, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::{conver_to_i1024, conver_to_i2048, conver_to_i256, conver_to_i512},
    runner::{run_template_v_wi, run_template_v_wv, run_template_v_wx, MaskType},
};

fn expected_op_vnclipu(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() * 2);

    let sew = result.len() * 8;
    match sew {
        8 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());

            let res = l.wrapping_shr(x as u32);
            let res2 = if (res >> 8) != 0 { u8::MAX } else { res as u8 };
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());

            let res = l.wrapping_shr(x as u32);
            let res2 = if (res >> 16) != 0 {
                u16::MAX
            } else {
                res as u16
            };
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());

            let res = l.wrapping_shr(x as u32);
            let res2 = if (res >> 32) != 0 {
                u32::MAX
            } else {
                res as u32
            };
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l = u128::from_le_bytes(lhs.try_into().unwrap());

            let res = l.wrapping_shr(x as u32);
            let res2 = if (res >> 64) != 0 {
                u64::MAX
            } else {
                res as u64
            };
            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = E256::get(lhs);
            let res = l.wrapping_shr(x as u32);

            let res2 = if res > E256::from(E128::MAX_U) {
                E128::MAX_U
            } else {
                res.0
            };
            res2.put(result);
        }
        256 => {
            let l = E512::get(lhs);
            let res = l.wrapping_shr(x as u32);

            let res2 = if res > E512::from(E256::MAX_U) {
                E256::MAX_U
            } else {
                res.0
            };
            res2.put(result);
        }
        512 => {
            let l = E1024::get(lhs);
            let res = l.wrapping_shr(x as u32);

            let res2 = if res > E1024::from(E512::MAX_U) {
                E512::MAX_U
            } else {
                res.0
            };
            res2.put(result);
        }
        1024 => {
            let l = E2048::get(lhs);
            let res = l.wrapping_shr(x as u32);

            let res2 = if res > E2048::from(E1024::MAX_U) {
                E1024::MAX_U
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
    fn exp_op(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        let x = match rhs.len() * 8 {
            8 => u8::from_le_bytes(rhs.try_into().unwrap()) as u64,
            16 => u16::from_le_bytes(rhs.try_into().unwrap()) as u64,
            32 => u32::from_le_bytes(rhs.try_into().unwrap()) as u64,
            64 => u64::from_le_bytes(rhs.try_into().unwrap()) as u64,
            128 => E128::get(rhs).u64(),
            256 => E256::get(rhs).u64(),
            512 => E512::get(rhs).u64(),
            1024 => E1024::get(rhs).u64(),
            _ => {
                panic!("unsupported length: {}", rhs.len());
            }
        };
        expected_op_vnclipu(lhs, x, result)
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

    run_template_v_wv(exp_op, op, true, "vnclipu.wv");
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

    run_template_v_wx(expected_op_vnclipu, op, true, "vnclipu.wx");
}
fn test_vnclipu_wi() {
    fn exp_op(lhs: &[u8], x: i64, result: &mut [u8]) {
        expected_op_vnclipu(lhs, x as u64, result);
    }
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                17 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 17, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 17");
                    }
                    _ => panic!("Abort"),
                },
                18 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 18, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 18");
                    }
                    _ => panic!("Abort"),
                },
                19 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 19, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 19");
                    }
                    _ => panic!("Abort"),
                },
                20 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 20, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 20");
                    }
                    _ => panic!("Abort"),
                },
                21 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 21, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 21");
                    }
                    _ => panic!("Abort"),
                },
                22 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 22, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 22");
                    }
                    _ => panic!("Abort"),
                },
                23 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 23, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 23");
                    }
                    _ => panic!("Abort"),
                },
                24 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 24, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 24");
                    }
                    _ => panic!("Abort"),
                },
                25 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 25, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 25");
                    }
                    _ => panic!("Abort"),
                },
                26 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 26, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 26");
                    }
                    _ => panic!("Abort"),
                },
                27 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 27, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 27");
                    }
                    _ => panic!("Abort"),
                },
                28 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 28, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 28");
                    }
                    _ => panic!("Abort"),
                },
                29 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 29, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 29");
                    }
                    _ => panic!("Abort"),
                },
                30 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 30, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 30");
                    }
                    _ => panic!("Abort"),
                },
                31 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclipu.wi v24, v8, 31, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclipu.wi v24, v8, 31");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_wi(exp_op, op, "vnclipu.wi");
}

fn expected_op_vnclip(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() * 2);

    let sew = result.len() * 8;
    match sew {
        8 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            let res = l.wrapping_shr(x as u32);

            let res2 = if res < i8::MIN as i16 {
                i8::MIN as u8
            } else if res > i8::MAX as i16 {
                i8::MAX as u8
            } else {
                res as u8
            };
            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            let res = l.wrapping_shr(x as u32);

            let res2 = if res < i16::MIN as i32 {
                i16::MIN as u16
            } else if res > i16::MAX as i32 {
                i16::MAX as u16
            } else {
                res as u16
            };
            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let res = l.wrapping_shr(x as u32);

            let res2 = if res < i32::MIN as i64 {
                i32::MIN as u32
            } else if res > i32::MAX as i64 {
                i32::MAX as u32
            } else {
                res as u32
            };
            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
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

        128 => {
            let l = E256::get(lhs);
            let res = l.wrapping_sra(x as u32);

            let res2 = if res.cmp_s(&conver_to_i256(E128::MIN_S)).is_lt() {
                E128::MIN_S
            } else if res.cmp_s(&conver_to_i256(E128::MAX_S)).is_gt() {
                E128::MAX_S
            } else {
                res.0
            };
            res2.put(result);
        }
        256 => {
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
        512 => {
            let l = E1024::get(lhs);
            let res = l.wrapping_sra(x as u32);

            let res2 = if res.cmp_s(&conver_to_i1024(E512::MIN_S)).is_lt() {
                E512::MIN_S
            } else if res.cmp_s(&conver_to_i1024(E512::MAX_S)).is_gt() {
                E512::MAX_S
            } else {
                res.0
            };
            res2.put(result);
        }
        1024 => {
            let l = E2048::get(lhs);
            let res = l.wrapping_sra(x as u32);

            let res2 = if res.cmp_s(&conver_to_i2048(E1024::MIN_S)).is_lt() {
                E1024::MIN_S
            } else if res.cmp_s(&conver_to_i2048(E1024::MAX_S)).is_gt() {
                E1024::MAX_S
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
    fn exp_op(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        let x = match rhs.len() * 8 {
            8 => u8::from_le_bytes(rhs.try_into().unwrap()) as u64,
            16 => u16::from_le_bytes(rhs.try_into().unwrap()) as u64,
            32 => u32::from_le_bytes(rhs.try_into().unwrap()) as u64,
            64 => u64::from_le_bytes(rhs.try_into().unwrap()) as u64,
            128 => E128::get(rhs).u64(),
            256 => E256::get(rhs).u64(),
            512 => E512::get(rhs).u64(),
            1024 => E1024::get(rhs).u64(),
            _ => {
                panic!("unsupported length: {}", rhs.len());
            }
        };
        expected_op_vnclip(lhs, x, result)
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

    run_template_v_wv(exp_op, op, true, "vnclip.wv");
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

    run_template_v_wx(expected_op_vnclip, op, true, "vnclip.wx");
}
fn test_vnclip_wi() {
    fn exp_op(lhs: &[u8], x: i64, result: &mut [u8]) {
        expected_op_vnclip(lhs, x as u64, result);
    }
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                17 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 17, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 17");
                    }
                    _ => panic!("Abort"),
                },
                18 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 18, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 18");
                    }
                    _ => panic!("Abort"),
                },
                19 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 19, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 19");
                    }
                    _ => panic!("Abort"),
                },
                20 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 20, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 20");
                    }
                    _ => panic!("Abort"),
                },
                21 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 21, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 21");
                    }
                    _ => panic!("Abort"),
                },
                22 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 22, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 22");
                    }
                    _ => panic!("Abort"),
                },
                23 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 23, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 23");
                    }
                    _ => panic!("Abort"),
                },
                24 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 24, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 24");
                    }
                    _ => panic!("Abort"),
                },
                25 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 25, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 25");
                    }
                    _ => panic!("Abort"),
                },
                26 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 26, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 26");
                    }
                    _ => panic!("Abort"),
                },
                27 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 27, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 27");
                    }
                    _ => panic!("Abort"),
                },
                28 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 28, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 28");
                    }
                    _ => panic!("Abort"),
                },
                29 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 29, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 29");
                    }
                    _ => panic!("Abort"),
                },
                30 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 30, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 30");
                    }
                    _ => panic!("Abort"),
                },
                31 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vnclip.wi v24, v8, 31, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vnclip.wi v24, v8, 31");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_wi(exp_op, op, "vnclip.wi");
}

pub fn test_narrowing_fixed_point_clip() {
    test_vnclipu_wv();
    test_vnclipu_wx();
    test_vnclipu_wi();
    test_vnclip_wv();
    test_vnclip_wx();
    test_vnclip_wi();
}
