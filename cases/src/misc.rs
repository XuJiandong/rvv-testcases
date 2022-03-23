use core::convert::TryInto;

use alloc::vec;
use alloc::vec::Vec;
use rvv_simulator_runtime::Uint;

static mut VERBOSE: bool = false;

pub type U256 = Uint<4>;
pub type U512 = Uint<8>;
pub type U1024 = Uint<16>;

pub trait Widening {
    type WideningType;
    fn sign_extend(&self) -> Self::WideningType;
}

impl Widening for u64 {
    type WideningType = U256;
    fn sign_extend(&self) -> Self::WideningType {
        if (self & 0x8000000000000000) != 0 {
            let r = u128::MAX;
            let r2 = (*self as i64) as u128;
            (U256::from(r) << 128) + U256::from(r2)
        } else {
            self.clone().into()
        }
    }
}

impl Widening for U256 {
    type WideningType = U512;
    fn sign_extend(&self) -> Self::WideningType {
        if self.bit(255) {
            let r: U512 = U256::MAX.into();
            let r2: U512 = self.clone().into();
            (r << 256) + r2
        } else {
            self.clone().into()
        }
    }
}

impl Widening for U512 {
    type WideningType = U1024;
    fn sign_extend(&self) -> Self::WideningType {
        if self.bit(511) {
            let r: U1024 = U512::MAX.into();
            let r2: U1024 = self.clone().into();
            (r << 512) + r2
        } else {
            self.clone().into()
        }
    }
}

pub fn create_vtype(sew: u64, lmul: i64) -> u64 {
    let lmul_bits = match lmul {
        1 => 0b000,
        2 => 0b001,
        4 => 0b010,
        8 => 0b011,
        -2 => 0b111,
        -4 => 0b110,
        -8 => 0b101,
        _ => 8,
    };
    let sew_bits = match sew {
        0b1000 => 0,
        0b10000 => 1,
        0b100000 => 2,
        0b1000000 => 3,
        0b10000000 => 4,
        0b100000000 => 5,
        0b1000000000 => 6,
        0b10000000000 => 7,
        _ => 8, // out of bound
    };
    assert!(sew_bits < 8);
    assert!(lmul_bits < 8);
    sew_bits << 3 | lmul_bits
}

pub fn log2(n: usize) -> usize {
    match n {
        1 => 0,
        2 => 1,
        4 => 2,
        8 => 3,
        16 => 4,
        32 => 5,
        64 => 6,
        128 => 7,
        256 => 8,
        512 => 9,
        1024 => 10,
        2048 => 11,
        4096 => 12,
        _ => panic!("not supported"),
    }
}

pub const VLEN: usize = 2048;

#[macro_export]
macro_rules! log {
    ($fmt:literal) => {
        debug(alloc::format!($fmt));
    };
    ($fmt:literal, $($args:expr),+) => {
        debug(alloc::format!($fmt, $($args), +));
    };
}

#[macro_export]
macro_rules! test_case {
    ($fun:path, $test_pattern:ident) => {{
        let fun_name = stringify!($fun);
        if $test_pattern.is_none() || fun_name.contains($test_pattern.unwrap()) {
            log!("test {} ...", fun_name);
            $fun();
            log!("test {}, OK", fun_name);
        }
    }};
}

// -16 to 15
pub fn shrink_to_imm(x: u64) -> i8 {
    let x2 = (x & 0b11111) as u64;
    if x2 < 16 {
        x2 as i8
    } else {
        x2 as i8 - 32
    }
}

pub fn avl_iterator(sew: u64, target_lmul: i64) -> Vec<u64> {
    if target_lmul > 0 {
        let avl = target_lmul as u64 * VLEN as u64 / sew as u64;
        vec![avl - 1, avl, avl + 1]
    } else {
        panic!("TODO")
    }
}

pub fn ceiling(a: usize, b: usize) -> usize {
    if a % b == 0 {
        a / b
    } else {
        a / b + 1
    }
}

pub fn get_bit(x: u8, index: usize) -> u8 {
    (x >> index) & 1
}

pub fn get_bit_in_slice(x: &[u8], index: usize) -> u8 {
    let byte_index = index / 8;
    let bit_index = index - 8 * byte_index;
    get_bit(x[byte_index], bit_index)
}

pub fn set_bit(x: &mut u8, index: usize, v: u8) {
    assert!(v == 0 || v == 1);
    let mask = !(1 << index);
    let res: u8 = (*x & mask) | (v << index);
    *x = res;
}

pub fn set_bit_in_slice(x: &mut [u8], index: usize, v: u8) {
    let byte_index = index / 8;
    let bit_index = index - 8 * byte_index;
    set_bit(&mut x[byte_index], bit_index, v);
}

