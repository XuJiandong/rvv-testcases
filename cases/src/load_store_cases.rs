use alloc::vec::Vec;
use ckb_std::syscalls::debug;
use rand::Rng;
use rvv_testcases::intrinsic::{vle_v1, vlse_v1, vlxei_v1, vs1r_v1, vse_v1, vsse_v1, vsxei_v1};
use rvv_testcases::log;
use rvv_testcases::misc::{MutSliceUtils, SliceUtils};
use rvv_testcases::{intrinsic::vsetvl, misc::VLEN, rng::BestNumberRng};

fn test_unit_stride(sew: usize) {
    let lmul = 1i64;
    let vl = (lmul as u64 * VLEN as u64) / sew as u64;
    let set_vl = vsetvl(vl, sew as u64, lmul);
    assert_eq!(set_vl, vl);

    let mut mem = [0u8; VLEN / 8];
    let mut mem2 = [0u8; VLEN / 8];

    let mut rng = BestNumberRng::default();
    rng.fill(&mut mem[..]);
    rng.fill(&mut mem2[..]);

    vle_v1(sew as u64, &mem[..]);
    vse_v1(sew as u64, &mut mem2[..]);

    if mem != mem2 {
        log!("test_unit_stride() failed, sew = {}", sew);
        log!(
            "Failed on test_unit_stride: {:?} (expected) {:?} (result)",
            mem,
            mem2
        );
        panic!("Abort");
    }
}

// `sew` is in bit `stride` is in bytes
fn test_stride(sew: usize, stride: usize) {
    assert!(stride >= sew as usize);
    let lmul = 1i64;
    let vl = (lmul as u64 * VLEN as u64) / sew as u64;
    let set_vl = vsetvl(vl, sew as u64, lmul);
    assert_eq!(set_vl, vl);
    let mut rng = BestNumberRng::default();

    let mut mem = Vec::<u8>::new();
    let mut mem2 = Vec::<u8>::new();
    let total_size = stride * vl as usize;
    mem.resize(total_size, 0);
    mem2.resize(total_size, 0);
    rng.fill(mem.as_mut_slice());
    rng.fill(mem2.as_mut_slice());

    vlse_v1(sew as u64, &mem[..], stride as u64);
    vsse_v1(sew as u64, &mem2[..], stride as u64);

    for i in 0..vl as usize {
        let range = i * stride as usize..i * stride + sew / 8;
        let expected = &mem[range.clone()];
        let result = &mem2[range.clone()];
        if expected != result {
            log!("test_failed, sew = {}, stride = {}", sew, stride);
            log!("expected = {:?}, result = {:?}", expected, result);
            panic!("Abort");
        }
    }
}

#[allow(dead_code)]
fn test_indexed_load(sew: usize, offset_sew: usize) {
    let lmul = 1i64;
    let vl = (lmul as usize * VLEN as usize) / sew as usize;
    let set_vl = vsetvl(vl as u64, sew as u64, lmul);
    assert_eq!(set_vl as usize, vl);

    let mut rng = BestNumberRng::default();

    let mut offset = Vec::<u8>::new();
    let mut mem = Vec::<u8>::new();
    let mut expected = Vec::<u8>::new();
    let mut result = Vec::<u8>::new();
    offset.resize(vl * offset_sew / 8, 0);
    mem.resize(vl * sew / 8, 0);
    expected.resize(vl * sew / 8, 0);
    result.resize(vl * sew / 8, 0);

    rng.fill(&mut mem[..]);
    rng.fill(&mut result[..]);

    for i in 0..vl {
        let index = i * sew / 8; // TODO: randomize it
        let mut offset = offset.as_mut_slice();
        match offset_sew {
            8 => {
                offset.write_u8(offset_sew, i, index as u8);
            }
            16 => {
                offset.write_u16(offset_sew, i, index as u16);
            }
            32 => {
                offset.write_u32(offset_sew, i, index as u32);
            }
            64 => {
                offset.write_u64(offset_sew, i, index as u64);
            }
            _ => {
                unreachable!()
            }
        }
        // let slice = &mem[index..index + sew / 8];
        // expected[i * sew / 8..(i + 1) * sew / 8].copy_from_slice(slice);
        let mem = mem.as_slice();
        let mut expected = expected.as_mut_slice();

        let slice = mem.get_element(sew, index);
        expected.get_mut_element(sew, i).copy_from_slice(slice);
    }

    vlxei_v1(offset_sew as u64, mem.as_slice(), offset.as_slice());

    assert!(result.len() >= VLEN / 8);
    vs1r_v1(result.as_mut_slice());

    for i in 0..vl {
        let result = result.as_slice();
        let offset = offset.as_slice();
        let mem = mem.as_slice();
        let result_element = result.get_element(sew, i);
        let index = match offset_sew {
            8 => offset.read_u8(offset_sew, i) as usize,
            16 => offset.read_u16(offset_sew, i) as usize,
            32 => offset.read_u32(offset_sew, i) as usize,
            64 => offset.read_u64(offset_sew, i) as usize,
            _ => {
                unreachable!()
            }
        };
        let expected_element = mem.get_element(sew, index);
        if result_element != expected_element {
            log!(
                "Failed on test_indexed, sew = {}, offset_sew = {}, i = {}",
                sew,
                offset_sew,
                i
            );
            log!(
                "result element = {:?}, expected element = {:?}, index = {}",
                result_element,
                expected_element,
                index
            );
            panic!("Abort");
        }
    }
}

pub fn test_load_store() {
    for sew in [8, 16, 32, 64, 128, 256, 512, 1024] {
        test_unit_stride(sew);
    }
    for sew in [8, 16, 32, 64, 128, 256, 512, 1024] {
        test_stride(sew, sew + 16);
    }
    // test_indexed_load(256, 8);
}
