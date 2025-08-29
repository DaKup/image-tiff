use crate::tags::{PhotometricInterpretation, SampleFormat};

macro_rules! integer_horizontal_predict {
    () => {
        fn horizontal_predict(row: &[Self::Inner], result: &mut Vec<Self::Inner>) {
            let sample_size = Self::SAMPLE_FORMAT.len();

            if row.len() < sample_size {
                debug_assert!(false);
                return;
            }

            let (start, rest) = row.split_at(sample_size);

            result.extend_from_slice(start);
            if result.capacity() - result.len() < rest.len() {
                return;
            }

            result.extend(
                row.into_iter()
                    .zip(rest)
                    .map(|(prev, current)| current.wrapping_sub(*prev)),
            );
        }
    };
}

/// Trait for different colortypes that can be encoded.
pub trait ColorType {
    /// The type of each sample of this colortype
    type Inner: super::TiffValue;
    /// The value of the tiff tag `PhotometricInterpretation`
    const TIFF_VALUE: PhotometricInterpretation;
    /// The value of the tiff tag `BitsPerSample`
    const BITS_PER_SAMPLE: &'static [u16];
    /// The value of the tiff tag `SampleFormat`
    const SAMPLE_FORMAT: &'static [SampleFormat];

    fn horizontal_predict(row: &[Self::Inner], result: &mut Vec<Self::Inner>);
}

pub struct Gray8;
impl ColorType for Gray8 {
    type Inner = u8;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::BlackIsZero;
    const BITS_PER_SAMPLE: &'static [u16] = &[8];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint];

    integer_horizontal_predict!();
}

pub struct GrayI8;
impl ColorType for GrayI8 {
    type Inner = i8;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::BlackIsZero;
    const BITS_PER_SAMPLE: &'static [u16] = &[8];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Int];

    integer_horizontal_predict!();
}

pub struct Gray16;
impl ColorType for Gray16 {
    type Inner = u16;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::BlackIsZero;
    const BITS_PER_SAMPLE: &'static [u16] = &[16];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint];

    integer_horizontal_predict!();
}

pub struct GrayI16;
impl ColorType for GrayI16 {
    type Inner = i16;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::BlackIsZero;
    const BITS_PER_SAMPLE: &'static [u16] = &[16];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Int];

    integer_horizontal_predict!();
}

pub struct Gray32;
impl ColorType for Gray32 {
    type Inner = u32;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::BlackIsZero;
    const BITS_PER_SAMPLE: &'static [u16] = &[32];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint];

    integer_horizontal_predict!();
}

pub struct GrayI32;
impl ColorType for GrayI32 {
    type Inner = i32;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::BlackIsZero;
    const BITS_PER_SAMPLE: &'static [u16] = &[32];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Int];

    integer_horizontal_predict!();
}

pub struct Gray32Float;
impl ColorType for Gray32Float {
    type Inner = f32;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::BlackIsZero;
    const BITS_PER_SAMPLE: &'static [u16] = &[32];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::IEEEFP];

    fn horizontal_predict(_: &[Self::Inner], _: &mut Vec<Self::Inner>) {
        unreachable!()
    }
}

pub struct Gray64;
impl ColorType for Gray64 {
    type Inner = u64;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::BlackIsZero;
    const BITS_PER_SAMPLE: &'static [u16] = &[64];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint];

    integer_horizontal_predict!();
}

pub struct GrayI64;
impl ColorType for GrayI64 {
    type Inner = i64;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::BlackIsZero;
    const BITS_PER_SAMPLE: &'static [u16] = &[64];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Int];

    integer_horizontal_predict!();
}

pub struct Gray64Float;
impl ColorType for Gray64Float {
    type Inner = f64;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::BlackIsZero;
    const BITS_PER_SAMPLE: &'static [u16] = &[64];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::IEEEFP];

    fn horizontal_predict(_: &[Self::Inner], _: &mut Vec<Self::Inner>) {
        unreachable!()
    }
}

