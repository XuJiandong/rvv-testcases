use alloc::vec::Vec;
use core::arch::asm;
use rand::Rng;
use rvv_asm::rvv_asm;

use ckb_std::syscalls::debug;
use rvv_testcases::intrinsic::{vs1r_v21, vsetvl};
use rvv_testcases::log;
use rvv_testcases::misc::{get_bit_in_slice, is_verbose, VLEN};
use rvv_testcases::rng::BestNumberRng;

fn rgather_vv(vd: &[u8], v2: &[u8], v1: &[u8], vm: &[u8], wide: usize) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    result.resize(vd.len(), 0);
    result.copy_from_slice(vd);

    let length = vd.len() / (wide / 8);

    for i in 0..length {
        if get_bit_in_slice(vm, i) == 0 {
            continue;
        }

        result[i] = {
            if v1[i] as usize >= length {
                0
            } else {
                v2[v1[i] as usize]
            }
        };
    }

    result
}

fn test_vrgather_vv(wide: usize) {
    if is_verbose() {
        log!("test vrgather.vv");
    }
    let mut mask = [0xFFu8; VLEN / 8];
    let mut expected_before = [0u8; VLEN / 8];
    let mut vs1 = [0u8; VLEN / 8];
    let mut vs2 = [0u8; VLEN / 8];
    let mut result = [0u8; VLEN / 8];

    let mut rng = BestNumberRng::default();

    rng.fill(&mut mask[..]);
    rng.fill(&mut vs1[..]);
    rng.fill(&mut vs2[..]);
    rng.fill(&mut expected_before[..]);

    let expected = rgather_vv(&expected_before, &vs2, &vs1, &mask, wide);
    let expected = expected.as_slice();

    let vl = vsetvl((VLEN / wide) as u64, wide as u64, 1) as usize;
    assert_eq!(vl, (VLEN / wide) as usize);

    unsafe {
        rvv_asm!(
            "mv t0, {}", "vl1re8.v v0, (t0)",
            "mv t0, {}", "vl1re8.v v1, (t0)",
            "mv t0, {}", "vl1re8.v v2, (t0)",
            "mv t0, {}", "vl1re8.v v21, (t0)",
            in (reg) mask.as_ptr(),in (reg) vs1.as_ptr(),
            in (reg) vs2.as_ptr(),
            in (reg) expected_before.as_ptr()
        );

        // TODO ckb-vm crash
        rvv_asm!("vrgather.vv v21, v2, v1, v0");
    }

    vs1r_v21(&mut result[..]);

    if result != expected {
        log!(
            "[describe = vrgather.vv] unexpected values found: \nresult = {:0>2X?} \nexpected = {:0>2X?}",
            result,
            expected
        );
        log!(
            "more information, \nvs1 = {:0>2X?}, \nvs2 = {:0>2X?}, \nmask = {:0>2X?}, \nexpected_befor = {:0>2X?}",
            vs1,
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

fn test_vrgatherei16_vv() {
    if is_verbose() {
        log!("test vrgatheri16.vv");
    }
    let mut mask = [0xFFu8; VLEN / 4];
    let mut expected_before = [0u8; VLEN / 4];
    let mut vs1 = [0u8; VLEN / 4];
    let mut vs2 = [0u8; VLEN / 4];
    let result = [0u8; VLEN / 4];

    let mut rng = BestNumberRng::default();

    rng.fill(&mut mask[..]);
    rng.fill(&mut vs1[..]);
    rng.fill(&mut vs2[..]);
    rng.fill(&mut expected_before[..]);

    let expected = rgather_vv(&expected_before, &vs2, &vs1, &mask, 8);
    let expected = expected.as_slice();

    let vl = vsetvl(256, 16, 2) as usize;
    assert_eq!(vl, 256);

    unsafe {
        rvv_asm!(
            "mv t0, {}", "vl1re8.v v0, (t0)",
            "mv t0, {}", "vl1re8.v v2, (t0)",
            "mv t0, {}", "vl1re8.v v4, (t0)",
            "mv t0, {}", "vl1re8.v v21, (t0)",
            in (reg) mask.as_ptr(),in (reg) vs1.as_ptr(),
            in (reg) vs2.as_ptr(),
            in (reg) expected_before.as_ptr()
        );

        //rvv_asm!("vrgatherei16.vv v21, v4, v2, v0");

        rvv_asm!("mv t0, {}", "vs1r.v v21, (t0)", in (reg) result.as_ptr());
    }

    if result != expected {
        log!(
            "[describe = vrgather.vv] unexpected values found: \nresult = {:0>2X?} \nexpected = {:0>2X?}",
            result,
            expected
        );
        log!(
            "more information, \nvs1 = {:0>2X?}, \nvs2 = {:0>2X?}, \nmask = {:0>2X?}, \nexpected_befor = {:0>2X?}",
            vs1,
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

fn rgather_vx(vd: &[u8], v2: &[u8], rs: u64, vm: &[u8], wide: usize) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    result.resize(vd.len(), 0);
    result.copy_from_slice(vd);

    let length = vd.len() / (wide / 8);

    for i in 0..length {
        if get_bit_in_slice(vm, i) == 0 {
            continue;
        }

        result[i] = {
            if rs as usize >= length {
                0
            } else {
                v2[rs as usize]
            }
        };
    }

    result
}

fn test_vrgatherer_vx(wide: usize) {
    if is_verbose() {
        log!("test vrgather.vx");
    }
    let mut mask = [0xFFu8; VLEN / 8];
    let mut expected_before = [0u8; VLEN / 8];
    let mut vs1 = [0u8; VLEN / 8];
    let mut vs2 = [0u8; VLEN / 8];
    let mut result = [0u8; VLEN / 8];

    let mut rng = BestNumberRng::default();

    rng.fill(&mut mask[..]);
    rng.fill(&mut vs1[..]);
    rng.fill(&mut vs2[..]);
    rng.fill(&mut expected_before[..]);

    let rs1: u64 = 11;

    let expected = rgather_vx(&expected_before, &vs2, rs1, &mask, wide);
    let expected = expected.as_slice();

    let vl = vsetvl((VLEN / wide) as u64, wide as u64, 1) as usize;
    assert_eq!(vl, (VLEN / wide) as usize);

    unsafe {
        rvv_asm!(
            "mv t0, {}", "vl1re8.v v0, (t0)",
            "mv t0, {}", "vl1re8.v v1, (t0)",
            "mv t0, {}", "vl1re8.v v2, (t0)",
            "mv t0, {}", "vl1re8.v v21, (t0)",
            in (reg) mask.as_ptr(),in (reg) vs1.as_ptr(),
            in (reg) vs2.as_ptr(),
            in (reg) expected_before.as_ptr()
        );

        rvv_asm!(
            "mv t0, {}",
            "vrgather.vx v21, v2, t0, v0",
            in (reg) rs1
        );
    }

    vs1r_v21(&mut result[..]);

    if result != expected {
        log!(
            "[describe = vrgather.vx] unexpected values found: \nresult = {:0>2X?} \nexpected = {:0>2X?}",
            result,
            expected
        );
        log!(
            "more information, \nvs1 = {:0>2X?}, \nvs2 = {:0>2X?}, \nmask = {:0>2X?}, \nexpected_befor = {:0>2X?}",
            vs1,
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

pub fn test_vector_register_gather() {
    test_vrgather_vv(8);
    test_vrgatherei16_vv();
    test_vrgatherer_vx(8);
}
