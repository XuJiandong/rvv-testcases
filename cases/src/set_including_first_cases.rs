use core::arch::asm;
use rand::Rng;
use rvv_asm::rvv_asm;

use ckb_std::syscalls::debug;
use rvv_testcases::intrinsic::{vl1r_v0, vl1r_v24, vl1r_v8, vs1r_v24, vsetvl};
use rvv_testcases::log;
use rvv_testcases::misc::{get_bit_in_slice, is_verbose, set_bit_in_slice, VLEN};
use rvv_testcases::rng::BestNumberRng;

fn run(enable_mask: bool) {
    if is_verbose() {
        log!("test vmsif.m");
    }
    let mut mask = [0u8; VLEN / 8];
    let mut expected_before = [0u8; VLEN / 8];
    let mut expected = [0u8; VLEN / 8];
    let mut vs2 = [0u8; VLEN / 8];
    let mut result = [0u8; VLEN / 8];

    let mut rng = BestNumberRng::default();

    rng.fill(&mut mask[..]);
    rng.fill(&mut vs2[..]);
    rng.fill(&mut expected_before[..]);
    expected.copy_from_slice(&expected_before[..]);

    let vl = vsetvl(8, 256, 1) as usize;
    assert_eq!(vl, 8);

    let mut index = VLEN as usize;
    for i in 0..vl as usize {
        if get_bit_in_slice(&vs2[..], i) == 1 {
            if enable_mask {
                if get_bit_in_slice(&mask[..], i) == 1 {
                    index = i;
                    break;
                }
            } else {
                index = i;
                break;
            }
        }
    }

    for i in 0..vl {
        if i <= index {
            if enable_mask {
                if get_bit_in_slice(&mask[..], i) == 1 {
                    set_bit_in_slice(&mut expected[..], i, 1);
                }
            } else {
                set_bit_in_slice(&mut expected[..], i, 1);
            }
        } else {
            if enable_mask {
                if get_bit_in_slice(&mask[..], i) == 1 {
                    set_bit_in_slice(&mut expected[..], i, 0);
                }
            } else {
                set_bit_in_slice(&mut expected[..], i, 0);
            }
        }
    }

    vl1r_v0(&mask[..]);
    vl1r_v8(&vs2[..]);
    vl1r_v24(&expected_before[..]);
    unsafe {
        if enable_mask {
            rvv_asm!("vmsif.m v24, v8, v0.t");
        } else {
            rvv_asm!("vmsif.m v24, v8");
        }
        vs1r_v24(&mut result[..]);
    }
    if result != expected {
        log!(
            "[describe = vmsif.m] unexpected values found: {:?} (result) {:?} (expected)",
            result,
            expected
        );
        log!(
            "more information, enable_mask = {}, index = {}, vs2 = {:?}, mask = {:?}",
            enable_mask,
            index,
            vs2,
            mask
        );
        panic!("Abort");
    }
    if is_verbose() {
        log!("finished");
    }
}

pub fn test_set_including_first() {
    run(false);
    run(true);
}
