mod vault;
mod entry;
mod password_gen;
mod import_export;
mod clipboard;
mod twofa;
mod cloud;
mod gui;
use entry::ServiceInfo;
use vault::{Vault, VaultError};
use rpassword::prompt_password;
use std::io::{self, Write};

fn clr() {
    print!("\x1B[2J\x1B[1;1H");
}
fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "gui" {
        gui::run_gui();
        return;
    }
    clr();
    let path = "vault.dat";
    let exists = std::path::Path::new(path).exists();
    let master = if exists {
        prompt_password("Enter master password: ").unwrap()
    } else {
        prompt_password("Set a new master password: ").unwrap()
    };
    let mut vault = if exists {
        match Vault::open(path, &master) {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid password or corrupted vault.");
                return;
            }
        }
    } else {
        Vault::new(&master)
    };
    loop {
        println!("Password manager menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search Entry");
        println!("4. Delete Entry");
        println!("5. Save & Quit");
        println!("6. Generate Password");
        println!("7. Export JSON");
        println!("8. Import JSON");
        println!("9. Export CSV");
        println!("10. Import CSV");
        println!("11. Copy Password to Clipboard");
        println!("12. Generate 2FA Code");
        println!("13. Cloud Upload");
        println!("14. Cloud Download");
        println!("15. Launch GUI");
        let choice = prompt("
> ");
        match choice.as_str() {
            "1" => {
                let service = prompt("Service: ");
                let username = prompt("Username: ");
                let password = prompt_password("Password: ").unwrap();
                vault.add(ServiceInfo::new(service, username, password));
                clr();
            }
            "2" => {
                clr();
                for (i, e) in vault.entries().iter().enumerate() {
                    println!("{}. {} | {} | {}", i + 1, e.service, e.username, e.password);
                }
            }
            "3" => {
                clr();
                let search = prompt("Search service: ");
                for e in vault.entries().iter().filter(|e| e.service == search) {
                    println!("{} | {} | {}", e.service, e.username, e.password);
                }
            }
            "4" => {
                clr();
                let search = prompt("Delete service: ");
                vault.delete(&search);
            }
            "5" => {
                clr();
                match vault.save(path, &master) {
                    Ok(_) => println!("Vault saved. Goodbye."),
                    Err(_) => println!("Failed to save vault."),
                }
                break;
            }
            "6" => {
                let len = prompt("Password length: ").parse().unwrap_or(16);
                let symbols = prompt("Include symbols? (y/n): ") == "y";
                let upper = prompt("Include uppercase? (y/n): ") == "y";
                let lower = prompt("Include lowercase? (y/n): ") == "y";
                let nums = prompt("Include numbers? (y/n): ") == "y";
                let pw = password_gen::generate(len, symbols, upper, lower, nums);
                println!("Generated: {}", pw);
            }
            "7" => {
                let file = prompt("Export JSON to file: ");
                import_export::export_json(&file, vault.entries());
            }
            "8" => {
                let file = prompt("Import JSON from file: ");
                let entries = import_export::import_json(&file);
                for e in entries { vault.add(e); }
            }
            "9" => {
                let file = prompt("Export CSV to file: ");
                import_export::export_csv(&file, vault.entries());
            }
            "10" => {
                let file = prompt("Import CSV from file: ");
                let entries = import_export::import_csv(&file);
                for e in entries { vault.add(e); }
            }
            "11" => {
                let service = prompt("Service to copy password: ");
                if let Some(e) = vault.entries().iter().find(|e| e.service == service) {
                    clipboard::copy_and_clear(&e.password, 15);
                    println!("Password copied to clipboard for 15 seconds.");
                }
            }
            "12" => {
                let secret = prompt("2FA Secret (base32): ");
                let code = twofa::generate_totp(&secret);
                println!("Current TOTP: {}", code);
            }
            "13" => {
                let url = prompt("Cloud upload URL: ");
                cloud::upload(&url, path);
            }
            "14" => {
                let url = prompt("Cloud download URL: ");
                cloud::download(&url, path);
            }
            "15" => {
                gui::run_gui();
            }
            _ => println!("Invalid choice.")
        }
        println!("");
    }
}
