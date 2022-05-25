use core::convert::TryInto;

use alloc::vec;
use alloc::vec::Vec;
use eint::{Eint, E1024, E128, E16, E256, E32, E512, E64, E8};
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
macro_rules! dbg_log {
    ($fmt:literal) => {
        //debug(alloc::format!($fmt));
    };
    ($fmt:literal, $($args:expr),+) => {
        //debug(alloc::format!($fmt, $($args), +));
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
    fn read_u8(&self, eew: usize, index: usize) -> E8;
    fn read_u16(&self, eew: usize, index: usize) -> E16;
    fn read_u32(&self, eew: usize, index: usize) -> E32;
    fn read_u64(&self, eew: usize, index: usize) -> E64;
    fn read_u128(&self, eew: usize, index: usize) -> E128;
    fn read_u256(&self, eew: usize, index: usize) -> E256;
    fn read_u512(&self, eew: usize, index: usize) -> E512;
    fn read_u1024(&self, eew: usize, index: usize) -> E1024;
}

pub trait MutSliceUtils<'a> {
    fn get_mut_element(&'a mut self, sew: usize, index: usize) -> &'a mut [u8];
    fn write_u8(&'a mut self, eew: usize, index: usize, v: E8);
    fn write_u16(&'a mut self, eew: usize, index: usize, v: E16);
    fn write_u32(&'a mut self, eew: usize, index: usize, v: E32);
    fn write_u64(&'a mut self, eew: usize, index: usize, v: E64);
    fn write_u128(&'a mut self, eew: usize, index: usize, v: E128);
    fn write_u256(&'a mut self, eew: usize, index: usize, v: E256);
    fn write_u512(&'a mut self, eew: usize, index: usize, v: E512);
    fn write_u1024(&'a mut self, eew: usize, index: usize, v: E1024);
}

impl<'a> SliceUtils<'a> for &'a [u8] {
    fn get_element(&'a self, eew: usize, index: usize) -> &'a [u8] {
        let start = eew / 8 * index;
        &self[start..start + eew / 8]
    }

    fn read_u8(&self, eew: usize, index: usize) -> E8 {
        assert!(eew == 8);
        let slice = self.get_element(eew, index);
        E8::get(slice.try_into().unwrap())
    }

    fn read_u16(&self, eew: usize, index: usize) -> E16 {
        assert!(eew == 16);
        let slice = self.get_element(eew, index);
        E16::get(slice.try_into().unwrap())
    }

    fn read_u32(&self, eew: usize, index: usize) -> E32 {
        assert!(eew == 32);
        let slice = self.get_element(eew, index);
        E32::get(slice.try_into().unwrap())
    }

    fn read_u64(&self, eew: usize, index: usize) -> E64 {
        assert!(eew == 64);
        let slice = self.get_element(eew, index);
        E64::get(slice.try_into().unwrap())
    }

    fn read_u128(&self, eew: usize, index: usize) -> E128 {
        assert!(eew == 128);
        let slice = self.get_element(eew, index);
        E128::get(slice.try_into().unwrap())
    }

    fn read_u256(&self, eew: usize, index: usize) -> E256 {
        assert!(eew == 256);
        let slice = self.get_element(eew, index);
        E256::get(slice.try_into().unwrap())
    }

    fn read_u512(&self, eew: usize, index: usize) -> E512 {
        assert!(eew == 512);
        let slice = self.get_element(eew, index);
        E512::get(slice.try_into().unwrap())
    }

    fn read_u1024(&self, eew: usize, index: usize) -> E1024 {
        assert!(eew == 1024);
        let slice = self.get_element(eew, index);
        E1024::get(slice.try_into().unwrap())
    }
}

impl<'a> MutSliceUtils<'a> for &'a mut [u8] {
    fn get_mut_element(&'a mut self, eew: usize, index: usize) -> &'a mut [u8] {
        let start = eew / 8 * index;
        &mut self[start..start + eew / 8]
    }

    fn write_u8(&'a mut self, eew: usize, index: usize, v: E8) {
        assert!(eew == 8);
        v.put(self.get_mut_element(eew, index));
    }

    fn write_u16(&'a mut self, eew: usize, index: usize, v: E16) {
        assert!(eew == 16);
        v.put(self.get_mut_element(eew, index));
    }

    fn write_u32(&'a mut self, eew: usize, index: usize, v: E32) {
        assert!(eew == 32);
        v.put(self.get_mut_element(eew, index));
    }

    fn write_u64(&'a mut self, eew: usize, index: usize, v: E64) {
        assert!(eew == 64);
        v.put(self.get_mut_element(eew, index));
    }

    fn write_u128(&'a mut self, eew: usize, index: usize, v: E128) {
        assert!(eew == 128);
        v.put(self.get_mut_element(eew, index));
    }

    fn write_u256(&'a mut self, eew: usize, index: usize, v: E256) {
        assert!(eew == 256);
        v.put(self.get_mut_element(eew, index));
    }

    fn write_u512(&'a mut self, eew: usize, index: usize, v: E512) {
        assert!(eew == 512);
        v.put(self.get_mut_element(eew, index));
    }

    fn write_u1024(&'a mut self, eew: usize, index: usize, v: E1024) {
        assert!(eew == 1024);
        v.put(self.get_mut_element(eew, index));
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

pub fn add_i512(a: &[u8; 64], b: &[u8; 64]) -> [u8; 64] {
    let mut r = [0u8; 64];

    let rr = U512::from_little_endian(a).wrapping_add(U512::from_little_endian(b));
    rr.to_little_endian(&mut r);

    r
}

#[macro_export]
macro_rules! conver_with_single {
    ($func_name: ident, $name: ident, $half:ident) => {
        pub fn $func_name(d: $half) -> $name {
            if d.is_negative() {
                $name(d, $half::MAX_U)
            } else {
                $name::from(d)
            }
        }
    };
}

conver_with_single!(conver_to_i512, E512, E256);
conver_with_single!(conver_to_i256, E256, E128);

#[inline]
pub fn to_u64(d: &[u8]) -> u64 {
    u64::from_le_bytes(d.try_into().unwrap())
}

#[inline]
pub fn to_i64(d: &[u8]) -> i64 {
    i64::from_le_bytes(d.try_into().unwrap())
}
