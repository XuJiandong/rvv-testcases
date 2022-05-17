use super::misc::create_vtype;
use core::arch::asm;
use rvv_asm::rvv_asm;

#[inline(never)]
pub fn v_setvl(avl: u64, sew: u64, lmul: i64) -> u64 {
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

#[inline(never)]
pub fn v_setvli(avl: u64, sew: u64, lmul: i64) -> u64 {
    unsafe {
        let mut vl: u64;
        match lmul {
            -8 => match sew {
                8 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e8, mf8", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                16 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e16, mf8", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                32 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e32, mf8", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                64 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e64, mf8", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                128 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e128, mf8", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                256 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e256, mf8", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                512 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e512, mf8", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                1024 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e1024, mf8", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                _ => panic!("Abort"),
            },
            -4 => match sew {
                8 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e8, mf4", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                16 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e16, mf4", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                32 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e32, mf4", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                64 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e64, mf4", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                128 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e128, mf4", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                256 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e256, mf4", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                512 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e512, mf4", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                1024 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e1024, mf4", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                _ => panic!("Abort"),
            },
            -2 => match sew {
                8 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e8, mf2", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                16 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e16, mf2", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                32 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e32, mf2", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                64 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e64, mf2", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                128 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e128, mf2", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                256 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e256, mf2", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                512 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e512, mf2", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                1024 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e1024, mf2", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                _ => panic!("Abort"),
            },
            1 => match sew {
                8 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e8, m1", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                16 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e16, m1", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                32 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e32, m1", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                64 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e64, m1", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                128 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e128, m1", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                256 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e256, m1", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                512 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e512, m1", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                1024 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e1024, m1", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                _ => panic!("Abort"),
            },
            2 => match sew {
                8 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e8, m2", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                16 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e16, m2", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                32 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e32, m2", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                64 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e64, m2", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                128 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e128, m2", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                256 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e256, m2", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                512 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e512, m2", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                1024 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e1024, m2", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                _ => panic!("Abort"),
            },
            4 => match sew {
                8 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e8, m4", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                16 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e16, m4", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                32 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e32, m4", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                64 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e64, m4", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                128 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e128, m4", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                256 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e256, m4", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                512 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e512, m4", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                1024 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e1024, m4", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                _ => panic!("Abort"),
            },
            8 => match sew {
                8 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e8, m8", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                16 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e16, m8", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                32 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e32, m8", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                64 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e64, m8", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                128 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e128, m8", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                256 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e256, m8", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                512 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e512, m8", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                1024 => {
                    rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e1024, m8", "mv {1}, t0", in (reg) avl, out (reg) vl);
                    vl
                }
                _ => panic!("Abort"),
            },
            _ => panic!("Abort"),
        }
    }
}

