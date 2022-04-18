use alloc::vec::Vec;
use ckb_std::syscalls::debug;
use core::arch::asm;
use eint::{Eint, E1024, E128, E16, E256, E32, E512, E64, E8};
use rand::Rng;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::{
    vle_v16, vle_v24, vle_v8, vloxei_v8, vlse_v8, vluxei_v8, vse_v16, vse_v24, vse_v8, vsoxei_v8,
    vsse_v8, vsuxei_v8,
};
use rvv_testcases::log;
use rvv_testcases::misc::{MutSliceUtils, SliceUtils};
use rvv_testcases::{intrinsic::vsetvl, misc::VLEN, rng::BestNumberRng};

fn fill_all_regisert() {
    let vl = vsetvl(2048, 8, 8);
    assert_eq!(vl, 2048);
    let data = [0x55u8; VLEN];
    unsafe {
        rvv_asm!(
            "mv t0, {}",
            "vle8.v v0, (t0)",
            "vle8.v v8, (t0)",
            "vle8.v v16, (t0)",
            "vle8.v v24, (t0)",
            in (reg) data.as_ptr());
    }
}

fn get_vl_by_lmul(sew: usize, lmul: i64) -> i64 {
    let vlen_byte = VLEN as i64;
    let res = match lmul {
        -8 => vlen_byte / 8,
        -4 => vlen_byte / 4,
        -2 => vlen_byte / 2,
        1 => vlen_byte,
        2 => vlen_byte * 2,
        4 => vlen_byte * 4,
        8 => vlen_byte * 8,
        _ => 0,
    };
    res / sew as i64
}

fn test_unit_stride(sew: usize, lmul: i64, offset: i64, register: usize) {
    fill_all_regisert();
    let vl = get_vl_by_lmul(sew, lmul);
    if vl == 0 {
        return;
    }
    let vl = vl + offset;
    if vl <= 0 {
        return;
    }
    let vl = vl as u64;
    let res_vl = vsetvl(vl, sew as u64, lmul);
    //log!("vsetvl failed, vsetvl result: {}, expected: {}", res_vl, vl);

    let mut mem: Vec<u8> = Vec::new();
    mem.resize(res_vl as usize * sew / 8, 1);

    let mut mem2: Vec<u8> = Vec::new();
    mem2.resize(res_vl as usize * sew / 8, 0);

    let mut rng = BestNumberRng::default();
    rng.fill(&mut mem[..]);
    rng.fill(&mut mem2[..]);

    match register {
        8 => {
            vle_v8(sew as u64, &mem[..]);
            vse_v8(sew as u64, &mem2[..]);
        }
        16 => {
            vle_v16(sew as u64, &mem[..]);
            vse_v16(sew as u64, &mem2[..]);
        }
        24 => {
            vle_v24(sew as u64, &mem[..]);
            vse_v24(sew as u64, &mem2[..]);
        }
        _ => {}
    };

    if mem != mem2 {
        log!(
            "test_unit_stride() failed, sew = {}, lmul = {}, register = {}",
            sew,
            lmul,
            register
        );
        log!(
            "Failed on test_unit_stride: {:0>2X?} (expected) {:0>2X?} (result)",
            mem,
            mem2
        );
        panic!("Abort");
    }
}

// `sew` is in bit `stride` is in bytes
fn test_stride(sew: usize, lmul: i64, stride: usize) {
    fill_all_regisert();
    let vl = get_vl_by_lmul(sew, lmul);
    if vl == 0 {
        return;
    }
    let vl = vl as u64;
    let res_vl = vsetvl(vl, sew as u64, lmul);
    // log!("vsetvl failed, vsetvl result: {}, expected: {}", res_vl, vl);

    let mut mem: Vec<u8> = Vec::new();
    mem.resize(res_vl as usize * sew / 8, 1);

    let mut mem2: Vec<u8> = Vec::new();
    mem2.resize(res_vl as usize * sew / 8, 0);

    let total_size = stride * vl as usize;
    mem.resize(total_size, 0);
    mem2.resize(total_size, 0);

    let mut rng = BestNumberRng::default();
    rng.fill(mem.as_mut_slice());
    rng.fill(mem2.as_mut_slice());

    vlse_v8(sew as u64, &mem[..], stride as u64);
    vsse_v8(sew as u64, &mem2[..], stride as u64);

    for i in 0..vl as usize {
        let range = i * stride as usize..i * stride + sew / 8;
        let expected = &mem[range.clone()];
        let result = &mem2[range.clone()];
        if expected != result {
            log!(
                "test_failed, sew = {}, lmul = {}, stride = {}",
                sew,
                lmul,
                stride
            );
            log!("expected = {:0>2X?}, result = {:0>2X?}", expected, result);
            panic!("Abort");
        }
    }
}

