use crate::misc::{compress_into_bits, get_bit_in_slice, set_bit_in_slice};

use super::misc::{create_vtype, VLEN};
use super::runner::WideningCategory;
use core::arch::asm;
use rvv_asm::rvv_asm;

// use super::log;
// use ckb_std::syscalls::debug;

#[inline(never)]
pub fn vsetvl(avl: u64, sew: u64, lmul: i64) -> u64 {
    let vtype = create_vtype(sew, lmul);
    let mut vl: u64;
    unsafe {
        rvv_asm!(
            "mv t1, {0}",
            "mv t2, {1}",
            "vsetvl t0, t1, t2",
            "mv {2}, t0",
            in (reg) avl,
            in (reg) vtype,
            out (reg) vl,
        );
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

pub fn vle_v1(sew: u64, buf: &[u8]) {
    let p = buf.as_ptr();
    unsafe {
        match sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vle8.v v1, (t0)", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vle16.v v1, (t0)", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vle32.v v1, (t0)", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vle64.v v1, (t0)", in (reg) p);
            }
            128 => {
                rvv_asm!("mv t0, {0}", "vle128.v v1, (t0)", in (reg) p);
            }
            256 => {
                rvv_asm!("mv t0, {0}", "vle256.v v1, (t0)", in (reg) p);
            }
            512 => {
                rvv_asm!("mv t0, {0}", "vle512.v v1, (t0)", in (reg) p);
            }
            1024 => {
                rvv_asm!("mv t0, {0}", "vle1024.v v1, (t0)", in (reg) p);
            }
            _ => {
                panic!("Invalid sew");
            }
        }
    }
}

pub fn vlse_v1(sew: u64, buf: &[u8], stride: u64) {
    let p = buf.as_ptr();
    unsafe {
        match sew {
            8 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vlse8.v v1, (t0), t1", in (reg) p, in (reg) stride);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vlse16.v v1, (t0), t1", in (reg) p, in (reg) stride);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vlse32.v v1, (t0), t1", in (reg) p, in (reg) stride);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vlse64.v v1, (t0), t1", in (reg) p, in (reg) stride);
            }
            128 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vlse128.v v1, (t0), t1", in (reg) p, in (reg) stride);
            }
            256 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vlse256.v v1, (t0), t1", in (reg) p, in (reg) stride);
            }
            512 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vlse512.v v1, (t0), t1", in (reg) p, in (reg) stride);
            }
            1024 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vlse1024.v v1, (t0), t1", in (reg) p, in (reg) stride);
            }
            _ => {
                panic!("Invalid sew");
            }
        }
    }
}

pub fn vlxei_v1(offset_sew: u64, buf: &[u8], offset: &[u8]) {
    let p = buf.as_ptr();
    let offset_p = offset.as_ptr();
    unsafe {
        match offset_sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vle8.v v29, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vluxei8.v v1, (t0), v29", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vle16.v v29, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vluxei16.v v1, (t0), v29", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vle32.v v29, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vluxei32.v v1, (t0), v29", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vle64.v v29, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vluxei64.v v1, (t0), v29", in (reg) p);
            }
            _ => {
                panic!("Invalid offset_sew = {}", offset_sew);
            }
        }
    }
}

pub fn vse_v1(sew: u64, buf: &[u8]) {
    let p = buf.as_ptr();
    unsafe {
        match sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vse8.v v1, (t0)", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vse16.v v1, (t0)", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vse32.v v1, (t0)", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vse64.v v1, (t0)", in (reg) p);
            }
            128 => {
                rvv_asm!("mv t0, {0}", "vse128.v v1, (t0)", in (reg) p);
            }
            256 => {
                rvv_asm!("mv t0, {0}", "vse256.v v1, (t0)", in (reg) p);
            }
            512 => {
                rvv_asm!("mv t0, {0}", "vse512.v v1, (t0)", in (reg) p);
            }
            1024 => {
                rvv_asm!("mv t0, {0}", "vse1024.v v1, (t0)", in (reg) p);
            }
            _ => {
                panic!("Invalid sew");
            }
        }
    }
}

