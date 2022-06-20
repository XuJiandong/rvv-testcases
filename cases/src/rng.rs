use super::misc::set_bit_in_slice;
use alloc::vec;
use core::mem;
use rand::prelude::*;

// use super::log;
// use ckb_std::syscalls::debug;

static mut SEED: u64 = 0;
static mut CUSTOMIZE_SEED: bool = false;

#[inline(never)]
pub fn customize_seed(seed: u64) {
    unsafe {
        CUSTOMIZE_SEED = true;
        SEED = seed;
    }
}

#[inline(never)]
pub fn is_customize_seed() -> bool {
    unsafe { CUSTOMIZE_SEED }
}

#[inline(never)]
pub fn get_seed() -> u64 {
    unsafe { SEED }
}

pub struct BestNumberRngSeed(pub [u8; 1024]);

impl Default for BestNumberRngSeed {
    fn default() -> BestNumberRngSeed {
        BestNumberRngSeed([0; 1024])
    }
}

impl AsMut<[u8]> for BestNumberRngSeed {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

pub struct BestNumberRng {
    data: [u64; 128],
    index: usize,
}

impl SeedableRng for BestNumberRng {
    type Seed = BestNumberRngSeed;

    fn from_seed(seed: BestNumberRngSeed) -> BestNumberRng {
        let data = unsafe { mem::transmute::<[u8; 1024], [u64; 128]>(seed.0) };
        BestNumberRng { index: 0, data }
    }
}

impl Default for BestNumberRng {
    fn default() -> Self {
        if is_customize_seed() {
            Self::seed_from_u64(get_seed())
        } else {
            let seed = unsafe {
                SEED += 1;
                if SEED >= 128 {
                    SEED = 0;
                }
                SEED
            } as usize;
            BestNumberRng {
                index: seed,
                // from https://github.com/mohanson/rvv-playground/blob/094ae03266ea73a891bc105f238dacfc22cb56e6/src/rvv_test_case.py#L4
                data: [
                    0x0000000000000000,
                    0x0000000000000001,
                    0xffffffffffffffff,
                    0x8000000000000000,
                    0x0000000000000004,
                    0x0000000000000040,
                    0x0000000000000080,
                    0x0000000000002000,
                    0x0000000000010000,
                    0x0000000000400000,
                    0x0000000001000000,
                    0x0000000100000000,
                    0x0000008000000000,
                    0x0000020000000000,
                    0x0000800000000000,
                    0x0010000000000000,
                    0x0100000000000000,
                    0x1000000000000000,
                    0x000000000000000d,
                    0x0000000000000000,
                    0x0000000000000067,
                    0x2e00000000000000,
                    0x000000000000015b,
                    0x9420000000000000,
                    0x00000000000075da,
                    0xe35a000000000000,
                    0x000000000003ed82,
                    0x8b4eb00000000000,
                    0x000000000000714c,
                    0xfad4dc0000000000,
                    0x000000000dd2966b,
                    0x686f332000000000,
                    0x00000000a865d7d4,
                    0x6edd225600000000,
                    0x0000000380f3cf69,
                    0xaf29109cc0000000,
                    0x000000a3714b9ad2,
                    0x7dc2ae94e4000000,
                    0x00000bea6a6af755,
                    0xea2177d8d5100000,
                    0x00004a9e26b7f794,
                    0x6d159abfb3030000,
                    0x00020e6dfbb7c441,
                    0xd251a40a022b9000,
                    0x00129af7f2440efe,
                    0xc7dee68fffbaf900,
                    0x05ada4e53975b451,
                    0x63eb500cce126b70,
                    0x314320aa7da5b1ef,
                    0xd27d2fde3497614c,
                    0xbe55668178139c8e,
                    0x9480583abdfb5837,
                    0x9d8dbb3a5bde4347,
                    0x61fd04828c93ce01,
                    0xdf9a26c8470349dd,
                    0xca9d54bd4e78980e,
                    0xb1db9b0fecbfaabe,
                    0xe79541e25d0dba6b,
                    0xff98837fda2a5bdf,
                    0xc3bd5e2cd52318a8,
                    0x02ab7bb54e687499,
                    0xbebf0929f41aa230,
                    0x58aee9fdc3f41b74,
                    0x62daff171a9fae42,
                    0xe5baa16ee5b5419e,
                    0x16b3a918e4278c9d,
                    0x4ab9cfc9a41744c4,
                    0x86ddce906c8cdb4d,
                    0x867e3492977cb1bb,
                    0x3d0e482377794618,
                    0x90e1bc8ba22d3294,
                    0xf48119b103954df1,
                    0x79780d4e5b2b3b2a,
                    0xb36eb1caa58ee7dc,
                    0xf0fe55be95a18d13,
                    0x1234769364d9eac9,
                    0x31a7445bdf8bcb5c,
                    0x1735808ee4398bca,
                    0x8f09996552504a5d,
                    0x4fcf7212bebfdd89,
                    0xdfd3a0870f60e072,
                    0x25474d793f2c7d32,
                    0xb9e2a99fdb7b2948,
                    0x0da24e08451a8d1a,
                    0x44a705073f90be80,
                    0x7f2e6910bdea3ffd,
                    0x7fc92593c865b4c2,
                    0x0f812a265e560f2b,
                    0xfecee737556609f5,
                    0x996d1b60923c18a6,
                    0x2c1fb5204d248917,
                    0x4cf560811e3465c5,
                    0xf2a6b292a535dc4e,
                    0x3b4de2fabe6d6476,
                    0xa6a669d1baba633e,
                    0xa73c905bcbc01878,
                    0x38be984c83ce8648,
                    0x262a15662b298944,
                    0xdf09e5c90a990b56,
                    0xa8519a5b46242cc0,
                    0x14d93f0c55095499,
                    0xbad28e0ca5854070,
                    0x93d7d7a9d87056f0,
                    0x3b0d936889b10a5d,
                    0x0ec6680cabb95f09,
                    0x27429c30e8b6cff7,
                    0x6465f271027abfa8,
                    0xd0abd7d3688aa0d7,
                    0x986a686578456056,
                    0xc10a152d71cb3f16,
                    0x4a6c986967d5ace8,
                    0x37269c228e8e3db1,
                    0xf5bad73c74be6d8a,
                    0x68323fe289df33d1,
                    0xcb9848f06e9659f6,
                    0x5052886f7169c8c5,
                    0xb040414dd8c98a14,
                    0xea59a91078581c00,
                    0x7c6bcb08155fac38,
                    0xbd6192029dd91d60,
                    0x8a4a182923bdf75a,
                    0x8c91e2fe14041a34,
                    0xc9d368e6546c1f00,
                    0xdfd83d690e5f073e,
                    0x34f2a050c605b6b0,
                    0xf3fbe985738811dd,
                    0x2d21e3da342cd6be,
                    0x31523358d080e093,
                ],
            }
        }
    }
}

impl RngCore for BestNumberRng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let result = self.data[self.index];
        self.index += 1;
        if self.index >= self.data.len() {
            self.index = self.index % self.data.len();
        }
        result
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        if dest.len() < 4 {
            return;
        }
        for i in 0..dest.len() / 8 {
            let next = self.next_u64();
            dest[i * 8..(i + 1) * 8].copy_from_slice(&next.to_le_bytes());
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        Ok(self.fill_bytes(dest))
    }
}

