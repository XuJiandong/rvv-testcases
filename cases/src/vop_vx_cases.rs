use core::arch::asm;
use core::convert::TryInto;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vop_vx;
use rvv_testcases::misc::{U1024, U256, U512};
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
            let (r, _) = U256::from_little_endian(lhs).overflowing_add(U256::from(x));
            r.to_little_endian(result);
        }
        64 => {
            let (r, _) = U512::from_little_endian(lhs).overflowing_add(U512::from(x));
            r.to_little_endian(result);
        }
        128 => {
            let (r, _) = U1024::from_little_endian(lhs).overflowing_add(U1024::from(x));
            r.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vop_vx() {
    // test combinations of lmul, sew, avl, etc
    fn add(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vx(lhs, x, result, sew, avl, lmul, |x: u64| unsafe {
            rvv_asm!("mv t0, {}", 
                     "vadd.vx v21, v1, t0",
                     in (reg) x);
        });
    }
    for lmul in [1, 2, 4, 8] {
        for avl in 99..=100 {
            run_vop_vx(
                256,
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
