use alloc::boxed::Box;
use core::arch::asm;

use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::{vl1r_v0, vl1r_v8};
use rvv_testcases::misc::get_bit_in_slice;
use rvv_testcases::misc::VLEN;
use rvv_testcases::runner::{run_vxop_m, ExpectedOp};

fn expected_cpop_op(mask_v0: &[u8], vs2: &[u8], rd: &mut [u8], enable_mask: bool, vl: usize) {
    assert_eq!(mask_v0.len(), VLEN / 8);
    assert_eq!(vs2.len(), VLEN / 8);
    assert_eq!(rd.len(), 8);

    assert!(vl <= vs2.len() * 8);
    let mut population = 0u64;
    for i in 0..vl {
        if get_bit_in_slice(vs2, i) == 1 {
            if enable_mask {
                if get_bit_in_slice(mask_v0, i) == 1 {
                    population += 1;
                }
            } else {
                population += 1;
            }
        }
    }
    rd.copy_from_slice(&population.to_le_bytes());
}

fn expected_first_op(mask_v0: &[u8], vs2: &[u8], rd: &mut [u8], enable_mask: bool, vl: usize) {
    assert_eq!(mask_v0.len(), VLEN / 8);
    assert_eq!(vs2.len(), VLEN / 8);
    assert_eq!(rd.len(), 8);
    let mut first: u64 = u64::MAX;
    assert!(vl <= vs2.len() * 8);

    for i in 0..vl {
        if get_bit_in_slice(vs2, i) == 1 {
            if enable_mask {
                if get_bit_in_slice(mask_v0, i) == 1 {
                    first = i as u64;
                    break;
                }
            } else {
                first = i as u64;
                break;
            }
        }
    }

    rd.copy_from_slice(&first.to_le_bytes());
}

fn cpop_op(mask_v0: &[u8], vs2: &[u8], rd: &mut [u8], enable_mask: bool) {
    let mut res: u64;
    unsafe {
        if enable_mask {
            vl1r_v0(mask_v0);
            vl1r_v8(vs2);
            rvv_asm!("vcpop.m t0, v8, v0.t", "mv {}, t0", out (reg) res);
        } else {
            vl1r_v8(vs2);
            rvv_asm!("vcpop.m t0, v8", "mv {}, t0", out (reg) res);
        }
    }
    rd.copy_from_slice(&res.to_le_bytes());
}

fn first_op(mask_v0: &[u8], vs2: &[u8], rd: &mut [u8], enable_mask: bool) {
    let mut res: u64;
    unsafe {
        if enable_mask {
            vl1r_v0(mask_v0);
            vl1r_v8(vs2);
            rvv_asm!("vfirst.m t0, v8, v0.t", "mv {}, t0", out (reg) res);
        } else {
            vl1r_v8(vs2);
            rvv_asm!("vfirst.m t0, v8", "mv {}, t0", out (reg) res);
        }
    }
    rd.copy_from_slice(&res.to_le_bytes());
}

pub fn test_count_population_in_mask() {
    run_vxop_m(
        ExpectedOp::EnableMask(Box::new(expected_cpop_op)),
        cpop_op,
        false,
        "vcpop.m",
    );
    run_vxop_m(
        ExpectedOp::EnableMask(Box::new(expected_cpop_op)),
        cpop_op,
        true,
        "vcpop.m",
    );

    run_vxop_m(
        ExpectedOp::EnableMask(Box::new(expected_first_op)),
        first_op,
        false,
        "vfirst.m",
    );
    run_vxop_m(
        ExpectedOp::EnableMask(Box::new(expected_first_op)),
        first_op,
        true,
        "vfirst.m",
    );
}
