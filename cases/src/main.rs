#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(unchecked_math)]
#![feature(asm_sym)]

mod adc_sbc_cases;
mod count_population_in_mask_cases;
mod integer_extension_cases;
mod integer_merge_cases;
mod integer_move_cases;
mod integer_scalar_move_cases;
mod load_store_cases;
mod mask_register_logical_cases;
mod misc_cases;
mod narrowing_integer_right_shift_cases;
mod set_before_first_cases;
mod set_including_first_cases;
mod set_only_first_cases;
mod single_width_averaging_cases;
mod single_width_integer_multiply_add_cases;
mod single_width_integer_reduction_cases;
mod single_width_shift_cases;
mod vector_element_index_cases;
mod vector_iota_cases;

mod vmsop_vi_cases;
mod vmsop_vv_cases;
mod vmsop_vx_cases;

mod vop_vi_cases;
mod vop_vv_cases;
mod vop_vx_cases;
mod vwop_vv_cases;
mod vwop_vx_cases;
mod vwop_wv_cases;
mod vwop_wx_cases;

use ckb_std::cstr_core::CStr;
use ckb_std::default_alloc;
use ckb_std::syscalls::debug;
use core::arch::asm;
use core::slice::from_raw_parts;
use core::stringify;
use rvv_testcases::misc::set_verbose;
use rvv_testcases::{log, test_case};

ckb_std::entry!(program_entry);
default_alloc!();

fn program_entry(argc: u64, argv: *const *const u8) -> i8 {
    let test_pattern = if argc > 0 {
        let args = unsafe { from_raw_parts(argv, argc as usize) };
        let s = unsafe { CStr::from_ptr(args[0]) };
        Some(s.to_str().unwrap())
    } else {
        None
    };
    if argc > 1 {
        let args = unsafe { from_raw_parts(argv, argc as usize) };
        let s = unsafe { CStr::from_ptr(args[1]) };
        if s.to_str().unwrap() == "verbose" {
            log!("verbose on");
            set_verbose(true)
        }
    }
    test_case!(misc_cases::test_add, test_pattern);
    test_case!(misc_cases::test_add_array, test_pattern);
    test_case!(vop_vv_cases::test_vop_vv, test_pattern);
    test_case!(vop_vx_cases::test_vop_vx, test_pattern);
    test_case!(vop_vi_cases::test_vop_vi, test_pattern);
    test_case!(vwop_vv_cases::test_vwop_vv, test_pattern);
    test_case!(vwop_wv_cases::test_vwop_wv, test_pattern);
    test_case!(vwop_vx_cases::test_vwop_vx, test_pattern);
    test_case!(vwop_wx_cases::test_vwop_wx, test_pattern);
    test_case!(
        integer_extension_cases::test_integer_extension,
        test_pattern
    );
    test_case!(
        single_width_averaging_cases::test_single_width_averaging_add_and_subtract,
        test_pattern
    );
    test_case!(
        single_width_shift_cases::test_single_width_shift,
        test_pattern
    );
    test_case!(
        narrowing_integer_right_shift_cases::test_narrowing_integer_right_shift,
        test_pattern
    );
    test_case!(vmsop_vv_cases::test_vmsop_vv, test_pattern);
    test_case!(vmsop_vx_cases::test_vmsop_vx, test_pattern);
    test_case!(vmsop_vi_cases::test_vmsop_vi, test_pattern);
    test_case!(integer_move_cases::test_integer_move, test_pattern);
    test_case!(
        mask_register_logical_cases::test_mask_register_logical,
        test_pattern
    );

    test_case!(
        single_width_integer_reduction_cases::test_vredop_vv,
        test_pattern
    );

    test_case!(
        single_width_integer_multiply_add_cases::test_single_width_integer_multiply_add,
        test_pattern
    );
    test_case!(
        count_population_in_mask_cases::test_count_population_in_mask,
        test_pattern
    );
    test_case!(set_before_first_cases::test_set_before_first, test_pattern);
    test_case!(
        set_including_first_cases::test_set_including_first,
        test_pattern
    );
    test_case!(set_only_first_cases::test_set_only_first, test_pattern);
    test_case!(vector_iota_cases::test_vector_iota, test_pattern);
    test_case!(
        vector_element_index_cases::test_vector_element_index,
        test_pattern
    );
    test_case!(
        integer_scalar_move_cases::test_integer_scalar_move,
        test_pattern
    );
    test_case!(load_store_cases::test_load_store, test_pattern);
    test_case!(integer_merge_cases::test_integer_merge, test_pattern);
    test_case!(adc_sbc_cases::test_adc_sbc, test_pattern);
    0
}
