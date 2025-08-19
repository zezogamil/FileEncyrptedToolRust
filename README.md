# FileEncyrptedToolRust
a file encrypt tool by rust, the tool will have both GUI options and CLI 

## FileEncryptedToolRust 🔐
A secure and efficient file encryption tool built in Rust, offering both command-line interface (CLI) and graphical user interface (GUI) options for encrypting and decrypting files and folders.

## 🚀 Features
     🔒 Strong Encryption: Uses AES-256-CBC encryption with PKCS7 padding
     📁 File & Folder Support: Encrypt individual files or entire directories
     🎯 Dual Interface: Both CLI and GUI options available
     ⚡ Fast Performance: Built with Rust for optimal speed and memory safety
     🛡️ Secure: 256-bit keys with randomly generated 128-bit IVs per file
     📋 Easy to Use: Interactive menu system with clear options


## 🏗️Pdf_Encryptor Structure
    Core/
    └── pdf_encryptor/
    ├── Cargo.toml
    └── src/
    ├── main.rs # Main menu and user interaction ✅ 
    ├── encrypt.rs # File/folder encryption logic ✅ 
    ├── decrypt.rs # File/folder decryption logic ✅
    ├── utiles.rs # Helper functions (e.g., prompt user input)
    └── constants.rs # Constants like key sizes, banners, etc.


## 🔧 Installation

 Prerequisites

     Rust (1.70 or later)
     Cargo package manager
     
Build from Source

    Clone the repository:

    bashgit clone https://github.com/zezogamil/FileEncyrptedToolRust.git
    cd FileEncyrptedToolRust

Build the project:

    bashcargo build --release

Run the tool:

    bashcargo run


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
