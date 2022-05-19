use alloc::vec::Vec;
use core::arch::asm;
use core::convert::TryInto;
use rand::Rng;
use rvv_asm::rvv_asm;

use ckb_std::syscalls::debug;
use rvv_testcases::intrinsic::{vs1r_v24, vsetvl};
use rvv_testcases::log;
use rvv_testcases::misc::{get_bit_in_slice, is_verbose, U1024, U256, U512, VLEN};
use rvv_testcases::rng::BestNumberRng;
use rvv_testcases::runner::{run_template, ExpectedParam, InstructionArgsType, MaskType};

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
    v8: &[u8],
    vm: &[u8],
    enable_mask: bool,
    wide: usize,
    v8_wide: usize,
) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    result.resize(vd.len(), 0);
    result.copy_from_slice(vd);

    let v8 = to_wide(v8, v8_wide);

    let wide = wide / 8;
    for i in 0..(vd.len() / wide) {
        if enable_mask && get_bit_in_slice(vm, i) == 0 {
            continue;
        }

        let index = v8[i];
        let pos = index as usize * wide;

        if pos >= vd.len() {
            for j in 0..wide {
                result[i * wide + j] = 0;
            }
        } else {
            result[i * wide..(i + 1) * wide].copy_from_slice(&v2[pos..pos + wide]);
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

        vs1r_v24(&mut result);
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

static mut ZERO_BUFFER: Vec<u8> = Vec::new();

fn expected_op_vrgather(exp_param: &mut ExpectedParam) {
    let x = exp_param.get_right_u64() as usize % 31;
    if (x + 1) > exp_param.get_vl() {
        unsafe {
            exp_param.set_result(&ZERO_BUFFER[..exp_param.sew_bytes]);
        }
    } else {
        let data = ExpectedParam::get_data_by_sclice(
            &exp_param.lhs,
            exp_param.lhs_type,
            exp_param.sew_bytes,
            exp_param.count * exp_param.theoretically_vl + x,
        );
        exp_param.set_result(&data);
    }
}

fn test_vrgather_vx() {
    fn rvv_op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap()) % 31;

        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("mv t0, {}", "vrgather.vx v24, v8, t0, v0.t", in (reg) x);
                }
                MaskType::Disable => {
                    rvv_asm!("mv t0, {}", "vrgather.vx v24, v8, t0", in (reg) x);
                }
                _ => panic!("Abort"),
            };
        }
    }

    run_template(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        InstructionArgsType::Scalar,
        MaskType::Enable,
        expected_op_vrgather,
        rvv_op,
        &[64, 256],
        &[-8, -2, 1, 4, 8],
        "vrgather.vx",
    );
}

fn test_vrgather_vi() {
    fn rvv_op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());

        unsafe {
            match imm {
                0 => {
                    rvv_asm!("vrgather.vi v24, v8, 0");
                }
                1 => {
                    rvv_asm!("vrgather.vi v24, v8, 1");
                }
                2 => {
                    rvv_asm!("vrgather.vi v24, v8, 2");
                }
                3 => {
                    rvv_asm!("vrgather.vi v24, v8, 3");
                }
                4 => {
                    rvv_asm!("vrgather.vi v24, v8, 4");
                }
                5 => {
                    rvv_asm!("vrgather.vi v24, v8, 5");
                }
                6 => {
                    rvv_asm!("vrgather.vi v24, v8, 6");
                }
                7 => {
                    rvv_asm!("vrgather.vi v24, v8, 7");
                }
                8 => {
                    rvv_asm!("vrgather.vi v24, v8, 8");
                }
                9 => {
                    rvv_asm!("vrgather.vi v24, v8, 9");
                }
                10 => {
                    rvv_asm!("vrgather.vi v24, v8, 10");
                }
                11 => {
                    rvv_asm!("vrgather.vi v24, v8, 11");
                }
                12 => {
                    rvv_asm!("vrgather.vi v24, v8, 12");
                }
                13 => {
                    rvv_asm!("vrgather.vi v24, v8, 13");
                }
                14 => {
                    rvv_asm!("vrgather.vi v24, v8, 14");
                }
                15 => {
                    rvv_asm!("vrgather.vi v24, v8, 15");
                }
                16 => {
                    rvv_asm!("vrgather.vi v24, v8, 16");
                }
                17 => {
                    rvv_asm!("vrgather.vi v24, v8, 17");
                }
                18 => {
                    rvv_asm!("vrgather.vi v24, v8, 18");
                }
                19 => {
                    rvv_asm!("vrgather.vi v24, v8, 19");
                }
                20 => {
                    rvv_asm!("vrgather.vi v24, v8, 20");
                }
                21 => {
                    rvv_asm!("vrgather.vi v24, v8, 21");
                }
                22 => {
                    rvv_asm!("vrgather.vi v24, v8, 22");
                }
                23 => {
                    rvv_asm!("vrgather.vi v24, v8, 23");
                }
                24 => {
                    rvv_asm!("vrgather.vi v24, v8, 24");
                }
                25 => {
                    rvv_asm!("vrgather.vi v24, v8, 25");
                }
                26 => {
                    rvv_asm!("vrgather.vi v24, v8, 26");
                }
                27 => {
                    rvv_asm!("vrgather.vi v24, v8, 27");
                }
                28 => {
                    rvv_asm!("vrgather.vi v24, v8, 28");
                }
                29 => {
                    rvv_asm!("vrgather.vi v24, v8, 29");
                }
                30 => {
                    rvv_asm!("vrgather.vi v24, v8, 30");
                }
                31 => {
                    rvv_asm!("vrgather.vi v24, v8, 31");
                }
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        InstructionArgsType::UImmediate,
        MaskType::Disable,
        expected_op_vrgather,
        rvv_op,
        &[64, 256],
        &[-8, -2, 1, 4, 8],
        "vrgather.vi",
    );
}

pub fn test_vrgatherer_vx() {
    unsafe {
        ZERO_BUFFER.resize(128, 0);
    }
    test_vrgather_vx();
    test_vrgather_vi();
}