impl BestNumberRng {
    pub fn fill_bytes_with_sew(&mut self, dest: &mut [u8], sew: u64) {
        if is_customize_seed() {
            self.fill_bytes(dest);
        } else {
            let sew = sew as usize;
            let sew_byte = sew / 8;
            let mut zero_data = vec::Vec::<u8>::new();
            zero_data.resize(sew_byte, 0);

            let mut true_data = vec::Vec::<u8>::new();
            true_data.resize(sew_byte, 0xFF);

            for i in 0..dest.len() / sew_byte {
                let v = self.next_u64();
                let t = v % 10;
                if t == 0 {
                    // zero
                    dest[i * sew_byte..(i + 1) * sew_byte].copy_from_slice(&zero_data);
                } else if t == 1 {
                    // fill 0xFF
                    dest[i * sew_byte..(i + 1) * sew_byte].copy_from_slice(&true_data);
                } else {
                    let mut data = vec::Vec::<u8>::new();
                    data.resize(sew_byte, 0);
                    self.fill_bytes(&mut data);
                    if t == 4 {
                        let z = (v as usize >> 2) % (sew / 2) + 1;
                        for i in 0..z {
                            set_bit_in_slice(&mut data, i, 0);
                        }
                    } else if t == 5 {
                        let z = (v as usize >> 2) % (sew / 2) + 1;
                        for i in z..(data.len() * 8) {
                            set_bit_in_slice(&mut data, i, 0);
                        }
                    } else if t == 6 {
                        let z = (v as usize >> 2) % (sew / 2) + 1;
                        for i in z..(data.len() * 8) {
                            set_bit_in_slice(&mut data, i, 1);
                        }
                    }
                    dest[i * sew_byte..(i + 1) * sew_byte].copy_from_slice(&data);
                }
            }
        }
    }

    pub fn fill_mask(&mut self, dest: &mut [u8]) {
        if is_customize_seed() {
            self.fill_bytes(dest);
        } else {
            let mut count = 0;
            let mut val = 0;
            for i in 0..dest.len() * 8 {
                if count == 0 {
                    count = self.next_u64() % 32 + 1;
                    val = if val == 1 { 0 } else { 1 }
                }
                set_bit_in_slice(dest, i, val);
                count -= 1
            }
        }
    }
}
