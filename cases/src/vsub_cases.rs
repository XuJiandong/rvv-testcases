use core::arch::asm;
use core::convert::TryInto;
use eint::{Eint, E1024, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::runner::{run_template_v_vv, MaskType};

fn expected_op_sub(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            result[0] = lhs[0].wrapping_sub(rhs[0]);
        }
        2 => {
            let r = u16::from_le_bytes(lhs.try_into().unwrap())
                .wrapping_sub(u16::from_le_bytes(rhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = u32::from_le_bytes(lhs.try_into().unwrap())
                .wrapping_sub(u32::from_le_bytes(rhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = u64::from_le_bytes(lhs.try_into().unwrap())
                .wrapping_sub(u64::from_le_bytes(rhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = u128::from_le_bytes(lhs.try_into().unwrap())
                .wrapping_sub(u128::from_le_bytes(rhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let (r, _) = E256::get(lhs).overflowing_sub_u(E256::get(rhs));
            r.put(result);
        }
        64 => {
            let (r, _) = E512::get(lhs).overflowing_sub_u(E512::get(rhs));
            r.put(result);
        }
        128 => {
            let (r, _) = E1024::get(lhs).overflowing_sub_u(E1024::get(rhs));
            r.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vsub() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vsub.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vsub.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op_sub, op, true, "vsub.vv");
}
