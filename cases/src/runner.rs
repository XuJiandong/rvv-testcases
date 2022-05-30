use alloc::boxed::Box;
use alloc::vec::Vec;
use core::convert::TryInto;
use core::fmt::{Display, Formatter, Result};
use core::ops::Range;

use ckb_std::syscalls::debug;
use rand::Rng;

use crate::intrinsic::{
    clean_cache_v16, clean_cache_v8, vl1r_v0, vle_v16, vle_v24, vle_v8, vse_v24, vsetvl,
};
use crate::misc::{avl_iterator, VLEN};

use super::log;
use super::misc::{get_bit_in_slice, is_verbose, set_bit_in_slice};
use super::rng::BestNumberRng;

pub enum WideningCategory {
    None,
    VdOnly,
    Vs2Only,
    VdVs1,
    VdVs2,
    // EEW, 1/2, 1/4, or 1/8 of SEW
    NarrowVs2(usize),
}

pub enum ExpectedOp {
    Normal(Box<dyn FnMut(&[u8], &[u8], &mut [u8])>),
    Reduction(Box<dyn FnMut(&[u8], &[u8], &mut [u8], usize)>),
    EnableMask(Box<dyn FnMut(&[u8], &[u8], &mut [u8], bool, usize)>),
    WithMask(Box<dyn FnMut(&[u8], &[u8], &mut [u8], u8)>),
}

pub fn run_vxop_m<T>(mut expected_op: ExpectedOp, mut v_op: T, enable_mask: bool, desc: &str)
where
    T: FnMut(&[u8], &[u8], &mut [u8], bool),
{
    if is_verbose() {
        log!("run with desc = {}", desc);
    }
    let mut mask_v0 = [0u8; VLEN / 8];
    let mut vs2 = [0u8; VLEN / 8];

    let mut rng = BestNumberRng::default();
    rng.fill(&mut mask_v0[..]);
    rng.fill(&mut vs2[..]);

    let vl = vsetvl(8, 256, 1) as usize;
    assert_eq!(vl, 8);

    let expected = if let ExpectedOp::EnableMask(ref mut op) = expected_op {
        let mut temp = [0u8; 8];
        op(&mask_v0[..], &vs2[..], &mut temp[..], enable_mask, vl);
        u64::from_le_bytes(temp)
    } else {
        panic!("Unexpected op")
    };

    let mut temp = [0u8; 8];
    v_op(
        mask_v0.as_slice(),
        vs2.as_slice(),
        temp.as_mut_slice(),
        enable_mask,
    );
    let result = u64::from_le_bytes(temp);

    if result != expected {
        log!(
            "[describe = {}] unexpected values found: {:?} (result) {:?} (expected)",
            desc,
            result,
            expected
        );
        panic!("Abort");
    }
    if is_verbose() {
        log!("finished");
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum InstructionArgsType {
    Vector,
    Vector2,
    VectorBit,
    VectorAlway16,
    VectorRed,
    VectorRed2,
    VectorNarrow2,
    VectorNarrow4,
    VectorNarrow8,
    Scalar,
    Immediate,
    UImmediate,
    None,
}

impl Display for InstructionArgsType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            InstructionArgsType::Vector => write!(f, "vector"),
            InstructionArgsType::Vector2 => write!(f, "vector2"),
            InstructionArgsType::VectorBit => write!(f, "vector_bit"),
            InstructionArgsType::VectorAlway16 => write!(f, "vector_alway16"),
            InstructionArgsType::VectorRed => write!(f, "vector_red"),
            InstructionArgsType::VectorRed2 => write!(f, "vector_red2"),
            InstructionArgsType::VectorNarrow2 => write!(f, "vector_n2"),
            InstructionArgsType::VectorNarrow4 => write!(f, "vector_n4"),
            InstructionArgsType::VectorNarrow8 => write!(f, "vector_n8"),
            InstructionArgsType::Scalar => write!(f, "scalar"),
            InstructionArgsType::Immediate => write!(f, "immediate"),
            InstructionArgsType::UImmediate => write!(f, "uimmediate"),
            InstructionArgsType::None => write!(f, "none"),
        }
    }
}

impl InstructionArgsType {
    fn is_imm(&self) -> bool {
        *self == InstructionArgsType::Immediate || *self == InstructionArgsType::UImmediate
    }

    fn get_buf_len(&self, sew_byte: usize, vl: usize) -> usize {
        match *self {
            InstructionArgsType::None => 1 * sew_byte,
            InstructionArgsType::Vector => vl * sew_byte,
            InstructionArgsType::Vector2 => vl * 2 * sew_byte,
            InstructionArgsType::VectorBit => vl * sew_byte,
            InstructionArgsType::VectorAlway16 => vl * 2, // The minimum sew is 8, sew * 2 >= 16
            InstructionArgsType::VectorRed => vl * sew_byte,
            InstructionArgsType::VectorRed2 => vl * 2 * sew_byte,
            InstructionArgsType::VectorNarrow2 => vl * sew_byte / 2,
            InstructionArgsType::VectorNarrow4 => vl * sew_byte / 4,
            InstructionArgsType::VectorNarrow8 => vl * sew_byte / 8,
            _ => 8,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MaskType {
    Disable,
    Enable,
    AsParam,
}

impl Display for MaskType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            MaskType::Disable => write!(f, "disable"),
            MaskType::Enable => write!(f, "enable"),
            MaskType::AsParam => write!(f, "as_param"),
        }
    }
}

