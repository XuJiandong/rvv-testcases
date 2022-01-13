#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(unchecked_math)]

use alloc::vec::Vec;

use ckb_std::debug;
use ckb_std::default_alloc;
use num_bigint::BigUint;
use num_traits::FromPrimitive;
use num_traits::{One, Zero};
use rvv_asm::rvv_asm;
use rvv_testcases::misc::{create_vtype, log2};

ckb_std::entry!(program_entry);
default_alloc!();

#[derive(Debug, Copy, Clone)]
enum VInstructionOp {
    Add,
    Sub,
    And,
    Or,
    Xor,
    ShiftLeft,
    ShiftRight,
    ShiftRightArithmetic,
    Invalid,
}

fn vop_vv(
    lhs: &[u8],
    rhs: &[u8],
    result: &mut [u8],
    avl: u64,
    vtype: u64,
    t: VInstructionOp,
    shift_amount: u64,
) {
    unsafe {
        rvv_asm!("mv t0, {0}" , in (reg) lhs.as_ref().as_ptr());
        rvv_asm!("mv t1, {0}", in (reg) rhs.as_ref().as_ptr());
        rvv_asm!("mv t2, {0}", in (reg) result.as_ref().as_ptr());
        rvv_asm!("mv t3, {0}", in (reg) avl);
        rvv_asm!("mv t4, {0}", in (reg) vtype);
        rvv_asm!("mv t6, {0}", in (reg) shift_amount);

        rvv_asm!("1:");
        rvv_asm!("vsetvl t5, t3, t4");
        rvv_asm!("sub t3, t3, t5"); // decrease avl

        // convert 'vl' to bytes: vl << shift_amount
        // E.G. U256, it's 32 bytes per element, which is `t5 << 5`
        rvv_asm!("sll t5, t5, t6");

        match shift_amount {
            0 => {
                rvv_asm!("vle8.v v1, (t0)"); // load lhs to v1
                rvv_asm!("vle8.v v11, (t1)"); // load rhs to v11
            }
            1 => {
                rvv_asm!("vle16.v v1, (t0)"); // load lhs to v1
                rvv_asm!("vle16.v v11, (t1)"); // load rhs to v11
            }
            2 => {
                rvv_asm!("vle32.v v1, (t0)"); // load lhs to v1
                rvv_asm!("vle32.v v11, (t1)"); // load rhs to v11
            }
            3 => {
                rvv_asm!("vle64.v v1, (t0)"); // load lhs to v1
                rvv_asm!("vle64.v v11, (t1)"); // load rhs to v11
            }
            4 => {
                rvv_asm!("vle128.v v1, (t0)"); // load lhs to v1
                rvv_asm!("vle128.v v11, (t1)"); // load rhs to v11
            }
            5 => {
                rvv_asm!("vle256.v v1, (t0)"); // load lhs to v1
                rvv_asm!("vle256.v v11, (t1)"); // load rhs to v11
            }
            6 => {
                rvv_asm!("vle512.v v1, (t0)"); // load lhs to v1
                rvv_asm!("vle512.v v11, (t1)"); // load rhs to v11
            }
            7 => {
                rvv_asm!("vle1024.v v1, (t0)"); // load lhs to v1
                rvv_asm!("vle1024.v v11, (t1)"); // load rhs to v11
            }
            _ => {
                panic!("Invalid shift_amount");
            }
        }

        rvv_asm!("add t0, t0, t5"); // increase lhs
        rvv_asm!("add t1, t1, t5"); // increase rhs

        // be careful while using this
        match t {
            VInstructionOp::Add => {
                rvv_asm!("vadd.vv v21, v1, v11"); // ADD
            }
            VInstructionOp::Sub => {
                rvv_asm!("vsub.vv v21, v1, v11"); // SUB
            }
            VInstructionOp::And => {
                rvv_asm!("vand.vv v21, v1, v11"); // AND
            }
            VInstructionOp::Or => {
                rvv_asm!("vor.vv v21, v1, v11"); // OR
            }
            VInstructionOp::Xor => {
                rvv_asm!("vxor.vv v21, v1, v11"); // XOR
            }
            VInstructionOp::ShiftLeft => {
                rvv_asm!("vsll.vv v21, v1, v11"); // shift left
            }
            VInstructionOp::ShiftRight => {
                rvv_asm!("vsrl.vv v21, v1, v11"); // shift right
            }
            VInstructionOp::ShiftRightArithmetic => {
                rvv_asm!("vsra.vv v21, v1, v11"); // shift right arithmetic
            }
            VInstructionOp::Invalid => {
                panic!("Invalid");
            }
        }
        // store v21 to result
        match shift_amount {
            0 => {
                rvv_asm!("vse8.v v21, (t2)");
            }
            1 => {
                rvv_asm!("vse16.v v21, (t2)");
            }
            2 => {
                rvv_asm!("vse32.v v21, (t2)");
            }
            3 => {
                rvv_asm!("vse64.v v21, (t2)");
            }
            4 => {
                rvv_asm!("vse128.v v21, (t2)");
            }
            5 => {
                rvv_asm!("vse256.v v21, (t2)");
            }
            6 => {
                rvv_asm!("vse512.v v21, (t2)");
            }
            7 => {
                rvv_asm!("vse1024.v v21, (t2)");
            }
            _ => {
                panic!("Invalid shift_amount");
            }
        }
        rvv_asm!("add t2, t2, t5"); // increase result
        rvv_asm!("bnez t3, 1b"); // finished?
    }
}