pub fn vsse_v1(sew: u64, buf: &[u8], stride: u64) {
    let p = buf.as_ptr();
    unsafe {
        match sew {
            8 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vsse8.v v1, (t0), t1", in (reg) p, in (reg) stride);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vsse16.v v1, (t0), t1", in (reg) p, in (reg) stride);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vsse32.v v1, (t0), t1", in (reg) p, in (reg) stride);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vsse64.v v1, (t0), t1", in (reg) p, in (reg) stride);
            }
            128 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vsse128.v v1, (t0), t1", in (reg) p, in (reg) stride);
            }
            256 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vsse256.v v1, (t0), t1", in (reg) p, in (reg) stride);
            }
            512 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vsse512.v v1, (t0), t1", in (reg) p, in (reg) stride);
            }
            1024 => {
                rvv_asm!("mv t0, {0}", "mv t1, {1}", "vsse1024.v v1, (t0), t1", in (reg) p, in (reg) stride);
            }
            _ => {
                panic!("Invalid sew");
            }
        }
    }
}

pub fn vsxei_v1(offset_sew: u64, buf: &[u8], offset: &[u8]) {
    let p = buf.as_ptr();
    let offset_p = offset.as_ptr();
    unsafe {
        match offset_sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vle8.v v29, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vsuxei8.v v1, (t0), v29", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vle16.v v29, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vsuxei16.v v1, (t0), v29", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vle32.v v29, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vsuxei32.v v1, (t0), v29", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vle64.v v29, (t0)", in (reg) offset_p);
                rvv_asm!("mv t0, {0}", "vsuxei64.v v1, (t0), v29", in (reg) p);
            }
            _ => {
                panic!("Invalid offset_sew = {}", offset_sew);
            }
        }
    }
}

fn vle_v11(sew: u64, buf: &[u8]) {
    let p = buf.as_ptr();
    unsafe {
        match sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vle8.v v11, (t0)", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vle16.v v11, (t0)", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vle32.v v11, (t0)", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vle64.v v11, (t0)", in (reg) p);
            }
            128 => {
                rvv_asm!("mv t0, {0}", "vle128.v v11, (t0)", in (reg) p);
            }
            256 => {
                rvv_asm!("mv t0, {0}", "vle256.v v11, (t0)", in (reg) p);
            }
            512 => {
                rvv_asm!("mv t0, {0}", "vle512.v v11, (t0)", in (reg) p);
            }
            1024 => {
                rvv_asm!("mv t0, {0}", "vle1024.v v11, (t0)", in (reg) p);
            }
            _ => {
                panic!("Invalid sew");
            }
        }
    }
}

fn vle_v21(sew: u64, buf: &[u8]) {
    let p = buf.as_ptr();
    unsafe {
        match sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vle8.v v21, (t0)", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vle16.v v21, (t0)", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vle32.v v21, (t0)", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vle64.v v21, (t0)", in (reg) p);
            }
            128 => {
                rvv_asm!("mv t0, {0}", "vle128.v v21, (t0)", in (reg) p);
            }
            256 => {
                rvv_asm!("mv t0, {0}", "vle256.v v21, (t0)", in (reg) p);
            }
            512 => {
                rvv_asm!("mv t0, {0}", "vle512.v v21, (t0)", in (reg) p);
            }
            1024 => {
                rvv_asm!("mv t0, {0}", "vle1024.v v21, (t0)", in (reg) p);
            }
            _ => {
                panic!("Invalid sew");
            }
        }
    }
}

