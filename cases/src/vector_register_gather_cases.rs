use alloc::vec::Vec;
use core::arch::asm;
use core::convert::TryInto;
use rand::Rng;
use rvv_asm::rvv_asm;

use ckb_std::syscalls::debug;
use rvv_testcases::intrinsic::{vs1r_v21, vsetvl};
use rvv_testcases::log;
use rvv_testcases::misc::{get_bit_in_slice, is_verbose, U1024, U256, U512, VLEN};
use rvv_testcases::rng::BestNumberRng;

fn to_wide(vs1: &[u8], wide: usize) -> Vec<u32> {
    let wide = wide / 8;
    let mut ret: Vec<u32> = Vec::new();
    ret.resize(vs1.len() / wide, 0);
    for i in 0..vs1.len() / wide {
        ret[i] = match wide {
            1 => vs1[i] as u32,
            2 => u16::from_le_bytes(vs1[i * wide..i * wide + 2].try_into().unwrap()) as u32,
            4 => u32::from_le_bytes(vs1[i * wide..i * wide + 4].try_into().unwrap()),
            8 => {
                let d = u64::from_le_bytes(vs1[i * wide..i * wide + 8].try_into().unwrap());
                if d >= 0xFFFF {
                    0xFFFF as u32
                } else {
                    d as u32
                }
            }
            16 => {
                let d = u128::from_le_bytes(vs1[i * wide..i * wide + 16].try_into().unwrap());
                if d >= 0xFFFF {
                    0xFFFF as u32
                } else {
                    d as u32
                }
            }
            32 => {
                let d = U256::from_little_endian(vs1[i * wide..i * wide + 32].try_into().unwrap());
                if d >= 0xFFFF.into() {
                    0xFFFF as u32
                } else {
                    d.as_u32()
                }
            }
            64 => {
                let d = U512::from_little_endian(vs1[i * wide..i * wide + 64].try_into().unwrap());
                if d >= 0xFFFF.into() {
                    0xFFFF as u32
                } else {
                    d.as_u32()
                }
            }
            128 => {
                let d = U512::from_little_endian(vs1[i * wide..i * wide + 128].try_into().unwrap());
                if d >= 0xFFFF.into() {
                    0xFFFF as u32
                } else {
                    d.as_u32()
                }
            }
            256 => {
                let d =
                    U1024::from_little_endian(vs1[i * wide..i * wide + 256].try_into().unwrap());
                if d >= 0xFFFF.into() {
                    0xFFFF as u32
                } else {
                    d.as_u32()
                }
            }
            _ => {
                assert!(false);
                0
            }
        };
    }
    ret
}

fn rgather_vv(
    vd: &[u8],
    v2: &[u8],
    v1: &[u8],
    vm: &[u8],
    enable_mask: bool,
    wide: usize,
    v1_wide: usize,
) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    result.resize(vd.len(), 0);
    result.copy_from_slice(vd);

    let v1 = to_wide(v1, v1_wide);

    let wide = wide / 8;
    for i in 0..(vd.len() / wide) {
        if enable_mask && get_bit_in_slice(vm, i) == 1 {
            continue;
        }

        let index = v1[i];
        if index as usize >= (vd.len() / wide) {
            for j in 0..wide {
                result[i * wide + j] = 0;
            }
        } else {
            for j in 0..wide {
                let d: usize = index as usize * wide;
                result[i * wide + j] = v2[d + j];
            }
        }
    }

    result
}

fn vrgather_vv(wide: usize, enable_mask: bool, enable_ei16: bool) {
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
    if !enable_mask {
        mask = [0; VLEN / 8];
    }

    let expected = if enable_ei16 {
        rgather_vv(&expected_before, &vs2, &vs1, &mask, enable_mask, wide, 16)
    } else {
        rgather_vv(&expected_before, &vs2, &vs1, &mask, enable_mask, wide, wide)
    };
    let expected = expected.as_slice();

    let vl = vsetvl((VLEN / wide) as u64, wide as u64, 1) as usize;
    assert_eq!(vl, VLEN / wide);

    unsafe {
        rvv_asm!(
            "mv t0, {}", "vl1re8.v v0, (t0)",
            "mv t0, {}", "vl1re8.v v8, (t0)",
            "mv t0, {}", "vl1re8.v v4, (t0)",
            "mv t0, {}", "vl1re8.v v5, (t0)",
            "mv t0, {}", "vl1re8.v v24, (t0)",
            in (reg) mask.as_ptr(),
            in (reg) vs2.as_ptr(),
            in (reg) vs1.as_ptr(),
            in (reg) vs1[256..].as_ptr(),
            in (reg) expected_before.as_ptr());

        if enable_ei16 {
            if enable_mask {
                rvv_asm!("vrgatherei16.vv v24, v8, v4, v0.t");
            } else {
                rvv_asm!("vrgatherei16.vv v24, v8, v4");
            }
        } else {
            if enable_mask {
                rvv_asm!("vrgather.vv v24, v8, v4, v0.t");
            } else {
                rvv_asm!("vrgather.vv v24, v8, v4");
            }
        }

        let p = result.as_mut_ptr();
        rvv_asm!("mv t0, {}", "vs1r.v v24, (t0)", in (reg) p);
    }

    if result != expected {
        log!(
            "[describe = vrgather.vv] unexpected values found: \nresult = {:0>2X?} \nexpected = {:0>2X?}",
            result,
            expected
        );
        log!(
            "more information, \nvs1 = {:0>2X?}, \nvs2 = {:0>2X?}, \nmask = {:0>2X?}, \nexpected_befor = {:0>2X?}, enable_mask = {}, enable_ei16 = {}, \nwide = {}",
            vs1,
            vs2,
            mask,
            expected_before,
            enable_mask,
            enable_ei16,
            wide
        );
        panic!("Abort");
    }
    if is_verbose() {
        log!("finished");
    }
}

pub fn test_vrgather_vv() {
    if is_verbose() {
        log!("test vrgather.vv");
    }

    vrgather_vv(8, false, false);
    vrgather_vv(16, false, false);
    vrgather_vv(128, false, false);

    vrgather_vv(8, true, false);
    vrgather_vv(16, true, false);
    vrgather_vv(128, true, false);
}

pub fn test_vrgatherei16_vv() {
    if is_verbose() {
        log!("test vrgatherei16.vv");
    }

    vrgather_vv(8, false, true);
    vrgather_vv(16, false, true);
    vrgather_vv(128, false, true);

    vrgather_vv(8, true, true);
    vrgather_vv(16, true, true);
    vrgather_vv(128, true, true);
}

fn rgather_vx(vd: &[u8], v2: &[u8], rs: u64, vm: &[u8], wide: usize) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    result.resize(vd.len(), 0);
    result.copy_from_slice(vd);

    let wide = wide / 8;
    let length = vd.len() / wide;

    for i in 0..length {
        if get_bit_in_slice(vm, i) == 1 {
            continue;
        }

        if rs as usize > vd.len() {
            for j in 0..wide {
                result[i * wide + j] = 0;
            }
        } else {
            for j in 0..wide {
                let d: usize = rs as usize * wide;
                result[i * wide + j] = v2[d + j];
            }
        }
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
            "vrgather.vx v21, v2, t0, v0.t",
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
