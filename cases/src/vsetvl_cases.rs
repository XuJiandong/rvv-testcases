use ckb_std::syscalls::debug;
use rvv_testcases::intrinsic_setvl::{v_setivli, v_setvl, v_setvli};
use rvv_testcases::log;

pub fn test_vsetvl() {
    for sew in [8, 16, 32, 64, 128, 256, 512, 1024] {
        for lmul in [-8, -4, -2, 1, 2, 4, 8] {
            for avl in 0..32 {
                let vl1 = v_setvl(avl, sew, lmul);
                let vl2 = v_setvli(avl, sew, lmul);
                let vl3 = v_setivli(avl, sew, lmul);

                if vl1 != vl2 || vl1 != vl3 {
                    log!(
                        "set vl failed, sew: {}, lmul: {}, avl: {}, vl1: {}, vl2: {}, vl3: {}",
                        sew,
                        lmul,
                        avl,
                        vl1,
                        vl2,
                        vl3
                    );
                    panic!("Abort");
                }
            }
        }
    }
}
