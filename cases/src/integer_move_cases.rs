use alloc::vec::Vec;
use core::{arch::asm, convert::TryInto};
use eint::{Eint, E1024, E128, E256, E512};
use rvv_asm::rvv_asm;
use rvv_testcases::{
    misc::VLEN,
    rng::fill_rand_bytes,
    runner::{run_template_v_vi, run_template_v_vv, run_template_v_vx, MaskType},
};

use ckb_std::syscalls::debug;
use rvv_testcases::log;

fn test_vmv_v_v() {
    fn expected_op(_: &[u8], rhs: &[u8], result: &mut [u8]) {
        assert!(rhs.len() == result.len());
        result.copy_from_slice(rhs);
    }
    fn op(_: &[u8], _: &[u8], _: MaskType) {
        unsafe {
            rvv_asm!("vmv.v.v v24, v16");
        }
    }

    run_template_v_vv(expected_op, op, false, "vmv.v.v");
}

fn test_vmv_v_x() {
    fn expected_op(_: &[u8], x: u64, result: &mut [u8]) {
        let sew = result.len() * 8;
        match sew {
            8 => {
                result.copy_from_slice(&(x as i8).to_le_bytes());
            }

            16 => {
                result.copy_from_slice(&(x as i16).to_le_bytes());
            }

            32 => {
                result.copy_from_slice(&(x as i32).to_le_bytes());
            }

            64 => {
                result.copy_from_slice(&(x as i64).to_le_bytes());
            }

            128 => {
                E128::from(x as i64).put(result);
            }

            256 => {
                E256::from(x as i64).put(result);
            }

            512 => {
                E512::from(x as i64).put(result);
            }

            1024 => {
                E1024::from(x as i64).put(result);
            }

            _ => {
                panic!("Abort");
            }
        }
    }
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let x = u64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            rvv_asm!("mv t0, {}", "vmv.v.x v24, t0", in (reg) x);
        }
    }

    run_template_v_vx(expected_op, op, false, "vmv.v.x");
}

fn test_vmv_v_i() {
    fn expected_op(_: &[u8], x: i64, result: &mut [u8]) {
        let sew = result.len() * 8;
        match sew {
            8 => {
                result.copy_from_slice(&(x as i8).to_le_bytes());
            }

            16 => {
                result.copy_from_slice(&(x as i16).to_le_bytes());
            }

            32 => {
                result.copy_from_slice(&(x as i32).to_le_bytes());
            }

            64 => {
                result.copy_from_slice(&(x as i64).to_le_bytes());
            }

            128 => {
                E128::from(x as i64).put(result);
            }

            256 => {
                E256::from(x as i64).put(result);
            }

            512 => {
                E512::from(x as i64).put(result);
            }

            1024 => {
                E1024::from(x as i64).put(result);
            }

            _ => {
                panic!("Abort");
            }
        }
    }
    fn op(_: &[u8], rhs: &[u8], _: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => {
                    rvv_asm!("vmv.v.i v24, -16");
                }
                -15 => {
                    rvv_asm!("vmv.v.i v24, -15");
                }
                -14 => {
                    rvv_asm!("vmv.v.i v24, -14");
                }
                -13 => {
                    rvv_asm!("vmv.v.i v24, -13");
                }
                -12 => {
                    rvv_asm!("vmv.v.i v24, -12");
                }
                -11 => {
                    rvv_asm!("vmv.v.i v24, -11");
                }
                -10 => {
                    rvv_asm!("vmv.v.i v24, -10");
                }
                -9 => {
                    rvv_asm!("vmv.v.i v24, -9");
                }
                -8 => {
                    rvv_asm!("vmv.v.i v24, -8");
                }
                -7 => {
                    rvv_asm!("vmv.v.i v24, -7");
                }
                -6 => {
                    rvv_asm!("vmv.v.i v24, -6");
                }
                -5 => {
                    rvv_asm!("vmv.v.i v24, -5");
                }
                -4 => {
                    rvv_asm!("vmv.v.i v24, -4");
                }
                -3 => {
                    rvv_asm!("vmv.v.i v24, -3");
                }
                -2 => {
                    rvv_asm!("vmv.v.i v24, -2");
                }
                -1 => {
                    rvv_asm!("vmv.v.i v24, -1");
                }
                0 => {
                    rvv_asm!("vmv.v.i v24, 0");
                }
                1 => {
                    rvv_asm!("vmv.v.i v24, 1");
                }
                2 => {
                    rvv_asm!("vmv.v.i v24, 2");
                }
                3 => {
                    rvv_asm!("vmv.v.i v24, 3");
                }
                4 => {
                    rvv_asm!("vmv.v.i v24, 4");
                }
                5 => {
                    rvv_asm!("vmv.v.i v24, 5");
                }
                6 => {
                    rvv_asm!("vmv.v.i v24, 6");
                }
                7 => {
                    rvv_asm!("vmv.v.i v24, 7");
                }
                8 => {
                    rvv_asm!("vmv.v.i v24, 8");
                }
                9 => {
                    rvv_asm!("vmv.v.i v24, 9");
                }
                10 => {
                    rvv_asm!("vmv.v.i v24, 10");
                }
                11 => {
                    rvv_asm!("vmv.v.i v24, 11");
                }
                12 => {
                    rvv_asm!("vmv.v.i v24, 12");
                }
                13 => {
                    rvv_asm!("vmv.v.i v24, 13");
                }
                14 => {
                    rvv_asm!("vmv.v.i v24, 14");
                }
                15 => {
                    rvv_asm!("vmv.v.i v24, 15");
                }
                _ => {
                    panic!("Abort, Val: {}", imm);
                }
            }
        }
    }

    run_template_v_vi(expected_op, op, false, true, "vmv.v.i");
}

