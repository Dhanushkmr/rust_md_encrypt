use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[clap(name = "Markdown file locker", version, author)]
pub struct CliArgs {
    pub key: String,
    #[arg(value_enum)]
    pub mode: Mode,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    #[clap(name = "encrypt")]
    Encrypt,
    #[clap(name = "decrypt")]
    Decrypt,
}
