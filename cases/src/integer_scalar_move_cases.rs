use core::arch::asm;
use rand::Rng;
use rvv_asm::rvv_asm;

use ckb_std::syscalls::debug;
use rvv_testcases::intrinsic::{vl1r_v1, vs1r_v1, vsetvl};
use rvv_testcases::log;
use rvv_testcases::misc::{is_verbose, VLEN};
use rvv_testcases::rng::BestNumberRng;

fn vmv_x_s() {
    if is_verbose() {
        log!("test vmv.x.s");
    }
    let mut vs2 = [1u8; VLEN / 8];
    let mut rng = BestNumberRng::default();
    rng.fill(&mut vs2[..]);
    rng.fill(&mut vs2[..]);

    let vl = vsetvl(32, 64, 1) as usize;
    assert_eq!(vl, 32);

    vl1r_v1(&vs2[..]);

    let index: u64 = 5;
    let mut result: u64;

    let result2 = {
        let mut d: u64 = 0;
        for i in 0..8 {
            d += (vs2[i] as u64) << i * 8;
        }
        d
    };

    unsafe {
        rvv_asm!(
            "vmv.x.s t0, v1",
            "mv {}, t0",
            out (reg) result
        );
    }

    log!("more information, vs2 = {:?}", vs2);
    log!(
        "more information, index = {}, result = {}, result2 = {}",
        index,
        result,
        result2
    );
    if result != result2 {
        panic!("Abort");
    }
    if is_verbose() {
        log!("finished");
    }
}

fn vmv_s_x() {
    if is_verbose() {
        log!("test vmv.s.x");
    }

    let mut data = [1u8; VLEN / 8];
    let mut rng = BestNumberRng::default();
    rng.fill(&mut data[..]);
    rng.fill(&mut data[..]);

    let vl = vsetvl(32, 64, 1) as usize;
    assert_eq!(vl, 32);

    let mut vs2 = [0u8; VLEN / 8];

    let result2 = {
        let mut d: u64 = 0;
        for i in 0..8 {
            d += (data[i] as u64) << i * 8;
        }
        d
    };

    unsafe {
        rvv_asm!(
            "mv t1, {}",
            "vmv.s.x v1, t1",
            in (reg) result2,
        );
        vs1r_v1(&mut vs2);
    }

    if data[0..8] != vs2[0..8] {
        log!("more information, vs2 = {:?}, \n x = {}", vs2, result2);
        panic!("Abort");
    }
    if is_verbose() {
        log!("finished");
    }
}

pub fn test_integer_scalar_move() {
    vmv_x_s();
    vmv_s_x();
}
