use super::misc::create_vtype;
use core::arch::asm;
use rvv_asm::rvv_asm;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum VInstructionOp {
    Add,
    Sub,
    And,
    Or,
    Xor,
    ShiftLeft,
    ShiftRight,
    ShiftRightArithmetic,
    Invalid,
}

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

fn vle_v1(sew: u64, buf: &[u8]) {
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

#[inline(never)]
pub fn vop_vv(
    lhs: &[u8],
    rhs: &[u8],
    result: &mut [u8],
    sew: u64,
    avl: u64,
    lmul: i64,
    t: VInstructionOp,
) {
    let mut avl = avl;
    let mut lhs = lhs;
    let mut rhs = rhs;
    let mut result = result;

    let sew_bytes = sew / 8;
    loop {
        let vl = vsetvl(avl as u64, sew, lmul);
        vle_v1(sew, lhs);
        vle_v11(sew, rhs);

        unsafe {
            match t {
                VInstructionOp::Add => {
                    rvv_asm!("vadd.vv v21, v1, v11"); // ADD
                }
                VInstructionOp::Sub => {
                    rvv_asm!("vsub.vv v21, v1, v11"); // SUB
                }
                VInstructionOp::And => {
                    rvv_asm!("vand.vv v21, v1, v11"); // AND
                }
                VInstructionOp::Or => {
                    rvv_asm!("vor.vv v21, v1, v11"); // OR
                }
                VInstructionOp::Xor => {
                    rvv_asm!("vxor.vv v21, v1, v11"); // XOR
                }
                VInstructionOp::ShiftLeft => {
                    rvv_asm!("vsll.vv v21, v1, v11"); // shift left
                }
                VInstructionOp::ShiftRight => {
                    rvv_asm!("vsrl.vv v21, v1, v11"); // shift right
                }
                VInstructionOp::ShiftRightArithmetic => {
                    rvv_asm!("vsra.vv v21, v1, v11"); // shift right arithmetic
                }
                VInstructionOp::Invalid => {
                    panic!("Invalid");
                }
            }
        }
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

#[allow(dead_code)]
#[inline(never)]
fn vop_vv_deprecated(
    lhs: &[u8],
    rhs: &[u8],
    result: &mut [u8],
    avl: u64,
    vtype: u64,
    t: VInstructionOp,
    shift_amount: u64,
) {
    unsafe {
        rvv_asm!("mv t0, {0}" , in (reg) lhs.as_ref().as_ptr());
        rvv_asm!("mv t1, {0}", in (reg) rhs.as_ref().as_ptr());
        rvv_asm!("mv t2, {0}", in (reg) result.as_ref().as_ptr());
        rvv_asm!("mv t3, {0}", in (reg) avl);
        rvv_asm!("mv t4, {0}", in (reg) vtype);
        rvv_asm!("mv t6, {0}", in (reg) shift_amount);

        rvv_asm!("1:");
        rvv_asm!("vsetvl t5, t3, t4");
        rvv_asm!("sub t3, t3, t5"); // decrease avl

        // convert 'vl' to bytes: vl << shift_amount
        // E.G. U256, it's 32 bytes per element, which is `t5 << 5`
        rvv_asm!("sll t5, t5, t6");

        match shift_amount {
            0 => {
                rvv_asm!("vle8.v v1, (t0)"); // load lhs to v1
                rvv_asm!("vle8.v v11, (t1)"); // load rhs to v11
            }
            1 => {
                rvv_asm!("vle16.v v1, (t0)"); // load lhs to v1
                rvv_asm!("vle16.v v11, (t1)"); // load rhs to v11
            }
            2 => {
                rvv_asm!("vle32.v v1, (t0)"); // load lhs to v1
                rvv_asm!("vle32.v v11, (t1)"); // load rhs to v11
            }
            3 => {
                rvv_asm!("vle64.v v1, (t0)"); // load lhs to v1
                rvv_asm!("vle64.v v11, (t1)"); // load rhs to v11
            }
            4 => {
                rvv_asm!("vle128.v v1, (t0)"); // load lhs to v1
                rvv_asm!("vle128.v v11, (t1)"); // load rhs to v11
            }
            5 => {
                rvv_asm!("vle256.v v1, (t0)"); // load lhs to v1
                rvv_asm!("vle256.v v11, (t1)"); // load rhs to v11
            }
            6 => {
                rvv_asm!("vle512.v v1, (t0)"); // load lhs to v1
                rvv_asm!("vle512.v v11, (t1)"); // load rhs to v11
            }
            7 => {
                rvv_asm!("vle1024.v v1, (t0)"); // load lhs to v1
                rvv_asm!("vle1024.v v11, (t1)"); // load rhs to v11
            }
            _ => {
                panic!("Invalid shift_amount");
            }
        }

        rvv_asm!("add t0, t0, t5"); // increase lhs
        rvv_asm!("add t1, t1, t5"); // increase rhs

        // be careful while using this
        match t {
            VInstructionOp::Add => {
                rvv_asm!("vadd.vv v21, v1, v11"); // ADD
            }
            VInstructionOp::Sub => {
                rvv_asm!("vsub.vv v21, v1, v11"); // SUB
            }
            VInstructionOp::And => {
                rvv_asm!("vand.vv v21, v1, v11"); // AND
            }
            VInstructionOp::Or => {
                rvv_asm!("vor.vv v21, v1, v11"); // OR
            }
            VInstructionOp::Xor => {
                rvv_asm!("vxor.vv v21, v1, v11"); // XOR
            }
            VInstructionOp::ShiftLeft => {
                rvv_asm!("vsll.vv v21, v1, v11"); // shift left
            }
            VInstructionOp::ShiftRight => {
                rvv_asm!("vsrl.vv v21, v1, v11"); // shift right
            }
            VInstructionOp::ShiftRightArithmetic => {
                rvv_asm!("vsra.vv v21, v1, v11"); // shift right arithmetic
            }
            VInstructionOp::Invalid => {
                panic!("Invalid");
            }
        }
        // store v21 to result
        match shift_amount {
            0 => {
                rvv_asm!("vse8.v v21, (t2)");
            }
            1 => {
                rvv_asm!("vse16.v v21, (t2)");
            }
            2 => {
                rvv_asm!("vse32.v v21, (t2)");
            }
            3 => {
                rvv_asm!("vse64.v v21, (t2)");
            }
            4 => {
                rvv_asm!("vse128.v v21, (t2)");
            }
            5 => {
                rvv_asm!("vse256.v v21, (t2)");
            }
            6 => {
                rvv_asm!("vse512.v v21, (t2)");
            }
            7 => {
                rvv_asm!("vse1024.v v21, (t2)");
            }
            _ => {
                panic!("Invalid shift_amount");
            }
        }
        rvv_asm!("add t2, t2, t5"); // increase result
        rvv_asm!("bnez t3, 1b"); // finished?
    }
}