pub struct RGB8;
impl ColorType for RGB8 {
    type Inner = u8;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::RGB;
    const BITS_PER_SAMPLE: &'static [u16] = &[8, 8, 8];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint; 3];

    integer_horizontal_predict!();
}

pub struct RGB16;
impl ColorType for RGB16 {
    type Inner = u16;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::RGB;
    const BITS_PER_SAMPLE: &'static [u16] = &[16, 16, 16];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint; 3];

    integer_horizontal_predict!();
}

pub struct RGB32;
impl ColorType for RGB32 {
    type Inner = u32;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::RGB;
    const BITS_PER_SAMPLE: &'static [u16] = &[32, 32, 32];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint; 3];

    integer_horizontal_predict!();
}

pub struct RGB32Float;
impl ColorType for RGB32Float {
    type Inner = f32;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::RGB;
    const BITS_PER_SAMPLE: &'static [u16] = &[32, 32, 32];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::IEEEFP; 3];
    fn horizontal_predict(_: &[Self::Inner], _: &mut Vec<Self::Inner>) {
        unreachable!()
    }
}

pub struct RGB64;
impl ColorType for RGB64 {
    type Inner = u64;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::RGB;
    const BITS_PER_SAMPLE: &'static [u16] = &[64, 64, 64];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint; 3];

    integer_horizontal_predict!();
}

pub struct RGB64Float;
impl ColorType for RGB64Float {
    type Inner = f64;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::RGB;
    const BITS_PER_SAMPLE: &'static [u16] = &[64, 64, 64];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::IEEEFP; 3];
    fn horizontal_predict(_: &[Self::Inner], _: &mut Vec<Self::Inner>) {
        unreachable!()
    }
}

pub struct RGBA8;
impl ColorType for RGBA8 {
    type Inner = u8;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::RGB;
    const BITS_PER_SAMPLE: &'static [u16] = &[8, 8, 8, 8];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint; 4];

    integer_horizontal_predict!();
}

pub struct RGBA16;
impl ColorType for RGBA16 {
    type Inner = u16;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::RGB;
    const BITS_PER_SAMPLE: &'static [u16] = &[16, 16, 16, 16];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint; 4];

    integer_horizontal_predict!();
}

pub struct RGBA32;
impl ColorType for RGBA32 {
    type Inner = u32;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::RGB;
    const BITS_PER_SAMPLE: &'static [u16] = &[32, 32, 32, 32];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint; 4];

    integer_horizontal_predict!();
}

pub struct RGBA32Float;
impl ColorType for RGBA32Float {
    type Inner = f32;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::RGB;
    const BITS_PER_SAMPLE: &'static [u16] = &[32, 32, 32, 32];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::IEEEFP; 4];
    fn horizontal_predict(_: &[Self::Inner], _: &mut Vec<Self::Inner>) {
        unreachable!()
    }
}

pub struct RGBA64;
impl ColorType for RGBA64 {
    type Inner = u64;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::RGB;
    const BITS_PER_SAMPLE: &'static [u16] = &[64, 64, 64, 64];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint; 4];

    integer_horizontal_predict!();
}

pub struct RGBA64Float;
impl ColorType for RGBA64Float {
    type Inner = f64;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::RGB;
    const BITS_PER_SAMPLE: &'static [u16] = &[64, 64, 64, 64];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::IEEEFP; 4];
    fn horizontal_predict(_: &[Self::Inner], _: &mut Vec<Self::Inner>) {
        unreachable!()
    }
}

pub struct CMYK8;
impl ColorType for CMYK8 {
    type Inner = u8;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::CMYK;
    const BITS_PER_SAMPLE: &'static [u16] = &[8, 8, 8, 8];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint; 4];

    integer_horizontal_predict!();
}