pub struct RVVTestData {
    pub lhs: Vec<u8>,
    pub lhs_type: InstructionArgsType,
    pub rhs: Vec<u8>,
    pub rhs_type: InstructionArgsType,
    pub mask: Vec<u8>,
    pub mask_type: MaskType,

    pub res_before: Vec<u8>,
    pub res_rvv: Vec<u8>,
    pub res_exp: Vec<u8>,
    pub res_type: InstructionArgsType,

    pub sew_bytes: usize,
    pub index: usize,

    pub sew: u64,
    pub lmul: i64,
    pub avl: u64,

    pub theoretically_vl: usize,
    pub count: usize,
}

#[derive(Clone, Copy)]
enum VectorCallbackType {
    None(fn(&mut RVVTestData)),
    VV(fn(&[u8], &[u8], &mut [u8])),
    VX(fn(&[u8], u64, &mut [u8])),
    VI(fn(&[u8], i64, &mut [u8])),
    VVM(fn(&[u8], &[u8], &mut [u8], bool)),
    VXM(fn(&[u8], u64, &mut [u8], bool)),
    VIM(fn(&[u8], i64, &mut [u8], bool)),
    MVVM(fn(&[u8], &[u8], &mut bool, bool)),
    MVXM(fn(&[u8], u64, &mut bool, bool)),
    MVIM(fn(&[u8], i64, &mut bool, bool)),
    MVV(fn(&[u8], &[u8], &mut bool)),
    MVX(fn(&[u8], u64, &mut bool)),
    MVI(fn(&[u8], i64, &mut bool)),
    MMM(fn(bool, bool, &mut bool)),
    VVR(fn(&[u8], &[u8], &mut [u8], usize)),
}

impl RVVTestData {
    fn new(
        lhs_type: InstructionArgsType,
        rhs_type: InstructionArgsType,
        res_type: InstructionArgsType,
        mask_type: MaskType,
        sew: u64,
        lmul: i64,
        avl: u64,
    ) -> Self {
        let vl = vsetvl(avl, sew, lmul);
        assert!(
            vl != 0,
            "vsetvl result {}, sew: {}, lmul {}, avl: {}",
            vl,
            sew,
            lmul,
            avl
        );
        RVVTestData {
            lhs: Vec::new(),
            lhs_type,
            rhs: Vec::new(),
            rhs_type,
            mask: Vec::new(),
            mask_type,
            res_before: Vec::new(),
            res_rvv: Vec::new(),
            res_exp: Vec::new(),
            res_type,

            sew_bytes: (sew / 8) as usize,

            index: 0,

            sew,
            lmul,
            avl,
            theoretically_vl: vl as usize,

            count: 0,
        }
    }

    fn rng_fill(&mut self) {
        let mut rng = BestNumberRng::default();

        // mask
        let mask_len = {
            let len = (self.avl / 8 + 1) as usize;
            if len < VLEN / 8 {
                VLEN / 8
            } else {
                len
            }
        };
        self.mask.resize(mask_len, 0xFF);
        rng.fill(self.mask.as_mut_slice());

        // lhs
        let lhs_len = self.lhs_type.get_buf_len(self.sew_bytes, self.avl as usize);
        self.lhs.resize(lhs_len, 0);
        rng.fill(self.lhs.as_mut_slice());

        // rhs
        let rhs_len = self.rhs_type.get_buf_len(self.sew_bytes, self.avl as usize);
        self.rhs.resize(rhs_len, 0);
        rng.fill(self.rhs.as_mut_slice());

        // res
        let res_len = self.res_type.get_buf_len(self.sew_bytes, self.avl as usize);
        self.res_before.resize(res_len, 0);
        rng.fill(self.res_before.as_mut_slice());
        self.res_rvv = self.res_before.clone();
        self.res_exp = self.res_before.clone();
    }

    fn get_args_range(&self, t: InstructionArgsType, index: usize) -> Range<usize> {
        match t {
            InstructionArgsType::None => {
                panic!("Can not get range for here")
            }
            InstructionArgsType::Vector => index * self.sew_bytes..(index + 1) * self.sew_bytes,
            InstructionArgsType::Vector2 => {
                index * self.sew_bytes * 2..(index + 1) * self.sew_bytes * 2
            }
            InstructionArgsType::VectorAlway16 => index * 2..(index + 1) * 2,
            InstructionArgsType::VectorRed => {
                let i = self.count * self.theoretically_vl;
                i * self.sew_bytes..(i + 1) * self.sew_bytes
            }
            InstructionArgsType::VectorRed2 => {
                let i = self.count * self.theoretically_vl;
                i * self.sew_bytes * 2..(i + 1) * self.sew_bytes * 2
            }
            InstructionArgsType::VectorNarrow2 => {
                ((index * self.sew_bytes) as f64 * 0.5) as usize
                    ..(((index + 1) * self.sew_bytes) as f64 * 0.5) as usize
            }
            InstructionArgsType::VectorNarrow4 => {
                ((index * self.sew_bytes) as f64 * 0.25) as usize
                    ..(((index + 1) * self.sew_bytes) as f64 * 0.25) as usize
            }
            InstructionArgsType::VectorNarrow8 => {
                ((index * self.sew_bytes) as f64 * 0.125) as usize
                    ..(((index + 1) * self.sew_bytes) as f64 * 0.125) as usize
            }
            InstructionArgsType::VectorBit => {
                panic!("VectorBit can not get range for here")
            }
            _ => 0..8,
        }
    }

