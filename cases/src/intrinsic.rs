use super::intrinsic_setvl::{v_setivli, v_setvl, v_setvli};
use super::misc::VLEN;
use core::arch::asm;
use rvv_asm::rvv_asm;

// use super::log;
// use ckb_std::syscalls::debug;

static mut VSET_TEST_COUNT: u64 = 0;
static mut RVV_LEN: u64 = 0;

pub fn get_rvv_len() -> u64 {
    unsafe { RVV_LEN }
}

pub fn vsetvl(avl: u64, sew: u64, lmul: i64) -> u64 {
    let count: u64;
    unsafe {
        count = VSET_TEST_COUNT;
        VSET_TEST_COUNT += 1
    }

    let vl = if avl >= 32 {
        match count % 2 {
            0 => v_setvl(avl, sew, lmul),
            1 => v_setvli(avl, sew, lmul),
            _ => panic!("Abort"),
        }
    } else {
        match count % 3 {
            0 => v_setvl(avl, sew, lmul),
            1 => v_setvli(avl, sew, lmul),
            2 => v_setivli(avl, sew, lmul),
            _ => panic!("Abort"),
        }
    };

    unsafe {
        RVV_LEN = vl * (sew >> 3);
    }

    vl
}

// TODO: rvv_asm! doesn't support this
#[allow(unused_macros)]
macro_rules! vle_arm {
    ($p:ident, $sew: literal, $reg: literal) => {
        {
            rvv_asm!(
                "mv t0, {0}",
                concat!("vle", $sew, ".v ",  $reg, " , (t0)"),
                in (reg) $p);
        }
    };
}

// TODO: rvv_asm! doesn't support this
#[allow(unused_macros)]
macro_rules! vse_arm {
    ($p:ident, $sew: literal, $reg: literal) => {
        {
            rvv_asm!(
                "mv t0, {0}",
                concat!("vse", $sew, ".v ", $reg, " , (t0)"),
                in (reg) $p);
        }
    };
}

pub fn vle_v8(sew: u64, buf: &[u8]) {
    let p = buf.as_ptr();
    unsafe {
        match sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vle8.v v8, (t0)", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vle16.v v8, (t0)", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vle32.v v8, (t0)", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vle64.v v8, (t0)", in (reg) p);
            }
            128 => {
                rvv_asm!("mv t0, {0}", "vle128.v v8, (t0)", in (reg) p);
            }
            256 => {
                rvv_asm!("mv t0, {0}", "vle256.v v8, (t0)", in (reg) p);
            }
            512 => {
                rvv_asm!("mv t0, {0}", "vle512.v v8, (t0)", in (reg) p);
            }
            1024 => {
                rvv_asm!("mv t0, {0}", "vle1024.v v8, (t0)", in (reg) p);
            }
            _ => {
                panic!("Invalid sew");
            }
        }
    }
}

pub fn vlse_v8(sew: u64, buf: &[u8], stride: u64) {
    let p = buf.as_ptr();
    unsafe {
        match sew {
            8 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vlse8.v v8, (t0), t1", in (reg) p, in (reg) stride);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vlse16.v v8, (t0), t1", in (reg) p, in (reg) stride);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vlse32.v v8, (t0), t1", in (reg) p, in (reg) stride);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vlse64.v v8, (t0), t1", in (reg) p, in (reg) stride);
            }
            128 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vlse128.v v8, (t0), t1", in (reg) p, in (reg) stride);
            }
            256 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vlse256.v v8, (t0), t1", in (reg) p, in (reg) stride);
            }
            512 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vlse512.v v8, (t0), t1", in (reg) p, in (reg) stride);
            }
            1024 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vlse1024.v v8, (t0), t1", in (reg) p, in (reg) stride);
            }
            _ => {
                panic!("Invalid sew");
            }
        }
    }
}

pub fn vluxei_v8(offset_sew: u64, buf: &[u8], offset: &[u8]) {
    let p = buf.as_ptr();
    let offset_p = offset.as_ptr();
    unsafe {
        match offset_sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vle8.v v16, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vluxei8.v v8, (t0), v16", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vle16.v v16, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vluxei16.v v8, (t0), v16", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vle32.v v16, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vluxei32.v v8, (t0), v16", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vle64.v v16, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vluxei64.v v8, (t0), v16", in (reg) p);
            }
            _ => {
                panic!("Invalid offset_sew = {}", offset_sew);
            }
        }
    }
}

pub fn vloxei_v8(offset_sew: u64, buf: &[u8], offset: &[u8]) {
    let p = buf.as_ptr();
    let offset_p = offset.as_ptr();
    unsafe {
        match offset_sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vle8.v v16, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vloxei8.v v8, (t0), v16", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vle16.v v16, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vloxei16.v v8, (t0), v16", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vle32.v v16, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vloxei32.v v8, (t0), v16", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vle64.v v16, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vloxei64.v v8, (t0), v16", in (reg) p);
            }
            _ => {
                panic!("Invalid offset_sew = {}", offset_sew);
            }
        }
    }
}

