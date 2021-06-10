use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
name = "AES",
version = "1.0",
author = "X5tar",
about = "A simple AES encryption and decryption program (Currently only supports 128-bit AES)"
)]
pub struct Opt {
    #[structopt(subcommand, verbatim_doc_comment)]
    pub method: Method
}

#[derive(Debug, StructOpt)]
pub enum Method {
    /// Encryption method
    Enc(Input),
    /// Decryption method
    Dec(Input),
}

#[derive(Debug, StructOpt)]
pub struct Input {
    /// Key for encryption and decryption (128-bit hex format)
    #[structopt(short, long)]
    pub key: String,

    /// Input file
    #[structopt(short = "i", long = "in")]
    pub filein: PathBuf,

    /// Output file
    #[structopt(short = "o", long = "out")]
    pub fileout: PathBuf,
}