    fn get_data_by_slice(&self, d: &[u8], t: InstructionArgsType) -> Vec<u8> {
        if t == InstructionArgsType::VectorBit {
            let mut res = Vec::<u8>::new();
            res.resize(1, 0);
            res[0] = get_bit_in_slice(d, self.index);
            res
        } else {
            d[self.get_args_range(t, self.index)].to_vec()
        }
    }

    pub fn get_data(&self, d: &[u8], t: InstructionArgsType, index: usize) -> Vec<u8> {
        if t == InstructionArgsType::VectorBit {
            let mut res = Vec::<u8>::new();
            res.resize(1, 0);
            res[0] = get_bit_in_slice(d, index);
            res
        } else {
            d[self.get_args_range(t, index)].to_vec()
        }
    }

    pub fn get_left(&self) -> Vec<u8> {
        self.get_data_by_slice(&self.lhs, self.lhs_type)
    }

    pub fn get_right(&self) -> Vec<u8> {
        self.get_data_by_slice(&self.rhs, self.rhs_type)
    }

    pub fn get_right_u64(&self) -> u64 {
        let d = self.get_right();
        if d.len() != 8 {
            panic!("get_right_u64, right len is: {}", d.len())
        }
        u64::from_le_bytes(self.get_right().try_into().unwrap())
    }

    fn get_result_befor(&self) -> Vec<u8> {
        self.get_data_by_slice(&self.res_before, self.res_type)
    }

    fn get_result_rvv(&self) -> Vec<u8> {
        self.get_data_by_slice(&self.res_rvv, self.res_type)
    }

    fn get_result_exp(&self) -> Vec<u8> {
        self.get_data_by_slice(&self.res_exp, self.res_type)
    }

    pub fn set_result_exp(&mut self, data: &[u8]) {
        let r = self.get_args_range(self.res_type, self.index);
        if data.len() != r.len() {
            panic!("set_result data len is ne: {}, {}", data.len(), r.len());
        }
        self.res_exp[r].copy_from_slice(data);
    }

    fn get_mask(&self) -> bool {
        match self.mask_type {
            MaskType::Enable => get_bit_in_slice(&self.mask, self.index) == 1,
            MaskType::AsParam => get_bit_in_slice(&self.mask, self.index) == 1,
            MaskType::Disable => true,
        }
    }

    fn get_rvv_slice(&self, buf: &[u8], buf_type: InstructionArgsType) -> Vec<u8> {
        if buf_type == InstructionArgsType::Immediate
            || buf_type == InstructionArgsType::UImmediate
            || buf_type == InstructionArgsType::Scalar
        {
            buf.to_vec()
        } else if buf_type == InstructionArgsType::VectorBit {
            let vl = buf_type.get_buf_len(self.sew_bytes, self.theoretically_vl);

            let begin = vl * self.count / self.sew_bytes;
            let mut end = vl * (self.count + 1) / self.sew_bytes;

            assert!(begin < end, "Abort begin: {} < end: {}", begin, end);

            if end > buf.len() * 8 {
                end = buf.len() * 8;
            }
            let buf_len = end - begin;
            let buf_len = buf_len / 8 + if buf_len % 8 != 0 { 1 } else { 0 };
            let mut ret_buf = Vec::<u8>::new();
            ret_buf.resize(buf_len, 0);
            for i in begin..end {
                set_bit_in_slice(&mut ret_buf, i - begin, get_bit_in_slice(buf, i));
            }

            ret_buf
        } else {
            let vl = buf_type.get_buf_len(self.sew_bytes, self.theoretically_vl);
            let begin = vl * self.count;
            let mut end = vl * (self.count + 1);
            assert!(begin < end, "Abort begin: {} < end: {}", begin, end);
            if end > buf.len() {
                end = buf.len();
            }
            buf[begin..end].to_vec()
        }
    }

    fn get_rvv_left(&self) -> Vec<u8> {
        self.get_rvv_slice(self.lhs.as_slice(), self.lhs_type)
    }

    fn get_rvv_right(&self) -> Vec<u8> {
        self.get_rvv_slice(self.rhs.as_slice(), self.rhs_type)
    }

    fn get_rvv_result(&self, buf: &[u8]) -> Vec<u8> {
        self.get_rvv_slice(buf, self.res_type)
    }

    fn set_rvv_result(&mut self, src: &[u8]) {
        if self.res_type == InstructionArgsType::Immediate
            || self.res_type == InstructionArgsType::UImmediate
            || self.res_type == InstructionArgsType::Scalar
        {
            self.res_rvv.copy_from_slice(src);
        } else if self.res_type == InstructionArgsType::VectorBit {
            let vl = self
                .res_type
                .get_buf_len(self.sew_bytes, self.theoretically_vl);

            let begin = vl * self.count / self.sew_bytes;
            let mut end = vl * (self.count + 1) / self.sew_bytes;
            if end > self.res_rvv.len() / self.sew_bytes {
                end = self.res_rvv.len() / self.sew_bytes;
            }
            assert!(begin < end);
            for i in begin..end {
                set_bit_in_slice(
                    self.res_rvv.as_mut_slice(),
                    i,
                    get_bit_in_slice(src, i - begin),
                );
            }
        } else {
            let vl = self
                .res_type
                .get_buf_len(self.sew_bytes, self.theoretically_vl);

            let begin = vl * self.count;
            let mut end = vl * (self.count + 1);
            if end > self.res_rvv.len() {
                end = self.res_rvv.len();
            }
            self.res_rvv[begin..end].copy_from_slice(src);
        }
    }

