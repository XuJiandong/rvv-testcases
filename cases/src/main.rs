#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use ckb_std::debug;
use ckb_std::default_alloc;
use rvv_asm::rvv_asm;
use rvv_testcases::misc::create_vtype;

ckb_std::entry!(program_entry);
default_alloc!();

enum VInstructionsOp {
    Add,
    Sub,
}

fn vop_vv(lhs: &[u8], rhs: &[u8], result: &mut [u8], avl: u64, vtype: u64, t: VInstructionsOp) {
    unsafe {
        rvv_asm!("mv t0, {0}" , in (reg) lhs.as_ref().as_ptr());
        rvv_asm!("mv t1, {0}", in (reg) rhs.as_ref().as_ptr());
        rvv_asm!("mv t2, {0}", in (reg) result.as_ref().as_ptr());
        rvv_asm!("mv t3, {0}", in (reg) avl);
        rvv_asm!("mv t4, {0}", in (reg) vtype);

        rvv_asm!("1:");
        rvv_asm!("vsetvl t5, t3, t4");
        // convert 'vl' to bytes: for U256, it's 32 bytes, which is `t5 << 5`
        // rvv_asm!("slli t5, t5, 5");
        rvv_asm!("sub t3, t3, t5"); // decrease avl

        rvv_asm!("vle8.v v1, (t0)"); // load lhs to v1
        rvv_asm!("add t0, t0, t5"); // increase lhs
        rvv_asm!("vle8.v v2, (t1)"); // load rhs to v2
        rvv_asm!("add t1, t1, t5"); // increase rhs

        // be careful while using this
        match t {
            VInstructionsOp::Add => {
                rvv_asm!("vadd.vv v3, v1, v2"); // ADD
            }
            VInstructionsOp::Sub => {
                rvv_asm!("vsub.vv v3, v1, v2"); // SUB
            }
        }

        rvv_asm!("vse8.v v3, (t2)"); // store v3 to result
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

fn test_vop_vv_by_avl(avl: usize, lmul: i64, t: VInstructionsOp) {
    let mut lhs = [0u8; 100];
    let mut rhs = [0u8; 100];
    let mut expected = [0u8; 100];
    for i in 0..lhs.len() {
        let v = i as u8 + 1;
        lhs[i] = v + 1;
        rhs[i] = v;

        expected[i] = match t {
            VInstructionsOp::Add => lhs[i].wrapping_add(rhs[i]),
            VInstructionsOp::Sub => lhs[i].wrapping_sub(rhs[i]),
        };
    }
    let mut result = [0u8; 100];

    let vtype = create_vtype(8, lmul);
    // debug!("setting vtype to {}", vtype);
    vop_vv(&lhs, &rhs, &mut result, avl as u64, vtype, t);

    for i in 0..avl {
        if result[i] != expected[i] {
            debug!(
                "unexpected values found at index {}: {} {}(expected)",
                i, result[i], expected[i]
            );
        }
        assert_eq!(result[i], expected[i]);
    }
}

fn test_vop_vv() {
    debug!("test_vop_vv, start ...");
    for i in 80..100 {
        if i % 2 == 0 {
            test_vop_vv_by_avl(i, 2, VInstructionsOp::Add);
        } else {
            test_vop_vv_by_avl(i, 2, VInstructionsOp::Sub);
        }
    }
    debug!("test_vop_vv, done");
}

fn program_entry() -> i8 {
    test_add();
    test_add_array();
    test_vop_vv();
    0
}