pub fn vse_v8(sew: u64, buf: &mut [u8]) {
    assert!(get_rvv_len() <= buf.len() as u64);
    let p = buf.as_ptr();
    unsafe {
        match sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vse8.v v8, (t0)", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vse16.v v8, (t0)", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vse32.v v8, (t0)", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vse64.v v8, (t0)", in (reg) p);
            }
            128 => {
                rvv_asm!("mv t0, {0}", "vse128.v v8, (t0)", in (reg) p);
            }
            256 => {
                rvv_asm!("mv t0, {0}", "vse256.v v8, (t0)", in (reg) p);
            }
            512 => {
                rvv_asm!("mv t0, {0}", "vse512.v v8, (t0)", in (reg) p);
            }
            1024 => {
                rvv_asm!("mv t0, {0}", "vse1024.v v8, (t0)", in (reg) p);
            }
            _ => {
                panic!("Invalid sew");
            }
        }
    }
}

pub fn vsse_v8(sew: u64, buf: &mut [u8], stride: u64) {
    assert!(get_rvv_len() <= buf.len() as u64);
    let p = buf.as_ptr();
    unsafe {
        match sew {
            8 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vsse8.v v8, (t0), t1", in (reg) p, in (reg) stride);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vsse16.v v8, (t0), t1", in (reg) p, in (reg) stride);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vsse32.v v8, (t0), t1", in (reg) p, in (reg) stride);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vsse64.v v8, (t0), t1", in (reg) p, in (reg) stride);
            }
            128 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vsse128.v v8, (t0), t1", in (reg) p, in (reg) stride);
            }
            256 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vsse256.v v8, (t0), t1", in (reg) p, in (reg) stride);
            }
            512 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vsse512.v v8, (t0), t1", in (reg) p, in (reg) stride);
            }
            1024 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vsse1024.v v8, (t0), t1", in (reg) p, in (reg) stride);
            }
            _ => {
                panic!("Invalid sew");
            }
        }
    }
}

pub fn vsuxei_v8(offset_sew: u64, buf: &mut [u8], offset: &[u8]) {
    assert!(get_rvv_len() <= buf.len() as u64);
    let p = buf.as_ptr();
    let offset_p = offset.as_ptr();
    unsafe {
        match offset_sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vle8.v v16, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vsuxei8.v v8, (t0), v16", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vle16.v v16, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vsuxei16.v v8, (t0), v16", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vle32.v v16, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vsuxei32.v v8, (t0), v16", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vle64.v v16, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vsuxei64.v v8, (t0), v16", in (reg) p);
            }
            _ => {
                panic!("Invalid offset_sew = {}", offset_sew);
            }
        }
    }
}

pub fn vsoxei_v8(offset_sew: u64, buf: &mut [u8], offset: &[u8]) {
    assert!(get_rvv_len() <= buf.len() as u64);
    let p = buf.as_ptr();
    let offset_p = offset.as_ptr();
    unsafe {
        match offset_sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vle8.v v16, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vsoxei8.v v8, (t0), v16", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vle16.v v16, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vsoxei16.v v8, (t0), v16", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vle32.v v16, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vsoxei32.v v8, (t0), v16", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vle64.v v16, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vsoxei64.v v8, (t0), v16", in (reg) p);
            }
            _ => {
                panic!("Invalid offset_sew = {}", offset_sew);
            }
        }
    }
}

pub fn vle_v16(sew: u64, buf: &[u8]) {
    let p = buf.as_ptr();
    unsafe {
        match sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vle8.v v16, (t0)", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vle16.v v16, (t0)", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vle32.v v16, (t0)", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vle64.v v16, (t0)", in (reg) p);
            }
            128 => {
                rvv_asm!("mv t0, {0}", "vle128.v v16, (t0)", in (reg) p);
            }
            256 => {
                rvv_asm!("mv t0, {0}", "vle256.v v16, (t0)", in (reg) p);
            }
            512 => {
                rvv_asm!("mv t0, {0}", "vle512.v v16, (t0)", in (reg) p);
            }
            1024 => {
                rvv_asm!("mv t0, {0}", "vle1024.v v16, (t0)", in (reg) p);
            }
            _ => {
                panic!("Invalid sew");
            }
        }
    }
}

