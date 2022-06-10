use core::arch::asm;

fn add(lhs: u64, rhs: u64) -> u64 {
    let mut result: u64;
    unsafe {
        asm!("add {0}, {1}, {2}", out(reg) result, in (reg) lhs, in (reg) rhs);
    }
    result
}

pub fn test_add() {
    let lhs = 1u64;
    let rhs = 0u64;
    let result = add(lhs, rhs);
    assert_eq!(result, lhs + rhs);
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

pub fn test_add_array() {
    let mut result = [0u64; 4];
    let lhs = [1, 2, 3, 4];
    let rhs = [2, 3, 4, 5];
    add_array(&lhs, &rhs, &mut result);
    assert_eq!(result[1], 5);
    assert_eq!(result[2], 7);
    assert_eq!(result[3], 9);
}