    fn get_rvv_mask(&self) -> Vec<u8> {
        self.get_rvv_slice(&self.mask, InstructionArgsType::VectorBit)
    }

    fn get_sew(sew: u64, t: InstructionArgsType) -> u64 {
        match t {
            InstructionArgsType::Vector2 => sew * 2,
            InstructionArgsType::VectorAlway16 => 16,
            InstructionArgsType::VectorRed2 => sew * 2,
            InstructionArgsType::VectorNarrow2 => (sew as f64 * 0.5) as u64,
            InstructionArgsType::VectorNarrow4 => (sew as f64 * 0.25) as u64,
            InstructionArgsType::VectorNarrow8 => (sew as f64 * 0.125) as u64,
            _ => sew,
        }
    }

    fn get_left_sew(&self, sew: u64) -> u64 {
        RVVTestData::get_sew(sew, self.lhs_type)
    }

    fn get_right_sew(&self, sew: u64) -> u64 {
        RVVTestData::get_sew(sew, self.rhs_type)
    }

    fn get_result_sew(&self, sew: u64) -> u64 {
        RVVTestData::get_sew(sew, self.res_type)
    }

    pub fn get_vl(&self) -> usize {
        let total_vl = match self.res_type {
            InstructionArgsType::Vector => self.res_before.len() / self.sew_bytes,
            InstructionArgsType::Vector2 => self.res_before.len() / self.sew_bytes / 2,
            InstructionArgsType::VectorRed => self.res_before.len() / self.sew_bytes,
            InstructionArgsType::VectorRed2 => self.res_before.len() / self.sew_bytes / 2,
            InstructionArgsType::VectorNarrow2 => {
                ((self.res_before.len() / self.sew_bytes) as f64 / 0.5) as usize
            }
            InstructionArgsType::VectorNarrow4 => {
                ((self.res_before.len() / self.sew_bytes) as f64 / 0.25) as usize
            }
            InstructionArgsType::VectorNarrow8 => {
                ((self.res_before.len() / self.sew_bytes) as f64 / 0.125) as usize
            }
            _ => panic!("Abort"),
        };

        let rem = total_vl % self.theoretically_vl;
        let total_vl = total_vl / self.theoretically_vl;
        if self.count < total_vl {
            self.theoretically_vl
        } else if self.count == total_vl {
            if rem == 0 {
                self.theoretically_vl
            } else {
                rem
            }
        } else {
            rem
        }
    }

    fn get_lmul(lmul: i64) -> f64 {
        match lmul {
            -8 => 0.125,
            -4 => 0.25,
            -2 => 0.5,
            1 => 1.0,
            2 => 2.0,
            4 => 4.0,
            8 => 8.0,
            _ => panic!("Abort"),
        }
    }

    fn get_rvv_index(&self) -> usize {
        self.index % self.theoretically_vl
    }
}

fn run_rvv_op(rvv_data: &mut RVVTestData, op: fn(&[u8], &[u8], MaskType)) {
    let empty_buf = [0u8; 1];

    let mut avl = rvv_data.avl as i64;
    let sew = rvv_data.sew;
    let mask_type = rvv_data.mask_type;

    rvv_data.count = 0;
    while avl > 0 {
        let vl = vsetvl(avl as u64, rvv_data.sew, rvv_data.lmul) as usize;
        if vl == 0 {
            panic!("Abort")
        }
        avl -= vl as i64;

        let l = if rvv_data.lhs_type == InstructionArgsType::Immediate
            || rvv_data.lhs_type == InstructionArgsType::UImmediate
            || rvv_data.lhs_type == InstructionArgsType::Scalar
        {
            rvv_data.lhs.as_slice()
        } else {
            clean_cache_v8();
            vle_v8(rvv_data.get_left_sew(sew), &rvv_data.get_rvv_left());
            &empty_buf
        };

        let r = if rvv_data.rhs_type == InstructionArgsType::Immediate
            || rvv_data.rhs_type == InstructionArgsType::UImmediate
            || rvv_data.rhs_type == InstructionArgsType::Scalar
        {
            rvv_data.rhs.as_slice()
        } else {
            clean_cache_v16();
            vle_v16(rvv_data.get_right_sew(sew), &rvv_data.get_rvv_right());
            &empty_buf
        };

        if mask_type == MaskType::Enable || mask_type == MaskType::AsParam {
            let mut buf = Vec::<u8>::new();
            buf.resize(VLEN / 8, 0);
            let mask = rvv_data.get_rvv_mask();
            buf[..mask.len()].copy_from_slice(&mask);

            vl1r_v0(&buf);
        }

        let (mut result, result_len) = {
            let d = rvv_data.get_rvv_result(&rvv_data.res_rvv);
            if rvv_data.res_type == InstructionArgsType::VectorBit {
                let mut buf = Vec::<u8>::new();
                buf.resize(rvv_data.theoretically_vl * rvv_data.sew_bytes, 0);

                buf[..d.len()].copy_from_slice(&d);

                (buf, d.len())
            } else {
                let dlen = d.len();
                (d, dlen)
            }
        };
        vle_v24(rvv_data.get_result_sew(sew), &result);
        op.clone()(l, r, mask_type);
        vse_v24(rvv_data.get_result_sew(sew), &mut result);
        rvv_data.set_rvv_result(&result[..result_len]);
        rvv_data.count += 1;
    }
}