fn vse_v21(sew: u64, buf: &[u8]) {
    let p = buf.as_ptr();
    unsafe {
        match sew {
            8 => {
                rvv_asm!("mv t0, {0}", "vse8.v v21, (t0)", in (reg) p);
            }
            16 => {
                rvv_asm!("mv t0, {0}", "vse16.v v21, (t0)", in (reg) p);
            }
            32 => {
                rvv_asm!("mv t0, {0}", "vse32.v v21, (t0)", in (reg) p);
            }
            64 => {
                rvv_asm!("mv t0, {0}", "vse64.v v21, (t0)", in (reg) p);
            }
            128 => {
                rvv_asm!("mv t0, {0}", "vse128.v v21, (t0)", in (reg) p);
            }
            256 => {
                rvv_asm!("mv t0, {0}", "vse256.v v21, (t0)", in (reg) p);
            }
            512 => {
                rvv_asm!("mv t0, {0}", "vse512.v v21, (t0)", in (reg) p);
            }
            1024 => {
                rvv_asm!("mv t0, {0}", "vse1024.v v21, (t0)", in (reg) p);
            }
            _ => {
                panic!("Invalid sew");
            }
        }
    }
}

pub fn vs1r_v1(buf: &mut [u8]) {
    assert_eq!(buf.len(), VLEN / 8);
    let p = buf.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vs1r.v v1, (t0)", in (reg) p);
    }
}

pub fn vs1r_v21(buf: &mut [u8]) {
    assert_eq!(buf.len(), VLEN / 8);
    let p = buf.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vs1r.v v21, (t0)", in (reg) p);
    }
}

pub fn vl1r_v21(buf: &[u8]) {
    assert_eq!(buf.len(), VLEN / 8);
    let p = buf.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vl1re8.v v21, (t0)", in (reg) p);
    }
}

pub fn vl1r_v1(buf: &[u8]) {
    assert_eq!(buf.len(), VLEN / 8);
    let p = buf.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vl1re8.v v1, (t0)", in (reg) p);
    }
}

pub fn vl1r_v0(buf: &[u8]) {
    assert_eq!(buf.len(), VLEN / 8);
    let p = buf.as_ptr();
    unsafe {
        rvv_asm!("mv t0, {}", "vl1re8.v v0, (t0)", in (reg) p);
    }
}

//
// format
// <vd>op_<vs2><vs1>
// possible choice
// v: SEW width
// w: SEW*2 width
// n: SEW/2 SEW/4, ...
// x: X Register
// i: immediate
#[inline(never)]
pub fn vop_vv<F>(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, avl: u64, lmul: i64, op: F)
where
    F: Fn(),
{
    let mut avl = avl;
    let mut lhs = lhs;
    let mut rhs = rhs;
    let mut result = result;

    let sew_bytes = sew / 8;

    loop {
        let vl = vsetvl(avl as u64, sew, lmul);
        vle_v1(sew, lhs);
        vle_v11(sew, rhs);

        op();

        vse_v21(sew, result);

        avl -= vl;
        if avl == 0 {
            break;
        }
        let offset = (vl * sew_bytes) as usize;
        result = &mut result[offset..];
        lhs = &lhs[offset..];
        rhs = &rhs[offset..];
    }
}

#[inline(never)]
pub fn vop_vvm<F>(
    lhs: &[u8],
    rhs: &[u8],
    result: &mut [u8],
    v0: &[u8],
    sew: u64,
    avl: u64,
    lmul: i64,
    op: F,
) where
    F: Fn(),
{
    let mut avl = avl;
    let mut lhs = lhs;
    let mut rhs = rhs;
    let mut result = result;
    let mut v0 = v0;

    let sew_bytes = sew / 8;

    loop {
        let vl = vsetvl(avl as u64, sew, lmul);
        vle_v1(sew, lhs);
        vle_v11(sew, rhs);
        let v0_masks = compress_into_bits(&v0[0..vl as usize]);
        vl1r_v0(v0_masks.as_slice());

        op();

        vse_v21(sew, result);

        avl -= vl;
        if avl == 0 {
            break;
        }
        let offset = (vl * sew_bytes) as usize;
        result = &mut result[offset..];
        lhs = &lhs[offset..];
        rhs = &rhs[offset..];
        v0 = &v0[vl as usize..];
    }
}

