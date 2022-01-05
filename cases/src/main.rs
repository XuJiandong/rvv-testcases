#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use ckb_std::debug;
use ckb_std::default_alloc;

ckb_std::entry!(program_entry);
default_alloc!();

// can't work, should work with library 'rvv'
// fn vadd_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
//     unsafe {
//         asm!("mv a0, {0}\n" , in (reg) lhs.as_ref().as_ptr());
//         asm!("mv a1, {0}\n", in (reg) rhs.as_ref().as_ptr());
//         asm!("mv a2, {0}\n", in (reg) result.as_ref().as_ptr());

//         asm!("1:");
//         asm!("vsetvli t0, a0, e8, m1, ta, ma");
//         asm!("vle8.v v0, (a1)");
//         asm!("sub a0, a0, t0");
//         asm!("slli t0, t0, 5");
//         asm!("add a1, a1, t0");
//         asm!("vle8.v v1, (a2)");
//         asm!("add a2, a2, t0");
//         asm!("vadd.vv v2, v0, v1");
//         asm!("vse8.v v2, (a3)");
//         asm!("add a3, a3, t0");
//         asm!("bnez a0, 1b");
//     }
// }

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
        asm!(
        "mv t5, {0}
        mv t0, {1}
        mv t1, {2}
        mv t6, {3}
        1: 
        ld t2, 0(t0)
        ld t3, 0(t1)
        add t2, t2, t3
        sd t2, 0(t6)
        addi t0, t0, 8
        addi t1, t1, 8
        addi t6, t6, 8
        addi t5, t5, -1
        bnez t5, 1b",
        in(reg) len,
        in(reg) lhs.as_ref().as_ptr(),
        in(reg) rhs.as_ref().as_ptr(),
        in(reg) result.as_ref().as_ptr());
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

fn program_entry() -> i8 {
    test_add();
    test_add_array();
    0
}
