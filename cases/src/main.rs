#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(unchecked_math)]
#![feature(asm_sym)]

use alloc::vec::Vec;
use core::arch::asm;
use rvv_asm::rvv_asm;

use ckb_std::default_alloc;
use ckb_std::syscalls::debug;
use num_bigint::BigUint;
use rvv_testcases::intrinsic::{vop_vv, VInstructionOp};
use rvv_testcases::log;
use rvv_testcases::misc::log2;
use rvv_testcases::misc::{U256, U512};
use rvv_testcases::runner::{expected_op_add, run};

ckb_std::entry!(program_entry);
default_alloc!();

fn add_array(lhs: &[u64], rhs: &[u64], result: &mut [u64]) {
    let len = lhs.len();
    unsafe {
        // make it easily spotted
        asm!("nop");
        asm!("nop");
        asm!("nop");
        asm!("mv t5, {0}",  in(reg) len);
        asm!("mv t0, {0}", in(reg) lhs.as_ref().as_ptr());
        asm!("mv t1, {0}", in(reg) rhs.as_ref().as_ptr());
        asm!("mv t6, {0}", in(reg) result.as_ref().as_ptr());
        asm!("1: ld t2, 0(t0)");
        asm!("ld t3, 0(t1)");
        asm!("add t2, t2, t3");
        asm!("sd t2, 0(t6)");
        asm!("addi t0, t0, 8");
        asm!("addi t1, t1, 8");
        asm!("addi t6, t6, 8");
        asm!("addi t5, t5, -1");
        asm!("bnez t5, 1b");
        // make it easily spotted
        asm!("nop");
        asm!("nop");
        asm!("nop");
    }
}

fn test_add_array() {
    let mut result = [0u64; 4];
    let lhs = [1, 2, 3, 4];
    let rhs = [2, 3, 4, 5];
    add_array(&lhs, &rhs, &mut result);
    log!("test_add_array, result = {}", result[0]);
    assert_eq!(result[0], 3);
    assert_eq!(result[1], 5);
    assert_eq!(result[2], 7);
    assert_eq!(result[3], 9);
}

fn add(lhs: u64, rhs: u64) -> u64 {
    let mut result: u64;
    unsafe {
        asm!("add {0}, {1}, {2}", out(reg) result, in (reg) lhs, in (reg) rhs);
    }
    result
}

fn test_add() {
    let lhs = 1u64;
    let rhs = 0u64;
    let result = add(lhs, rhs);
    log!("test_add, result = {}", result);
    assert_eq!(result, lhs + rhs);
}

fn copy_biguint(u: &BigUint, buf: &mut [u8]) {
    let bytes = u.to_bytes_le();
    buf[0..bytes.len()].copy_from_slice(bytes.as_ref());
}

fn test_vop_vv_by_inputs(avl: usize, lmul: i64, t: VInstructionOp, sew: u64) {
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

fn test_vop_vv() {
    log!("test_vop_vv, start ...");
    log!("add, sub ...");

    // for lmul in [1, 2, 4] {
    //     for sew in [8, 16, 32, 64, 128, 256, 512, 1024] {
    //         let avl = VLEN * lmul / (sew / 8);
    //         log!("test lmul = {}, sew = {}", lmul, sew);
    //         test_vop_vv_by_inputs(avl, lmul as i64, VInstructionOp::Add, sew as u64);
    //         test_vop_vv_by_inputs(avl, lmul as i64, VInstructionOp::Sub, sew as u64);
    //     }
    // }
    test_vop_vv_by_inputs(100, 1, VInstructionOp::Add, 256);
    test_vop_vv_by_inputs(100, 1, VInstructionOp::Sub, 256);

    // using runner:
    // vadd.vv
    fn add(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_vv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vadd.vv v21, v1, v11");
        });
    }
    run(256, 1, 100, expected_op_add, add, "vadd.vv");

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
    run(256, 1, 100, expected_vaaddu_vv, vaaddu_vv, "vaaddu.vv");

    // vaadd.vv
    // bug
    /*
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
    run(256, 1, 100, expected_vaadd_vv, vaadd_vv, "vaadd.vv");
    */

    // vasubu.vv
    // bug
    /*
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
                let r = l.wrapping_sub(r);
                (r >> 1).to_little_endian(result);
            }
            _ => {
                panic!("expected_op_aaddu");
            }
        }
    }
    run(256, 1, 100, expected_vasubu_vv, vasubu_vv, "vasubu.vv");
    */

    log!("test_vop_vv, done");
}

fn program_entry() -> i8 {
    test_add();
    test_add_array();
    test_vop_vv();
    0
}
