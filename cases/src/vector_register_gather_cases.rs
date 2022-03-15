use alloc::vec::Vec;
use core::arch::asm;
use rand::Rng;
use rvv_asm::rvv_asm;

use ckb_std::syscalls::debug;
use rvv_testcases::intrinsic::{vs1r_v21, vsetvl};
use rvv_testcases::log;
use rvv_testcases::misc::{get_bit_in_slice, is_verbose, VLEN};
use rvv_testcases::rng::BestNumberRng;

fn to_wide(vs1: &[u8], wide: usize) -> Vec<u32> {
    let wide = wide / 8;
    let mut ret: Vec<u32> = Vec::new();
    ret.resize(vs1.len() / wide, 0);
    for i in 0..vs1.len() / wide {
        if wide == 1 {
            ret[i] = vs1[i] as u32;
        } else {
            ret[i] = u16::from_le_bytes([vs1[i * wide], vs1[i * wide + 1]]) as u32;
            if wide > 2 {
                for j in 2..wide {
                    if vs1[i * wide + j] != 0 {
                        ret[i] = 0xFFFF;
                    }
                }
            }
        }
    }

    ret
}

fn rgather_vv(vd: &[u8], v2: &[u8], v1: &[u8], vm: &[u8], wide: usize, v1_wide: usize) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    result.resize(vd.len(), 0);
    result.copy_from_slice(vd);

    let v1 = to_wide(v1, v1_wide);

    let wide = wide / 8;
    for i in 0..vd.len() / wide {
        if get_bit_in_slice(vm, i) == 1 {
            continue;
        }

        if v1[i] as usize > vd.len() {
            for j in 0..wide {
                result[i * wide + j] = 0;
            }
        } else {
            for j in 0..wide {
                let d: usize = v1[i] as usize * wide;
                result[i * wide + j] = v2[d + j];
            }
        }
    }

    result
}

fn vrgather_vv(wide: usize) {
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

    let expected = rgather_vv(&expected_before, &vs2, &vs1, &mask, wide, wide);
    let expected = expected.as_slice();

    let vl = vsetvl((VLEN / wide) as u64, wide as u64, 2) as usize;
    assert_eq!(vl, (VLEN / wide) as usize);

    unsafe {
        rvv_asm!(
            "mv t0, {}", "vl1re8.v v0, (t0)",
            "mv t0, {}", "vl1re8.v v8, (t0)",
            "mv t0, {}", "vl1re8.v v4, (t0)",
            "mv t0, {}", "vl1re8.v v21, (t0)",
            "vrgather.vv v21, v8, v4, v0.t",
            in (reg) mask.as_ptr(),
            in (reg) vs2.as_ptr(),
            in (reg) vs1.as_ptr(),
            in (reg) expected_before.as_ptr()
        );

        let p = result.as_mut_ptr();
        rvv_asm!("mv t0, {}", "vs1r.v v21, (t0)", in (reg) p);
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

pub fn test_vrgather_vv() {
    vrgather_vv(8);
    vrgather_vv(16);
    vrgather_vv(128);
}

fn vrgatherei16_vv(wide: usize) {
    if is_verbose() {
        log!("test vrgatherei16.vv");
    }
    let mut mask = [0u8; VLEN / 8];
    let mut expected_before = [0u8; VLEN / 8];
    let mut vs1 = [0u8; VLEN / 4];
    let mut vs2 = [0u8; VLEN / 8];
    let mut result = [0u8; VLEN / 8];

    let mut rng = BestNumberRng::default();

    rng.fill(&mut mask[..]);
    rng.fill(&mut vs1[..]);
    rng.fill(&mut vs2[..]);
    rng.fill(&mut expected_before[..]);

    let expected = rgather_vv(&expected_before, &vs2, &vs1, &mask, wide, 16);
    let expected = expected.as_slice();

    let vl = vsetvl(256, wide as u64, 2) as usize;
    assert_eq!(vl, 256);

    unsafe {
        rvv_asm!(
            "mv t0, {}", "vl1re8.v v0, (t0)",
            "mv t0, {}", "vl1re16.v v4, (t0)",
            "mv t0, {}", "vl1re8.v v8, (t0)",
            "mv t0, {}", "vl1re8.v v24, (t0)",
            "vrgatherei16.vv v24, v8, v4, v0.t",
            "mv t0, {}", "vs1r.v v24, (t0)",
            in (reg) mask.as_ptr(),
            in (reg) vs1.as_ptr(),
            in (reg) vs2.as_ptr(),
            in (reg) expected_before.as_ptr(),
            in (reg) result.as_mut_ptr(),
        );
    }

    if result != expected {
        log!(
            "[describe = vrgatherei16.vv] unexpected values found: \nresult = {:0>2X?} \nexpected = {:0>2X?}",
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

pub fn test_vrgatherei16_vv() {
    vrgatherei16_vv(8);
    //vrgatherei16_vv(16);
    //vrgatherei16_vv(128);
}

fn rgather_vx(vd: &[u8], v2: &[u8], rs: u64, vm: &[u8], wide: usize) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    result.resize(vd.len(), 0);
    result.copy_from_slice(vd);

    let length = vd.len() / (wide / 8);

    for i in 0..length {
        if get_bit_in_slice(vm, i) == 1 {
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

fn vrgatherer_vx(wide: usize) {
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

pub fn test_vrgatherer_vx() {
    vrgatherer_vx(8);
    vrgatherer_vx(16);
    vrgatherer_vx(128);
}
