#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use ckb_std::debug;
use ckb_std::default_alloc;
use rvv_encoder::rvv_asm;

ckb_std::entry!(program_entry);
default_alloc!();

fn vadd_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8], avl: u64, vtype: u64) {    
    unsafe {
        rvv_asm!("mv a0, {0}" , in (reg) lhs.as_ref().as_ptr());
        rvv_asm!("mv a1, {0}", in (reg) rhs.as_ref().as_ptr());
        rvv_asm!("mv a2, {0}", in (reg) result.as_ref().as_ptr());
        rvv_asm!("mv a4, {0}", in (reg) avl);
        rvv_asm!("mv a5, {0}", in (reg) vtype);

        rvv_asm!("1:");

        rvv_asm!("vsetvl t0, a4, a5");

        rvv_asm!("vle8.v v0, (a1)");
        rvv_asm!("sub a0, a0, t0");
        rvv_asm!("slli t0, t0, 5");
        rvv_asm!("add a1, a1, t0");
        rvv_asm!("vle8.v v1, (a2)");
        rvv_asm!("add a2, a2, t0");
        rvv_asm!("vadd.vv v2, v0, v1");
        rvv_asm!("vse8.v v2, (a3)");
        rvv_asm!("add a3, a3, t0");
        rvv_asm!("bnez a0, 1b");
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

fn program_entry() -> i8 {
    test_add();
    test_add_array();
    0
}