pub fn test_load_store() {
    for sew in [8, 16, 32, 64, 128, 256, 512, 1024] {
        for lmul in [-8, -4, -2, 1, 2, 4, 8] {
            for offset in [-2, 0, 1] {
                for register in [8, 16, 24] {
                    test_unit_stride(sew, lmul, offset, register);
                }
            }
            test_stride(sew, lmul, sew + 16);
        }
    }
}

fn get_offset_val(sew: usize, index: usize, offset: &[u8]) -> usize {
    let data = &offset[index * sew >> 3..(index + 1) * sew >> 3];
    match sew {
        8 => E8::get(data).u32() as usize,
        16 => E16::get(data).u32() as usize,
        32 => E32::get(data).u32() as usize,
        64 => {
            let v = E64::get(data);
            if v > E32::MAX_U.into() {
                0xFFFFFFFF as usize
            } else {
                v.u32() as usize
            }
        }
        128 => {
            let v = E128::get(data);
            if v > E32::MAX_U.into() {
                0xFFFFFFFF as usize
            } else {
                v.u32() as usize
            }
        }
        256 => {
            let v = E256::get(data);
            if v > E32::MAX_U.into() {
                0xFFFFFFFF as usize
            } else {
                v.u32() as usize
            }
        }
        512 => {
            let v = E512::get(data);
            if v > E32::MAX_U.into() {
                0xFFFFFFFF as usize
            } else {
                v.u32() as usize
            }
        }
        1024 => {
            let v = E1024::get(data);
            if v > E32::MAX_U.into() {
                0xFFFFFFFF as usize
            } else {
                v.u32() as usize
            }
        }
        _ => {
            panic!("Abort")
        }
    }
}