fn run_op(
    rvv_data: &mut RVVTestData,
    rvv_op: fn(&[u8], &[u8], MaskType),
    exp_op: VectorCallbackType,
    masked_op: fn(&mut RVVTestData),
    desc: &str,
) {
    if is_verbose() {
        log!(
            "run with sew = {}, lmul = {}, avl = {}, desc = {}",
            rvv_data.sew,
            rvv_data.lmul,
            rvv_data.avl,
            desc
        );
    }

    for i in 0..rvv_data.avl as usize {
        rvv_data.index = i;
        rvv_data.count = i / rvv_data.theoretically_vl as usize;
        if rvv_data.mask_type == MaskType::Enable && !rvv_data.get_mask() {
            masked_op.clone()(rvv_data);
            continue;
        }
        match exp_op {
            VectorCallbackType::None(op) => {
                op(rvv_data);
            }
            VectorCallbackType::VV(op) => {
                let mut res = rvv_data.get_result_befor();
                op(
                    rvv_data.get_left().as_slice(),
                    rvv_data.get_right().as_slice(),
                    res.as_mut_slice(),
                );
                rvv_data.set_result_exp(&res);
            }
            VectorCallbackType::VX(op) => {
                let mut res = rvv_data.get_result_befor();
                op(
                    rvv_data.get_left().as_slice(),
                    rvv_data.get_right_u64(),
                    res.as_mut_slice(),
                );
                rvv_data.set_result_exp(&res);
            }
            VectorCallbackType::VI(op) => {
                let mut res = rvv_data.get_result_befor();
                op(
                    rvv_data.get_left().as_slice(),
                    rvv_data.get_right_u64() as i64,
                    res.as_mut_slice(),
                );
                rvv_data.set_result_exp(&res);
            }
            VectorCallbackType::VVM(op) => {
                let mut res = rvv_data.get_result_befor();
                op(
                    rvv_data.get_left().as_slice(),
                    rvv_data.get_right().as_slice(),
                    res.as_mut_slice(),
                    rvv_data.get_mask(),
                );
                rvv_data.set_result_exp(&res);
            }
            VectorCallbackType::VXM(op) => {
                let mut res = rvv_data.get_result_befor();
                op(
                    rvv_data.get_left().as_slice(),
                    rvv_data.get_right_u64(),
                    res.as_mut_slice(),
                    rvv_data.get_mask(),
                );
                rvv_data.set_result_exp(&res);
            }
            VectorCallbackType::VIM(op) => {
                let mut res = rvv_data.get_result_befor();
                op(
                    rvv_data.get_left().as_slice(),
                    rvv_data.get_right_u64() as i64,
                    res.as_mut_slice(),
                    rvv_data.get_mask(),
                );
                rvv_data.set_result_exp(&res);
            }
            VectorCallbackType::MVVM(op) => {
                let mut res = get_bit_in_slice(&rvv_data.res_before, i) == 1;
                op(
                    rvv_data.get_left().as_slice(),
                    rvv_data.get_right().as_slice(),
                    &mut res,
                    rvv_data.get_mask(),
                );
                set_bit_in_slice(&mut rvv_data.res_exp, i, res as u8);
            }
            VectorCallbackType::MVXM(op) => {
                let mut res = get_bit_in_slice(&rvv_data.res_before, i) == 1;
                op(
                    rvv_data.get_left().as_slice(),
                    rvv_data.get_right_u64(),
                    &mut res,
                    rvv_data.get_mask(),
                );
                set_bit_in_slice(&mut rvv_data.res_exp, i, res as u8);
            }
            VectorCallbackType::MVIM(op) => {
                let mut res = get_bit_in_slice(&rvv_data.res_before, i) == 1;
                op(
                    rvv_data.get_left().as_slice(),
                    rvv_data.get_right_u64() as i64,
                    &mut res,
                    rvv_data.get_mask(),
                );
                set_bit_in_slice(&mut rvv_data.res_exp, i, res as u8);
            }
            VectorCallbackType::MVV(op) => {
                let mut res = get_bit_in_slice(&rvv_data.res_before, i) == 1;
                op(
                    rvv_data.get_left().as_slice(),
                    rvv_data.get_right().as_slice(),
                    &mut res,
                );
                set_bit_in_slice(&mut rvv_data.res_exp, i, res as u8);
            }
            VectorCallbackType::MVX(op) => {
                let mut res = get_bit_in_slice(&rvv_data.res_before, i) == 1;
                op(
                    rvv_data.get_left().as_slice(),
                    rvv_data.get_right_u64(),
                    &mut res,
                );
                set_bit_in_slice(&mut rvv_data.res_exp, i, res as u8);
            }
            VectorCallbackType::MVI(op) => {
                let mut res = get_bit_in_slice(&rvv_data.res_before, i) == 1;
                op(
                    rvv_data.get_left().as_slice(),
                    rvv_data.get_right_u64() as i64,
                    &mut res,
                );
                set_bit_in_slice(&mut rvv_data.res_exp, i, res as u8);
            }
            VectorCallbackType::MMM(op) => {
                let mut res = get_bit_in_slice(&rvv_data.res_before, i) == 1;
                let lhs = get_bit_in_slice(&rvv_data.lhs, rvv_data.index) == 1;
                let rhs = get_bit_in_slice(&rvv_data.rhs, rvv_data.index) == 1;

                op(lhs, rhs, &mut res);
                set_bit_in_slice(&mut rvv_data.res_exp, i, res as u8);
            }
            VectorCallbackType::VVR(op) => {
                let mut res = rvv_data.get_result_exp();
                let index = rvv_data.get_rvv_index();
                op(
                    rvv_data.get_left().as_slice(),
                    rvv_data.get_right().as_slice(),
                    res.as_mut_slice(),
                    index,
                );
                rvv_data.set_result_exp(&res);
            }
        }
    }

    run_rvv_op(rvv_data, rvv_op);

    if rvv_data.res_exp != rvv_data.res_rvv {
        for i in 0..rvv_data.avl as usize {
            rvv_data.index = i;
            rvv_data.count = i / rvv_data.theoretically_vl as usize;
            let exp = rvv_data.get_result_exp();
            let res = rvv_data.get_result_rvv();
            let exp_befor = rvv_data.get_result_befor();

            if exp != res {
                log!(
                "[sew = {}, describe = {}] unexpected values found at index {} \nresult = {:0>2X?} \nexpected = {:0>2X?}",
                rvv_data.sew, desc, i, res, exp
            );
                log!(
                "more information, \nlhs = {:0>2X?} \nrhs = {:0>2X?} \nexpected_before = {:0>2X?}",
                rvv_data.get_left(),
                rvv_data.get_right(),
                exp_befor
            );

                log!(
                    "-lmul = {}, avl = {}, vl = {}, mask = {}",
                    rvv_data.lmul,
                    rvv_data.avl,
                    rvv_data.theoretically_vl,
                    rvv_data.mask_type
                );

                log!("-expected: {:0>2X?}", rvv_data.res_exp);
                log!("-result: {:0>2X?}", rvv_data.res_rvv);
                log!("-res_before: {:0>2X?}", rvv_data.res_before);
                if rvv_data.mask_type != MaskType::Disable {
                    log!("-mask = {:0>2X?}", &rvv_data.mask);
                }
                log!("-lhs: {:0>2X?}", rvv_data.lhs);
                log!("-rhs: {:0>2X?}", rvv_data.rhs);
                panic!("Abort");
            }
        }
    }
    if is_verbose() {
        log!("finished");
    }
}

