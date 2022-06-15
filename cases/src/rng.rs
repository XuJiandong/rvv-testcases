use super::{misc::set_bit_in_slice, rng_data::G_DATA};
use alloc::vec::Vec;

use super::log;
use ckb_std::syscalls::debug;

static mut SEED: usize = 0;

pub fn set_seed(seed: usize) {
    unsafe {
        SEED = seed % G_DATA.len();
    }
}

pub fn get_seed() -> usize {
    unsafe { SEED }
}

pub fn fill_rand_bytes(dest: &mut [u8]) {
    let mut begin_pos = 0;
    let mut seed = get_seed();
    while begin_pos < dest.len() {
        let mut end_pos = dest.len();
        if seed + (end_pos - begin_pos) > G_DATA.len() {
            end_pos = G_DATA.len() - seed + begin_pos;
            dest[begin_pos..end_pos].copy_from_slice(&G_DATA[seed..]);
            seed = 0;
        } else {
            dest[begin_pos..end_pos].copy_from_slice(&G_DATA[seed..seed + end_pos - begin_pos]);
            seed += end_pos - begin_pos;
        }
        begin_pos = end_pos;
    }

    set_seed(seed);
}

fn gen_rand_u8() -> u8 {
    let seed = get_seed();
    let ret = G_DATA[seed];
    set_seed(seed + 1);
    ret
}

pub fn fill_rand_mask(dest: &mut [u8]) {
    let mut count = 0;
    let mut val = 0;
    for i in 0..dest.len() * 8 {
        if count == 0 {
            count = gen_rand_u8() % 32 + 1;
            val = if val == 1 { 0 } else { 1 }
        }
        set_bit_in_slice(dest, i, val);
        count -= 1
    }
}

pub fn fill_rand_sew(dest: &mut [u8], sew: u64) {
    let sew = sew as usize;
    let sew_byte = sew / 8;
    let mut zero_data = Vec::<u8>::new();
    zero_data.resize(sew_byte, 8);
    for i in 0..dest.len() / sew_byte {
        let v = gen_rand_u8();
        let t = v % 10;
        if t == 0 {
            // zero
            dest[i * sew_byte..(i + 1) * sew_byte].copy_from_slice(&zero_data);
        } else {
            let mut data = Vec::<u8>::new();
            data.resize(sew_byte, 0);
            fill_rand_bytes(&mut data);
            if t == 1 {
                let z = (v as usize >> 2) % (sew / 2) + 1;
                for i in 0..z {
                    set_bit_in_slice(&mut data, i, 0);
                }
            } else if t == 3 {
                let z = (v as usize >> 2) % (sew / 2) + 1;
                for i in z..(data.len() * 8) {
                    set_bit_in_slice(&mut data, i, 0);
                }
            } else if t  == 5 {
                let z = (v as usize >> 2) % (sew / 2) + 1;
                for i in z..(data.len() * 8) {
                    set_bit_in_slice(&mut data, i, 1);
                }
            }
            dest[i * sew_byte..(i + 1) * sew_byte].copy_from_slice(&data);
        }
    }
    //log!("--rand data: \n{:0>2X?}", dest);
}