#[inline(never)]
pub fn vop_vv_destructive<F>(
    lhs: &[u8],
    rhs: &[u8],
    result: &mut [u8],
    sew: u64,
    avl: u64,
    lmul: i64,
    op: F,
) where
    F: Fn(),
{
    let mut avl = avl;
    let mut lhs = lhs;
    let mut rhs = rhs;
    let mut result = result;

    let sew_bytes = sew / 8;

    loop {
        let vl = vsetvl(avl as u64, sew, lmul);
        vle_v1(sew, lhs);
        vle_v11(sew, rhs);
        vle_v21(sew, result);

        op();

        vse_v21(sew, result);

        avl -= vl;
        if avl == 0 {
            break;
        }
        let offset = (vl * sew_bytes) as usize;
        result = &mut result[offset..];
        lhs = &lhs[offset..];
        rhs = &rhs[offset..];
    }
}

#[inline(never)]
pub fn vop_vv_destructive_wide<F>(
    lhs: &[u8],
    rhs: &[u8],
    result: &mut [u8],
    sew: u64,
    avl: u64,
    lmul: i64,
    op: F,
) where
    F: Fn(),
{
    let mut avl = avl;
    let mut lhs = lhs;
    let mut rhs = rhs;
    let mut result = result;

    let sew_bytes = sew / 8;

    loop {
        let vl = vsetvl(avl as u64, sew, lmul);
        vle_v1(sew, lhs);
        vle_v11(sew, rhs);
        vle_v21(sew * 2, result);

        op();

        vse_v21(sew * 2, result);

        avl -= vl;
        if avl == 0 {
            break;
        }
        let offset = (vl * sew_bytes) as usize;
        result = &mut result[offset * 2..];
        lhs = &lhs[offset..];
        rhs = &rhs[offset..];
    }
}

#[inline(never)]
pub fn vredop_vs<F>(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, avl: u64, lmul: i64, op: F)
where
    F: Fn(),
{
    let mut avl = avl;
    let mut lhs = lhs;

    let sew_bytes = sew / 8;

    let mut index = 0;
    loop {
        // vd[0] =  sum(vs2[*], vs1[0])
        let vl = vsetvl(avl as u64, sew, lmul);
        vle_v1(sew, lhs); // vs2
        if index == 0 {
            vle_v11(sew, rhs); // vs1
        }

        op();

        vse_v21(sew, result);
        // copy back to vs1
        unsafe {
            rvv_asm!("vmv.v.v v11, v21");
        }

        avl -= vl;
        if avl == 0 {
            break;
        }
        let offset = (vl * sew_bytes) as usize;
        // only advance vs2
        lhs = &lhs[offset..];
        index += 1;
    }
}

#[inline(never)]
pub fn vwredop_vs<F>(
    lhs: &[u8],
    rhs: &[u8],
    result: &mut [u8],
    sew: u64,
    avl: u64,
    lmul: i64,
    op: F,
) where
    F: Fn(),
{
    let mut avl = avl;
    let mut lhs = lhs;

    let sew_bytes = sew / 8;

    let mut index = 0;
    loop {
        // vd[0] =  sum(vs2[*], vs1[0])
        let vl = vsetvl(avl as u64, sew, lmul);
        vle_v1(sew, lhs); // vs2
        if index == 0 {
            vle_v11(sew, rhs); // vs1
        }

        op();

        vse_v21(sew * 2, result);
        // copy back to vs1
        unsafe {
            rvv_asm!("vmv.v.v v11, v21");
        }

        avl -= vl;
        if avl == 0 {
            break;
        }
        let offset = (vl * sew_bytes) as usize;
        lhs = &lhs[offset..];
        index += 1;
    }
}

