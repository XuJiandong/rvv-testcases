use core::arch::asm;
use core::convert::TryInto;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vop_vx;
use rvv_testcases::misc::{avl_iterator, Widening, U256};
use rvv_testcases::runner::{run_vop_vx, WideningCategory};

fn expected_op_add(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            result[0] = lhs[0] + x as u8;
        }
        2 => {
            let r = u16::from_le_bytes(lhs.try_into().unwrap()) + x as u16;
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = u32::from_le_bytes(lhs.try_into().unwrap()) + x as u32;
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = u64::from_le_bytes(lhs.try_into().unwrap()) + x as u64;
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = u128::from_le_bytes(lhs.try_into().unwrap()) + x as u128;
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let (r, _) = U256::from_little_endian(lhs).overflowing_add(x.sign_extend());
            r.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

fn test_vadd_vx() {
    fn add(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", 
                     "vadd.vx v24, v8, t0",
                     in (reg) x);
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vx(
                sew,
                lmul,
                avl,
                expected_op_add,
                add,
                WideningCategory::None,
                "vadd.vx",
            );
        }
    }
}

fn expected_op_sub(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            result[0] = lhs[0] - x as u8;
        }
        2 => {
            let r = u16::from_le_bytes(lhs.try_into().unwrap()) - x as u16;
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = u32::from_le_bytes(lhs.try_into().unwrap()) - x as u32;
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = u64::from_le_bytes(lhs.try_into().unwrap()) - x as u64;
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = u128::from_le_bytes(lhs.try_into().unwrap()) - x as u128;
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let (r, _) = U256::from_little_endian(lhs).overflowing_sub(x.sign_extend());
            r.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

fn test_vsub_vx() {
    fn sub(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", 
                     "vsub.vx v24, v8, t0",
                     in (reg) x);
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vx(
                sew,
                lmul,
                avl,
                expected_op_sub,
                sub,
                WideningCategory::None,
                "vsub.vx",
            );
        }
    }
}

fn expected_op_rsub(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len());
    match lhs.len() {
        1 => {
            result[0] = x as u8 - lhs[0];
        }
        2 => {
            let r = x as u16 - u16::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = x as u32 - u32::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = x as u64 - u64::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = x as u128 - u128::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let (r, _) = x
                .sign_extend()
                .overflowing_sub(U256::from_little_endian(lhs));
            r.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

fn test_vrsub_vx() {
    fn rsub(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", 
                     "vrsub.vx v24, v8, t0",
                     in (reg) x);
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vx(
                sew,
                lmul,
                avl,
                expected_op_rsub,
                rsub,
                WideningCategory::None,
                "vrsub.vx",
            );
        }
    }
}

pub fn test_vop_vx() {
    // test combinations of lmul, sew, avl, etc
    test_vadd_vx();
    test_vsub_vx();
    test_vrsub_vx();
}
