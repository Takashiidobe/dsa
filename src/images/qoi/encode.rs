use super::*;
use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EncodeError {
    NotEnoughPixelData,
    OutputIsTooSmall,
}

impl Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncodeError::NotEnoughPixelData => f.write_str("Pixels buffer is too small for image"),
            EncodeError::OutputIsTooSmall => {
                f.write_str("Output buffer is too small to fit encoded image")
            }
        }
    }
}

impl std::error::Error for EncodeError {}

impl Qoi {
    #[inline]
    pub fn encode(&self, pixels: &[u8], output: &mut [u8]) -> Result<usize, EncodeError> {
        if output.len() <= HEADER_SIZE {
            return Err(EncodeError::OutputIsTooSmall);
        }

        output[0..4].copy_from_slice(&MAGIC.to_be_bytes());
        output[4..8].copy_from_slice(&self.width.to_be_bytes());
        output[8..12].copy_from_slice(&self.height.to_be_bytes());

        let channels = match self.colors {
            Colors::Rgb => {
                output[12] = 3;
                output[13] = 1;
                3
            }
            Colors::Rgba => {
                output[12] = 4;
                output[13] = 1;
                4
            }
            Colors::Srgb => {
                output[12] = 3;
                output[13] = 0;
                3
            }
            Colors::SrgbLinA => {
                output[12] = 4;
                output[13] = 0;
                4
            }
        };

        let px_len = self.width as usize * self.height as usize * channels;

        let pixels = match pixels.get(..px_len) {
            None => {
                return Err(EncodeError::NotEnoughPixelData);
            }
            Some(pixels) => pixels,
        };

        let size = match self.colors.has_alpha() {
            true => Self::encode_range::<4>(
                &mut [[0; 4]; 64],
                &mut Pixel::new_opaque(),
                &mut 0,
                pixels,
                &mut output[HEADER_SIZE..],
            )?,
            false => Self::encode_range::<3>(
                &mut [[0; 4]; 64],
                &mut Pixel::new_opaque(),
                &mut 0,
                pixels,
                &mut output[HEADER_SIZE..],
            )?,
        };

        if output.len() < size + PADDING + HEADER_SIZE {
            return Err(EncodeError::OutputIsTooSmall);
        }

        output[HEADER_SIZE + size..][..PADDING - 1].fill(0);
        output[HEADER_SIZE + size + PADDING - 1] = 1;

        Ok(size + PADDING + HEADER_SIZE)
    }

    #[inline]
    pub fn encode_range<const N: usize>(
        index: &mut [[u8; 4]; 64],
        px_prev: &mut [u8; N],
        run: &mut usize,
        pixels: &[u8],
        output: &mut [u8],
    ) -> Result<usize, EncodeError>
    where
        [u8; N]: Pixel,
    {
        let mut rest = &mut *output;

        assert_eq!(pixels.len() % N, 0);

        let mut pixels = bytemuck::cast_slice::<_, [u8; N]>(pixels);

        while let [px, tail @ ..] = pixels {
            pixels = tail;
            if rest.len() > 7 {
                if *px == *px_prev {
                    if *run == 61 || pixels.is_empty() {
                        rest[0] = OP_RUN | (*run as u8);
                        rest = &mut rest[1..];
                        *run = 0;
                    } else {
                        *run += 1;
                    }
                } else {
                    match run {
                        0 => {}
                        1 => {
                            let index_pos = px_prev.hash();
                            if index_pos == 0x35 && index[0x35] == [0; 4] {
                                rest[0] = OP_RUN;
                            } else {
                                rest[0] = OP_INDEX | index_pos;
                            }
                            rest = &mut rest[1..];
                            *run = 0;
                        }
                        _ => {
                            rest[0] = OP_RUN | (*run - 1) as u8;
                            rest = &mut rest[1..];
                            *run = 0;
                        }
                    }

                    match rest {
                        [b1, b2, b3, b4, b5, ..] => {
                            let index_pos = px.hash();

                            if index[index_pos as usize] == px.rgba() {
                                *b1 = OP_INDEX | index_pos;
                                rest = &mut rest[1..];
                            } else {
                                index[index_pos as usize] = px.rgba();

                                if N == 4 && px_prev.a() != px.a() {
                                    let [r, g, b, a] = px.rgba();
                                    *b1 = OP_RGBA;
                                    *b2 = r;
                                    *b3 = g;
                                    *b4 = b;
                                    *b5 = a;
                                    rest = &mut rest[5..];
                                } else {
                                    let v = px.var(px_prev);

                                    if let Some(diff) = v.diff() {
                                        *b1 = diff;
                                        rest = &mut rest[1..];
                                    } else if let Some([lu, ma]) = v.luma() {
                                        *b1 = lu;
                                        *b2 = ma;
                                        rest = &mut rest[2..];
                                    } else {
                                        let [r, g, b] = px.rgb();
                                        *b1 = OP_RGB;
                                        *b2 = r;
                                        *b3 = g;
                                        *b4 = b;
                                        rest = &mut rest[4..];
                                    }
                                }
                            }
                            *px_prev = *px;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
            } else {
                return Err(EncodeError::OutputIsTooSmall);
            }
        }

        let tail = rest.len();

        Ok(output.len() - tail)
    }

    #[inline]
    pub fn encoded_size_limit(&self) -> usize {
        self.width as usize * self.height as usize * (self.colors.has_alpha() as usize + 4)
            + HEADER_SIZE
            + PADDING
    }

    #[inline]
    pub fn encode_alloc(&self, pixels: &[u8]) -> Result<Vec<u8>, EncodeError> {
        let limit = self.encoded_size_limit();
        let mut output = vec![0; limit];
        match self.encode(pixels, &mut output) {
            Ok(size) => {
                output.truncate(size);
                Ok(output)
            }
            Err(EncodeError::OutputIsTooSmall) => unreachable!(),
            Err(err) => Err(err),
        }
    }
}
