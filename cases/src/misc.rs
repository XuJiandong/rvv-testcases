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
