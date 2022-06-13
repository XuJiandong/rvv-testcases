use core::arch::asm;
use core::convert::TryInto;
use eint::{Eint, E1024, E128, E2048, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::misc::{conver_to_i1024, conver_to_i2048, conver_to_i256, conver_to_i512};
use rvv_testcases::runner::{run_template_wr_vw, MaskType};

// use ckb_std::syscalls::debug;
// use rvv_testcases::log;

fn expected_op_sumu(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() * 2 == rhs.len() && rhs.len() == result.len());

    let lhs = {
        let len = lhs.len();
        let mut l = lhs.to_vec();
        l.resize(len * 2, 0);
        l
    };

    let rhs = if index == 0 {
        rhs.to_vec()
    } else {
        result.to_vec()
    };

    let sew = lhs.len() * 8 / 2;
    match sew {
        8 => {
            let l = u16::from_le_bytes(lhs.as_slice().try_into().unwrap());
            let r = u16::from_le_bytes(rhs.as_slice().try_into().unwrap());
            let res2 = { l.wrapping_add(r) };

            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l = u32::from_le_bytes(lhs.as_slice().try_into().unwrap());
            let r = u32::from_le_bytes(rhs.as_slice().try_into().unwrap());
            let res2 = { l.wrapping_add(r) };

            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = u64::from_le_bytes(lhs.as_slice().try_into().unwrap());
            let r = u64::from_le_bytes(rhs.as_slice().try_into().unwrap());
            let res2 = { l.wrapping_add(r) };

            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l = u128::from_le_bytes(lhs.as_slice().try_into().unwrap());
            let r = u128::from_le_bytes(rhs.as_slice().try_into().unwrap());
            let res2 = { l.wrapping_add(r) };

            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = E256::get(lhs.as_slice());
            let r = E256::get(rhs.as_slice());

            let (res2, _) = l.overflowing_add_u(r);
            res2.put(result);
        }
        256 => {
            let l = E512::get(lhs.as_slice());
            let r = E512::get(rhs.as_slice());

            let (res2, _) = l.overflowing_add_u(r);
            res2.put(result);
        }
        512 => {
            let l = E1024::get(lhs.as_slice());
            let r = E1024::get(rhs.as_slice());

            let (res2, _) = l.overflowing_add_u(r);
            res2.put(result);
        }
        1024 => {
            let l = E2048::get(lhs.as_slice());
            let r = E2048::get(rhs.as_slice());

            let (res2, _) = l.overflowing_add_u(r);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
pub fn test_vwredsumu_vs() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vwredsumu.vs v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vwredsumu.vs v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_wr_vw(expected_op_sumu, op, true, "vwredsumu.vs");
}

fn expected_op_sum(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() * 2 == rhs.len() && rhs.len() == result.len());
    let rhs = if index == 0 {
        rhs.to_vec()
    } else {
        result.to_vec()
    };

    let sew = lhs.len() * 8;
    match sew {
        8 => {
            let l = i8::from_le_bytes(lhs.try_into().unwrap()) as i16;
            let r = i16::from_le_bytes(rhs.try_into().unwrap());
            let res2 = l.wrapping_add(r);

            result.copy_from_slice(&res2.to_le_bytes());
        }
        16 => {
            let l = i16::from_le_bytes(lhs.try_into().unwrap()) as i32;
            let r = i32::from_le_bytes(rhs.try_into().unwrap());
            let res2 = l.wrapping_add(r);

            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = i32::from_le_bytes(lhs.try_into().unwrap()) as i64;
            let r = i64::from_le_bytes(rhs.try_into().unwrap());
            let res2 = l.wrapping_add(r);

            result.copy_from_slice(&res2.to_le_bytes());
        }
        64 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = i128::from_le_bytes(rhs.try_into().unwrap());
            let res2 = l.wrapping_add(r);

            result.copy_from_slice(&res2.to_le_bytes());
        }

        128 => {
            let l = conver_to_i256(E128::get(lhs));
            let r = E256::get(&rhs);

            let res2 = l.wrapping_add(r);
            res2.put(result);
        }
        256 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = E512::get(&rhs);

            let res2 = l.wrapping_add(r);
            res2.put(result);
        }
        512 => {
            let l = conver_to_i1024(E512::get(lhs));
            let r = E1024::get(&rhs);

            let res2 = l.wrapping_add(r);
            res2.put(result);
        }
        1024 => {
            let l = conver_to_i2048(E1024::get(lhs));
            let r = E2048::get(&rhs);

            let res2 = l.wrapping_add(r);
            res2.put(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}
pub fn test_vwredsum_vs() {
    fn op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vwredsum.vs v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vwredsum.vs v24, v8, v16");
                }
                _ => panic!("Abort"),
            }
        }
    }
    run_template_wr_vw(expected_op_sum, op, true, "vwredsum.vs");
}
