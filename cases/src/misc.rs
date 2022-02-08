use rvv_simulator_runtime::Uint;

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
            let r: U256 = u128::MAX.into();
            let r2: U256 = self.clone().into();
            (r << 128) + r2
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
