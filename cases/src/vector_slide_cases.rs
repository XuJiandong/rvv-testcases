use alloc::vec::Vec;
use core::arch::asm;
use rvv_asm::rvv_asm;

use ckb_std::syscalls::debug;
use rvv_testcases::{
    intrinsic::{vl1r_v0, vl1r_v24, vl1r_v8, vs1r_v24, vsetvl},
    log,
    misc::{get_bit_in_slice, is_verbose, VLEN},
    rng::fill_rand_bytes,
};

fn sideup(
    data: &[u8],
    offset: usize,
    length: usize,
    wide: usize,
    expected: &[u8],
    mask: &[u8],
) -> Vec<u8> {
    assert!(length * wide <= VLEN);
    assert!(wide % 8 == 0);
    assert!(offset <= length);
    assert!(expected.len() == data.len());
    assert!(mask.len() * 8 >= length);

    let mut cache: Vec<u8> = Vec::new();
    cache.resize(data.len(), 0);
    cache.copy_from_slice(expected);

    let wide = wide / 8;
    for i in offset..length as usize {
        if get_bit_in_slice(mask, i) == 0 {
            continue;
        }
        if (i as i64) - (offset as i64) < 0 {
            continue;
        }

        for j in 0..wide {
            if i * wide + j >= cache.len() {
                break;
            }
            cache[i * wide + j] = data[(i - offset) * wide + j];
        }
    }

    cache
}

