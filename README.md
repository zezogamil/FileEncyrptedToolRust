<h1 align="center">FILEENCRYPTEDTOOLRUST</h1>
<p align="center">
  Secure File Encryption & Decryption with AES-256 Protection
</p>
<p align="center">
  <img src="https://img.shields.io/github/last-commit/zezogamil/FileEncyrptedToolRust?style=for-the-badge" />
  <img src="https://img.shields.io/github/languages/top/zezogamil/FileEncyrptedToolRust?style=for-the-badge" />
  <img src="https://img.shields.io/github/languages/count/zezogamil/FileEncyrptedToolRust?style=for-the-badge" />
</p>
<p align="center">
  Built with the tools and technologies:
</p>
<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white"/>
  <img src="https://img.shields.io/badge/Cargo-000000?style=for-the-badge&logo=rust&logoColor=white"/>
  <img src="https://img.shields.io/badge/AES_256-FF6B6B?style=for-the-badge&logo=security&logoColor=white"/>
  <img src="https://img.shields.io/badge/Cryptography-4285F4?style=for-the-badge&logo=keepassxc&logoColor=white"/>
  <img src="https://img.shields.io/badge/CLI-4EAA25?style=for-the-badge&logo=gnubash&logoColor=white"/>
  <img src="https://img.shields.io/badge/Cross_Platform-FF7139?style=for-the-badge&logo=rust&logoColor=white"/>
  <img src="https://img.shields.io/badge/Memory_Safe-DEA584?style=for-the-badge&logo=rust&logoColor=white"/>
</p>

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


Encrypting Files/Folders

    Select option 1 for encryption
    Provide the path to the file or folder you want to encrypt
    Enter a secure password/key
    The tool will create encrypted versions of your files

Decrypting Files/Folders

    Select option 2 for decryption
    Provide the path to the encrypted file or folder
    Enter the correct password/key used for encryption
    The tool will restore the original files

Getting Help

    Select option 3 to view detailed help information and usage examples.


## 📦 Dependencies
 
The project uses the following Rust crates:

      aes - AES encryption implementation
      cbc - Cipher Block Chaining mode
      rand - Random number generation for IVs
      clap - Command line argument parsing (if CLI args are supported)
      Additional crates as specified in Cargo.toml
## 🧠 How It Works
        AES-256-CBC is used with PKCS7 padding.
        A 256-bit key and 128-bit IV are randomly generated per file.
        The IV is prepended to the encrypted output.
        The key must be securely stored or remembered for decryption.

## 🤝 Contributing
     Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.
## 👨‍💻 Author 
    Abdelaziz Gamil 
    Software Engineer

## ⚠️ Disclaimer
This tool is provided as-is for educational and legitimate use cases. Users are responsible for:

     Keeping their encryption keys secure
     Complying with local laws and regulations
     Using the tool ethically and responsibly

Important: Always backup your important files before encryption. Lost encryption keys cannot be recovered.    
## ©️ Copyright
    © 2025 Abdelaziz Gamil. All rights reserved.

______________________________________________________________________________________________________________
⭐ If you find this project useful, please consider giving it a star on GitHub!
