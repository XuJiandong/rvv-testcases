use alloc::vec::Vec;
use ckb_std::syscalls::debug;
use core::arch::asm;
use eint::{Eint, E16, E32, E64, E8};
use rand::Rng;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::{
    vle_v16, vle_v24, vle_v8, vloxei_v8, vlse_v8, vluxei_v8, vse_v16, vse_v24, vse_v8, vsoxei_v8,
    vsse_v8, vsuxei_v8,
};
use rvv_testcases::log;
use rvv_testcases::misc::MutSliceUtils;
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
    let data = &offset[index * (sew >> 3)..(index + 1) * (sew >> 3)];
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
        _ => {
            panic!("Abort")
        }
    }
}

fn test_indexed_unordered(sew: usize, offset_sew: usize, lmul: i64, test_ordered: bool) {
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
        buf.resize(vl * sew_byte, 0);
        rng.fill(&mut buf[..]);
        buf
    };

    let offset = {
        let max_offset = mem.len() - sew_byte;

        let mut buf: Vec<u8> = Vec::new();
        buf.resize(vl * (offset_sew / 8), 0);
        rng.fill(&mut buf[..]);

        let mut index: usize = 0;
        for i in 0..vl {
            let val = get_offset_val(offset_sew, i, &buf);
            if val > max_offset {
                let mut buf = buf.as_mut_slice();
                match offset_sew {
                    8 => buf.write_u8(offset_sew, i, E8::from(index as u64)),
                    16 => buf.write_u16(offset_sew, i, E16::from(index as u64)),
                    32 => buf.write_u32(offset_sew, i, E32::from(index as u64)),
                    64 => buf.write_u64(offset_sew, i, E64::from(index as u64)),
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
    if test_ordered {
        vloxei_v8(offset_sew as u64, &mem, &offset);
    } else {
        vluxei_v8(offset_sew as u64, &mem, &offset);
    }
    vse_v8(sew as u64, &result1);

    let expected1 = {
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(vl * sew_byte, 0xFF);

        for i in 0..vl {
            let index = get_offset_val(offset_sew, i, offset.as_slice());
            if index + sew_byte > mem.len() {
                log!("index: {} too long", index);
                panic!("Abort");
            } else {
                let data = &mem[index..index + sew_byte];
                buf[i * sew_byte..(i + 1) * sew_byte].copy_from_slice(data);
            }
        }
        buf
    };

    if expected1 != result1 {
        log!(
            "Failed on test_indexed_unordered, sew = {}, offset_sew = {}, lmul = {}, vl = {}",
            sew,
            offset_sew,
            lmul,
            vl
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

    let result2 = {
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(vl * sew_byte, 0xFF);

        buf
    };
    if test_ordered {
        vsoxei_v8(offset_sew as u64, &result2, &offset);
    } else {
        vsuxei_v8(offset_sew as u64, &result2, &offset);
    }

    let expected2 = {
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(vl * sew_byte, 0xFF);
        let expected1 = expected1.as_slice();
        for i in 0..vl {
            let index = get_offset_val(offset_sew, i, offset.as_slice());
            if index >= vl * sew_byte {
                log!("index {} is out of bound", index);
                panic!("Abore");
            } else {
                let data = &expected1[i * sew_byte..i * sew_byte + sew_byte];
                let buf_len = buf.len();
                if index + sew_byte > buf_len {
                    buf[index..].copy_from_slice(&data[..buf_len - index]);
                } else {
                    buf[index..index + sew_byte].copy_from_slice(data);
                }
            }
        }
        buf
    };

    if expected2 != result2 {
        log!(
            "Failed on test_indexed_unordered, sew = {}, offset_sew = {}, lmul = {}, vl = {}",
            sew,
            offset_sew,
            lmul,
            vl
        );
        log!(
            "More infomation:\nresult1: {:0>2X?}\nexpected1: {:0>2X?}\nresult2: {:0>2X?}\nexpected2: {:0>2X?}\noffset: {:0>2X?}\nmem: {:0>2X?}",
            result1,
            expected1,
            result2,
            expected2,
            offset,
            mem
        );
        panic!("Abort");
    }
}

pub fn test_load_store_uxei() {
    for sew in [8, 16, 32, 64, 128, 256, 512, 1024] {
        for offset_sew in [8, 16, 32, 64] {
            for lmul in [-8, -4, -2, 1, 2, 4, 8] {
                test_indexed_unordered(sew, offset_sew, lmul, false);
                test_indexed_unordered(sew, offset_sew, lmul, true);
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
            "vlm.v v8, (t0)",
        };

        rvv_asm!(
            "mv t0, {}",
            "vsm.v v8, (t0)",
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

fn store_whole_v8(whole: usize, out_buf: &mut [u8]) {
    unsafe {
        rvv_asm!(
            "mv t0, {}",
            in (reg) out_buf.as_ptr()
        );
    }

    match whole {
        1 => unsafe {
            rvv_asm!("vs1r.v v8, (t0)");
        },
        2 => unsafe {
            rvv_asm!("vs2r.v v8, (t0)");
        },
        4 => unsafe {
            rvv_asm!("vs4r.v v8, (t0)");
        },
        8 => unsafe {
            rvv_asm!("vs8r.v v8, (t0)");
        },
        _ => panic!("Abort"),
    }
}

fn load_whole_v8(whole: usize, whole_len: usize, buf: &[u8]) {
    unsafe {
        rvv_asm!(
            "mv t0, {}",
            in (reg) buf.as_ptr()
        );
    }

    match whole {
        1 => match whole_len {
            8 => unsafe {
                rvv_asm!("vl1re8.v v8, (t0)");
            },
            16 => unsafe {
                rvv_asm!("vl1re16.v v8, (t0)");
            },
            32 => unsafe {
                rvv_asm!("vl1re32.v v8, (t0)");
            },
            64 => unsafe {
                rvv_asm!("vl1re64.v v8, (t0)");
            },
            _ => panic!("Abort"),
        },
        2 => match whole_len {
            8 => unsafe {
                rvv_asm!("vl2re8.v v8, (t0)");
            },
            16 => unsafe {
                rvv_asm!("vl2re16.v v8, (t0)");
            },
            32 => unsafe {
                rvv_asm!("vl2re32.v v8, (t0)");
            },
            64 => unsafe {
                rvv_asm!("vl2re64.v v8, (t0)");
            },
            _ => panic!("Abort"),
        },
        4 => match whole_len {
            8 => unsafe {
                rvv_asm!("vl4re8.v v8, (t0)");
            },
            16 => unsafe {
                rvv_asm!("vl4re16.v v8, (t0)");
            },
            32 => unsafe {
                rvv_asm!("vl4re32.v v8, (t0)");
            },
            64 => unsafe {
                rvv_asm!("vl4re64.v v8, (t0)");
            },
            _ => panic!("Abort"),
        },
        8 => match whole_len {
            8 => unsafe {
                rvv_asm!("vl8re8.v v8, (t0)");
            },
            16 => unsafe {
                rvv_asm!("vl8re16.v v8, (t0)");
            },
            32 => unsafe {
                rvv_asm!("vl8re32.v v8, (t0)");
            },
            64 => unsafe {
                rvv_asm!("vl8re64.v v8, (t0)");
            },
            _ => panic!("Abort"),
        },
        _ => {
            log!("can run vl{}re{}", whole, whole_len);
            panic!("Abort")
        }
    }
}

fn get_whole_expected(
    load_whole: usize,
    load_whole_len: usize,
    store_whole: usize,
    mem: &[u8],
) -> Vec<u8> {
    let load_len = VLEN / 8 / (load_whole_len / 8) * load_whole;
    let mut data = [0x55u8; 2048];
    data[..load_len].copy_from_slice(&mem[..load_len]);

    let store_len = VLEN / 8 * store_whole;
    let mut data2 = [0u8; 2048];

    data2[..store_len].copy_from_slice(&data[..store_len]);

    data2.to_vec()
}

fn check_whole(
    mem: &[u8],
    load_whole: usize,
    load_whole_len: usize,
    store_whole: usize,
    sew: usize,
    lmul: i64,
) {
    vsetvl(2048, 8, 8);
    let data = [0x55u8; VLEN];
    unsafe {
        rvv_asm!(
            "mv t0, {}",
            "vle8.v v8, (t0)",
            in (reg) data.as_ptr());
    }

    let vl = get_vl_by_lmul(sew, lmul);
    if vl == 0 {
        return;
    }
    let set_vl = vsetvl(vl as u64, sew as u64, lmul);
    assert_eq!(set_vl, vl as u64);

    load_whole_v8(load_whole, load_whole_len, &mem);

    let mut result = [0u8; 2048];
    store_whole_v8(store_whole, &mut result);

    let expected = get_whole_expected(load_whole, load_whole_len, store_whole, &mem);
    if result != expected.as_slice() {
        log!(
            "Failed on test_whole_load_store vl{}re{}.v vs{}r.v, sew = {}, lmul = {}",
            load_whole,
            load_whole_len,
            1,
            sew,
            lmul
        );
        log!(
            "More infomation:\nresult: {:0>2X?}\nexpected: {:0>2X?}",
            result,
            expected
        );
        panic!("Abort");
    }
}

fn whole_load_store(sew: usize, lmul: i64) {
    let mem = {
        let mut rng = BestNumberRng::default();
        let mut buf = [1u8; 2048];
        rng.fill(&mut buf[..]);
        buf
    };

    for load_whole in [1, 2, 4, 8] {
        for load_whole_len in [8, 16, 32, 64] {
            for store_whole in [1, 2, 4, 8] {
                check_whole(&mem, load_whole, load_whole_len, store_whole, sew, lmul);
            }
        }
    }
}

pub fn test_whole_load_store() {
    for sew in [8, 16, 32, 64, 128, 256, 512, 1024] {
        for lmul in [-8, -4, -2, 1, 2, 4, 8] {
            whole_load_store(sew, lmul);
        }
    }
}
