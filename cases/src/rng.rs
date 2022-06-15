use super::rng_data::G_DATA;

// use super::log;
// use ckb_std::syscalls::debug;

static mut SEED: usize = 0;

pub fn set_seed(seed: usize) {
    unsafe {
        SEED = seed;
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
