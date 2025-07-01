// Handles encryption
use aes::Aes256;
use cbc::{Encryptor, Decryptor};
use cipher::{
    block_padding::Pkcs7,
    BlockEncryptMut, BlockDecryptMut,
    KeyIvInit,
    BlockSizeUser
};
use base64::{engine::general_purpose, Engine};

use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::ffi::OsString;
use walkdir::WalkDir;
use anyhow::{Context, Result};



pub fn encrypt_path(path: &str) -> Result<()> {
    let path = Path::new(path);
    if path.is_file() {
        encrypt_file(path)?;
    } else if path.is_dir() {
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().is_file())
        {
            encrypt_file(entry.path())?;
        }
    } else {
        println!("❌ Invalid path provided.");
    }

    Ok(())
}

fn encrypt_file(file_path: &Path) -> anyhow::Result<Vec<u8>> {
    let mut file = File::open(file_path).context("Failed to open input file")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let plaintext = &buffer;

    // Generate key and IV
    let key = rand::thread_rng().r#gen::<[u8; 32]>();
    let iv = rand::thread_rng().r#gen::<[u8; 16]>();
    let msg_len = buffer.len(); // original data size
    let block_size = <Aes256 as BlockSizeUser>::block_size();
    let pad_len = block_size - (msg_len % block_size);
    let new_len = msg_len + pad_len;

buffer.resize(new_len, 0u8); 

    // Create cipher
    let cipher = Encryptor::<Aes256>::new((&key).into(), (&iv).into());
    let ciphertext = cipher.encrypt_padded_mut::<Pkcs7>(&mut buffer, msg_len)
    .map_err(|e| anyhow::anyhow!("Padding error: {}", e))?;


    let ext = file_path.extension()
        .map(|ext| format!("{}.aes", ext.to_string_lossy()))
        .unwrap_or_else(|| "aes".to_string());

    // Save encrypted file
    let encrypted_path = file_path.with_extension(ext);
    let mut output = File::create(&encrypted_path).context("Failed to create encrypted file")?;
    output.write_all(&iv)?; // Prepend IV
    output.write_all(ciphertext)?;

    println!("🔐 Encrypted: {}", encrypted_path.display());

    // For demo: print key (you should store this securely)
    println!(
        "🔑 Key (Base64): {}",
        base64::engine::general_purpose::STANDARD.encode(&key)
    );
    println!(
        "📍 IV  (Base64): {}",
        base64::engine::general_purpose::STANDARD.encode(&iv)
    );

    Ok(ciphertext.to_vec())

}