pub fn vse_v16(sew: u64, buf: &mut [u8]) {
    assert!(get_rvv_len() <= buf.len() as u64);
    let p = buf.as_ptr();
    unsafe {
        match sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vse8.v v16, (t0)", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vse16.v v16, (t0)", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vse32.v v16, (t0)", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vse64.v v16, (t0)", in (reg) p);
            }
            128 => {
                rvv_asm!("mv t0, {0}", "vse128.v v16, (t0)", in (reg) p);
            }
            256 => {
                rvv_asm!("mv t0, {0}", "vse256.v v16, (t0)", in (reg) p);
            }
            512 => {
                rvv_asm!("mv t0, {0}", "vse512.v v16, (t0)", in (reg) p);
            }
            1024 => {
                rvv_asm!("mv t0, {0}", "vse1024.v v16, (t0)", in (reg) p);
            }
            _ => {
                panic!("Invalid sew");
            }
        }
    }
}

pub fn vle_v24(sew: u64, buf: &[u8]) {
    let p = buf.as_ptr();
    unsafe {
        match sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vle8.v v24, (t0)", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vle16.v v24, (t0)", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vle32.v v24, (t0)", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vle64.v v24, (t0)", in (reg) p);
            }
            128 => {
                rvv_asm!("mv t0, {0}", "vle128.v v24, (t0)", in (reg) p);
            }
            256 => {
                rvv_asm!("mv t0, {0}", "vle256.v v24, (t0)", in (reg) p);
            }
            512 => {
                rvv_asm!("mv t0, {0}", "vle512.v v24, (t0)", in (reg) p);
            }
            1024 => {
                rvv_asm!("mv t0, {0}", "vle1024.v v24, (t0)", in (reg) p);
            }
            _ => {
                panic!("Invalid sew");
            }
        }
    }
}

pub fn vse_v24(sew: u64, buf: &mut [u8]) {
    assert!(
        get_rvv_len() <= buf.len() as u64,
        "buf too small, buf len: {}, rvv len: {}",
        buf.len(),
        get_rvv_len()
    );
    let p = buf.as_ptr();
    unsafe {
        match sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vse8.v v24, (t0)", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vse16.v v24, (t0)", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vse32.v v24, (t0)", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vse64.v v24, (t0)", in (reg) p);
            }
            128 => {
                rvv_asm!("mv t0, {0}", "vse128.v v24, (t0)", in (reg) p);
            }
            256 => {
                rvv_asm!("mv t0, {0}", "vse256.v v24, (t0)", in (reg) p);
            }
            512 => {
                rvv_asm!("mv t0, {0}", "vse512.v v24, (t0)", in (reg) p);
            }
            1024 => {
                rvv_asm!("mv t0, {0}", "vse1024.v v24, (t0)", in (reg) p);
            }
            _ => {
                panic!("Invalid sew");
            }
        }
    }
}

pub fn vs1r_v8(buf: &mut [u8]) {
    assert!(buf.len() >= VLEN / 8);
    let p = buf.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vs1r.v v8, (t0)", in (reg) p);
    }
}

pub fn vs2r_v8(buf: &mut [u8]) {
    assert!(buf.len() >= VLEN / 4);
    let p = buf.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vs2r.v v8, (t0)", in (reg) p);
    }
}

pub fn vs4r_v8(buf: &mut [u8]) {
    assert!(buf.len() >= VLEN / 2);
    let p = buf.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vs4r.v v8, (t0)", in (reg) p);
    }
}

pub fn vs8r_v8(buf: &mut [u8]) {
    assert!(buf.len() >= VLEN);
    let p = buf.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vs8r.v v8, (t0)", in (reg) p);
    }
}

pub fn vs1r_v24(buf: &mut [u8]) {
    assert_eq!(buf.len(), VLEN / 8);
    let p = buf.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vs1r.v v24, (t0)", in (reg) p);
    }
}

pub fn vl1r_v24(buf: &[u8]) {
    assert_eq!(buf.len(), VLEN / 8);
    let p = buf.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vl1re8.v v24, (t0)", in (reg) p);
    }
}

pub fn vl1r_v8(buf: &[u8]) {
    assert_eq!(buf.len(), VLEN / 8);
    let p = buf.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vl1re8.v v8, (t0)", in (reg) p);
    }
}

pub fn vl1r_v0(buf: &[u8]) {
    assert_eq!(buf.len(), VLEN / 8);
    let p = buf.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vl1re8.v v0, (t0)", in (reg) p);
    }
}

pub fn vsm_v_v8(buf: &mut [u8]) {
    assert!(get_rvv_len() <= buf.len() as u64);
    let p = buf.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vsm.v v8, (t0)", in (reg) p);
    }
}

pub fn clean_cache_v8() {
    let temp_buffer = [0u8; 2048];
    let p = temp_buffer.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vl8re8.v v8, (t0)", in (reg) p);
    }
}

pub fn clean_cache_v16() {
    let temp_buffer = [0u8; 2048];
    let p = temp_buffer.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vl8re8.v v16, (t0)", in (reg) p);
    }
}

pub fn clean_cache_v24() {
    let temp_buffer = [0u8; 2048];
    let p = temp_buffer.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vl8re8.v v24, (t0)", in (reg) p);
    }
}
