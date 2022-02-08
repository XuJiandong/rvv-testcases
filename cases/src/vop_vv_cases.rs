use alloc::vec::Vec;
use ckb_std::syscalls::debug;
use core::arch::asm;
use core::convert::TryInto;
use num_bigint::BigUint;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::{vop_vv, VInstructionOp};
use rvv_testcases::log;
use rvv_testcases::misc::log2;
use rvv_testcases::misc::{Widening, U1024, U256, U512};
use rvv_testcases::runner::{run_vop_vv, WideningCategory};

#[allow(dead_code)]
fn copy_biguint(u: &BigUint, buf: &mut [u8]) {
    let bytes = u.to_bytes_le();
    buf[0..bytes.len()].copy_from_slice(bytes.as_ref());
}

fn expected_op_add(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
    match lhs.len() {
        1 => {
            result[0] = lhs[0] + rhs[0];
        }
        2 => {
            let r = u16::from_le_bytes(lhs.try_into().unwrap())
                + u16::from_le_bytes(rhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = u32::from_le_bytes(lhs.try_into().unwrap())
                + u32::from_le_bytes(rhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = u64::from_le_bytes(lhs.try_into().unwrap())
                + u64::from_le_bytes(rhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = u128::from_le_bytes(lhs.try_into().unwrap())
                + u128::from_le_bytes(rhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let (r, _) =
                U256::from_little_endian(lhs).overflowing_add(U256::from_little_endian(rhs));
            r.to_little_endian(result);
        }
        64 => {
            let (r, _) =
                U512::from_little_endian(lhs).overflowing_add(U512::from_little_endian(rhs));
            r.to_little_endian(result);
        }
        128 => {
            let (r, _) =
                U1024::from_little_endian(lhs).overflowing_add(U1024::from_little_endian(rhs));
            r.to_little_endian(result);
        }
        _ => {
            panic!("Invalid sew");
        }
    }
}

#[allow(dead_code)]
pub fn test_vop_vv_by_inputs(avl: usize, lmul: i64, t: VInstructionOp, sew: u64) {
    let modulus = BigUint::from(1u32) << sew;
    let shift_amount = log2(sew as usize / 8) as u64;

    let avl_bytes = sew as usize / 8 * avl * 2; // double, for widening operations
    let sew_bytes = sew as usize / 8;
    let mut lhs = Vec::<u8>::new();
    lhs.resize(avl_bytes, 0u8);
    let mut rhs = Vec::<u8>::new();
    rhs.resize(avl_bytes, 0u8);
    let mut expected = Vec::<u8>::new();
    expected.resize(avl_bytes, 0u8);
    let mut result = Vec::<u8>::new();
    result.resize(avl_bytes, 0u8);

    for i in 0..avl {
        // TODO: randomize it
        // TODO: BigUint is very slow
        let operand1 = BigUint::from(i) % &modulus;
        let operand2 = BigUint::from(i) % &modulus;
        copy_biguint(&operand1, &mut lhs[i * sew_bytes..(i + 1) * sew_bytes]);
        copy_biguint(&operand2, &mut rhs[i * sew_bytes..(i + 1) * sew_bytes]);

        let expected_result = match t {
            VInstructionOp::Add => (operand1 + operand2) % &modulus,
            VInstructionOp::Sub => {
                if operand1 >= operand2 {
                    operand1 - operand2
                } else {
                    operand1 + &modulus - operand2
                }
            }
            VInstructionOp::And => operand1 & operand2,
            VInstructionOp::Or => operand1 | operand2,
            VInstructionOp::Xor => operand1 ^ operand2,
            _ => panic!("Invalid"),
        };
        // log!("expected_result = {:?}", &expected_result);
        copy_biguint(
            &expected_result,
            &mut expected[i * sew_bytes..(i + 1) * sew_bytes],
        );
    }
    // log!("setting vtype to {}", vtype);
    // log!("shift_amount = {}", shift_amount);
    match t {
        VInstructionOp::Add => {
            vop_vv(&lhs, &rhs, &mut result, sew, avl as u64, lmul, || unsafe {
                rvv_asm!("vadd.vv v21, v1, v11");
            });
        }
        VInstructionOp::Sub => {
            vop_vv(&lhs, &rhs, &mut result, sew, avl as u64, lmul, || unsafe {
                rvv_asm!("vsub.vv v21, v1, v11");
            });
        }
        _ => {
            unreachable!();
        }
    }

    for i in 0..avl {
        let left = &lhs[i * sew_bytes..(i + 1) * sew_bytes];
        let right = &rhs[i * sew_bytes..(i + 1) * sew_bytes];

        let res = &result[i * sew_bytes..(i + 1) * sew_bytes];
        let exp = &expected[i * sew_bytes..(i + 1) * sew_bytes];
        if res != exp {
            log!(
                "[sew = {}, op = {:?}] unexpected values found at index {} (nth-element): {:?} (result) {:?} (expected)",
                sew, &t, i, res, exp
            );
            log!(
                "more information, lhs = {:?}, rhs = {:?}, shift_amount = {}, lmul = {}, avl = {}",
                left,
                right,
                shift_amount,
                lmul,
                avl
            );
            panic!("Abort");
        }
    }
}

fn test_single_width_averaging_add_and_subtract() {
    // vaaddu.vv
    fn vaaddu_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vaaddu.vv v21, v1, v11");
        });
    }
    pub fn expected_vaaddu_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
        let l = U256::from_little_endian(lhs);
        let r = U256::from_little_endian(rhs);
        match lhs.len() {
            32 => {
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
    run_vop_vv(
        256,
        1,
        100,
        expected_vaaddu_vv,
        vaaddu_vv,
        WideningCategory::None,
        "vaaddu.vv",
    );

    // vaadd.vv
    fn vaadd_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vaadd.vv v21, v1, v11");
        });
    }
    pub fn expected_vaadd_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
        let l = U256::from_little_endian(lhs);
        let r = U256::from_little_endian(rhs);
        match lhs.len() {
            32 => {
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
    run_vop_vv(
        256,
        1,
        100,
        expected_vaadd_vv,
        vaadd_vv,
        WideningCategory::None,
        "vaadd.vv",
    );

    // vasubu.vv
    fn vasubu_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vasubu.vv v21, v1, v11");
        });
    }
    pub fn expected_vasubu_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
        let l = U256::from_little_endian(lhs);
        let r = U256::from_little_endian(rhs);
        match lhs.len() {
            32 => {
                let l: U512 = l.into();
                let r: U512 = r.into();
                let r = l.wrapping_sub(r);
                let r2: U256 = (r >> 1).into();
                r2.to_little_endian(result);
            }
            _ => {
                panic!("expected_op_asubu");
            }
        }
    }
    run_vop_vv(
        256,
        1,
        100,
        expected_vasubu_vv,
        vasubu_vv,
        WideningCategory::None,
        "vasubu.vv",
    );

    // vasub.vv
    fn vasub_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vasub.vv v21, v1, v11");
        });
    }
    pub fn expected_vasub_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(lhs.len() == rhs.len() && rhs.len() == result.len());
        let l = U256::from_little_endian(lhs);
        let r = U256::from_little_endian(rhs);
        match lhs.len() {
            32 => {
                let l: U512 = l.sign_extend();
                let r: U512 = r.sign_extend();
                let (r, _) = l.overflowing_sub(r);
                let r2: U256 = (r >> 1).into();
                r2.to_little_endian(result)
            }
            _ => {
                panic!("expected_op_asub");
            }
        }
    }
    run_vop_vv(
        256,
        1,
        100,
        expected_vasub_vv,
        vasub_vv,
        WideningCategory::None,
        "vasub.vv",
    );
}

pub fn test_vop_vv() {
    test_single_width_averaging_add_and_subtract();

    // test combinations of lmul, sew, avl, etc
    fn add(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vadd.vv v21, v1, v11");
        });
    }
    for lmul in [1, 2, 4, 8] {
        for avl in 99..=100 {
            run_vop_vv(
                256,
                lmul,
                avl,
                expected_op_add,
                add,
                WideningCategory::None,
                "vadd.vv",
            );
        }
    }
}
