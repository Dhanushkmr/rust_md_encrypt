# Rust Markdown encrypt

A CLI tool written with CLAP for easy encrypting/decrypting .md files in a folder. Meant to be used to lock/unlock Obsidian vaults.

## Usage

1. Clone this repo
`git clone https://github.com/Dhanushkmr/rust_md_encrypt/`

2. Open the repo
`cd rust_md_encrypt`

3. Build it!
`cargo build --release`

4. Add to your binaries
`cargo install --path .`

5. Go to your obsidian Vault
`cd <PATH-TO-OBSIDIAN-VAULT>`

6. To encrypt:
`rust_md_encrypt <KEY> encrypt`

!!! Do not forget your key (support for environment variable coming soon!) !!!

7. To decrypt:
`rust_md_encrypt <KEY> decrypt`


