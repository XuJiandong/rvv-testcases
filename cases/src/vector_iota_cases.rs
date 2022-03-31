use core::arch::asm;
use rand::Rng;
use rvv_asm::rvv_asm;

use ckb_std::syscalls::debug;
use rvv_testcases::intrinsic::{vl1r_v0, vl1r_v1, vl1r_v24, vs1r_v24, vsetvl};
use rvv_testcases::log;
use rvv_testcases::misc::{get_bit_in_slice, is_verbose, VLEN};
use rvv_testcases::rng::BestNumberRng;

fn run(enable_mask: bool) {
    if is_verbose() {
        log!("test viota.m");
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

    let mut index = VLEN as usize;
    for i in 0..(VLEN / 16) as usize {
        if enable_mask && get_bit_in_slice(&mask, i) == 0 {
            continue;
        } else {
            expected[i * 2] = index as u8;
            expected[i * 2 + 1] = 0;
        }
        if get_bit_in_slice(&vs2, i) != 0 {
            index += 1;
        }
    }

    let vl = vsetvl(128, 16, 1) as usize;
    assert_eq!(vl, 128);

    vl1r_v0(&mask[..]);
    vl1r_v1(&vs2[..]);
    vl1r_v24(&expected_before[..]);
    unsafe {
        if enable_mask {
            rvv_asm!("viota.m v24, v1, v0.t");
        } else {
            rvv_asm!("viota.m v24, v1");
        }
        vs1r_v24(&mut result[..]);
    }
    if result != expected {
        log!(
            "[describe = viota.m] unexpected values found: result = {:?}, expected = {:?}",
            result,
            expected
        );
        log!(
            "more information, enable_mask = {}, index = {}, vs2 = {:?}, mask = {:?}, expected_before = {:?}",
            enable_mask,
            index,
            vs2,
            mask,
            expected_before
        );
        panic!("Abort");
    }
    if is_verbose() {
        log!("finished");
    }
}

pub fn test_vector_iota() {
    run(false);
    run(true);
}
