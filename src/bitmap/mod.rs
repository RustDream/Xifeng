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
    image_data: Vec<u8>,
}

impl Image {
    pub fn new() -> Self {
        Image {
            width: 0,
            height: 0,
            channels: 1,
            image_data: Vec::new(),
        }
    }

    pub fn load(&mut self, path: &str) -> bool {
        // TODO: Complete this function
        let mut bmp_file_header = BitMapFileHeader::new();
        let mut file: Document = Document::new(path);
        match file.read() {
            Err(e) => println!("{}", e),
            _ => {},
        }
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