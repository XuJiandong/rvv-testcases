use alloc::vec::Vec;
use core::arch::asm;
use core::convert::TryInto;
use rvv_asm::rvv_asm;

use eint::{Eint, E256};
use rvv_testcases::runner::{run_template, RVVTestData, InstructionArgsType, MaskType};

// use rvv_testcases::log;
// use ckb_std::syscalls::debug;

static mut ZERO_BUFFER: Vec<u8> = Vec::new();

fn alway_true(_: f64, _: f64, _: u64) -> bool {
    true
}

fn expected_op_vrgather(rvv_data: &mut RVVTestData) {
    let x = rvv_data.get_right_u64() as usize % 32;
    if (x + 1) > rvv_data.get_vl() {
        unsafe {
            rvv_data.set_result_exp(&ZERO_BUFFER[..rvv_data.sew_bytes]);
        }
    } else {
        let data = rvv_data.get_data(
            &rvv_data.lhs,
            rvv_data.lhs_type,
            rvv_data.count * rvv_data.theoretically_vl + x,
        );
        rvv_data.set_result_exp(&data);
    }
}

fn test_vrgather_vv() {
    fn expected_op(rvv_data: &mut RVVTestData) {
        let rhs = rvv_data.get_right();
        let x = match rhs.len() {
            8 => u64::from_le_bytes(rhs.try_into().unwrap()),
            32 => {
                let r = E256::get(&rhs);
                if r >= E256::from(0xffff) {
                    0xffffu64
                } else {
                    r.u64()
                }
            }
            _ => panic!("Abort"),
        } as usize;

        let (x_end, overflow) = x.overflowing_add(1);
        if overflow || x_end > rvv_data.get_vl() {
            unsafe {
                rvv_data.set_result_exp(&ZERO_BUFFER[..rvv_data.sew_bytes]);
            }
        } else {
            let data = rvv_data.get_data(
                &rvv_data.lhs,
                rvv_data.lhs_type,
                rvv_data.count * rvv_data.theoretically_vl + x,
            );
            rvv_data.set_result_exp(&data);
        }
    }
    fn rvv_op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vrgather.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vrgather.vv v24, v8, v16, v0");
                }
                _ => panic!("Abort"),
            };
        }
    }

    run_template(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        MaskType::Enable,
        expected_op,
        rvv_op,
        alway_true,
        "vrgather.vv",
    );
}

fn test_vrgather16_vv() {
    fn expected_op(rvv_data: &mut RVVTestData) {
        let rhs = rvv_data.get_right();
        let x = u16::from_le_bytes(rhs.try_into().unwrap()) as usize;

        if (x + 1) > rvv_data.get_vl() {
            unsafe {
                rvv_data.set_result_exp(&ZERO_BUFFER[..rvv_data.sew_bytes]);
            }
            //log!("-- index: {} set zero", rvv_data.index);
        } else {
            let data = rvv_data.get_data(
                &rvv_data.lhs,
                rvv_data.lhs_type,
                rvv_data.count * rvv_data.theoretically_vl + x,
            );
            rvv_data.set_result_exp(&data);
            //log!("-- index: {}, count: {}, x: {}, data: {:0>2X?}", rvv_data.index, rvv_data.count, x, data);

        }
    }
    fn rvv_op(_: &[u8], _: &[u8], mask_type: MaskType) {
        unsafe {
            match mask_type {
                MaskType::Enable => {
                    rvv_asm!("vrgatherei16.vv v24, v8, v16, v0.t");
                }
                MaskType::Disable => {
                    rvv_asm!("vrgatherei16.vv v24, v8, v16, v0");
                }
                _ => panic!("Abort"),
            };
        }
    }

    fn before_op(sew: f64, lmul: f64, _: u64) -> bool {
        let emul = 16.0 / sew as f64 * lmul;
        emul >= 0.125 && emul <= 8.0
    }

    run_template(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        InstructionArgsType::VectorAlway16,
        MaskType::Enable,
        expected_op,
        rvv_op,
        before_op,
        "vrgatherei16.vv",
    );
}

fn test_vrgather_vx() {
    fn rvv_op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap()) % 32;

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
        alway_true,
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
        alway_true,
        "vrgather.vi",
    );
}

pub fn test_vrgatherer() {
    unsafe {
        ZERO_BUFFER.resize(128, 0);
    }
    test_vrgather_vx();
    test_vrgather16_vv();
    test_vrgather_vi();
    test_vrgather_vv();
}
