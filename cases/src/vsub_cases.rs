use core::arch::asm;
use core::convert::TryInto;

use alloc::boxed::Box;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vop_vv;

use rvv_testcases::misc::{avl_iterator, U1024, U256, U512};
use rvv_testcases::runner::{run_vop_vv, ExpectedOp, WideningCategory};

fn expected_op_sub(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            result[0] = lhs[0] + rhs[0];
        }
        2 => {
            let r = u16::from_le_bytes(lhs.try_into().unwrap())
                - u16::from_le_bytes(rhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = u32::from_le_bytes(lhs.try_into().unwrap())
                - u32::from_le_bytes(rhs.try_into().unwrap());
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
            let (r, _) =
                U256::from_little_endian(lhs).overflowing_sub(U256::from_little_endian(rhs));
            r.to_little_endian(result);
        }
        64 => {
            let (r, _) =
                U512::from_little_endian(lhs).overflowing_sub(U512::from_little_endian(rhs));
            r.to_little_endian(result);
        }
        128 => {
            let (r, _) =
                U1024::from_little_endian(lhs).overflowing_sub(U1024::from_little_endian(rhs));
            r.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

pub fn test_vsub() {
    fn add(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vsub.vv v24, v8, v16");
        });
    }
    let sew = 256u64;
    for lmul in [-8, -2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                ExpectedOp::Normal(Box::new(expected_op_sub)),
                add,
                WideningCategory::None,
                "vsub.vv",
            );
        }
    }
}