fn test_indexed_unordered(sew: usize, offset_sew: usize, lmul: i64) {
    let vl = get_vl_by_lmul(sew, lmul);

    if lmul > 1 && offset_sew > sew && offset_sew / sew * lmul as usize >= 8 {
        return;
    }
    if lmul > 0 && lmul as usize * (offset_sew / sew) > 16 {
        return;
    }
    if vl == 0 {
        return;
    }

    let set_vl = vsetvl(vl as u64, sew as u64, lmul);
    assert_eq!(set_vl, vl as u64);
    let vl = vl as usize;
    let sew_byte = sew / 8;

    let mut rng = BestNumberRng::default();
    let mem: Vec<u8> = {
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(VLEN, 0);
        rng.fill(&mut buf[..]);
        buf
    };

    let offset = {
        let max_offset = mem.len() - 1 - sew_byte;
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(vl * (offset_sew / 8), 0);
        rng.fill(&mut buf[..]);

        let mut index: usize = 1;
        for i in 0..vl {
            let val = get_offset_val(offset_sew, i, &buf);
            if val >= vl {
                let mut buf = buf.as_mut_slice();
                match offset_sew {
                    8 => buf.write_u8(offset_sew, i, E8::from(index as u64)),
                    16 => buf.write_u16(offset_sew, i, E16::from(index as u64)),
                    32 => buf.write_u32(offset_sew, i, E32::from(index as u64)),
                    64 => buf.write_u64(offset_sew, i, E64::from(index as u64)),
                    128 => buf.write_u128(offset_sew, i, E128::from(index as u64)),
                    256 => buf.write_u256(offset_sew, i, E256::from(index as u64)),
                    512 => buf.write_u512(offset_sew, i, E512::from(index as u64)),
                    1024 => buf.write_u1024(offset_sew, i, E1024::from(index as u64)),
                    _ => {
                        log!("unspported offset sew: {}", offset_sew);
                        panic!("Abort");
                    }
                }

                index += 1;
                if index >= max_offset {
                    index = 0;
                }
            }
        }

        buf
    };

    let result1 = {
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(vl * sew_byte, 0xFF);

        buf
    };

    let result2 = {
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(vl * sew_byte, 0xFF);

        buf
    };

    let expected1 = {
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(vl * sew_byte, 0xFF);

        let mem = mem.as_slice();

        for i in 0..vl {
            let index = get_offset_val(offset_sew, i, offset.as_slice());
            if index > mem.len() {
                panic!("Abort");
            } else {
                let data = match sew {
                    8 => &mem[index..index + 1],
                    16 => &mem[index..index + 2],
                    32 => &mem[index..index + 4],
                    64 => &mem[index..index + 8],
                    128 => &mem[index..index + 16],
                    256 => &mem[index..index + 32],
                    512 => &mem[index..index + 64],
                    1024 => &mem[index..index + 128],
                    _ => {
                        log!("unspported sew: {}", sew);
                        panic!("Abort");
                    }
                };
                buf[i * sew_byte..(i + 1) * sew_byte].copy_from_slice(data);
            }
        }
        buf
    };

    vluxei_v8(offset_sew as u64, &mem, &offset);
    vse_v8(sew as u64, &result1);
    if expected1 != result1 {
        log!(
            "Failed on test_indexed_unordered, sew = {}, offset_sew = {}, lmul = {}",
            sew,
            offset_sew,
            &lmul
        );
        log!(
            "More infomation:\nresult1: {:0>2X?}\nexpected1: {:0>2X?}\noffset: {:0>2X?}\nmem: {:0>2X?}",
            result1,
            expected1,
            offset,
            mem
        );
        panic!("Abort");
    }

    let expected2 = {
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(vl * sew_byte, 0xFF);
        let expected1 = expected1.as_slice();
        for i in 0..vl {
            let index = get_offset_val(offset_sew, i, offset.as_slice());
            if index > vl {
                continue;
            } else {
                let mut data: Vec<u8> = Vec::new();
                data.resize(sew_byte, 0);
                match sew {
                    8 => expected1.read_u8(sew, i).put(&mut data),
                    16 => expected1.read_u16(sew, i).put(&mut data),
                    32 => expected1.read_u32(sew, i).put(&mut data),
                    64 => expected1.read_u64(sew, i).put(&mut data),
                    128 => expected1.read_u128(sew, i).put(&mut data),
                    256 => expected1.read_u256(sew, i).put(&mut data),
                    512 => expected1.read_u512(sew, i).put(&mut data),
                    1024 => expected1.read_u1024(sew, i).put(&mut data),
                    _ => {
                        log!("unspported sew: {}", sew);
                        panic!("Abort");
                    }
                };
                let buf_len = buf.len();
                if index + sew_byte > buf_len {
                    buf[index..].copy_from_slice(&data[..buf_len - index]);
                } else {
                    buf[index..index + sew_byte].copy_from_slice(data.as_slice());
                }
            }
        }
        buf
    };
    vsuxei_v8(offset_sew as u64, &result2, &offset);

    if expected2 != result2 {
        log!(
            "Failed on test_indexed_unordered, sew = {}, offset_sew = {}, lmul = {}",
            sew,
            offset_sew,
            &lmul
        );
        log!(
            "More infomation:\nresult2: {:0>2X?}\nexpected2: {:0>2X?}\noffset: {:0>2X?}\nmem: {:0>2X?}",
            result2,
            expected2,
            offset,
            mem
        );
        panic!("Abort");
    }
}

