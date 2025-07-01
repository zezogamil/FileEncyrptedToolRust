# FileEncyrptedToolRust
a file encrypt tool by rust, the tool will have both GUI options and CLI 

## 🔐 FilesEncrypted CLI
A secure and simple Rust-based command-line tool to encrypt and decrypt files or folders using AES-256 encryption. Designed for speed, clarity, and modular structure.


## Pdf_Encryptor Structure
    Core/
    └── pdf_encryptor/
    ├── Cargo.toml
    └── src/
    ├── main.rs # Main menu and user interaction ✅ 
    ├── encrypt.rs # File/folder encryption logic ✅ 
    ├── decrypt.rs # File/folder decryption logic ✅
    ├── utiles.rs # Helper functions (e.g., prompt user input)
    └── constants.rs # Constants like key sizes, banners, etc.

## 🔐 Hello to FilesEncrypted CLI 🔐
       ✔ Choose the desired option:
       › 1- Encrypt
         2- Decrypt
         3- Help
  
## 🧠 How It Works
        AES-256-CBC is used with PKCS7 padding.
        A 256-bit key and 128-bit IV are randomly generated per file.
        The IV is prepended to the encrypted output.
        The key must be securely stored or remembered for decryption.
## 👨‍💻 Author 
    Abdelaziz Gamil 
    Software Engineer
## ©️ Copyright
    © 2025 Abdelaziz Gamil. All rights reserved.