fn run_template_ext(
    vd_type: InstructionArgsType,
    left_type: InstructionArgsType,
    right_type: InstructionArgsType,
    mask_type: MaskType,
    rvv_op: fn(&[u8], &[u8], MaskType),
    exp_op: VectorCallbackType,
    before_op: fn(f64, f64, u64) -> bool,
    masked_op: fn(&mut RVVTestData),
    desc: &str,
) {
    fn get_imm_begin(l: InstructionArgsType, r: InstructionArgsType) -> i64 {
        if l == InstructionArgsType::Immediate || r == InstructionArgsType::Immediate {
            -16
        } else if l == InstructionArgsType::UImmediate || r == InstructionArgsType::UImmediate {
            0
        } else {
            0
        }
    }

    let mut enable_mask = true;
    let imm_begin = get_imm_begin(left_type, right_type);
    let mut imm = imm_begin;

    let sews = &[64, 256];
    let lmuls = &[-8, -4, -2, 1, 2, 4, 8];

    for sew in sews {
        for lmul in lmuls {
            for avl in avl_iterator(sew.clone(), *lmul, 2) {
                if vsetvl(avl, *sew, *lmul) == 0 {
                    continue;
                }

                if !before_op.clone()(*sew as f64, RVVTestData::get_lmul(*lmul), avl) {
                    continue;
                }

                let e_mask_type = if mask_type == MaskType::Enable {
                    enable_mask = !enable_mask;
                    if !enable_mask {
                        MaskType::Enable
                    } else {
                        MaskType::Disable
                    }
                } else {
                    mask_type
                };

                let mut rvv_data = RVVTestData::new(
                    left_type,
                    right_type,
                    vd_type,
                    e_mask_type,
                    sew.clone(),
                    lmul.clone(),
                    avl,
                );
                rvv_data.rng_fill();
                if left_type.is_imm() {
                    rvv_data.lhs.copy_from_slice(&imm.to_le_bytes());
                }
                if right_type.is_imm() {
                    rvv_data.rhs.copy_from_slice(&imm.to_le_bytes());
                }

                run_op(&mut rvv_data, rvv_op, exp_op, masked_op, desc);

                if left_type.is_imm() || right_type.is_imm() {
                    imm += 1;
                    if imm == imm_begin + 32 {
                        imm = imm_begin
                    }
                }
            }
        }
    }
    //log!("{}", desc);
}

fn befor_op_default(_: f64, _: f64, _: u64) -> bool {
    true
}

fn befor_op_wide(sew: f64, lmul: f64, _: u64) -> bool {
    if sew * 2.0 > 1024.0 {
        return false;
    }
    let emul = lmul * 2.0;
    if emul >= 0.125 && emul <= 8.0 {
        true
    } else {
        false
    }
}

fn masked_op_default(_: &mut RVVTestData) {}

fn masked_op_red(rvv_data: &mut RVVTestData) {
    let index = rvv_data.get_rvv_index();
    if index == 0 {
        let rhs = rvv_data.get_right();
        rvv_data.set_result_exp(&rhs);
    }
}

