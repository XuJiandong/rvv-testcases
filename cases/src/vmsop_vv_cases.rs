use core::{arch::asm, convert::TryInto};
use eint::{Eint, E1024, E128, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::runner::{run_template_m_vv, MaskType};

fn expected_eq(lhs: &[u8], rhs: &[u8], result: &mut bool) {
    assert_eq!(lhs.len(), rhs.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());
            let r = u8::from_le_bytes(rhs.try_into().unwrap());
            *result = l == r;
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            let r = u16::from_le_bytes(rhs.try_into().unwrap());
            *result = l == r;
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            let r = u32::from_le_bytes(rhs.try_into().unwrap());
            *result = l == r;
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            *result = l == r;
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            *result = l == r;
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            *result = l == r;
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            *result = l == r;
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            *result = l == r;
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmseq() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vmseq.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vmseq.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_m_vv(expected_eq, op, true, "vmseq.vv");
}

fn expected_ne(lhs: &[u8], rhs: &[u8], result: &mut bool) {
    assert_eq!(lhs.len(), rhs.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());
            let r = u8::from_le_bytes(rhs.try_into().unwrap());
            *result = l != r;
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            let r = u16::from_le_bytes(rhs.try_into().unwrap());
            *result = l != r;
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            let r = u32::from_le_bytes(rhs.try_into().unwrap());
            *result = l != r;
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            *result = l != r;
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            *result = l != r;
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            *result = l != r;
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            *result = l != r;
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            *result = l != r;
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsne() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vmsne.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vmsne.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_m_vv(expected_ne, op, true, "vmsne.vv");
}

fn expected_ltu(lhs: &[u8], rhs: &[u8], result: &mut bool) {
    assert_eq!(lhs.len(), rhs.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());
            let r = u8::from_le_bytes(rhs.try_into().unwrap());
            *result = l < r;
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            let r = u16::from_le_bytes(rhs.try_into().unwrap());
            *result = l < r;
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            let r = u32::from_le_bytes(rhs.try_into().unwrap());
            *result = l < r;
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            *result = l < r;
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            *result = l < r;
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            *result = l < r;
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            *result = l < r;
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            *result = l < r;
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsltu() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vmsltu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vmsltu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_m_vv(expected_ltu, op, true, "vmsltu.vv");
}

fn expected_lt(lhs: &[u8], rhs: &[u8], result: &mut bool) {
    assert_eq!(lhs.len(), rhs.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());
            let r = i8::from_le_bytes(rhs.try_into().unwrap());
            *result = l < r;
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            let r = i16::from_le_bytes(rhs.try_into().unwrap());
            *result = l < r;
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            let r = i32::from_le_bytes(rhs.try_into().unwrap());
            *result = l < r;
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap());
            *result = l < r;
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            *result = l.cmp_s(&r).is_lt();
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            *result = l.cmp_s(&r).is_le();
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            *result = l.cmp_s(&r).is_le();
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            *result = l.cmp_s(&r).is_le();
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmslt() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vmslt.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vmslt.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_m_vv(expected_lt, op, true, "vmslt.vv");
}

fn expected_leu(lhs: &[u8], rhs: &[u8], result: &mut bool) {
    assert_eq!(lhs.len(), rhs.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = u8::from_le_bytes(lhs.try_into().unwrap());
            let r = u8::from_le_bytes(rhs.try_into().unwrap());
            *result = l <= r;
        }
        16 => {
            let l = u16::from_le_bytes(lhs.try_into().unwrap());
            let r = u16::from_le_bytes(rhs.try_into().unwrap());
            *result = l <= r;
        }
        32 => {
            let l = u32::from_le_bytes(lhs.try_into().unwrap());
            let r = u32::from_le_bytes(rhs.try_into().unwrap());
            *result = l <= r;
        }
        64 => {
            let l = u64::from_le_bytes(lhs.try_into().unwrap());
            let r = u64::from_le_bytes(rhs.try_into().unwrap());
            *result = l <= r;
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            *result = l <= r;
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            *result = l <= r;
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            *result = l <= r;
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            *result = l <= r;
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsleu() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vmsleu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vmsleu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_m_vv(expected_leu, op, true, "vmsleu.vv");
}

fn expected_le(lhs: &[u8], rhs: &[u8], result: &mut bool) {
    assert_eq!(lhs.len(), rhs.len());
    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap());
            let r = i8::from_le_bytes(rhs.try_into().unwrap());
            *result = l <= r;
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap());
            let r = i16::from_le_bytes(rhs.try_into().unwrap());
            *result = l <= r;
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap());
            let r = i32::from_le_bytes(rhs.try_into().unwrap());
            *result = l <= r;
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap());
            *result = l <= r;
        }

        128 => {
            let l = E128::get(lhs);
            let r = E128::get(rhs);
            let ord = l.cmp_s(&r);
            *result = ord.is_eq() || ord.is_le();
        }
        256 => {
            let l = E256::get(lhs);
            let r = E256::get(rhs);
            let ord = l.cmp_s(&r);
            *result = ord.is_eq() || ord.is_le();
        }
        512 => {
            let l = E512::get(lhs);
            let r = E512::get(rhs);
            let ord = l.cmp_s(&r);
            *result = ord.is_eq() || ord.is_le();
        }
        1024 => {
            let l = E1024::get(lhs);
            let r = E1024::get(rhs);
            let ord = l.cmp_s(&r);
            *result = ord.is_eq() || ord.is_le();
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vmsle() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vmsle.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vmsle.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_m_vv(expected_le, op, true, "vmsle.vv");
}

pub fn test_vmsop_vv() {
    test_vmseq();
    test_vmsne();
    test_vmsltu();
    test_vmslt();
    test_vmsleu();
    test_vmsle();
}
