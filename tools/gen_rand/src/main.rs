use rand_core::RngCore;
use rand_seeder::{Seeder, SipRng};
use std::env;
use std::fs;
use std::string::String;

fn main() {
    let args: Vec<String> = env::args().collect();
    let seed = if args.len() > 1 {
        args[1].parse::<usize>().unwrap()
    } else {
        0
    };
    println!("Using seed: {}", seed);

    let mut code = String::new();

    code += "pub static G_DATA: [u8; 512] = [";

    let mut rng: SipRng = Seeder::from(seed).make_rng();
    let data = {
        let mut d = [0u8; 512];
        rng.fill_bytes(&mut d);
        d
    };

    let mut i = 0;
    for d in data {
        if i == 0 {
            code += "\n    ";
        }
        code += &format!("0x{:0>2X?}, ", d);
        i = (i + 1) % 16
    }

    code += "\n];\n";

    // rust

    fs::write("../../cases/src/rng_data.rs", code.clone()).unwrap();

    //print!("{}", code);
}