pub fn run_template(
    vd_type: InstructionArgsType,
    left_type: InstructionArgsType,
    right_type: InstructionArgsType,
    mask_type: MaskType,
    expected_op: fn(&mut RVVTestData),
    rvv_op: fn(&[u8], &[u8], MaskType),
    before_op: fn(f64, f64, u64) -> bool,
    desc: &str,
) {
    run_template_ext(
        vd_type,
        left_type,
        right_type,
        mask_type,
        rvv_op,
        VectorCallbackType::None(expected_op),
        before_op,
        masked_op_default,
        desc,
    );
}

pub fn run_template_v_vvm(
    expected_op: fn(&[u8], &[u8], &mut [u8], bool),
    rvv_op: fn(&[u8], &[u8], MaskType),
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        MaskType::AsParam,
        rvv_op,
        VectorCallbackType::VVM(expected_op),
        befor_op_default,
        masked_op_default,
        desc,
    );
}

pub fn run_template_v_vxm(
    expected_op: fn(&[u8], u64, &mut [u8], bool),
    rvv_op: fn(&[u8], &[u8], MaskType),
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        InstructionArgsType::Scalar,
        MaskType::AsParam,
        rvv_op,
        VectorCallbackType::VXM(expected_op),
        befor_op_default,
        masked_op_default,
        desc,
    );
}

pub fn run_template_v_vim(
    expected_op: fn(&[u8], i64, &mut [u8], bool),
    rvv_op: fn(&[u8], &[u8], MaskType),
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        InstructionArgsType::Immediate,
        MaskType::AsParam,
        rvv_op,
        VectorCallbackType::VIM(expected_op),
        befor_op_default,
        masked_op_default,
        desc,
    );
}

pub fn run_template_m_vvm(
    expected_op: fn(&[u8], &[u8], &mut bool, bool),
    rvv_op: fn(&[u8], &[u8], MaskType),
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::VectorBit,
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        MaskType::AsParam,
        rvv_op,
        VectorCallbackType::MVVM(expected_op),
        befor_op_default,
        masked_op_default,
        desc,
    );
}

pub fn run_template_m_vxm(
    expected_op: fn(&[u8], u64, &mut bool, bool),
    rvv_op: fn(&[u8], &[u8], MaskType),
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::VectorBit,
        InstructionArgsType::Vector,
        InstructionArgsType::Scalar,
        MaskType::AsParam,
        rvv_op,
        VectorCallbackType::MVXM(expected_op),
        befor_op_default,
        masked_op_default,
        desc,
    );
}

pub fn run_template_m_vim(
    expected_op: fn(&[u8], i64, &mut bool, bool),
    rvv_op: fn(&[u8], &[u8], MaskType),
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::VectorBit,
        InstructionArgsType::Vector,
        InstructionArgsType::Immediate,
        MaskType::AsParam,
        rvv_op,
        VectorCallbackType::MVIM(expected_op),
        befor_op_default,
        masked_op_default,
        desc,
    );
}

pub fn run_template_m_vv(
    expected_op: fn(&[u8], &[u8], &mut bool),
    rvv_op: fn(&[u8], &[u8], MaskType),
    enable_mask: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::VectorBit,
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::MVV(expected_op),
        befor_op_default,
        masked_op_default,
        desc,
    );
}

pub fn run_template_m_vx(
    expected_op: fn(&[u8], u64, &mut bool),
    rvv_op: fn(&[u8], &[u8], MaskType),
    enable_mask: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::VectorBit,
        InstructionArgsType::Vector,
        InstructionArgsType::Scalar,
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::MVX(expected_op),
        befor_op_default,
        masked_op_default,
        desc,
    );
}

pub fn run_template_m_vi(
    expected_op: fn(&[u8], i64, &mut bool),
    rvv_op: fn(&[u8], &[u8], MaskType),
    enable_mask: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::VectorBit,
        InstructionArgsType::Vector,
        InstructionArgsType::Immediate,
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::MVI(expected_op),
        befor_op_default,
        masked_op_default,
        desc,
    );
}

pub fn run_template_m_mm(
    expected_op: fn(bool, bool, &mut bool),
    rvv_op: fn(&[u8], &[u8], MaskType),
    enable_mask: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::VectorBit,
        InstructionArgsType::VectorBit,
        InstructionArgsType::VectorBit,
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::MMM(expected_op),
        befor_op_default,
        masked_op_default,
        desc,
    );
}

pub fn run_template_w_vv(
    expected_op: fn(&[u8], &[u8], &mut [u8]),
    rvv_op: fn(&[u8], &[u8], MaskType),
    enable_mask: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector2,
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::VV(expected_op),
        befor_op_wide,
        masked_op_default,
        desc,
    );
}

pub fn run_template_w_vx(
    expected_op: fn(&[u8], u64, &mut [u8]),
    rvv_op: fn(&[u8], &[u8], MaskType),
    enable_mask: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector2,
        InstructionArgsType::Vector,
        InstructionArgsType::Scalar,
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::VX(expected_op),
        befor_op_wide,
        masked_op_default,
        desc,
    );
}

pub fn run_template_v_wv(
    expected_op: fn(&[u8], &[u8], &mut [u8]),
    rvv_op: fn(&[u8], &[u8], MaskType),
    enable_mask: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector2,
        InstructionArgsType::Vector,
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::VV(expected_op),
        befor_op_wide,
        masked_op_default,
        desc,
    );
}

pub fn run_template_v_wx(
    expected_op: fn(&[u8], u64, &mut [u8]),
    rvv_op: fn(&[u8], &[u8], MaskType),
    enable_mask: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector2,
        InstructionArgsType::Scalar,
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::VX(expected_op),
        befor_op_wide,
        masked_op_default,
        desc,
    );
}

