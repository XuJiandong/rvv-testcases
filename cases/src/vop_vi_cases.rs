use core::arch::asm;
use core::convert::TryInto;
use eint::{Eint, E1024, E128, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::runner::{run_template_v_vi, MaskType};

fn expected_op_add(lhs: &[u8], imm: i64, result: &mut [u8]) {
    assert!(lhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let (r, _) = u8::from_le_bytes(lhs.try_into().unwrap()).overflowing_add(imm as u8);
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let (r, _) = u16::from_le_bytes(lhs.try_into().unwrap()).overflowing_add(imm as u16);
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let (r, _) = u32::from_le_bytes(lhs.try_into().unwrap()).overflowing_add(imm as u32);
            result.copy_from_slice(&r.to_le_bytes());
        }
        64 => {
            let (r, _) = u64::from_le_bytes(lhs.try_into().unwrap()).overflowing_add(imm as u64);
            result.copy_from_slice(&r.to_le_bytes());
        }

        128 => {
            let x = imm as u64;
            let (r, _) = E128::get(lhs).overflowing_add_u(E128::from(x as i64));
            r.put(result);
        }
        256 => {
            let x = imm as u64;
            let (r, _) = E256::get(lhs).overflowing_add_u(E256::from(x as i64));
            r.put(result);
        }
        512 => {
            let x = imm as u64;
            let (r, _) = E512::get(lhs).overflowing_add_u(E512::from(x as i64));
            r.put(result);
        }
        1024 => {
            let x = imm as u64;
            let (r, _) = E1024::get(lhs).overflowing_add_u(E1024::from(x as i64));
            r.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vadd_vi() {
    // test combinations of lmul, sew, avl, etc
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }
    run_template_v_vi(expected_op_add, op, true, true, "vadd.vi");
}

fn expected_op_sub(lhs: &[u8], imm: i64, result: &mut [u8]) {
    assert!(lhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let (r, _) = (imm as u8).overflowing_sub(u8::from_le_bytes(lhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let (r, _) = (imm as u16).overflowing_sub(u16::from_le_bytes(lhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let (r, _) = (imm as u32).overflowing_sub(u32::from_le_bytes(lhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        64 => {
            let (r, _) = (imm as u64).overflowing_sub(u64::from_le_bytes(lhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        128 => {
            let x = imm as u64;
            let (r, _) = E128::from(x as i64).overflowing_sub_u(E128::get(lhs));
            r.put(result);
        }
        256 => {
            let x = imm as u64;
            let (r, _) = E256::from(x as i64).overflowing_sub_u(E256::get(lhs));
            r.put(result);
        }
        512 => {
            let x = imm as u64;
            let (r, _) = E512::from(x as i64).overflowing_sub_u(E512::get(lhs));
            r.put(result);
        }
        1024 => {
            let x = imm as u64;
            let (r, _) = E1024::from(x as i64).overflowing_sub_u(E1024::get(lhs));
            r.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vrsub_vi() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_vi(expected_op_sub, op, true, true, "vrsub.vi");
}

fn expected_op_and(lhs: &[u8], imm: i64, result: &mut [u8]) {
    assert!(lhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let r = imm as u8 & u8::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = imm as u16 & u16::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let r = imm as u32 & u32::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        64 => {
            let r = imm as u64 & u64::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }

        128 => {
            let x = imm as i64;
            let r = E128::from(x) & E128::get(lhs);
            r.put(result);
        }
        256 => {
            let x = imm as i64;
            let r = E256::from(x) & E256::get(lhs);
            r.put(result);
        }
        512 => {
            let x = imm as i64;
            let r = E512::from(x) & E512::get(lhs);
            r.put(result);
        }
        1024 => {
            let x = imm as i64;
            let r = E1024::from(x) & E1024::get(lhs);
            r.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vand_vi() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_vi(expected_op_and, op, true, true, "vand.vi");
}

fn expected_op_or(lhs: &[u8], imm: i64, result: &mut [u8]) {
    assert!(lhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let r = imm as u8 | u8::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = imm as u16 | u16::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let r = imm as u32 | u32::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        64 => {
            let r = imm as u64 | u64::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }

        128 => {
            let x = imm as i64;
            let r = E128::from(x) | E128::get(lhs);
            r.put(result);
        }
        256 => {
            let x = imm as i64;
            let r = E256::from(x) | E256::get(lhs);
            r.put(result);
        }
        512 => {
            let x = imm as i64;
            let r = E512::from(x) | E512::get(lhs);
            r.put(result);
        }
        1024 => {
            let x = imm as i64;
            let r = E1024::from(x) | E1024::get(lhs);
            r.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vor_vi() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_vi(expected_op_or, op, true, true, "vor.vi");
}

fn expected_op_xor(lhs: &[u8], imm: i64, result: &mut [u8]) {
    assert!(lhs.len() == result.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let r = imm as u8 ^ u8::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = imm as u16 ^ u16::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let r = imm as u32 ^ u32::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        64 => {
            let r = imm as u64 ^ u64::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }

        128 => {
            let x = imm as i64;
            let r = E128::from(x) ^ E128::get(lhs);
            r.put(result);
        }
        256 => {
            let x = imm as i64;
            let r = E256::from(x) ^ E256::get(lhs);
            r.put(result);
        }
        512 => {
            let x = imm as i64;
            let r = E512::from(x) ^ E512::get(lhs);
            r.put(result);
        }
        1024 => {
            let x = imm as i64;
            let r = E1024::from(x) ^ E1024::get(lhs);
            r.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vxor_vi() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_vi(expected_op_xor, op, true, true, "vxor.vi");
}

pub fn test_vop_vi() {
    test_vadd_vi();
    test_vrsub_vi();
    test_vand_vi();
    test_vor_vi();
    test_vxor_vi();
}
