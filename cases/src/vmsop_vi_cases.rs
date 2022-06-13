use core::{arch::asm, convert::TryInto};
use eint::{Eint, E1024, E128, E256, E512, E64};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::{conver_to_i1024, conver_to_i128, conver_to_i256, conver_to_i512},
    runner::{run_template_m_vi, MaskType},
};

fn expected_eq(lhs: &[u8], imm: i64, result: &mut bool) {
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());
            *result = l == imm as i8;
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            *result = l == imm as i16;
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            *result = l == imm as i32;
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            *result = l == imm as i64;
        }

        128 => {
            let l = E128::get(lhs);
            let r = conver_to_i128(E64::from(imm as i64));
            *result = l == r;
        }
        256 => {
            let l = E256::get(lhs);
            let r = conver_to_i256(E128::from(imm as i64));
            *result = l == r;
        }
        512 => {
            let l = E512::get(lhs);
            let r = conver_to_i512(E256::from(imm as i64));
            *result = l == r;
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = conver_to_i1024(E512::from(imm as i64));
            *result = l == r;
        }
        _ => {
            panic!("Invalid sew");
        }
    };
}
fn test_vmseq() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmseq.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmseq.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_m_vi(expected_eq, op, true, "vmseq.vi");
}

fn expected_ne(lhs: &[u8], imm: i64, result: &mut bool) {
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());
            *result = l != imm as i8;
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            *result = l != imm as i16;
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            *result = l != imm as i32;
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            *result = l != imm as i64;
        }

        128 => {
            let l = E128::get(lhs);
            let r = conver_to_i128(E64::from(imm as i64));
            *result = l != r;
        }
        256 => {
            let l = E256::get(lhs);
            let r = conver_to_i256(E128::from(imm as i64));
            *result = l != r;
        }
        512 => {
            let l = E512::get(lhs);
            let r = conver_to_i512(E256::from(imm as i64));
            *result = l != r;
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = conver_to_i1024(E512::from(imm as i64));
            *result = l != r;
        }
        _ => {
            panic!("Invalid sew");
        }
    };
}
fn test_vmsne() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsne.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsne.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_m_vi(expected_ne, op, true, "vmsne.vi");
}

fn expected_leu(lhs: &[u8], imm: i64, result: &mut bool) {
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());
            *result = l <= imm as u8;
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            *result = l <= imm as u16;
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            *result = l <= imm as u32;
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            *result = l <= imm as u64;
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::from(imm);
            *result = l.cmp_u(&r).is_le();
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::from(imm);
            *result = l.cmp_u(&r).is_le();
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::from(imm);
            *result = l.cmp_u(&r).is_le();
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::from(imm);
            *result = l.cmp_u(&r).is_le();
        }
        _ => {
            panic!("Invalid sew");
        }
    };
}
fn test_vmsleu() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsleu.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsleu.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_m_vi(expected_leu, op, true, "vmsleu.vi");
}

fn expected_le(lhs: &[u8], imm: i64, result: &mut bool) {
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());
            *result = l <= imm as i8;
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            *result = l <= imm as i16;
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            *result = l <= imm as i32;
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            *result = l <= imm as i64;
        }

        128 => {
            let l = E128::get(lhs);
            let r = conver_to_i128(E64::from(imm as i64));
            *result = l.cmp_s(&r).is_le();
        }
        256 => {
            let l = E256::get(lhs);
            let r = conver_to_i256(E128::from(imm as i64));
            *result = l.cmp_s(&r).is_le();
        }
        512 => {
            let l = E512::get(lhs);
            let r = conver_to_i512(E256::from(imm as i64));
            *result = l.cmp_s(&r).is_le();
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = conver_to_i1024(E512::from(imm as i64));
            *result = l.cmp_s(&r).is_le();
        }
        _ => {
            panic!("Invalid sew");
        }
    };
}
fn test_vmsle() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsle.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsle.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_m_vi(expected_le, op, true, "vmsle.vi");
}

fn expected_gtu(lhs: &[u8], imm: i64, result: &mut bool) {
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());
            *result = l > imm as u8;
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            *result = l > imm as u16;
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            *result = l > imm as u32;
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            *result = l > imm as u64;
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::from(imm);
            *result = l.cmp_u(&r).is_gt();
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::from(imm);
            *result = l.cmp_u(&r).is_gt();
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::from(imm);
            *result = l.cmp_u(&r).is_gt();
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::from(imm);
            *result = l.cmp_u(&r).is_gt();
        }
        _ => {
            panic!("Invalid sew");
        }
    };
}
fn test_vmsgtu() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgtu.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_m_vi(expected_gtu, op, true, "vmsgtu.vi");
}

fn expected_gt(lhs: &[u8], imm: i64, result: &mut bool) {
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());
            *result = l > imm as i8;
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            *result = l > imm as i16;
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            *result = l > imm as i32;
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            *result = l > imm;
        }

        128 => {
            let l = E128::get(lhs);
            let r = conver_to_i128(E64::from(imm as i64));
            *result = l.cmp_s(&r).is_gt();
        }
        256 => {
            let l = E256::get(lhs);
            let r = conver_to_i256(E128::from(imm as i64));
            *result = l.cmp_s(&r).is_gt();
        }
        512 => {
            let l = E512::get(lhs);
            let r = conver_to_i512(E256::from(imm as i64));
            *result = l.cmp_s(&r).is_gt();
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = conver_to_i1024(E512::from(imm as i64));
            *result = l.cmp_s(&r).is_gt();
        }
        _ => {
            panic!("Invalid sew");
        }
    };
}
fn test_vmsgt() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vmsgt.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vmsgt.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
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
