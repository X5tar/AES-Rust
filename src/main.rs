mod cmdline;
mod utils;

use structopt::StructOpt;
use aes::AES;

fn main() {
    let opt: cmdline::Opt = cmdline::Opt::from_args();

    match opt.method {
        cmdline::Method::Enc(input) => {
            let parse = utils::Parse::parse(&input);
            match parse {
                Ok(p) => {
                    let aes = AES::new(&p.key);
                    let result = aes.encrypt(&p.content);
                    if let Err(e) = utils::save_result(&input.fileout, &result) {
                        utils::handler_err(e);
                    }
                },
                Err(e) => {
                    utils::handler_err(e);
                }
            };
        },
        cmdline::Method::Dec(input) => {
            let parse = utils::Parse::parse(&input);
            match parse {
                Ok(p) => {
                    let aes = AES::new(&p.key);
                    let result = aes.decrypt(&p.content);
                    if let Err(e) = utils::save_result(&input.fileout, &result) {
                        utils::handler_err(e);
                    }
                },
                Err(e) => {
                    utils::handler_err(e);
                }
            };
        }
    }
}
