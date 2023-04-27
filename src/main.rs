use rust_md_encrypt as helpers;
mod args;
use args::{CliArgs, Mode};
use clap::Parser;

fn main() {
    let args = CliArgs::parse();
    let mc = helpers::make_mc(args.key);
    let md_files = helpers::list_md_files();
    for file in md_files {
        // encrypt or decrypt
        let contents = helpers::read_file(&file);
        match args.mode {
            Mode::Encrypt => {
                let encrypted = helpers::encrypt(contents, &mc);
                println!("{}", &encrypted);
                helpers::overwrite_file(&file, encrypted);
            }
            Mode::Decrypt => {
                let decrypted = helpers::decrypt(contents, &mc).unwrap();
                println!("{}", &decrypted);
                helpers::overwrite_file(&file, decrypted);
            }
        }
    }
}
