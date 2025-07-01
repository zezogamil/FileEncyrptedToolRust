// Entry point, user interaction
use dialoguer::{theme::ColorfulTheme, Select};
use std::process;
use clap::{Parser, Subcommand};
use cli::{Cli, Commands};

mod encrypt;
use encrypt::encrypt_path;

mod decrypt;
mod utiles;
mod cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Encrypt { path }) => {
            println!("🔐 Encrypting file: {}", path);
            encrypt::encrypt_path(&path).unwrap(); // Adjust based on return type
        }
        Some(Commands::Decrypt { path, key }) => {
            println!("🔓 Decrypting file: {}", path);
            decrypt::decrypt_path(&path, key.as_bytes());
        }
        None => {
            launch_interactive();
        }
    }
}


fn launch_interactive() {
    println!("🔐 Hello to FilesEncrypted CLI 🔐");

    let options = vec!["1- Encrypt", "2-Decrypt", "3-Help"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the desired option")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();


    match selection {
        0 => {
            println!("\nEncrypting...");
            use dialoguer::Input;
            let input_path: String = Input::new()
                .with_prompt("Enter the file or folder path to encrypt")
                .interact_text()
                .unwrap();

            if let Err(e) = encrypt::encrypt_path(&input_path) {
                println!("❌ Encryption failed: {e}");
            }
        }
        1 => {
            println!("\nDecrypting...");
            decrypt::handle_decryption();
        }
        2 => {
            println!("\nℹ️ Help:\n1 - Encrypt: Encrypt a file or folder\n2 - Decrypt: Decrypt an encrypted file or folder\n3 - Help: Show this help message");
        }
        3 => {
            println!("\n👋 Exiting...");
            process::exit(0);
        }
        _ => {
            println!("❌ Invalid option.");
        }
    }
}

