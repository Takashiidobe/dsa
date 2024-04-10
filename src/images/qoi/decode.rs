use super::*;
use std::fmt::{self, Display};

use super::Pixel;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DecodeError {
    NotEnoughData,
    InvalidMagic,
    InvalidChannelsValue,
    InvalidColorSpaceValue,
    OutputIsTooSmall,
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::NotEnoughData => {
                f.write_str("Buffer does not contain enough encoded data")
            }
            DecodeError::InvalidMagic => f.write_str("Encoded header contains invalid magic value"),
            DecodeError::InvalidChannelsValue => {
                f.write_str("Encoded header contains invalud channels number. Must be 3 or 4")
            }
            DecodeError::InvalidColorSpaceValue => {
                f.write_str("Encoded header contains invalud color space value. Must be 0 or 1")
            }
            DecodeError::OutputIsTooSmall => {
                f.write_str("Output buffer is too small to fit decoded image")
            }
        }
    }
}

impl std::error::Error for DecodeError {}

impl Qoi {
    #[inline]
    pub fn decoded_size(&self) -> usize {
        self.width as usize * self.height as usize * self.colors.channels()
    }

    pub fn decode_header(bytes: &[u8]) -> Result<Self, DecodeError> {
        if bytes.len() < HEADER_SIZE {
            return Err(DecodeError::NotEnoughData);
        }

        let magic = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        if magic != MAGIC {
            return Err(DecodeError::InvalidMagic);
        }

        let w = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
        let h = u32::from_be_bytes(bytes[8..12].try_into().unwrap());

        let channels = bytes[12];
        let colors = bytes[13];

        Ok(Qoi {
            width: w,
            height: h,
            colors: match (channels, colors) {
                (3, 0) => Colors::Srgb,
                (4, 0) => Colors::SrgbLinA,
                (3, 1) => Colors::Rgb,
                (4, 1) => Colors::Rgba,
                (_, 0 | 1) => return Err(DecodeError::InvalidChannelsValue),
                (_, _) => return Err(DecodeError::InvalidColorSpaceValue),
            },
        })
    }

    #[inline]
    pub fn decode(bytes: &[u8], output: &mut [u8]) -> Result<Self, DecodeError> {
        let qoi = Self::decode_header(bytes)?;
        qoi.decode_skip_header(&bytes[HEADER_SIZE..], output)?;
        Ok(qoi)
    }

    #[inline]
    pub fn decode_skip_header(&self, bytes: &[u8], output: &mut [u8]) -> Result<(), DecodeError> {
        if self.width == 0 || self.height == 0 {
            return Ok(());
        }

        let px_len = self.decoded_size();

        let output = match output.get_mut(..px_len) {
            None => return Err(DecodeError::OutputIsTooSmall),
            Some(output) => output,
        };

        if self.colors.has_alpha() {
            Self::decode_range::<4>(
                &mut [Pixel::new(); 64],
                &mut Pixel::new_opaque(),
                &mut 0,
                bytes,
                output,
            )?;
        } else {
            Self::decode_range::<3>(
                &mut [Pixel::new(); 64],
                &mut Pixel::new_opaque(),
                &mut 0,
                bytes,
                output,
            )?;
        }
        Ok(())
    }

    #[inline]
    pub fn decode_range<const N: usize>(
        index: &mut [[u8; N]; 64],
        ppx: &mut [u8; N],
        prun: &mut usize,
        bytes: &[u8],
        pixels: &mut [u8],
    ) -> Result<usize, DecodeError>
    where
        [u8; N]: Pixel,
    {
        assert_eq!(pixels.len() % N, 0);

        let mut pixels = bytemuck::cast_slice_mut(pixels);

        let mut px = *ppx;

        if *prun > 0 {
            let (head, tail) = pixels.split_at_mut((*prun).min(pixels.len()));

            pixels = tail;
            head.fill(px);

            if pixels.is_empty() {
                *prun -= head.len();
                return Ok(0);
            } else {
                *prun = 0;
            }
        }

        let mut rest = bytes;

        while let [out, tail @ ..] = pixels {
            pixels = tail;
            match rest {
                [b1 @ 0b00000000..=0b00111111, tail @ ..] => {
                    px = index[*b1 as usize];
                    *out = px;

                    rest = tail;
                    continue;
                }
                [b1 @ 0b01000000..=0b01111111, tail @ ..] => {
                    let vr = ((b1 >> 4) & 0x03).wrapping_sub(2);
                    let vg = ((b1 >> 2) & 0x03).wrapping_sub(2);
                    let vb = (b1 & 0x03).wrapping_sub(2);
                    px.add_rgb([vr, vg, vb]);

                    rest = tail;
                }
                [b1 @ 0b10000000..=0b10111111, b2, tail @ ..] => {
                    let vg = (b1 & 0x3f).wrapping_sub(32);
                    let vr = ((b2 >> 4) & 0x0f).wrapping_sub(8).wrapping_add(vg);
                    let vb = (b2 & 0x0f).wrapping_sub(8).wrapping_add(vg);
                    px.add_rgb([vr, vg, vb]);

                    rest = tail;
                }
                [0b11111110, b2, b3, b4, tail @ ..] => {
                    px.set_rgb([*b2, *b3, *b4]);

                    rest = tail;
                }
                [0b11111111, b2, b3, b4, _, tail @ ..] if N == 3 => {
                    px.set_rgb([*b2, *b3, *b4]);

                    rest = tail;
                }
                [0b11111111, b2, b3, b4, b5, tail @ ..] => {
                    px.set_rgba([*b2, *b3, *b4, *b5]);
                    rest = tail;
                }
                [b1 @ 0b11000000..=0b11111101, dtail @ ..] => {
                    *out = px;
                    let run = *b1 as usize & 0x3f;
                    let (head, tail) = pixels.split_at_mut(run);
                    head.fill(px);
                    pixels = tail;
                    rest = dtail;

                    if pixels.is_empty() {
                        *prun = run - head.len();
                        break;
                    }

                    continue;
                }
                _ => {
                    return Err(DecodeError::NotEnoughData);
                }
            }

            index[px.hash() as usize] = px;

            *out = px;
        }

        *ppx = px;

        Ok(bytes.len() - rest.len())
    }

    #[inline]
    pub fn decode_alloc(bytes: &[u8]) -> Result<(Self, Vec<u8>), DecodeError> {
        let qoi = Self::decode_header(bytes)?;

        let size = qoi.decoded_size();
        let mut output = vec![0; size];
        let qoi = Self::decode(bytes, &mut output)?;
        Ok((qoi, output))
    }
}
