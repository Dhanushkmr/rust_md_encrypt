use vault as helpers;
mod args;
use args::{CliArgs, Mode};
use clap::Parser;

fn main() {
    let args = CliArgs::parse();
    let mc = helpers::make_mc(args.key);
    let md_files = helpers::list_md_files().unwrap();
    for file in md_files {
        // encrypt or decrypt

        println!("{} ... {}", args.mode.to_string(), &file);
        let contents = helpers::read_file(&file);
        match args.mode {
            Mode::Encrypt => {
                let encrypted = helpers::encrypt(contents, &mc);
                helpers::overwrite_file(&file, encrypted);
            }
            Mode::Decrypt => {
                let decrypted = helpers::decrypt(contents, &mc).unwrap();
                helpers::overwrite_file(&file, decrypted);
            }
        }
    }
}