#[inline(never)]
pub fn vmsop_vv<F>(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, avl: u64, lmul: i64, op: F)
where
    F: Fn(),
{
    let mut avl = avl;
    let mut lhs = lhs;
    let mut rhs = rhs;

    let sew_bytes = sew / 8;

    let mut index = 0;
    loop {
        let vl = vsetvl(avl as u64, sew, lmul);
        vle_v1(sew, lhs);
        vle_v11(sew, rhs);

        op();

        let mut temp = [0u8; VLEN / 8];
        vs1r_v21(&mut temp);
        for i in 0..vl {
            let bit = get_bit_in_slice(&temp[..], i as usize);
            set_bit_in_slice(result, index, bit);
            index += 1;
        }

        avl -= vl;
        if avl == 0 {
            break;
        }
        let offset = (vl * sew_bytes) as usize;
        lhs = &lhs[offset..];
        rhs = &rhs[offset..];
    }
}

#[inline(never)]
pub fn vmsop_vx<F>(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, avl: u64, lmul: i64, op: F)
where
    F: Fn(u64),
{
    let mut avl = avl;
    let mut lhs = lhs;

    let sew_bytes = sew / 8;

    let mut index = 0;
    loop {
        let vl = vsetvl(avl as u64, sew, lmul);
        vle_v1(sew, lhs);

        op(x);

        let mut temp = [0u8; VLEN / 8];
        vs1r_v21(&mut temp);
        for _ in 0..vl {
            let bit = get_bit_in_slice(&temp[..], index);
            set_bit_in_slice(result, index, bit);
            index += 1;
        }

        avl -= vl;
        if avl == 0 {
            break;
        }
        let offset = (vl * sew_bytes) as usize;
        lhs = &lhs[offset..];
    }
}

#[inline(never)]
pub fn vop_nv<F>(
    lhs: &[u8],
    rhs: &[u8],
    result: &mut [u8],
    sew: u64,
    avl: u64,
    lmul: i64,
    op: F,
    cat: WideningCategory,
) where
    F: Fn(),
{
    let mut avl = avl;
    let mut lhs = lhs;
    let mut rhs = rhs;
    let mut result = result;

    let sew_bytes = sew / 8;

    let n = match cat {
        WideningCategory::NarrowVs2(n) => n,
        _ => 1,
    };

    loop {
        let vl = vsetvl(avl as u64, sew, lmul);
        vle_v1(sew, lhs);
        vle_v11(sew / n as u64, rhs);

        op();

        vse_v21(sew, result);

        avl -= vl;
        if avl == 0 {
            break;
        }
        let offset = (vl * sew_bytes) as usize;
        result = &mut result[offset..];
        lhs = &lhs[offset / n..];
        rhs = &rhs[offset..];
    }
}

#[inline(never)]
pub fn vop_vx<F>(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, avl: u64, lmul: i64, op: F)
where
    F: Fn(u64),
{
    let mut avl = avl;
    let mut lhs = lhs;
    let mut result = result;

    let sew_bytes = sew / 8;
    loop {
        let vl = vsetvl(avl as u64, sew, lmul);
        vle_v1(sew, lhs);

        op(x);

        vse_v21(sew, result);

        avl -= vl;
        if avl == 0 {
            break;
        }
        let offset = (vl * sew_bytes) as usize;
        result = &mut result[offset..];
        lhs = &lhs[offset..];
    }
}

#[inline(never)]
pub fn vop_wx<F>(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, avl: u64, lmul: i64, op: F)
where
    F: Fn(u64),
{
    let mut avl = avl;
    let mut lhs = lhs;
    let mut result = result;

    let sew_bytes = sew / 8;
    loop {
        let vl = vsetvl(avl as u64, sew, lmul);
        vle_v1(sew * 2, lhs);

        op(x);

        vse_v21(sew, result);

        avl -= vl;
        if avl == 0 {
            break;
        }
        let offset = (vl * sew_bytes) as usize;
        result = &mut result[offset..];
        lhs = &lhs[offset * 2..];
    }
}