fn test_vmv1r_v_v() {
    let mut buffer = Vec::<u8>::new();
    buffer.resize(VLEN, 0);
    fill_rand_bytes(buffer.as_mut_slice());

    let mut result = Vec::<u8>::new();
    result.resize(VLEN, 0);
    fill_rand_bytes(result.as_mut_slice());

    let result2 = result.clone();

    unsafe {
        rvv_asm!(
            "mv t0, {}",
            "vl1re8.v v8, (t0)",
            in (reg) buffer.as_ptr(),

        );

        rvv_asm!("vmv1r.v v24, v8");

        rvv_asm!(
            "mv t1, {}",
            "vs1r.v v24, (t1)",
            in (reg) result.as_ptr(),
        );

        rvv_asm!(
            "mv t2, {}",
            "vs1r.v v24, (t2)",
            in (reg) result2.as_ptr(),
        );
    }

    if buffer != result || buffer != result2 {
        log!("befor: {:0>2X?}, \nreslut: {:0>2X?}", buffer, result);
        panic!("Abort");
    }
}

fn test_vmv2r_v_v() {
    let mut buffer = Vec::<u8>::new();
    buffer.resize(VLEN, 0);
    fill_rand_bytes(buffer.as_mut_slice());

    let mut result = Vec::<u8>::new();
    result.resize(VLEN, 0);
    fill_rand_bytes(result.as_mut_slice());

    let result2 = result.clone();

    unsafe {
        rvv_asm!(
            "mv t0, {}",
            "vl2re8.v v8, (t0)",
            in (reg) buffer.as_ptr(),

        );

        rvv_asm!("vmv2r.v v24, v8");

        rvv_asm!(
            "mv t1, {}",
            "vs2r.v v24, (t1)",
            in (reg) result.as_ptr(),
        );

        rvv_asm!(
            "mv t2, {}",
            "vs1r.v v24, (t2)",
            in (reg) result2.as_ptr(),
        );
        rvv_asm!(
            "mv t3, {}",
            "vs1r.v v25, (t3)",
            in (reg) result2[(VLEN/8)..].as_ptr()
        );
    }

    if buffer != result || buffer != result2 {
        log!("befor: {:0>2X?}, reslut: {:0>2X?}", buffer, result);
        panic!("Abort");
    }
}

