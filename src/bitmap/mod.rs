// TODO: There are a lot of non-standard naming in this mod,
// if you are a patient developer, please help us to modify it
use super::document::Document;

// FIXME: Please modify all the identifiers which named with the Camel-Case rule
struct BitMapFileHeader {
    bfType: [i8;2],
    bfSize: u32,
    bfReserved1: u16,
    bfReserved2: u16,
    bfOffBits: u32,
}

impl BitMapFileHeader {
    fn new() -> Self {
        BitMapFileHeader {
            bfType: [0x4D, 0x42],
            bfSize: 0,
            bfReserved1: 0,
            bfReserved2: 0,
            bfOffBits: 0,
        }
    }
}

// FIXME: Please modify all the identifiers which named with the Camel-Case rule
struct BitMapInfoHeader {
    biSize: u32,
    biWidth: u32,
    biHeight: u32,
    biPlanes: u16,
    biBitCount: u16,
    biCompression: u32,
    biSizeImage: u32,
    biXPelsPerMeter: u32,
    biYPelsPerMeter: u32,
    biClrUsed: u32,
    biClrImportant: u32,
}

impl BitMapInfoHeader {
	fn new() -> Self {
		BitMapInfoHeader {
			biSize: 0,
			biWidth: 0,
			biHeight: 0,
			biPlanes: 0,
			biBitCount: 0,
			biCompression: 0,
			biSizeImage: 0,
			biXPelsPerMeter: 0,
			biYPelsPerMeter: 0,
			biClrUsed: 0,
			biClrImportant: 0,
		}
	}
}

// FIXME: Please modify all the identifiers which named with the Camel-Case rule
struct RgbQuad {
    rgbBlue: u8,
    rgbGreen: u8,
    rgbRed: u8,
    rgbReserved: u8,
}

pub struct Image {
    width: u32,
    height: u32,
    channels: u32,
	file_header: BitMapFileHeader,
	info_header: BitMapInfoHeader,
    image_data: Vec<u8>,
}

impl Image {
    pub fn new() -> Self {
        Image {
            width: 0,
            height: 0,
            channels: 1,
			file_header: BitMapFileHeader::new(),
			info_header: BitMapInfoHeader::new(),
            image_data: Vec::new(),
        }
    }

    pub fn load(&mut self, path: &str) -> bool {
        // TODO: Complete this function
        let mut bmp_file_header = BitMapFileHeader::new();
        let mut file: Document = Document::new(path);
        match file.read() {
            Err(e) => {println!("{}", e); return false;},
            _ => {},
        }
		let date = file.date();
		if press(&date[0..2]) != 0x4D42 {
			println!("Not BitMap file");
			return false;
		}
		bmp_file_header.bfSize = press(&date[2..7]) as u32;
		bmp_file_header.bfReserved1 = press(&date[7..10]) as u16;
		bmp_file_header.bfReserved2 = press(&date[10..13]) as u16;
		bmp_file_header.bfOffBits = press(&date[13..18]) as u32;
		self.file_header = bmp_file_header;
        true
    }

    pub fn save(&mut self, path: &str) -> bool {
        // TODO: Complete this function
        true
    }

    pub fn width(&self) -> u32 {
        self.width 
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn channels(&self) -> u32 {
        self.channels
    }
}

fn press(bit: &[u8]) -> usize {
    let mut foo: usize = 0;
    for i in bit {
        foo *= 256;
        foo += *i as usize;
    }
    foo
}