#[inline(never)]
pub fn vwop_vv<F>(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, avl: u64, lmul: i64, op: F)
where
    F: Fn(),
{
    let mut avl = avl;
    let mut lhs = lhs;
    let mut rhs = rhs;
    let mut result = result;

    let sew_bytes = sew / 8;
    loop {
        let vl = vsetvl(avl as u64, sew, lmul);
        vle_v1(sew, lhs);
        vle_v11(sew, rhs);

        op();

        // different than `vop_vv`
        vse_v21(sew * 2, result);

        avl -= vl;
        if avl == 0 {
            break;
        }
        let offset = (vl * sew_bytes) as usize;
        // different than `vop_vv`
        result = &mut result[offset * 2..];
        lhs = &lhs[offset..];
        rhs = &rhs[offset..];
    }
}

#[inline(never)]
pub fn vwop_vx<F>(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, avl: u64, lmul: i64, op: F)
where
    F: Fn(u64),
{
    let mut avl = avl;
    let mut lhs = lhs;
    let mut result = result;

    let sew_bytes = sew / 8;
    loop {
        let vl = vsetvl(avl as u64, sew, lmul);
        vle_v1(sew, lhs);

        op(x);

        // different than `vop_vv`
        vse_v21(sew * 2, result);

        avl -= vl;
        if avl == 0 {
            break;
        }
        let offset = (vl * sew_bytes) as usize;
        // different than `vop_vv`
        result = &mut result[offset * 2..];
        lhs = &lhs[offset..];
    }
}

#[inline(never)]
pub fn vwop_wx<F>(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, avl: u64, lmul: i64, op: F)
where
    F: Fn(u64),
{
    let mut avl = avl;
    let mut lhs = lhs;
    let mut result = result;

    let sew_bytes = sew / 8;
    loop {
        let vl = vsetvl(avl as u64, sew, lmul);
        vle_v1(sew * 2, lhs);

        op(x);

        vse_v21(sew * 2, result);

        avl -= vl;
        if avl == 0 {
            break;
        }
        let offset = (vl * sew_bytes) as usize;
        result = &mut result[offset * 2..];
        lhs = &lhs[offset * 2..];
    }
}

#[inline(never)]
pub fn vwop_wv<F>(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, avl: u64, lmul: i64, op: F)
where
    F: Fn(),
{
    let mut avl = avl;
    let mut lhs = lhs;
    let mut rhs = rhs;
    let mut result = result;

    let sew_bytes = sew / 8;
    loop {
        let vl = vsetvl(avl as u64, sew, lmul);
        // different than `vop_vv`
        vle_v1(sew * 2, lhs);
        vle_v11(sew, rhs);

        op();

        // different than `vop_vv`
        vse_v21(sew * 2, result);

        avl -= vl;
        if avl == 0 {
            break;
        }
        let offset = (vl * sew_bytes) as usize;
        // different than `vop_vv`
        result = &mut result[offset * 2..];
        lhs = &lhs[offset * 2..];
        rhs = &rhs[offset..];
    }
}

#[inline(never)]
pub fn vop_wv<F>(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, avl: u64, lmul: i64, op: F)
where
    F: Fn(),
{
    let mut avl = avl;
    let mut lhs = lhs;
    let mut rhs = rhs;
    let mut result = result;

    let sew_bytes = sew / 8;
    loop {
        let vl = vsetvl(avl as u64, sew, lmul);
        vle_v1(sew * 2, lhs);
        vle_v11(sew, rhs);

        op();

        vse_v21(sew, result);

        avl -= vl;
        if avl == 0 {
            break;
        }
        let offset = (vl * sew_bytes) as usize;
        result = &mut result[offset..];
        lhs = &lhs[offset * 2..];
        rhs = &rhs[offset..];
    }
}
