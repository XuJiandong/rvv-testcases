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
mod narrowing_fixed_point_clip_cases;
mod narrowing_integer_right_shift_cases;
mod set_before_first_cases;
mod set_including_first_cases;
mod set_only_first_cases;
mod single_saturating_add_subtract_cases;
mod single_width_averaging_cases;
mod single_width_integer_multiply_add_cases;
mod single_width_integer_reduction_cases;
mod single_width_scaling_shift;
mod single_width_shift_cases;
mod vector_compress_cases;
mod vector_element_index_cases;
mod vector_iota_cases;
mod vsetvl_cases;
mod vsub_cases;
mod widening_integer_reduction_cases;

mod vector_register_gather_cases;
mod vector_slide_cases;

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
use rvv_testcases::misc::{is_simple, is_verbose, set_simple, set_verbose};
use rvv_testcases::rng::{get_seed, set_seed};
use rvv_testcases::{log, test_case};

ckb_std::entry!(program_entry);
default_alloc!();

fn program_entry(argc: u64, argv: *const *const u8) -> i8 {
    let mut test_pattern = Option::<&str>::None;

    let args = unsafe { from_raw_parts(argv, argc as usize) };
    for i in 0..argc as usize {
        let data = unsafe { CStr::from_ptr(args[i]) }.to_str().unwrap();
        if data.find("--case").is_some() {
            let pos = data.find("--case=").unwrap() + 7;
            test_pattern = Some(&data[pos..data.len()]);
        } else if data.find("--verbose").is_some() {
            set_verbose(true);
        } else if data.find("--simple").is_some() {
            set_simple(true);
        } else if data.find("--seed=").is_some() {
            let pos = data.find("--seed=").unwrap() + 7;
            let seed = data[pos..data.len()].parse::<usize>().unwrap();
            set_seed(seed);
        }
    }

    log!(
        "--StartTesting, case: {:?}, verbose: {}, simple: {}, seed: {}",
        test_pattern,
        is_verbose(),
        is_simple(),
        get_seed()
    );

    test_case!(vsetvl_cases::test_vsetvl, test_pattern);

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
    test_case!(
        narrowing_integer_right_shift_cases::test_narrowing_integer_right_shift_arithmetic,
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
        single_width_integer_reduction_cases::test_vred_op,
        test_pattern
    );
    test_case!(
        widening_integer_reduction_cases::test_vwredsumu_vs,
        test_pattern
    );
    test_case!(
        widening_integer_reduction_cases::test_vwredsum_vs,
        test_pattern
    );

    test_case!(
        single_width_integer_multiply_add_cases::test_widening_width_multiply_add,
        test_pattern
    );
    test_case!(vsub_cases::test_vsub, test_pattern);
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
    test_case!(load_store_cases::test_load_store_uxei, test_pattern);
    test_case!(load_store_cases::test_vector_unit_stride, test_pattern);
    test_case!(load_store_cases::test_whole_load_store, test_pattern);
    test_case!(integer_merge_cases::test_integer_merge, test_pattern);
    test_case!(adc_sbc_cases::test_adc_sbc, test_pattern);
    test_case!(vector_compress_cases::test_vector_compress, test_pattern);
    test_case!(vector_slide_cases::test_vector_slide_up, test_pattern);
    test_case!(vector_slide_cases::test_vector_slide_down, test_pattern);
    test_case!(vector_register_gather_cases::test_vrgatherer, test_pattern);
    test_case!(
        single_width_scaling_shift::test_single_with_scaling_shift,
        test_pattern
    );
    test_case!(
        single_saturating_add_subtract_cases::test_single_saturating_add_subtract,
        test_pattern
    );
    test_case!(
        narrowing_fixed_point_clip_cases::test_narrowing_fixed_point_clip,
        test_pattern
    );

    0
}
