//Handles decryption
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use anyhow::{Context, Result};
use aes::Aes256;
use cbc::Decryptor;
use cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
use dialoguer::Input;
use base64::{engine::general_purpose, Engine};
use walkdir::WalkDir;




const IV_SIZE: usize = 16;

pub fn decrypt_file(file_path: &Path, key: &[u8]) -> Result<()> {
    if key.len() != 32 {
        anyhow::bail!("Key must be 32 bytes for AES-256");
    }

    let mut file = File::open(file_path).context("Failed to open encrypted file")?;
    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data).context("Failed to read encrypted file")?;

    if encrypted_data.len() < IV_SIZE {
        anyhow::bail!("Encrypted file is too small to contain IV");
    }

    let (iv, ciphertext) = encrypted_data.split_at(IV_SIZE);
    let cipher = Decryptor::<Aes256>::new(key.into(), iv.into());

    let mut buf = ciphertext.to_vec();
    let plaintext = cipher
    .decrypt_padded_mut::<Pkcs7>(&mut buf)
    .map_err(|_| anyhow::anyhow!("Decryption failed or invalid padding"))?
    .to_vec();

    let output_path = file_path.with_extension(""); 
    let mut output = File::create(&output_path).context("Failed to create decrypted file")?;
    output.write_all(&plaintext).context("Failed to write decrypted content")?;

    println!("✅ Decrypted: {}", output_path.display());
    Ok(())
}

pub fn handle_decryption() {
    let input_path: String = Input::new()
        .with_prompt("Enter the file or folder path to decrypt")
        .interact_text()
        .unwrap();

    let key_b64: String = Input::new()
        .with_prompt("Enter the base64-encoded key used during encryption")
        .interact_text()
        .unwrap();

   let key: Vec<u8> = match general_purpose::STANDARD.decode(&key_b64) {
    Ok(k) => k,
    Err(_) => {
        println!("❌ Failed to decode base64 key.");
        return;
    }
};


    if let Err(e) = super::decrypt::decrypt_path(&input_path, &key) {
        println!("❌ Decryption failed: {e}");
    }
}

pub fn decrypt_path(path: &str, key: &[u8]) -> Result<()> {
    let path = Path::new(path);
    if path.is_file() {
        decrypt_file(path, key)?;
    } else if path.is_dir() {
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "enc"))
        {
            decrypt_file(entry.path(), key)?;
        }
    } else {
        println!("❌ Invalid path provided.");
    }

    Ok(())
}