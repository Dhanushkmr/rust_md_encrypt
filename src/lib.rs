use magic_crypt::{new_magic_crypt, MagicCryptError, MagicCryptTrait};
use rpassword::prompt_password;
use std::error::Error;

pub fn read_password_hidden() -> String {
    let password = prompt_password("Enter your key: ");
    password.unwrap()
}

pub fn make_mc(key: String) -> magic_crypt::MagicCrypt256 {
    println!("This is your key *do not forget it!* : {} \n", key);
    let mc = new_magic_crypt!(key, 256);
    return mc;
}

pub fn encrypt(raw_input: String, mc: &magic_crypt::MagicCrypt256) -> String {
    let encrypted = mc.encrypt_str_to_base64(raw_input);
    return encrypted;
}

pub fn decrypt(
    encrypted_input: String,
    mc: &magic_crypt::MagicCrypt256,
) -> Result<String, MagicCryptError> {
    match mc.decrypt_base64_to_string(encrypted_input) {
        Ok(decrypted) => Ok(decrypted),
        Err(e) => match e {
            MagicCryptError::DecryptError(_) => {
                eprintln!("Wrong key! Please try again.");
                Err(e)
            }
            MagicCryptError::Base64Error(_) => {
                eprintln!("Your file is not encrypted!");
                Err(e)
            }
            _ => {
                eprintln!("Error: {}", e);
                Err(e)
            }
        },
    }
}

pub fn read_file(file_path: &String) -> String {
    let contents = std::fs::read_to_string(file_path).unwrap();
    return contents;
}

pub fn overwrite_file(file_path: &String, contents: String) {
    std::fs::write(file_path, contents).unwrap();
}

pub fn list_md_files() -> Result<Vec<String>, Box<dyn Error>> {
    let mut files: Vec<String> = Vec::new();
    let paths = std::fs::read_dir("./").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap();
        if path_str.ends_with(".md") {
            files.push(path_str.to_string());
            // println!("{}", &path_str);
        }
    }
    if files.len() == 0 {
        return Err("No markdown files found".into());
    }
    Ok(files)
}
