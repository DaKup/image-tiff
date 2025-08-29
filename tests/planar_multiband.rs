use std::io::{Cursor, Seek, SeekFrom};

use tiff::decoder::{Decoder, DecodingResult};
use tiff::encoder::{colortype, Compression, TiffEncoder};
use tiff::ColorType;

#[test]
fn planar_multiband_roundtrip() {
    const WIDTH: u32 = 3;
    const HEIGHT: u32 = 2;
    const BANDS: usize = 5;

    let mut data = Vec::new();
    for band in 0..BANDS {
        for i in 0..(WIDTH * HEIGHT) as usize {
            data.push((band * 10 + i) as f64);
        }
    }

    let mut cursor = Cursor::new(Vec::new());
    {
        let mut tiff =
            TiffEncoder::new(&mut cursor).unwrap().with_compression(Compression::Lzw);
        tiff
            .write_image_planar::<colortype::MultiBandF64<BANDS>>(WIDTH, HEIGHT, &data)
            .unwrap();
    }

    cursor.seek(SeekFrom::Start(0)).unwrap();
    let mut decoder = Decoder::new(&mut cursor).unwrap();
    assert_eq!(
        decoder.colortype().unwrap(),
        ColorType::Multiband {
            bit_depth: 64,
            num_samples: BANDS as u16,
        }
    );
    let mut expected = Vec::new();
    for i in 0..(WIDTH * HEIGHT) as usize {
        for band in 0..BANDS {
            expected.push((band * 10 + i) as f64);
        }
    }
    match decoder.read_image().unwrap() {
        DecodingResult::F64(buf) => assert_eq!(buf, expected),
        r => panic!("unexpected result: {:?}", r),
    }
}