pub fn run_template_v_wi(
    expected_op: fn(&[u8], i64, &mut [u8]),
    rvv_op: fn(&[u8], &[u8], MaskType),
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector2,
        InstructionArgsType::UImmediate,
        MaskType::Enable,
        rvv_op,
        VectorCallbackType::VI(expected_op),
        befor_op_wide,
        masked_op_default,
        desc,
    );
}

pub fn run_template_v_vv(
    expected_op: fn(&[u8], &[u8], &mut [u8]),
    rvv_op: fn(&[u8], &[u8], MaskType),
    enable_mask: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::VV(expected_op),
        befor_op_default,
        masked_op_default,
        desc,
    );
}

pub fn run_template_v_vx(
    expected_op: fn(&[u8], u64, &mut [u8]),
    rvv_op: fn(&[u8], &[u8], MaskType),
    enable_mask: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        InstructionArgsType::Scalar,
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::VX(expected_op),
        befor_op_default,
        masked_op_default,
        desc,
    );
}

pub fn run_template_v_vi(
    expected_op: fn(&[u8], i64, &mut [u8]),
    rvv_op: fn(&[u8], &[u8], MaskType),
    enable_mask: bool,
    single: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        if single {
            InstructionArgsType::Immediate
        } else {
            InstructionArgsType::UImmediate
        },
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::VI(expected_op),
        befor_op_default,
        masked_op_default,
        desc,
    );
}

pub fn run_template_w_wv(
    expected_op: fn(&[u8], &[u8], &mut [u8]),
    rvv_op: fn(&[u8], &[u8], MaskType),
    enable_mask: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector2,
        InstructionArgsType::Vector2,
        InstructionArgsType::Vector,
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::VV(expected_op),
        befor_op_wide,
        masked_op_default,
        desc,
    );
}

pub fn run_template_w_wx(
    expected_op: fn(&[u8], u64, &mut [u8]),
    rvv_op: fn(&[u8], &[u8], MaskType),
    enable_mask: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::Vector2,
        InstructionArgsType::Vector2,
        InstructionArgsType::Scalar,
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::VX(expected_op),
        befor_op_wide,
        masked_op_default,
        desc,
    );
}

pub fn run_template_r_vv(
    expected_op: fn(&[u8], &[u8], &mut [u8], usize),
    rvv_op: fn(&[u8], &[u8], MaskType),
    enable_mask: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::VectorRed,
        InstructionArgsType::Vector,
        InstructionArgsType::Vector,
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::VVR(expected_op),
        befor_op_default,
        masked_op_red,
        desc,
    );
}

pub fn run_template_wr_vw(
    expected_op: fn(&[u8], &[u8], &mut [u8], usize),
    rvv_op: fn(&[u8], &[u8], MaskType),
    enable_mask: bool,
    desc: &str,
) {
    run_template_ext(
        InstructionArgsType::VectorRed2,
        InstructionArgsType::Vector,
        InstructionArgsType::Vector2,
        if enable_mask {
            MaskType::Enable
        } else {
            MaskType::Disable
        },
        rvv_op,
        VectorCallbackType::VVR(expected_op),
        befor_op_wide,
        masked_op_red,
        desc,
    );
}

pub fn run_template_v_n(
    expected_op: fn(&[u8], &[u8], &mut [u8]),
    rvv_op: fn(&[u8], &[u8], MaskType),
    narrow: u64,
    enable_mask: bool,
    desc: &str,
) {
    fn before_op_2(_: f64, lmul: f64, _: u64) -> bool {
        let v = 0.5;
        let emul = lmul * v;
        if emul >= 0.125 && emul <= 8.0 {
            true
        } else {
            false
        }
    }
    fn before_op_4(_: f64, lmul: f64, _: u64) -> bool {
        let v = 0.25;
        let emul = lmul * v;
        if emul >= 0.125 && emul <= 8.0 {
            true
        } else {
            false
        }
    }
    fn before_op_8(_: f64, lmul: f64, _: u64) -> bool {
        let v = 0.125;
        let emul = lmul * v;
        if emul >= 0.125 && emul <= 8.0 {
            true
        } else {
            false
        }
    }
    match narrow {
        2 => run_template_ext(
            InstructionArgsType::Vector,
            InstructionArgsType::VectorNarrow2,
            InstructionArgsType::Vector,
            if enable_mask {
                MaskType::Enable
            } else {
                MaskType::Disable
            },
            rvv_op,
            VectorCallbackType::VV(expected_op),
            before_op_2,
            masked_op_default,
            desc,
        ),
        4 => run_template_ext(
            InstructionArgsType::Vector,
            InstructionArgsType::VectorNarrow4,
            InstructionArgsType::Vector,
            if enable_mask {
                MaskType::Enable
            } else {
                MaskType::Disable
            },
            rvv_op,
            VectorCallbackType::VV(expected_op),
            before_op_4,
            masked_op_default,
            desc,
        ),
        8 => run_template_ext(
            InstructionArgsType::Vector,
            InstructionArgsType::VectorNarrow8,
            InstructionArgsType::Vector,
            if enable_mask {
                MaskType::Enable
            } else {
                MaskType::Disable
            },
            rvv_op,
            VectorCallbackType::VV(expected_op),
            before_op_8,
            masked_op_default,
            desc,
        ),
        _ => panic!("Abort"),
    };
}
