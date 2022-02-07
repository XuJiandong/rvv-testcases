#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(unchecked_math)]
#![feature(asm_sym)]

mod misc_cases;
mod vop_vv_cases;
mod vop_vx_cases;
use ckb_std::default_alloc;
use core::arch::asm;
use rvv_testcases::intrinsic::VInstructionOp;

ckb_std::entry!(program_entry);
default_alloc!();

fn program_entry() -> i8 {
    misc_cases::test_add();
    misc_cases::test_add_array();
    vop_vv_cases::test_vop_vv_by_inputs(100, 1, VInstructionOp::Add, 256);
    vop_vv_cases::test_vop_vv_by_inputs(100, 1, VInstructionOp::Sub, 256);
    vop_vv_cases::test_vop_vv();
    vop_vx_cases::test_vop_vx();
    0
}