pub struct CMYK16;
impl ColorType for CMYK16 {
    type Inner = u16;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::CMYK;
    const BITS_PER_SAMPLE: &'static [u16] = &[16, 16, 16, 16];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint; 4];

    integer_horizontal_predict!();
}

pub struct CMYK32;
impl ColorType for CMYK32 {
    type Inner = u32;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::CMYK;
    const BITS_PER_SAMPLE: &'static [u16] = &[32, 32, 32, 32];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint; 4];

    integer_horizontal_predict!();
}

pub struct CMYK32Float;
impl ColorType for CMYK32Float {
    type Inner = f32;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::CMYK;
    const BITS_PER_SAMPLE: &'static [u16] = &[32, 32, 32, 32];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::IEEEFP; 4];

    fn horizontal_predict(_: &[Self::Inner], _: &mut Vec<Self::Inner>) {
        unreachable!()
    }
}

pub struct CMYK64;
impl ColorType for CMYK64 {
    type Inner = u64;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::CMYK;
    const BITS_PER_SAMPLE: &'static [u16] = &[64, 64, 64, 64];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint; 4];

    integer_horizontal_predict!();
}

pub struct CMYK64Float;
impl ColorType for CMYK64Float {
    type Inner = f64;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::CMYK;
    const BITS_PER_SAMPLE: &'static [u16] = &[64, 64, 64, 64];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::IEEEFP; 4];

    fn horizontal_predict(_: &[Self::Inner], _: &mut Vec<Self::Inner>) {
        unreachable!()
    }
}

pub struct YCbCr8;
impl ColorType for YCbCr8 {
    type Inner = u8;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::YCbCr;
    const BITS_PER_SAMPLE: &'static [u16] = &[8, 8, 8];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint; 3];

    integer_horizontal_predict!();
}

pub struct CMYKA8;
impl ColorType for CMYKA8 {
    type Inner = u8;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::CMYK;
    const BITS_PER_SAMPLE: &'static [u16] = &[8, 8, 8, 8, 8];
    const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::Uint; 5];

    integer_horizontal_predict!();
}

macro_rules! multiband_int_color {
    ($name:ident, $ty:ty, $sf:expr) => {
        pub struct $name<const N: usize>;
        impl<const N: usize> ColorType for $name<N> {
            type Inner = $ty;
            const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::BlackIsZero;
            const BITS_PER_SAMPLE: &'static [u16] = &[ (std::mem::size_of::<$ty>() as u16 * 8); N ];
            const SAMPLE_FORMAT: &'static [SampleFormat] = &[$sf; N];
            integer_horizontal_predict!();
        }
    };
}

macro_rules! multiband_float_color {
    ($name:ident, $ty:ty) => {
        pub struct $name<const N: usize>;
        impl<const N: usize> ColorType for $name<N> {
            type Inner = $ty;
            const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::BlackIsZero;
            const BITS_PER_SAMPLE: &'static [u16] = &[ (std::mem::size_of::<$ty>() as u16 * 8); N ];
            const SAMPLE_FORMAT: &'static [SampleFormat] = &[SampleFormat::IEEEFP; N];
            fn horizontal_predict(_: &[Self::Inner], _: &mut Vec<Self::Inner>) {
                unreachable!()
            }
        }
    };
}

multiband_int_color!(MultiBandU8, u8, SampleFormat::Uint);
multiband_int_color!(MultiBandI8, i8, SampleFormat::Int);
multiband_int_color!(MultiBandU16, u16, SampleFormat::Uint);
multiband_int_color!(MultiBandI16, i16, SampleFormat::Int);
multiband_int_color!(MultiBandU32, u32, SampleFormat::Uint);
multiband_int_color!(MultiBandI32, i32, SampleFormat::Int);
multiband_int_color!(MultiBandU64, u64, SampleFormat::Uint);
multiband_int_color!(MultiBandI64, i64, SampleFormat::Int);
multiband_float_color!(MultiBandF32, f32);
multiband_float_color!(MultiBandF64, f64);