fn test_indexed_ordered(sew: usize, offset_sew: usize, lmul: i64) {
    let vl = get_vl_by_lmul(sew, lmul);

    if lmul > 1 && offset_sew > sew && offset_sew / sew * lmul as usize >= 8 {
        return;
    }
    if lmul > 0 && lmul as usize * (offset_sew / sew) > 16 {
        return;
    }
    if vl == 0 {
        return;
    }

    let set_vl = vsetvl(vl as u64, sew as u64, lmul);
    assert_eq!(set_vl, vl as u64);
    let vl = vl as usize;
    let sew_byte = sew / 8;

    let mut rng = BestNumberRng::default();
    let mem: Vec<u8> = {
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(VLEN, 0);
        rng.fill(&mut buf[..]);
        buf
    };

    let offset = {
        let max_offset = mem.len() - 1 - sew_byte;
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(vl * (offset_sew / 8), 0);
        rng.fill(&mut buf[..]);

        let mut index: usize = 1;
        for i in 0..vl {
            let val = get_offset_val(offset_sew, i, &buf);
            if val >= vl {
                let mut buf = buf.as_mut_slice();
                match offset_sew {
                    8 => buf.write_u8(offset_sew, i, E8::from(index as u64)),
                    16 => buf.write_u16(offset_sew, i, E16::from(index as u64)),
                    32 => buf.write_u32(offset_sew, i, E32::from(index as u64)),
                    64 => buf.write_u64(offset_sew, i, E64::from(index as u64)),
                    128 => buf.write_u128(offset_sew, i, E128::from(index as u64)),
                    256 => buf.write_u256(offset_sew, i, E256::from(index as u64)),
                    512 => buf.write_u512(offset_sew, i, E512::from(index as u64)),
                    1024 => buf.write_u1024(offset_sew, i, E1024::from(index as u64)),
                    _ => {
                        log!("unspported offset sew: {}", offset_sew);
                        panic!("Abort");
                    }
                }

                index += 1;
                if index >= max_offset {
                    index = 0;
                }
            }
        }

        buf
    };

    let result1 = {
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(vl * sew_byte, 0xFF);

        buf
    };

    let result2 = {
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(vl * sew_byte, 0xFF);

        buf
    };

    let expected1 = {
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(vl * sew_byte, 0xFF);

        let mem = mem.as_slice();

        for i in 0..vl {
            let index = get_offset_val(offset_sew, i, offset.as_slice());
            if index > mem.len() {
                panic!("Abort");
            } else {
                let data = match sew {
                    8 => &mem[index..index + 1],
                    16 => &mem[index..index + 2],
                    32 => &mem[index..index + 4],
                    64 => &mem[index..index + 8],
                    128 => &mem[index..index + 16],
                    256 => &mem[index..index + 32],
                    512 => &mem[index..index + 64],
                    1024 => &mem[index..index + 128],
                    _ => {
                        log!("unspported sew: {}", sew);
                        panic!("Abort");
                    }
                };
                buf[i * sew_byte..(i + 1) * sew_byte].copy_from_slice(data);
            }
        }
        buf
    };

    vloxei_v8(offset_sew as u64, &mem, &offset);
    vse_v8(sew as u64, &result1);
    if expected1 != result1 {
        log!(
            "Failed on test_indexed_unordered, sew = {}, offset_sew = {}, lmul = {}",
            sew,
            offset_sew,
            &lmul
        );
        log!(
            "More infomation:\nresult1: {:0>2X?}\nexpected1: {:0>2X?}\noffset: {:0>2X?}\nmem: {:0>2X?}",
            result1,
            expected1,
            offset,
            mem
        );
        panic!("Abort");
    }

    let expected2 = {
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(vl * sew_byte, 0xFF);
        let expected1 = expected1.as_slice();
        for i in 0..vl {
            let index = get_offset_val(offset_sew, i, offset.as_slice());
            if index > vl {
                continue;
            } else {
                let mut data: Vec<u8> = Vec::new();
                data.resize(sew_byte, 0);
                match sew {
                    8 => expected1.read_u8(sew, i).put(&mut data),
                    16 => expected1.read_u16(sew, i).put(&mut data),
                    32 => expected1.read_u32(sew, i).put(&mut data),
                    64 => expected1.read_u64(sew, i).put(&mut data),
                    128 => expected1.read_u128(sew, i).put(&mut data),
                    256 => expected1.read_u256(sew, i).put(&mut data),
                    512 => expected1.read_u512(sew, i).put(&mut data),
                    1024 => expected1.read_u1024(sew, i).put(&mut data),
                    _ => {
                        log!("unspported sew: {}", sew);
                        panic!("Abort");
                    }
                };
                let buf_len = buf.len();
                if index + sew_byte > buf_len {
                    buf[index..].copy_from_slice(&data[..buf_len - index]);
                } else {
                    buf[index..index + sew_byte].copy_from_slice(data.as_slice());
                }
            }
        }
        buf
    };
    vsoxei_v8(offset_sew as u64, &result2, &offset);

    if expected2 != result2 {
        log!(
            "Failed on test_indexed_unordered, sew = {}, offset_sew = {}, lmul = {}",
            sew,
            offset_sew,
            &lmul
        );
        log!(
            "More infomation:\nresult2: {:0>2X?}\nexpected2: {:0>2X?}\noffset: {:0>2X?}\nmem: {:0>2X?}",
            result2,
            expected2,
            offset,
            mem
        );
        panic!("Abort");
    }
}

