use std::fs;
use std::path::PathBuf;
use hex;

use crate::cmdline::Input;

pub struct Parse {
    pub key: Vec<u8>,
    pub content: Vec<u8>,
}

pub enum UtilsErr {
    KeyNotHex,
    KeyLengthErr,
    ReadFileErr,
    WriteFileErr,
}

impl Parse {
    pub fn parse(input: &Input) -> Result<Parse, UtilsErr> {
        let key;
        match hex::decode(&input.key) {
            Ok(k) => {
                key = k;
            },
            Err(_) => {
                return Err(UtilsErr::KeyNotHex);
            }
        }
        if key.len() != 16 {
            return Err(UtilsErr::KeyLengthErr);
        }

        let content;
        match fs::read(&input.filein) {
            Ok(c) => {
                content = c;
            },
            Err(_) => {
                return Err(UtilsErr::ReadFileErr);
            }
        }

        Ok(Parse { key, content })
    }
}

pub fn handler_err(err: UtilsErr) {
    match err {
        UtilsErr::KeyNotHex => {
            println!("The key must be hex format");
        },
        UtilsErr::KeyLengthErr => {
            println!("The key must be 128-bit also known as 32 hex numbers");
        },
        UtilsErr::ReadFileErr => {
            println!("The input file does not exist or cannot be read");
        },
        UtilsErr::WriteFileErr => {
            println!("The output file cannot be create or write");
        }
    }
}

pub fn save_result(filename: &PathBuf, result: &Vec<u8>) -> Result<(), UtilsErr> {
    match fs::write(filename, result) {
        Ok(_) => Ok(()),
        Err(_) => Err(UtilsErr::WriteFileErr),
    }
}
