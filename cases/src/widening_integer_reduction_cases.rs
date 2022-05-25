use core::arch::asm;
use core::convert::TryInto;
use eint::{Eint, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::misc::{conver_to_i512, U512};
use rvv_testcases::runner::{run_template_wr_vw, MaskType};

// use ckb_std::syscalls::debug;
// use rvv_testcases::log;

fn expected_op_sumu(lhs: &[u8], rhs: &[u8], result: &mut [u8], index: usize) {
    assert!(lhs.len() * 2 == rhs.len() && rhs.len() == result.len());
    let len = lhs.len();

    let lhs = {
        let mut l = lhs.to_vec();
        l.resize(len * 2, 0);
        l
    };

    let rhs = if index == 0 {
        rhs.to_vec()
    } else {
        result.to_vec()
    };

    match len {
        4 => {
            let l = u64::from_le_bytes(lhs.as_slice().try_into().unwrap());
            let r = u64::from_le_bytes(rhs.as_slice().try_into().unwrap());
            let res2 = { l.wrapping_add(r) };

            result.copy_from_slice(&res2.to_le_bytes());
        }
        8 => {
            let l = u128::from_le_bytes(lhs.as_slice().try_into().unwrap());
            let r = u128::from_le_bytes(rhs.as_slice().try_into().unwrap());
            let res2 = { l.wrapping_add(r) };

            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = U512::from_little_endian(lhs.as_slice());
            let r = U512::from_little_endian(rhs.as_slice());

            let res2: U512 = l.wrapping_add(r).into();
            res2.to_little_endian(result);
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
    let len = lhs.len();

    let rhs = if index == 0 {
        rhs.to_vec()
    } else {
        result.to_vec()
    };

    match len {
        4 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap());
            let r = i64::from_le_bytes(rhs.try_into().unwrap());
            let res2 = l.wrapping_add(r);

            result.copy_from_slice(&res2.to_le_bytes());
        }
        8 => {
            let l = i64::from_le_bytes(lhs.try_into().unwrap()) as i128;
            let r = i128::from_le_bytes(rhs.try_into().unwrap());
            let res2 = l.wrapping_add(r);

            result.copy_from_slice(&res2.to_le_bytes());
        }
        32 => {
            let l = conver_to_i512(E256::get(lhs));
            let r = E512::get(&rhs);

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