pub fn test_load_store_uxei() {
    // TODO vsuxei may cause memory out of bounds
    test_indexed_unordered(512, 64, -4);
    for sew in [8, 16, 32, 64, 128, 256, 512, 1024] {
        for offset_sew in [8, 16, 32, 64] {
            for lmul in [-8, -4, -2, 1, 2, 4, 8] {
                test_indexed_unordered(sew, offset_sew, lmul);
                test_indexed_ordered(sew, offset_sew, lmul);
            }
        }
    }
}

fn test_vlm_v(mem: &Vec<u8>, sew: usize, lmul: i64) {
    fill_all_regisert();

    let vl = get_vl_by_lmul(sew, lmul);
    if vl == 0 {
        return;
    }
    let set_vl = vsetvl(vl as u64, sew as u64, lmul);
    assert_eq!(set_vl, vl as u64);
    let vl = vl as usize;

    let sew_byte = sew >> 3;
    let mem_len = vl * sew_byte;
    let mut ceil_len = mem_len / sew;
    if ceil_len == 0 {
        ceil_len = 1;
    }

    let mut result1: Vec<u8> = Vec::new();
    result1.resize(ceil_len, 0xFF);

    unsafe {
        rvv_asm!(
            "mv t0, {}",
            in (reg) mem.as_ptr()
        );

        rvv_asm! {
            "vlm.v v8, t0",
        };

        rvv_asm!(
            "mv t0, {}",
            "vsm.v v8, t0",
            in (reg) result1.as_ptr()
        );
    }

    let expected1 = &mem[..ceil_len];

    if expected1 != result1 {
        log!(
            "Failed on test_vector_unit_stride, sew = {}, lmul = {}, vl = {}",
            sew,
            lmul,
            vl
        );
        log!(
            "More infomation:\nresult: {:0>2X?}\nexpected: {:0>2X?}",
            result1,
            expected1
        );
        log!(
            "More infomation: sew_byte: {}, mem len: {}, ceil len: {}",
            sew_byte,
            mem_len,
            ceil_len
        );
        panic!("Abort");
    }

    let mut result2: Vec<u8> = Vec::new();
    result2.resize(mem_len, 0xFF);

    vse_v8(sew as u64, &result2);

    let expected2 = {
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(mem_len, 0x55);

        buf[..ceil_len].copy_from_slice(&mem[..ceil_len]);
        buf
    };

    if expected2 != result2 {
        log!(
            "Failed on test_vector_unit_stride, sew = {}, lmul = {}, vl = {}",
            sew,
            lmul,
            vl
        );
        log!(
            "More infomation:\nresult2: {:0>2X?}\nexpected2: {:0>2X?}",
            result2,
            expected2
        );
        log!(
            "More infomation: sew_byte: {}, mem len: {}, ceil len: {}",
            sew_byte,
            mem_len,
            ceil_len
        );
        panic!("Abort");
    }
}

pub fn test_vector_unit_stride() {
    let mut rng = BestNumberRng::default();
    let mem: Vec<u8> = {
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(VLEN, 0x77);
        rng.fill(&mut buf[..]);
        buf
    };

    for sew in [8, 16, 32, 64, 128, 256, 512, 1024] {
        for lmul in [-8, -4, -2, 1, 2, 4, 8] {
            test_vlm_v(&mem, sew, lmul);
        }
    }
}
