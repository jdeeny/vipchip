use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub enum LoaderType {
    Auto,
    Hex,
    Binary,
}

pub trait Loader {
    fn get_bytes(&mut self) -> Vec<u8>;
}

pub struct BinaryLoader {
    file: File,
}
impl BinaryLoader {
    fn new(file: File) -> Box<Loader> {
        Box::new(BinaryLoader {
            file: file,
        })
    }
}
impl Loader for BinaryLoader {
    fn get_bytes(&mut self) -> Vec<u8> {
        let mut program = Vec::new();
        self.file.read_to_end(&mut program);
        program
    }
}

pub struct HexLoader {
    file: File,
}
impl HexLoader {
    fn new(mut file: File) -> Box<Loader> {
        Box::new(HexLoader {
            file: file,
        })
    }
}
impl Loader for HexLoader {
    fn get_bytes(&mut self) -> Vec<u8> {
        let mut program = Vec::new();
        self.file.read_to_end(&mut program);
        program
    }
}

pub fn load_file(path: &str, loader_type: LoaderType) -> Vec<u8> {
    let file = File::open(path).unwrap();
    match loader_type {
        LoaderType::Auto => load_autodetect(file).get_bytes(),
        LoaderType::Hex => HexLoader::new(file).get_bytes(),
        LoaderType::Binary => BinaryLoader::new(file).get_bytes(),
    }
}

pub fn load_autodetect(mut file: File) -> Box<Loader> {
    let hex_chars: Vec<u8> = "0123456789abcdefABCDEFxX[], \r\n\t".bytes().collect();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);
    let mut binary_data = false;
    for b in data {
        if hex_chars.contains(&b) { binary_data = true; }
    }
    file.seek(SeekFrom::Start(0));
    if binary_data {
        BinaryLoader::new(file)
    } else {
        HexLoader::new(file)
    }
}