fn test_vslideup(wide: usize) {
    if is_verbose() {
        log!("test vslideup");
    }
    let mut mask = [0u8; VLEN / 8];
    let mut expected_before = [0u8; VLEN / 8];
    let mut expected = [0u8; VLEN / 8];
    let mut vs2 = [0u8; VLEN / 8];
    let mut result = [0u8; VLEN / 8];
    let mut result2 = [0u8; VLEN / 8];

    fill_rand_bytes(&mut mask[..]);
    fill_rand_bytes(&mut vs2[..]);
    fill_rand_bytes(&mut expected_before[..]);

    let ref_offset = sideup(&vs2, 3, VLEN / wide, wide, &expected_before, &mask);

    let vl = vsetvl((VLEN / wide) as u64, wide as u64, 1) as usize;
    assert_eq!(vl, VLEN / wide);

    expected.copy_from_slice(ref_offset.as_slice());

    vl1r_v0(&mask[..]);
    vl1r_v8(&vs2[..]);
    vl1r_v24(&expected_before[..]);
    let x: u64 = 3;
    unsafe {
        rvv_asm!(
            "mv t0, {}",
            "vslideup.vx v24, v8, t0, v0.t",
            in (reg) x
        );
        vs1r_v24(&mut result[..]);
    }

    vl1r_v0(&mask[..]);
    vl1r_v8(&vs2[..]);
    vl1r_v24(&expected_before[..]);
    unsafe {
        rvv_asm!("vslideup.vi v24, v8, 3, v0.t",);
        vs1r_v24(&mut result2[..]);
    }

    if result != expected || result != result2 {
        log!(
            "[describe = vslideup.vx] unexpected values found: \nresult = {:0>2X?}, \nresult2 = {:0>2X?}, \nexpected = {:0>2X?}",
            result,
            result2,
            expected
        );
        log!(
            "more information, \nwide = {}, \nvs2 = {:0>2X?}, \nmask = {:0>2X?}, \nexpected_before = {:0>2X?}",
            wide,
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

fn test_vslide1up(wide: usize) {
    if is_verbose() {
        log!("test vslide1up");
    }
    let mut mask = [0u8; VLEN / 8];
    let mut expected_before = [0u8; VLEN / 8];
    let mut vs2 = [0u8; VLEN / 8];
    let mut result = [0u8; VLEN / 8];

    fill_rand_bytes(&mut mask[..]);
    fill_rand_bytes(&mut vs2[..]);
    fill_rand_bytes(&mut expected_before[..]);

    let mut expected = sideup(&vs2, 1, VLEN / wide, wide, &expected_before, &mask);

    let vl = vsetvl((VLEN / wide) as u64, wide as u64, 1) as usize;
    assert_eq!(vl, VLEN / wide);

    vl1r_v0(&mask[..]);
    vl1r_v8(&vs2[..]);
    vl1r_v24(&expected_before[..]);
    let x: u64 = 12;

    if get_bit_in_slice(&mask, 0) == 1 {
        expected[0] = x as u8;
        for i in 1..wide / 8 {
            expected[i] = 0;
        }
    }

    unsafe {
        rvv_asm!(
            "mv t0, {}",
            "vslide1up.vx v24, v8, t0, v0.t",
            in (reg) x
        );
        vs1r_v24(&mut result[..]);
    }

    if result != expected.as_slice() {
        log!(
            "[describe = vslide1up.vx] unexpected values found: \nresult = {:0>2X?}, \nexpected = {:0>2X?}",
            result,
            expected
        );
        log!(
            "more information, wide = {} \nvs2 = {:0>2X?}, \nmask = {:0>2X?}, \nexpected_before = {:0>2X?}",
            wide,
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

pub fn test_vector_slide_up() {
    test_vslideup(8);
    test_vslideup(16);
    test_vslideup(128);

    test_vslide1up(8);
    test_vslide1up(16);
    test_vslide1up(128);
}

fn sidedown(
    data: &[u8],
    offset: usize,
    length: usize,
    wide: usize,
    expected: &[u8],
    mask: &[u8],
) -> Vec<u8> {
    assert!(length * wide <= VLEN);
    assert!(wide % 8 == 0);
    assert!(offset <= length);
    assert!(expected.len() == data.len());
    assert!(mask.len() * 8 >= length);

    let mut cache: Vec<u8> = Vec::new();
    cache.resize(data.len(), 0);
    cache.copy_from_slice(expected);

    let wide = wide / 8;
    for i in 0..length - offset as usize {
        if get_bit_in_slice(mask, i) == 0 {
            continue;
        }
        for j in 0..wide {
            cache[i * wide + j] = data[(i + offset) * wide + j];
        }
    }

    for i in length - offset..length as usize {
        if get_bit_in_slice(mask, i) == 0 {
            continue;
        }
        for j in 0..wide {
            cache[i * wide + j] = 0;
        }
    }

    cache
}

fn test_vslidedown(wide: usize) {
    if is_verbose() {
        log!("test vslidedown");
    }
    let mut mask = [0u8; VLEN / 8];
    let mut expected_before = [0u8; VLEN / 8];
    let mut vs2 = [0u8; VLEN / 8];
    let mut result = [0u8; VLEN / 8];
    let mut result2 = [0u8; VLEN / 8];

    fill_rand_bytes(&mut mask[..]);
    fill_rand_bytes(&mut vs2[..]);
    fill_rand_bytes(&mut expected_before[..]);

    let expected = sidedown(&vs2, 3, VLEN / wide, wide, &expected_before, &mask);

    let vl = vsetvl((VLEN / wide) as u64, wide as u64, 1) as usize;
    assert_eq!(vl, VLEN / wide);

    vl1r_v0(&mask[..]);
    vl1r_v8(&vs2[..]);
    vl1r_v24(&expected_before[..]);
    let x: u64 = 3;
    unsafe {
        rvv_asm!(
            "mv t0, {}",
            "vslidedown.vx v24, v8, t0, v0.t",
            in (reg) x
        );
        vs1r_v24(&mut result[..]);
    }

    vl1r_v0(&mask[..]);
    vl1r_v8(&vs2[..]);
    vl1r_v24(&expected_before[..]);
    unsafe {
        rvv_asm!("vslidedown.vi v24, v8, 3, v0.t",);
        vs1r_v24(&mut result2[..]);
    }

    if result != expected.as_slice() || result != result2 {
        log!(
            "[describe = vslidedown.vx] unexpected values found: \nresult = {:0>2X?}, \nresult2 = {:0>2X?}, \nexpected = {:0>2X?}",
            result,
            result2,
            expected
        );
        log!(
            "more information, \nwide = {}, \nvs2 = {:0>2X?}, \nmask = {:0>2X?}, \nexpected_before = {:0>2X?}",
            wide,
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

fn test_vslide1down(wide: usize) {
    if is_verbose() {
        log!("test vslide1down");
    }
    let mut mask = [0u8; VLEN / 8];
    let mut expected_before = [0u8; VLEN / 8];
    let mut vs2 = [0u8; VLEN / 8];
    let mut result = [0u8; VLEN / 8];

    fill_rand_bytes(&mut mask[..]);
    fill_rand_bytes(&mut vs2[..]);
    fill_rand_bytes(&mut expected_before[..]);

    let mut expected = sidedown(&vs2, 1, VLEN / wide, wide, &expected_before, &mask);

    let vl = vsetvl((VLEN / wide) as u64, wide as u64, 1) as usize;
    assert_eq!(vl, VLEN / wide);

    vl1r_v0(&mask[..]);
    vl1r_v8(&vs2[..]);
    vl1r_v24(&expected_before[..]);
    let x: u64 = 12;

    if get_bit_in_slice(&mask, vl - 1) == 1 {
        let pos = (wide / 8) * (vl - 1);
        expected[pos] = x as u8;
        for i in pos + 1..vs2.len() {
            expected[i] = 0;
        }
    }

    unsafe {
        rvv_asm!(
            "mv t0, {}",
            "vslide1down.vx v24, v8, t0, v0.t",
            in (reg) x
        );
        vs1r_v24(&mut result[..]);
    }

    if result != expected.as_slice() {
        log!(
            "[describe = vslide1down.vx] unexpected values found: \nresult = {:0>2X?}, \nexpected = {:0>2X?}",
            result,
            expected
        );
        log!(
            "more information, \nvs2 = {:0>2X?}, \nmask = {:0>2X?}, \nexpected_before = {:0>2X?}",
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

pub fn test_vector_slide_down() {
    test_vslidedown(8);
    test_vslidedown(16);
    test_vslidedown(128);

    test_vslide1down(8);
    test_vslide1down(16);
    test_vslide1down(128);
}
