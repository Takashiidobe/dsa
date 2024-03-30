// Taken from https://github.com/zakarumych/rapid-qoi with minor changes
const OP_INDEX: u8 = 0x00;
const OP_DIFF: u8 = 0x40;
const OP_LUMA: u8 = 0x80;
const OP_RUN: u8 = 0xc0;
const OP_RGB: u8 = 0xfe;
const OP_RGBA: u8 = 0xff;

const MAGIC: u32 = u32::from_be_bytes(*b"qoif");
const HEADER_SIZE: usize = 14;
const PADDING: usize = 8;

mod decode;
mod encode;

pub use decode::DecodeError;
pub use encode::EncodeError;

pub trait Pixel: Copy + Eq {
    const HAS_ALPHA: bool;

    fn new() -> Self;
    fn new_opaque() -> Self;
    fn read(&mut self, bytes: &[u8]);
    fn write(&self, bytes: &mut [u8]);
    fn var(&self, prev: &Self) -> Var;
    fn a(&self) -> u8;
    fn rgb(&self) -> [u8; 3];
    fn rgba(&self) -> [u8; 4];
    fn set_rgb(&mut self, other: [u8; 3]);
    fn set_rgba(&mut self, other: [u8; 4]);
    fn add_rgb(&mut self, other: [u8; 3]);
    fn hash(&self) -> u8;
}

impl Pixel for [u8; 3] {
    const HAS_ALPHA: bool = false;

    fn new() -> Self {
        [0; 3]
    }

    fn new_opaque() -> Self {
        Self::new()
    }

    fn read(&mut self, bytes: &[u8]) {
        self.copy_from_slice(bytes);
    }

    fn write(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(self)
    }

    fn var(&self, prev: &Self) -> Var {
        let [curr_r, curr_g, curr_b] = *self;
        let [other_r, other_g, other_b] = *prev;
        let r = curr_r.wrapping_sub(other_r);
        let g = curr_g.wrapping_sub(other_g);
        let b = curr_b.wrapping_sub(other_b);

        Var { r, g, b }
    }

    fn rgb(&self) -> [u8; 3] {
        *self
    }

    fn rgba(&self) -> [u8; 4] {
        let [r, g, b] = *self;
        [r, g, b, 0xff]
    }

    fn set_rgb(&mut self, other: [u8; 3]) {
        let [r, g, b] = other;
        self[0] = r;
        self[1] = g;
        self[2] = b;
    }

    fn set_rgba(&mut self, other: [u8; 4]) {
        let [r, g, b, _] = other;
        self[0] = r;
        self[1] = g;
        self[2] = b;
    }

    fn add_rgb(&mut self, other: [u8; 3]) {
        let [curr_r, curr_g, curr_b] = *self;
        let [r, g, b] = other;
        self[0] = curr_r.wrapping_add(r);
        self[1] = curr_g.wrapping_add(g);
        self[2] = curr_b.wrapping_add(b);
    }

    #[inline]
    fn hash(&self) -> u8 {
        let [r, g, b] = *self;
        let v = u32::from_ne_bytes([r, g, b, 0xff]);
        let s = (((v as u64) << 32) | (v as u64)) & 0xFF00FF0000FF00FF;

        s.wrapping_mul(0x030007000005000Bu64.to_le()).swap_bytes() as u8 & 63
    }

    fn a(&self) -> u8 {
        0xff
    }
}

impl Pixel for [u8; 4] {
    const HAS_ALPHA: bool = true;

    fn new() -> Self {
        [0; 4]
    }

    fn new_opaque() -> Self {
        [0, 0, 0, 0xff]
    }

    fn read(&mut self, bytes: &[u8]) {
        match bytes.try_into() {
            Ok(rgba) => {
                *self = rgba;
            }
            _ => unreachable!(),
        }
    }

    fn write(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(self)
    }

    fn var(&self, prev: &Self) -> Var {
        let [r, g, b, _] = *self;
        let [pr, pg, pb, _] = *prev;

        let r = r.wrapping_sub(pr);
        let g = g.wrapping_sub(pg);
        let b = b.wrapping_sub(pb);

        Var { r, g, b }
    }

    fn a(&self) -> u8 {
        self[3]
    }

    fn rgb(&self) -> [u8; 3] {
        let [r, g, b, _] = *self;
        [r, g, b]
    }

    fn rgba(&self) -> [u8; 4] {
        *self
    }

    fn set_rgb(&mut self, other: [u8; 3]) {
        let [r, g, b] = other;
        *self = [r, g, b, self[3]]
    }

    fn set_rgba(&mut self, other: [u8; 4]) {
        *self = other;
    }

    fn add_rgb(&mut self, other: [u8; 3]) {
        let [curr_r, curr_g, curr_b, _] = *self;
        let [r, g, b] = other;
        self[0] = curr_r.wrapping_add(r);
        self[1] = curr_g.wrapping_add(g);
        self[2] = curr_b.wrapping_add(b);
    }

    fn hash(&self) -> u8 {
        let v = u32::from_ne_bytes(*self);
        let s = (((v as u64) << 32) | (v as u64)) & 0xFF00FF0000FF00FF;

        s.wrapping_mul(0x030007000005000Bu64.to_le()).swap_bytes() as u8 & 63
    }
}

#[repr(C)]
pub struct Var {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Var {
    #[inline]
    fn diff(&self) -> Option<u8> {
        let r = self.r.wrapping_add(2);
        let g = self.g.wrapping_add(2);
        let b = self.b.wrapping_add(2);

        match r | g | b {
            0x00..=0x03 => Some(OP_DIFF | (r << 4) | (g << 2) | b),
            _ => None,
        }
    }

    #[inline]
    fn luma(&self) -> Option<[u8; 2]> {
        let r = self.r.wrapping_add(8).wrapping_sub(self.g);
        let g = self.g.wrapping_add(32);
        let b = self.b.wrapping_add(8).wrapping_sub(self.g);

        match (r | b, g) {
            (0x00..=0x0F, 0x00..=0x3F) => Some([OP_LUMA | g, r << 4 | b]),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Colors {
    Srgb,
    SrgbLinA,
    Rgb,
    Rgba,
}

impl Colors {
    #[inline]
    pub const fn has_alpha(&self) -> bool {
        match self {
            Colors::Rgb | Colors::Srgb => false,
            Colors::Rgba | Colors::SrgbLinA => true,
        }
    }

    #[inline]
    pub const fn channels(&self) -> usize {
        match self {
            Colors::Rgb | Colors::Srgb => 3,
            Colors::Rgba | Colors::SrgbLinA => 4,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Qoi {
    pub width: u32,
    pub height: u32,
    pub colors: Colors,
}
