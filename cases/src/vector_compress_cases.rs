use core::arch::asm;
use rand::Rng;
use rvv_asm::rvv_asm;

use ckb_std::syscalls::debug;
use rvv_testcases::intrinsic::{vl1r_v0, vl1r_v1, vl1r_v21, vs1r_v21, vsetvl};
use rvv_testcases::log;
use rvv_testcases::misc::{get_bit_in_slice, is_verbose, VLEN};
use rvv_testcases::rng::BestNumberRng;

pub fn test_vector_compress() {
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

    let mut pos: usize = 0;
    for i in 0..128 as usize {
        if get_bit_in_slice(&mask, i) == 1 {
            expected[pos * 2] = vs2[i * 2];
            expected[pos * 2 + 1] = vs2[i * 2 + 1];
            pos += 1;
        }
    }

    let vl = vsetvl(128, 16, 1) as usize;
    assert_eq!(vl, 128);

    vl1r_v0(&mask[..]);
    vl1r_v1(&vs2[..]);
    vl1r_v21(&expected_before[..]);
    unsafe {
        rvv_asm!("vcompress.vm v21, v1, v0");
        vs1r_v21(&mut result[..]);
    }
    if result != expected {
        log!(
            "[describe = vmsif.m] unexpected values found: \nresult = {:0>2X?}, \nexpected = {:0>2X?}",
            result,
            expected
        );
        log!(
            "more information \nvs2 = {:0>2X?}, \nexpected_befor = {:0>2X?}, \nmask = {:0>2X?}",
            vs2,
            expected_before,
            mask
        );
        panic!("Abort");
    }
    if is_verbose() {
        log!("finished");
    }
}