pub fn is_verbose() -> bool {
    unsafe { VERBOSE }
}

pub fn set_verbose(b: bool) {
    unsafe {
        VERBOSE = b;
    }
}

pub trait SliceUtils<'a> {
    fn get_element(&'a self, sew: usize, index: usize) -> &'a [u8];
    fn read_u8(&self, eew: usize, index: usize) -> u8;
    fn read_u16(&self, eew: usize, index: usize) -> u16;
    fn read_u32(&self, eew: usize, index: usize) -> u32;
    fn read_u64(&self, eew: usize, index: usize) -> u64;
}

pub trait MutSliceUtils<'a> {
    fn get_mut_element(&'a mut self, sew: usize, index: usize) -> &'a mut [u8];
    fn write_u8(&'a mut self, eew: usize, index: usize, v: u8);
    fn write_u16(&'a mut self, eew: usize, index: usize, v: u16);
    fn write_u32(&'a mut self, eew: usize, index: usize, v: u32);
    fn write_u64(&'a mut self, eew: usize, index: usize, v: u64);
}

impl<'a> SliceUtils<'a> for &'a [u8] {
    fn get_element(&'a self, eew: usize, index: usize) -> &'a [u8] {
        let start = eew / 8 * index;
        &self[start..start + eew / 8]
    }

    fn read_u8(&self, eew: usize, index: usize) -> u8 {
        assert!(eew == 8);
        let slice = self.get_element(eew, index);
        u8::from_le_bytes(slice.try_into().unwrap())
    }

    fn read_u16(&self, eew: usize, index: usize) -> u16 {
        assert!(eew == 16);
        let slice = self.get_element(eew, index);
        u16::from_le_bytes(slice.try_into().unwrap())
    }

    fn read_u32(&self, eew: usize, index: usize) -> u32 {
        assert!(eew == 32);
        let slice = self.get_element(eew, index);
        u32::from_le_bytes(slice.try_into().unwrap())
    }

    fn read_u64(&self, eew: usize, index: usize) -> u64 {
        assert!(eew == 64);
        let slice = self.get_element(eew, index);
        u64::from_le_bytes(slice.try_into().unwrap())
    }
}

impl<'a> MutSliceUtils<'a> for &'a mut [u8] {
    fn get_mut_element(&'a mut self, eew: usize, index: usize) -> &'a mut [u8] {
        let start = eew / 8 * index;
        &mut self[start..start + eew / 8]
    }

    fn write_u8(&'a mut self, eew: usize, index: usize, v: u8) {
        assert!(eew == 8);
        let slice = self.get_mut_element(eew, index);
        slice.copy_from_slice(v.to_le_bytes().as_slice());
    }

    fn write_u16(&'a mut self, eew: usize, index: usize, v: u16) {
        assert!(eew == 16);
        let slice = self.get_mut_element(eew, index);
        slice.copy_from_slice(v.to_le_bytes().as_slice());
    }

    fn write_u32(&'a mut self, eew: usize, index: usize, v: u32) {
        assert!(eew == 32);
        let slice = self.get_mut_element(eew, index);
        slice.copy_from_slice(v.to_le_bytes().as_slice());
    }

    fn write_u64(&'a mut self, eew: usize, index: usize, v: u64) {
        assert!(eew == 64);
        let slice = self.get_mut_element(eew, index);
        slice.copy_from_slice(v.to_le_bytes().as_slice());
    }
}

pub fn compress_into_bits(data: &[u8]) -> [u8; 256] {
    let mut res = [0u8; 256];
    for i in 0..data.len() {
        set_bit_in_slice(&mut res, i, data[i]);
    }
    res
}

pub fn greater_i256(l: &[u8], r: &[u8]) -> bool {
    if l == r {
        return false;
    }

    let l_s = get_bit_in_slice(l, 255);
    let r_s = get_bit_in_slice(r, 255);

    if l_s != r_s {
        if l_s == 0 {
            true
        } else {
            false
        }
    } else {
        let l = U256::from_little_endian(l);
        let r = U256::from_little_endian(r);
        l > r
    }
}


pub fn less_i256(l: &[u8], r: &[u8]) -> bool {
    if l == r {
        return false;
    }

    let l_s = get_bit_in_slice(l, 255);
    let r_s = get_bit_in_slice(r, 255);

    if l_s != r_s {
        if l_s == 0 {
            false
        } else {
            true
        }
    } else {
        let l = U256::from_little_endian(l);
        let r = U256::from_little_endian(r);
        l < r
    }
}