#[inline(never)]
pub fn v_setivli(avl: u64, sew: u64, lmul: i64) -> u64 {
    unsafe {
        let mut vl: u64;
        match avl {
            0 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 0, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 0, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 0, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 0, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 0, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 0, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 0, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 0, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 0, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 0, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 0, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 0, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 0, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 0, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 0, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 0, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 0, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 0, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 0, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 0, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 0, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 0, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 0, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 0, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 0, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 0, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 0, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 0, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 0, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 0, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 0, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 0, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 0, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 0, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 0, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 0, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 0, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 0, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 0, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 0, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 0, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 0, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 0, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 0, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 0, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 0, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 0, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 0, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 0, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 0, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 0, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 0, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 0, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 0, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 0, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 0, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            1 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 1, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 1, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 1, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 1, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 1, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 1, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 1, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 1, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 1, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 1, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 1, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 1, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 1, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 1, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 1, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 1, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 1, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 1, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 1, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 1, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 1, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 1, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 1, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 1, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 1, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 1, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 1, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 1, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 1, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 1, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 1, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 1, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 1, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 1, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 1, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 1, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 1, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 1, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 1, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 1, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 1, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 1, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 1, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 1, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 1, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 1, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 1, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 1, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 1, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 1, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 1, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 1, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 1, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 1, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 1, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 1, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            2 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 2, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 2, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 2, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 2, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 2, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 2, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 2, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 2, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 2, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 2, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 2, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 2, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 2, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 2, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 2, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 2, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 2, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 2, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 2, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 2, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 2, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 2, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 2, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 2, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 2, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 2, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 2, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 2, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 2, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 2, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 2, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 2, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 2, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 2, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 2, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 2, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 2, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 2, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 2, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 2, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 2, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 2, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 2, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 2, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 2, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 2, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 2, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 2, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 2, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 2, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 2, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 2, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 2, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 2, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 2, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 2, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            3 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 3, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 3, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 3, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 3, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 3, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 3, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 3, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 3, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 3, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 3, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 3, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 3, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 3, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 3, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 3, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 3, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 3, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 3, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 3, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 3, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 3, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 3, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 3, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 3, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 3, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 3, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 3, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 3, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 3, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 3, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 3, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 3, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 3, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 3, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 3, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 3, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 3, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 3, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 3, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 3, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 3, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 3, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 3, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 3, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 3, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 3, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 3, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 3, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 3, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 3, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 3, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 3, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 3, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 3, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 3, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 3, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            4 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 4, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 4, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 4, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 4, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 4, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 4, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 4, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 4, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 4, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 4, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 4, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 4, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 4, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 4, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 4, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 4, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 4, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 4, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 4, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 4, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 4, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 4, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 4, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 4, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 4, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 4, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 4, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 4, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 4, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 4, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 4, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 4, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 4, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 4, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 4, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 4, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 4, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 4, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 4, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 4, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 4, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 4, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 4, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 4, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 4, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 4, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 4, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 4, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 4, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 4, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 4, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 4, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 4, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 4, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 4, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 4, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            5 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 5, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 5, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 5, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 5, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 5, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 5, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 5, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 5, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 5, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 5, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 5, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 5, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 5, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 5, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 5, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 5, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 5, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 5, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 5, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 5, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 5, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 5, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 5, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 5, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 5, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 5, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 5, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 5, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 5, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 5, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 5, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 5, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 5, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 5, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 5, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 5, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 5, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 5, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 5, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 5, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 5, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 5, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 5, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 5, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 5, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 5, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 5, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 5, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 5, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 5, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 5, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 5, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 5, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 5, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 5, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 5, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            6 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 6, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 6, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 6, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 6, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 6, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 6, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 6, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 6, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 6, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 6, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 6, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 6, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 6, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 6, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 6, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 6, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 6, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 6, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 6, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 6, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 6, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 6, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 6, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 6, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 6, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 6, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 6, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 6, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 6, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 6, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 6, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 6, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 6, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 6, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 6, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 6, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 6, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 6, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 6, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 6, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 6, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 6, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 6, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 6, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 6, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 6, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 6, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 6, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 6, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 6, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 6, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 6, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 6, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 6, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 6, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 6, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            7 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 7, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 7, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 7, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 7, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 7, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 7, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 7, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 7, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 7, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 7, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 7, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 7, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 7, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 7, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 7, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 7, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 7, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 7, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 7, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 7, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 7, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 7, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 7, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 7, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 7, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 7, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 7, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 7, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 7, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 7, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 7, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 7, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 7, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 7, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 7, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 7, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 7, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 7, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 7, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 7, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 7, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 7, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 7, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 7, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 7, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 7, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 7, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 7, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 7, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 7, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 7, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 7, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 7, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 7, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 7, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 7, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            8 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 8, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 8, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 8, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 8, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 8, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 8, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 8, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 8, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 8, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 8, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 8, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 8, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 8, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 8, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 8, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 8, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 8, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 8, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 8, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 8, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 8, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 8, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 8, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 8, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 8, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 8, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 8, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 8, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 8, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 8, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 8, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 8, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 8, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 8, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 8, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 8, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 8, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 8, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 8, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 8, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 8, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 8, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 8, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 8, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 8, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 8, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 8, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 8, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 8, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 8, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 8, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 8, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 8, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 8, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 8, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 8, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            9 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 9, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 9, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 9, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 9, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 9, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 9, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 9, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 9, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 9, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 9, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 9, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 9, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 9, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 9, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 9, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 9, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 9, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 9, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 9, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 9, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 9, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 9, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 9, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 9, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 9, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 9, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 9, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 9, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 9, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 9, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 9, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 9, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 9, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 9, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 9, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 9, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 9, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 9, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 9, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 9, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 9, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 9, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 9, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 9, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 9, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 9, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 9, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 9, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 9, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 9, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 9, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 9, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 9, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 9, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 9, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 9, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            10 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 10, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 10, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 10, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 10, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 10, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 10, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 10, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 10, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 10, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 10, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 10, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 10, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 10, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 10, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 10, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 10, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 10, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 10, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 10, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 10, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 10, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 10, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 10, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 10, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 10, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 10, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 10, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 10, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 10, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 10, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 10, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 10, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 10, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 10, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 10, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 10, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 10, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 10, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 10, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 10, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 10, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 10, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 10, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 10, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 10, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 10, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 10, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 10, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 10, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 10, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 10, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 10, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 10, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 10, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 10, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 10, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            11 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 11, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 11, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 11, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 11, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 11, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 11, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 11, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 11, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 11, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 11, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 11, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 11, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 11, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 11, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 11, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 11, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 11, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 11, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 11, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 11, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 11, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 11, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 11, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 11, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 11, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 11, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 11, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 11, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 11, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 11, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 11, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 11, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 11, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 11, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 11, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 11, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 11, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 11, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 11, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 11, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 11, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 11, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 11, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 11, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 11, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 11, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 11, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 11, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 11, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 11, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 11, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 11, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 11, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 11, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 11, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 11, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            12 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 12, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 12, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 12, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 12, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 12, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 12, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 12, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 12, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 12, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 12, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 12, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 12, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 12, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 12, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 12, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 12, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 12, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 12, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 12, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 12, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 12, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 12, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 12, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 12, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 12, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 12, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 12, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 12, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 12, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 12, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 12, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 12, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 12, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 12, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 12, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 12, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 12, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 12, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 12, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 12, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 12, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 12, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 12, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 12, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 12, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 12, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 12, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 12, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 12, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 12, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 12, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 12, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 12, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 12, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 12, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 12, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            13 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 13, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 13, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 13, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 13, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 13, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 13, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 13, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 13, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 13, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 13, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 13, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 13, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 13, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 13, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 13, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 13, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 13, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 13, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 13, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 13, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 13, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 13, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 13, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 13, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 13, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 13, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 13, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 13, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 13, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 13, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 13, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 13, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 13, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 13, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 13, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 13, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 13, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 13, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 13, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 13, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 13, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 13, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 13, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 13, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 13, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 13, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 13, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 13, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 13, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 13, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 13, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 13, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 13, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 13, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 13, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 13, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            14 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 14, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 14, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 14, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 14, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 14, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 14, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 14, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 14, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 14, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 14, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 14, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 14, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 14, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 14, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 14, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 14, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 14, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 14, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 14, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 14, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 14, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 14, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 14, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 14, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 14, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 14, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 14, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 14, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 14, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 14, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 14, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 14, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 14, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 14, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 14, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 14, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 14, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 14, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 14, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 14, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 14, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 14, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 14, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 14, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 14, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 14, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 14, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 14, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 14, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 14, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 14, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 14, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 14, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 14, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 14, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 14, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            15 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 15, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 15, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 15, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 15, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 15, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 15, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 15, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 15, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 15, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 15, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 15, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 15, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 15, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 15, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 15, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 15, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 15, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 15, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 15, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 15, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 15, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 15, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 15, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 15, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 15, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 15, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 15, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 15, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 15, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 15, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 15, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 15, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 15, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 15, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 15, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 15, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 15, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 15, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 15, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 15, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 15, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 15, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 15, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 15, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 15, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 15, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 15, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 15, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 15, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 15, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 15, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 15, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 15, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 15, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 15, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 15, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            16 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 16, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 16, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 16, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 16, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 16, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 16, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 16, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 16, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 16, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 16, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 16, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 16, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 16, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 16, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 16, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 16, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 16, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 16, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 16, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 16, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 16, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 16, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 16, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 16, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 16, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 16, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 16, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 16, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 16, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 16, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 16, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 16, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 16, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 16, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 16, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 16, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 16, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 16, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 16, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 16, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 16, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 16, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 16, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 16, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 16, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 16, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 16, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 16, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 16, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 16, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 16, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 16, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 16, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 16, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 16, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 16, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            17 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 17, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 17, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 17, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 17, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 17, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 17, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 17, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 17, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 17, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 17, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 17, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 17, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 17, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 17, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 17, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 17, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 17, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 17, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 17, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 17, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 17, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 17, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 17, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 17, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 17, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 17, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 17, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 17, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 17, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 17, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 17, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 17, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 17, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 17, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 17, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 17, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 17, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 17, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 17, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 17, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 17, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 17, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 17, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 17, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 17, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 17, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 17, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 17, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 17, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 17, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 17, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 17, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 17, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 17, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 17, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 17, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            18 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 18, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 18, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 18, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 18, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 18, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 18, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 18, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 18, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 18, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 18, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 18, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 18, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 18, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 18, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 18, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 18, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 18, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 18, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 18, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 18, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 18, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 18, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 18, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 18, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 18, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 18, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 18, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 18, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 18, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 18, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 18, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 18, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 18, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 18, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 18, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 18, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 18, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 18, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 18, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 18, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 18, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 18, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 18, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 18, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 18, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 18, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 18, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 18, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 18, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 18, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 18, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 18, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 18, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 18, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 18, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 18, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            19 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 19, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 19, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 19, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 19, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 19, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 19, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 19, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 19, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 19, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 19, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 19, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 19, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 19, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 19, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 19, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 19, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 19, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 19, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 19, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 19, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 19, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 19, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 19, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 19, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 19, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 19, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 19, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 19, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 19, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 19, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 19, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 19, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 19, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 19, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 19, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 19, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 19, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 19, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 19, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 19, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 19, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 19, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 19, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 19, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 19, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 19, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 19, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 19, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 19, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 19, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 19, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 19, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 19, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 19, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 19, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 19, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            20 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 20, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 20, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 20, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 20, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 20, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 20, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 20, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 20, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 20, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 20, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 20, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 20, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 20, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 20, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 20, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 20, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 20, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 20, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 20, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 20, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 20, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 20, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 20, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 20, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 20, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 20, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 20, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 20, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 20, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 20, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 20, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 20, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 20, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 20, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 20, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 20, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 20, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 20, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 20, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 20, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 20, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 20, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 20, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 20, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 20, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 20, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 20, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 20, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 20, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 20, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 20, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 20, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 20, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 20, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 20, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 20, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            21 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 21, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 21, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 21, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 21, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 21, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 21, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 21, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 21, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 21, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 21, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 21, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 21, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 21, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 21, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 21, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 21, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 21, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 21, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 21, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 21, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 21, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 21, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 21, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 21, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 21, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 21, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 21, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 21, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 21, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 21, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 21, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 21, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 21, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 21, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 21, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 21, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 21, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 21, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 21, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 21, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 21, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 21, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 21, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 21, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 21, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 21, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 21, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 21, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 21, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 21, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 21, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 21, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 21, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 21, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 21, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 21, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            22 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 22, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 22, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 22, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 22, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 22, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 22, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 22, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 22, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 22, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 22, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 22, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 22, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 22, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 22, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 22, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 22, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 22, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 22, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 22, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 22, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 22, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 22, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 22, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 22, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 22, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 22, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 22, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 22, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 22, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 22, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 22, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 22, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 22, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 22, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 22, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 22, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 22, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 22, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 22, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 22, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 22, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 22, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 22, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 22, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 22, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 22, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 22, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 22, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 22, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 22, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 22, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 22, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 22, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 22, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 22, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 22, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            23 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 23, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 23, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 23, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 23, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 23, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 23, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 23, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 23, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 23, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 23, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 23, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 23, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 23, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 23, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 23, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 23, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 23, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 23, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 23, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 23, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 23, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 23, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 23, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 23, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 23, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 23, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 23, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 23, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 23, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 23, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 23, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 23, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 23, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 23, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 23, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 23, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 23, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 23, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 23, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 23, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 23, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 23, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 23, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 23, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 23, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 23, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 23, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 23, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 23, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 23, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 23, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 23, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 23, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 23, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 23, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 23, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            24 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 24, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 24, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 24, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 24, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 24, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 24, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 24, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 24, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 24, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 24, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 24, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 24, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 24, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 24, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 24, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 24, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 24, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 24, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 24, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 24, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 24, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 24, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 24, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 24, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 24, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 24, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 24, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 24, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 24, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 24, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 24, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 24, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 24, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 24, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 24, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 24, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 24, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 24, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 24, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 24, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 24, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 24, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 24, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 24, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 24, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 24, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 24, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 24, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 24, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 24, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 24, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 24, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 24, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 24, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 24, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 24, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            25 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 25, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 25, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 25, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 25, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 25, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 25, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 25, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 25, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 25, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 25, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 25, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 25, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 25, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 25, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 25, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 25, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 25, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 25, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 25, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 25, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 25, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 25, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 25, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 25, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 25, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 25, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 25, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 25, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 25, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 25, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 25, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 25, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 25, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 25, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 25, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 25, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 25, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 25, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 25, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 25, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 25, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 25, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 25, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 25, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 25, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 25, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 25, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 25, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 25, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 25, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 25, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 25, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 25, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 25, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 25, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 25, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            26 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 26, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 26, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 26, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 26, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 26, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 26, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 26, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 26, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 26, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 26, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 26, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 26, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 26, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 26, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 26, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 26, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 26, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 26, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 26, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 26, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 26, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 26, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 26, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 26, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 26, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 26, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 26, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 26, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 26, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 26, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 26, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 26, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 26, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 26, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 26, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 26, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 26, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 26, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 26, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 26, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 26, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 26, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 26, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 26, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 26, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 26, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 26, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 26, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 26, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 26, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 26, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 26, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 26, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 26, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 26, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 26, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            27 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 27, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 27, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 27, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 27, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 27, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 27, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 27, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 27, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 27, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 27, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 27, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 27, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 27, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 27, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 27, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 27, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 27, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 27, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 27, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 27, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 27, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 27, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 27, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 27, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 27, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 27, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 27, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 27, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 27, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 27, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 27, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 27, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 27, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 27, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 27, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 27, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 27, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 27, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 27, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 27, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 27, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 27, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 27, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 27, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 27, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 27, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 27, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 27, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 27, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 27, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 27, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 27, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 27, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 27, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 27, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 27, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            28 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 28, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 28, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 28, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 28, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 28, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 28, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 28, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 28, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 28, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 28, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 28, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 28, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 28, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 28, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 28, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 28, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 28, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 28, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 28, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 28, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 28, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 28, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 28, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 28, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 28, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 28, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 28, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 28, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 28, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 28, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 28, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 28, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 28, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 28, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 28, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 28, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 28, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 28, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 28, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 28, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 28, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 28, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 28, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 28, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 28, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 28, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 28, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 28, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 28, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 28, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 28, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 28, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 28, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 28, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 28, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 28, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            29 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 29, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 29, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 29, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 29, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 29, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 29, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 29, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 29, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 29, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 29, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 29, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 29, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 29, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 29, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 29, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 29, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 29, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 29, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 29, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 29, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 29, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 29, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 29, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 29, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 29, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 29, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 29, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 29, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 29, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 29, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 29, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 29, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 29, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 29, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 29, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 29, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 29, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 29, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 29, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 29, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 29, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 29, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 29, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 29, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 29, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 29, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 29, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 29, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 29, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 29, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 29, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 29, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 29, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 29, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 29, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 29, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            30 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 30, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 30, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 30, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 30, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 30, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 30, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 30, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 30, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 30, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 30, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 30, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 30, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 30, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 30, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 30, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 30, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 30, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 30, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 30, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 30, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 30, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 30, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 30, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 30, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 30, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 30, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 30, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 30, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 30, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 30, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 30, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 30, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 30, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 30, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 30, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 30, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 30, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 30, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 30, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 30, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 30, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 30, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 30, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 30, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 30, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 30, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 30, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 30, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 30, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 30, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 30, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 30, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 30, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 30, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 30, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 30, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            31 => match lmul {
                -8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 31, e8, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 31, e16, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 31, e32, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 31, e64, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 31, e128, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 31, e256, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 31, e512, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 31, e1024, mf8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 31, e8, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 31, e16, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 31, e32, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 31, e64, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 31, e128, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 31, e256, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 31, e512, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 31, e1024, mf4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                -2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 31, e8, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 31, e16, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 31, e32, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 31, e64, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 31, e128, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 31, e256, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 31, e512, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 31, e1024, mf2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                1 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 31, e8, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 31, e16, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 31, e32, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 31, e64, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 31, e128, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 31, e256, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 31, e512, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 31, e1024, m1", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                2 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 31, e8, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 31, e16, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 31, e32, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 31, e64, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 31, e128, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 31, e256, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 31, e512, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 31, e1024, m2", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                4 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 31, e8, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 31, e16, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 31, e32, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 31, e64, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 31, e128, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 31, e256, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 31, e512, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 31, e1024, m4", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                8 => match sew {
                    8 => {
                        rvv_asm!("vsetivli t0, 31, e8, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    16 => {
                        rvv_asm!("vsetivli t0, 31, e16, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    32 => {
                        rvv_asm!("vsetivli t0, 31, e32, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    64 => {
                        rvv_asm!("vsetivli t0, 31, e64, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    128 => {
                        rvv_asm!("vsetivli t0, 31, e128, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    256 => {
                        rvv_asm!("vsetivli t0, 31, e256, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    512 => {
                        rvv_asm!("vsetivli t0, 31, e512, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    1024 => {
                        rvv_asm!("vsetivli t0, 31, e1024, m8", "mv {0}, t0", out (reg) vl);
                        vl
                    }
                    _ => panic!("Abort"),
                },
                _ => panic!("Abort"),
            },
            _ => panic!("Abort"),
        }
    }
}
