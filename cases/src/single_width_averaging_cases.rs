use core::arch::asm;
use rvv_asm::rvv_asm;
use rvv_testcases::misc::{to_i64, to_u64, Widening, U256, U512};
use rvv_testcases::runner::{run_template_v_vv, MaskType};

fn test_vaaddu_vv() {
    fn expected_op(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
        match lhs.len() {
            8 => {
                let l = to_u64(lhs);
                let r = to_u64(rhs);

                let (r, _) = (l as u128).overflowing_add(r as u128);
                let r2 = (r >> 1) as u64;
                result.copy_from_slice(&r2.to_le_bytes());
            }
            32 => {
                let l = U256::from_little_endian(lhs);
                let r = U256::from_little_endian(rhs);

                // widening
                let (r, _) = U512::from(l).overflowing_add(U512::from(r));
                // narrow again
                let r2: U256 = (r >> 1).into();
                r2.to_little_endian(result);
            }
            _ => {
                panic!("expected_op_aaddu");
            }
        }
    }
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vaaddu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vaaddu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op, op, true, "vaaddu.vv");
}

fn test_vaadd_vv() {
    fn expected_op(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
        match lhs.len() {
            8 => {
                let l = to_i64(lhs);
                let r = to_i64(rhs);

                let (r, _) = (l as i128).overflowing_add(r as i128);
                let r2 = (r >> 1) as i64;
                result.copy_from_slice(&r2.to_le_bytes());
            }
            32 => {
                let l = U256::from_little_endian(lhs);
                let r = U256::from_little_endian(rhs);

                let (r, _) = l.sign_extend().overflowing_add(r.sign_extend());
                let r2 = r >> 1;
                let r3: U256 = r2.into();
                r3.to_little_endian(result)
            }
            _ => {
                panic!("expected_op_aadd");
            }
        }
    }
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vaadd.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vaadd.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op, op, true, "vaadd.vv");
}

fn test_vasubu_vv() {
    fn expected_op(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
        match lhs.len() {
            8 => {
                let l = to_u64(lhs) as u128;
                let r = to_u64(rhs) as u128;

                let r = l.wrapping_sub(r);
                let r2 = (r >> 1) as u64;
                result.copy_from_slice(&r2.to_le_bytes());
            }
            32 => {
                let l = U256::from_little_endian(lhs);
                let r = U256::from_little_endian(rhs);

                let l: U512 = l.into();
                let r: U512 = r.into();
                let r = l.wrapping_sub(r);
                let r2: U256 = (r >> 1).into();
                r2.to_little_endian(result);
            }
            _ => {
                panic!("expected_op_aadd");
            }
        }
    }
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vasubu.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vasubu.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op, op, true, "vasubu.vv");
}

fn test_vasub_vv() {
    fn expected_op(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
        match lhs.len() {
            8 => {
                let l = to_i64(lhs) as i128;
                let r = to_i64(rhs) as i128;

                let r = l.wrapping_sub(r);
                let r2 = (r >> 1) as i64;
                result.copy_from_slice(&r2.to_le_bytes());
            }
            32 => {
                let l = U256::from_little_endian(lhs);
                let r = U256::from_little_endian(rhs);

                let l: U512 = l.sign_extend();
                let r: U512 = r.sign_extend();
                let (r, _) = l.overflowing_sub(r);
                let r2: U256 = (r >> 1).into();
                r2.to_little_endian(result)
            }
            _ => {
                panic!("expected_op_aadd");
            }
        }
    }
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vasub.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vasub.vv v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_v_vv(expected_op, op, true, "vasub.vv");
}

pub fn test_single_width_averaging_add_and_subtract() {
    test_vaaddu_vv();
    test_vaadd_vv();
    test_vasubu_vv();
    test_vasub_vv();
}
