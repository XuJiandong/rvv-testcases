use core::{arch::asm, convert::TryInto};
use eint::{Eint, E1024, E128, E256, E512, E64};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::{conver_to_i1024, conver_to_i128, conver_to_i256, conver_to_i512},
    runner::{run_template_v_n, MaskType},
};

// use ckb_std::syscalls::debug;
// use rvv_testcases::log;

fn expected_op_vzext_vf2(lhs: &[u8], _: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() / 2);

    let sew = result.len() * 8;
    match sew {
        16 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap()) as u16;
            result.copy_from_slice(&l.to_le_bytes());
        }
        32 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap()) as u32;
            result.copy_from_slice(&l.to_le_bytes());
        }
        64 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap()) as u64;
            result.copy_from_slice(&l.to_le_bytes());
        }

        128 => {
            E128::from(E64::get(lhs)).put(result);
        }
        256 => {
            E256::from(E128::get(lhs)).put(result);
        }
        512 => {
            E512::from(E256::get(lhs)).put(result);
        }
        1024 => {
            E1024::from(E512::get(lhs)).put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vzext_vf2() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vzext.vf2 v24, v8, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vzext.vf2 v24, v8");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_n(expected_op_vzext_vf2, op, 2, true, "vzext.vf2");
}

fn expected_op_vsext_vf2(lhs: &[u8], _: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() / 2);

    let sew = result.len() * 8;
    match sew {
        16 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap()) as i16;
            result.copy_from_slice(&l.to_le_bytes());
        }
        32 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap()) as i32;
            result.copy_from_slice(&l.to_le_bytes());
        }
        64 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap()) as i64;
            result.copy_from_slice(&l.to_le_bytes());
        }
        128 => {
            conver_to_i128(E64::get(lhs)).put(result);
        }
        256 => {
            conver_to_i256(E128::get(lhs)).put(result);
        }
        512 => {
            conver_to_i512(E256::get(lhs)).put(result);
        }
        1024 => {
            conver_to_i1024(E512::get(lhs)).put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vsext_vf2() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vsext.vf2 v24, v8, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vsext.vf2 v24, v8");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_n(expected_op_vsext_vf2, op, 2, true, "vsext.vf2");
}

fn expected_op_vzext_vf4(lhs: &[u8], _: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() / 4);

    let sew = result.len() * 8;
    match sew {
        32 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap()) as u32;
            result.copy_from_slice(&l.to_le_bytes());
        }
        64 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap()) as u64;
            result.copy_from_slice(&l.to_le_bytes());
        }
        128 => {
            E128::from(u32::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        256 => {
            E256::from(u64::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        512 => {
            E512::from(u128::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        1024 => {
            E1024::from(E256::get(lhs)).put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vzext_vf4() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vzext.vf4 v24, v8, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vzext.vf4 v24, v8");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_n(expected_op_vzext_vf4, op, 4, true, "vzext.vf4");
}

fn expected_op_vsext_vf4(lhs: &[u8], _: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() / 4);

    let sew = result.len() * 8;
    match sew {
        32 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap()) as i32;
            result.copy_from_slice(&l.to_le_bytes());
        }
        64 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap()) as i64;
            result.copy_from_slice(&l.to_le_bytes());
        }

        128 => {
            E128::from(i32::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        256 => {
            E256::from(i64::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        512 => {
            E512::from(i128::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        1024 => {
            E1024::from(conver_to_i1024(conver_to_i512(E256::get(lhs)))).put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vsext_vf4() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vsext.vf4 v24, v8, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vsext.vf4 v24, v8");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_n(expected_op_vsext_vf4, op, 4, true, "vsext.vf4");
}

fn expected_op_vzext_vf8(lhs: &[u8], _: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() / 8);

    let sew = result.len() * 8;
    match sew {
        8 => {
            let l = lhs[0] as u8;
            result.copy_from_slice(&l.to_le_bytes());
        }
        16 => {
            let l = lhs[0] as u16;
            result.copy_from_slice(&l.to_le_bytes());
        }
        32 => {
            let l = lhs[0] as u32;
            result.copy_from_slice(&l.to_le_bytes());
        }
        64 => {
            let l = lhs[0] as u64;
            result.copy_from_slice(&l.to_le_bytes());
        }

        128 => {
            E128::from(u16::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        256 => {
            E256::from(u32::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        512 => {
            E512::from(u64::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        1024 => {
            E1024::from(u128::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vzext_vf8() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vzext.vf8 v24, v8, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vzext.vf8 v24, v8");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_n(expected_op_vzext_vf8, op, 8, true, "vzext.vf8");
}

fn expected_op_vsext_vf8(lhs: &[u8], _: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() / 8);

    let sew = result.len() * 8;
    match sew {
        8 => {
            let l = lhs[0] as i8 as i8;
            result.copy_from_slice(&l.to_le_bytes());
        }
        16 => {
            let l = lhs[0] as i8 as i16;
            result.copy_from_slice(&l.to_le_bytes());
        }
        32 => {
            let l = lhs[0] as i8 as i32;
            result.copy_from_slice(&l.to_le_bytes());
        }
        64 => {
            let l = lhs[0] as i8 as i64;
            result.copy_from_slice(&l.to_le_bytes());
        }

        128 => {
            E128::from(i16::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        256 => {
            E256::from(i32::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        512 => {
            E512::from(i64::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        1024 => {
            E1024::from(i128::from_le_bytes(lhs.try_into().unwrap())).put(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}
fn test_vsext_vf8() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vsext.vf8 v24, v8, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vsext.vf8 v24, v8");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_n(expected_op_vsext_vf8, op, 8, true, "vsext.vf8");
}

pub fn test_integer_extension() {
    test_vzext_vf2();
    test_vsext_vf2();

    test_vzext_vf4();
    test_vsext_vf4();

    test_vzext_vf8();
    test_vsext_vf8();
}
