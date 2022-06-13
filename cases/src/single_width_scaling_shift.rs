use core::{arch::asm, convert::TryInto};
use eint::{Eint, E1024, E128, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::runner::{run_template_v_vi, run_template_v_vv, run_template_v_vx, MaskType};

fn expected_op_vssrl_vx(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() * 8 {
        8 => {
            let res = lhs[0].wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let res = u16::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let res = u32::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let res = u64::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        128 => {
            let l = E128::get(lhs);
            let r = l.wrapping_shr(x as u32);
            r.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = l.wrapping_shr(x as u32);
            r.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = l.wrapping_shr(x as u32);
            r.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
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
    match lhs.len() * 8 {
        8 => {
            let res = lhs[0].wrapping_shr(rhs[0] as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let x = u16::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = u16::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let x = u32::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = u32::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let x = u64::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = u64::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x);
            result.copy_from_slice(&res.to_le_bytes());
        }
        128 => {
            let x = E128::get(rhs).u32();
            let l = E128::get(lhs);
            let r = l.wrapping_shr(x);
            r.put(result);
        }
        256 => {
            let x = E256::get(rhs).u32();
            let l = E256::get(lhs);
            let r = l.wrapping_shr(x);
            r.put(result);
        }
        512 => {
            let x = E512::get(rhs).u32();
            let l = E512::get(lhs);
            let r = l.wrapping_shr(x);
            r.put(result);
        }
        1024 => {
            let x = E1024::get(rhs).u32();
            let l = E1024::get(lhs);
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
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                17 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 17, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 17");
                    }
                    _ => panic!("Abort"),
                },
                18 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 18, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 18");
                    }
                    _ => panic!("Abort"),
                },
                19 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 19, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 19");
                    }
                    _ => panic!("Abort"),
                },
                20 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 20, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 20");
                    }
                    _ => panic!("Abort"),
                },
                21 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 21, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 21");
                    }
                    _ => panic!("Abort"),
                },
                22 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 22, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 22");
                    }
                    _ => panic!("Abort"),
                },
                23 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 23, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 23");
                    }
                    _ => panic!("Abort"),
                },
                24 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 24, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 24");
                    }
                    _ => panic!("Abort"),
                },
                25 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 25, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 25");
                    }
                    _ => panic!("Abort"),
                },
                26 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 26, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 26");
                    }
                    _ => panic!("Abort"),
                },
                27 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 27, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 27");
                    }
                    _ => panic!("Abort"),
                },
                28 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 28, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 28");
                    }
                    _ => panic!("Abort"),
                },
                29 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 29, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 29");
                    }
                    _ => panic!("Abort"),
                },
                30 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 30, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 30");
                    }
                    _ => panic!("Abort"),
                },
                31 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssrl.vi v24, v8, 31, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssrl.vi v24, v8, 31");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_vi(expected_op_vssrl_vi, op, true, false, "vssrl.vi");
}

fn expected_op_vssra_vx(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() * 8 {
        8 => {
            let res = (lhs[0] as i8).wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let res = i16::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let res = i32::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let res = i64::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        128 => {
            let l = E128::get(lhs);
            let r = l.wrapping_sra(x as u32);
            r.put(result);
        }
        256 => {
            let l = E256::get(lhs);
            let r = l.wrapping_sra(x as u32);
            r.put(result);
        }
        512 => {
            let l = E512::get(lhs);
            let r = l.wrapping_sra(x as u32);
            r.put(result);
        }
        1024 => {
            let l = E1024::get(lhs);
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
    match lhs.len() * 8 {
        8 => {
            let res = (lhs[0] as i8).wrapping_shr(rhs[0] as u32);
            result.copy_from_slice(&res.to_le_bytes());
        }
        16 => {
            let x = u16::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = i16::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x);
            result.copy_from_slice(&res.to_le_bytes());
        }
        32 => {
            let x = u32::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = i32::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x);
            result.copy_from_slice(&res.to_le_bytes());
        }
        64 => {
            let x = u64::from_le_bytes(rhs.try_into().unwrap()) as u32;
            let res = i64::from_le_bytes(lhs.try_into().unwrap()).wrapping_shr(x);
            result.copy_from_slice(&res.to_le_bytes());
        }
        128 => {
            let x = E128::get(rhs).u32();
            let l = E128::get(lhs);
            let r = l.wrapping_sra(x);
            r.put(result);
        }
        256 => {
            let x = E256::get(rhs).u32();
            let l = E256::get(lhs);
            let r = l.wrapping_sra(x);
            r.put(result);
        }
        512 => {
            let x = E512::get(rhs).u32();
            let l = E512::get(lhs);
            let r = l.wrapping_sra(x);
            r.put(result);
        }
        1024 => {
            let x = E1024::get(rhs).u32();
            let l = E1024::get(lhs);
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
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                17 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 17, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 17");
                    }
                    _ => panic!("Abort"),
                },
                18 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 18, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 18");
                    }
                    _ => panic!("Abort"),
                },
                19 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 19, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 19");
                    }
                    _ => panic!("Abort"),
                },
                20 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 20, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 20");
                    }
                    _ => panic!("Abort"),
                },
                21 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 21, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 21");
                    }
                    _ => panic!("Abort"),
                },
                22 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 22, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 22");
                    }
                    _ => panic!("Abort"),
                },
                23 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 23, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 23");
                    }
                    _ => panic!("Abort"),
                },
                24 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 24, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 24");
                    }
                    _ => panic!("Abort"),
                },
                25 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 25, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 25");
                    }
                    _ => panic!("Abort"),
                },
                26 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 26, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 26");
                    }
                    _ => panic!("Abort"),
                },
                27 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 27, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 27");
                    }
                    _ => panic!("Abort"),
                },
                28 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 28, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 28");
                    }
                    _ => panic!("Abort"),
                },
                29 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 29, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 29");
                    }
                    _ => panic!("Abort"),
                },
                30 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 30, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 30");
                    }
                    _ => panic!("Abort"),
                },
                31 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vssra.vi v24, v8, 31, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vssra.vi v24, v8, 31");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_vi(expected_op_vssra_vi, op, true, false, "vssra.vi");
}

pub fn test_single_with_scaling_shift() {
    test_vssrl_vx();
    test_vssrl_vv();
    test_vssrl_vi();

    test_vssra_vx();
    test_vssra_vv();
    test_vssra_vi();
}