fn add(lhs: u64, rhs: u64) -> u64 {
    let mut result: u64;
    unsafe {
        asm!("add {0}, {1}, {2}", out(reg) result, in (reg) lhs, in (reg) rhs);
    }
    result
}

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
    debug!("test_add_array, result = {}", result[0]);
    assert_eq!(result[0], 3);
    assert_eq!(result[1], 5);
    assert_eq!(result[2], 7);
    assert_eq!(result[3], 9);
}

fn test_add() {
    let lhs = 1u64;
    let rhs = 0u64;
    let result = add(lhs, rhs);
    debug!("test_add, result = {}", result);
    assert_eq!(result, lhs + rhs);
}

fn copy_biguint(u: &BigUint, buf: &mut [u8]) {
    let bytes = u.to_bytes_le();
    buf[0..bytes.len()].copy_from_slice(bytes.as_ref());
}

fn test_vop_vv_by_avl(avl: usize, lmul: i64, t: VInstructionOp, sew: u64) {
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
        let operand1 = BigUint::from(i);
        let operand2 = BigUint::from(i);

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
        // debug!("expected_result = {:?}", &expected_result);
        copy_biguint(
            &expected_result,
            &mut expected[i * sew_bytes..(i + 1) * sew_bytes],
        );
    }

    let vtype = create_vtype(sew, lmul);
    // debug!("setting vtype to {}", vtype);
    // debug!("shift_amount = {}", shift_amount);
    vop_vv(&lhs, &rhs, &mut result, avl as u64, vtype, t, shift_amount);

    for i in 0..avl {
        let left = &lhs[i * sew_bytes..(i + 1) * sew_bytes];
        let right = &rhs[i * sew_bytes..(i + 1) * sew_bytes];

        let res = &result[i * sew_bytes..(i + 1) * sew_bytes];
        let exp = &expected[i * sew_bytes..(i + 1) * sew_bytes];
        if res != exp {
            debug!(
                "[sew = {}, op = {:?}] unexpected values found at index {} (nth-element): {:?} (result) {:?} (expected)",
                sew, &t, i, res, exp
            );
            debug!(
                "more information, lhs = {:?}, rhs = {:?}, shift_amount = {}, lmul = {}, avl = {}",
                left, right, shift_amount, lmul, avl
            );
            panic!("Abort");
        }
    }
}

fn test_vop_vv() {
    debug!("test_vop_vv, start ...");
    debug!("add, sub ...");
    for i in 80..100 {
        let op = match i % 2 {
            0 => VInstructionOp::Add,
            1 => VInstructionOp::Sub,
            _ => VInstructionOp::Invalid,
        };
        let sew = 1 << (i % 8 + 3); // 8, 16, ..., 1024
        test_vop_vv_by_avl(i, 2, op, sew);
    }
    // debug!("bit and, or, xor ...");
    // for i in 80..100 {
    //     let op = match i % 3 {
    //         0 => VInstructionOp::And,
    //         1 => VInstructionOp::Or,
    //         2 => VInstructionOp::Xor,
    //         _ => VInstructionOp::Invalid,
    //     };
    //     test_vop_vv_by_avl(i, 2, op, 8);
    // }
    // debug!("Shift left, shift right ...");
    // for i in 80..100 {
    //     let op = match i % 3 {
    //         0 => VInstructionOp::ShiftLeft,
    //         1 => VInstructionOp::ShiftRight,
    //         2 => VInstructionOp::ShiftRightArithmetic,
    //         _ => VInstructionOp::Invalid,
    //     };
    //     test_vop_vv_by_avl(i, 2, op, 8);
    // }
    debug!("test_vop_vv, done");
}

fn program_entry() -> i8 {
    test_add();
    test_add_array();
    test_vop_vv();
    0
}
