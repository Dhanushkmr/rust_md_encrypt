use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "Markdown file locker", version, author)]
pub struct CliArgs {
    // pub key: String,
    #[arg(value_enum)]
    pub mode: Mode,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    #[clap(name = "encrypt", alias = "e")]
    Encrypt,
    #[clap(name = "decrypt", alias = "d")]
    Decrypt,
}

impl Mode {
    // std::fmt::Display trait is used to convert the mode to a string
    pub fn to_string(&self) -> String {
        match self {
            Mode::Encrypt => "Encrypting",
            Mode::Decrypt => "Decrypting",
        }
        .to_string()
    }
}