fn test_vmv4r_v_v() {
    let mut buffer = Vec::<u8>::new();
    buffer.resize(VLEN, 0);
    fill_rand_bytes(buffer.as_mut_slice());

    let mut result = Vec::<u8>::new();
    result.resize(VLEN, 0);
    fill_rand_bytes(result.as_mut_slice());

    let result2 = result.clone();

    unsafe {
        rvv_asm!(
            "mv t0, {}",
            "vl4re8.v v8, (t0)",
            in (reg) buffer.as_ptr(),

        );

        rvv_asm!("vmv4r.v v24, v8");

        rvv_asm!(
            "mv t1, {}",
            "vs4r.v v24, (t1)",
            in (reg) result.as_ptr(),
        );

        rvv_asm!(
            "mv t0, {}",
            "vs1r.v v24, (t0)",
            in (reg) result2.as_ptr(),
        );
        rvv_asm!(
            "mv t0, {}",
            "vs1r.v v25, (t0)",
            in (reg) result2[(VLEN/8)..].as_ptr()
        );
        rvv_asm!(
            "mv t0, {}",
            "vs1r.v v26, (t0)",
            in (reg) result2[(VLEN/8 * 2)..].as_ptr()
        );
        rvv_asm!(
            "mv t0, {}",
            "vs1r.v v27, (t0)",
            in (reg) result2[(VLEN/8 * 3)..].as_ptr()
        );
    }

    if buffer != result || buffer != result2 {
        log!("befor: {:0>2X?}, reslut: {:0>2X?}", buffer, result);
        panic!("Abort");
    }
}

fn test_vmv8r_v_v() {
    let mut buffer = Vec::<u8>::new();
    buffer.resize(VLEN, 0);
    fill_rand_bytes(buffer.as_mut_slice());

    let mut result = Vec::<u8>::new();
    result.resize(VLEN, 0);
    fill_rand_bytes(result.as_mut_slice());

    let result2 = result.clone();

    unsafe {
        rvv_asm!(
            "mv t0, {}",
            "vl8re8.v v8, (t0)",
            in (reg) buffer.as_ptr(),

        );

        rvv_asm!("vmv8r.v v24, v8");

        rvv_asm!(
            "mv t1, {}",
            "vs8r.v v24, (t1)",
            in (reg) result.as_ptr(),
        );

        rvv_asm!(
            "mv t0, {}",
            "vs1r.v v24, (t0)",
            in (reg) result2.as_ptr(),
        );
        rvv_asm!(
            "mv t0, {}",
            "vs1r.v v25, (t0)",
            in (reg) result2[(VLEN/8)..].as_ptr()
        );
        rvv_asm!(
            "mv t0, {}",
            "vs1r.v v26, (t0)",
            in (reg) result2[(VLEN/8 * 2)..].as_ptr()
        );
        rvv_asm!(
            "mv t0, {}",
            "vs1r.v v27, (t0)",
            in (reg) result2[(VLEN/8 * 3)..].as_ptr()
        );
        rvv_asm!(
            "mv t0, {}",
            "vs1r.v v28, (t0)",
            in (reg) result2[(VLEN/8 * 4)..].as_ptr()
        );
        rvv_asm!(
            "mv t0, {}",
            "vs1r.v v29, (t0)",
            in (reg) result2[(VLEN/8 * 5)..].as_ptr()
        );
        rvv_asm!(
            "mv t0, {}",
            "vs1r.v v30, (t0)",
            in (reg) result2[(VLEN/8 * 6)..].as_ptr()
        );
        rvv_asm!(
            "mv t0, {}",
            "vs1r.v v31, (t0)",
            in (reg) result2[(VLEN/8 * 7)..].as_ptr()
        );
    }

    if buffer != result || buffer != result2 {
        log!("befor: {:0>2X?}, reslut: {:0>2X?}", buffer, result);
        panic!("Abort");
    }
}

pub fn test_integer_move() {
    test_vmv_v_v();
    test_vmv_v_x();
    test_vmv_v_i();
    test_vmv1r_v_v();
    test_vmv2r_v_v();
    test_vmv4r_v_v();
    test_vmv8r_v_v();
}
