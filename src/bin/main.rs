//! # input-validator CLI
//!
//! A command-line tool to quickly test all 10 validators.
//!
//! ## Usage
//! ```
//! input-validator --email "user@example.com"
//! input-validator --phone "+12025550173"
//! input-validator --url "https://rust-lang.org"
//! input-validator --date "2024-02-29"
//! input-validator --uuid "550e8400-e29b-41d4-a716-446655440000"
//! input-validator --card "4111111111111111"
//! input-validator --ip "192.168.1.1"
//! input-validator --postal "90210" --country US
//! input-validator --username "nick_99"
//! input-validator --password "Secure1pass!"
//! input-validator --interactive
//! ```

use input_validation_engine::{
    validators::{
        email::{EmailConfig, EmailValidator},
        phone::{PhoneConfig, PhoneValidator},
        url::{UrlConfig, UrlValidator},
        date::{DateConfig, DateValidator},
        uuid::{UuidConfig, UuidValidator},
        credit_card::{CreditCardConfig, CreditCardValidator},
        ip_address::{IpConfig, IpValidator},
        postal_code::{Country, PostalCodeConfig, PostalCodeValidator},
        username::{UsernameConfig, UsernameValidator},
        password::{PasswordConfig, PasswordValidator},
    },
    Validator,
};
use std::{env, io::{self, BufRead, Write}};

// ── Colour helpers ────────────────────────────────────────────────────────────
fn green(s: &str) -> String { format!("\x1b[32m{s}\x1b[0m") }
fn red(s: &str)   -> String { format!("\x1b[31m{s}\x1b[0m") }
fn bold(s: &str)  -> String { format!("\x1b[1m{s}\x1b[0m") }
fn cyan(s: &str)  -> String { format!("\x1b[36m{s}\x1b[0m") }
fn yellow(s: &str) -> String { format!("\x1b[33m{s}\x1b[0m") }

fn print_result(label: &str, value: &str, result: Result<(), impl std::fmt::Display>) {
    let status = match &result {
        Ok(_)  => green("✅ VALID"),
        Err(_) => red("❌ INVALID"),
    };
    println!("{} {} → {}", bold(label), cyan(&format!("\"{}\"", value)), status);
    if let Err(e) = result {
        println!("   {} {}", yellow("↳"), e);
    }
}

// ── Validate-all demo ─────────────────────────────────────────────────────────
fn demo_all() {
    println!("\n{}\n", bold("═══════ Input Validation Engine — Demo Run ═══════"));

    let samples: Vec<(&str, &str)> = vec![
        ("email",    "alice@example.com"),
        ("email",    "not-an-email"),
        ("phone",    "+12025550173"),
        ("phone",    "12025550173"),
        ("url",      "https://www.rust-lang.org/tools"),
        ("url",      "not-a-url"),
        ("date",     "2024-02-29"),
        ("date",     "2023-02-29"),
        ("uuid",     "550e8400-e29b-41d4-a716-446655440000"),
        ("uuid",     "not-a-uuid"),
        ("card",     "4111111111111111"),
        ("card",     "1234567890123456"),
        ("ip",       "192.168.1.1"),
        ("ip",       "256.0.0.1"),
        ("postal",   "90210"),
        ("username", "nick_99"),
        ("username", "admin"),
        ("password", "Secure1pass!"),
        ("password", "weak"),
    ];

    for (kind, value) in samples {
        run_single(kind, value, "US");
    }
    println!();
}

fn run_single(kind: &str, value: &str, country: &str) {
    match kind {
        "email" => print_result("Email   ", value,
            EmailValidator::validate(value, &EmailConfig::default())),
        "phone" => print_result("Phone   ", value,
            PhoneValidator::validate(value, &PhoneConfig::default())),
        "url"   => print_result("URL     ", value,
            UrlValidator::validate(value, &UrlConfig::default())),
        "date"  => print_result("Date    ", value,
            DateValidator::validate(value, &DateConfig::default())),
        "uuid"  => print_result("UUID    ", value,
            UuidValidator::validate(value, &UuidConfig::default())),
        "card"  => print_result("Card    ", value,
            CreditCardValidator::validate(value, &CreditCardConfig::default())),
        "ip"    => print_result("IP      ", value,
            IpValidator::validate(value, &IpConfig::default())),
        "postal" => {
            let country_enum = match country.to_uppercase().as_str() {
                "US" => Country::US,
                "UK" => Country::UK,
                "CA" => Country::CA,
                "DE" => Country::DE,
                "AU" => Country::AU,
                "KE" => Country::KE,
                _ => {
                    println!("{}", red(&format!("Unknown country code: {country}")));
                    return;
                }
            };
            print_result("Postal  ", value,
                PostalCodeValidator::validate(value, &PostalCodeConfig { country: country_enum }));
        }
        "username" => print_result("Username", value,
            UsernameValidator::validate(value, &UsernameConfig::default())),
        "password" => print_result("Password", value,
            PasswordValidator::validate(value, &PasswordConfig::strict())),
        _ => println!("{}", red(&format!("Unknown type: {kind}"))),
    }
}

// ── Interactive REPL ──────────────────────────────────────────────────────────
fn interactive_mode() {
    println!("\n{}", bold("═══════ Input Validation Engine — Interactive Mode ═══════"));
    println!("Commands: email | phone | url | date | uuid | card | ip | postal | username | password | quit\n");

    let stdin = io::stdin();
    loop {
        print!("{} ", cyan("validator>"));
        io::stdout().flush().unwrap();

        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let parts: Vec<&str> = line.trim().splitn(3, ' ').collect();

        match parts.as_slice() {
            ["quit"] | ["exit"] | ["q"] => {
                println!("Goodbye!");
                break;
            }
            [kind, value] => run_single(kind, value, "US"),
            [kind, value, country] => run_single(kind, value, country),
            _ => println!("{}", yellow("Usage: <type> <value> [country]")),
        }
    }
}

// ── main ──────────────────────────────────────────────────────────────────────
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        demo_all();
        return;
    }

    let mut i = 1;
    let mut ran = false;
    let mut country = "US".to_string();

    while i < args.len() {
        match args[i].as_str() {
            "--interactive" | "-i" => { interactive_mode(); ran = true; }
            "--country" => {
                i += 1;
                if i < args.len() { country = args[i].clone(); }
            }
            "--help" | "-h" => {
                println!("{}", bold("input-validator — Input Validation Engine CLI"));
                println!("\nUsage: input-validator [OPTION] <value>\n");
                println!("  {:<20} Validate an email address",      "--email <value>");
                println!("  {:<20} Validate a phone number (E.164)","--phone <value>");
                println!("  {:<20} Validate a URL",                 "--url <value>");
                println!("  {:<20} Validate an ISO 8601 date",      "--date <value>");
                println!("  {:<20} Validate a UUID",                "--uuid <value>");
                println!("  {:<20} Validate a credit card number",  "--card <value>");
                println!("  {:<20} Validate an IP address",         "--ip <value>");
                println!("  {:<20} Validate a postal code",         "--postal <value>");
                println!("  {:<20} Validate a username",            "--username <value>");
                println!("  {:<20} Validate a password",            "--password <value>");
                println!("  {:<20} Set country for postal (default US)","--country <code>");
                println!("  {:<20} Interactive REPL mode",          "--interactive");
                ran = true;
            }
            flag => {
                if let Some(kind) = flag.strip_prefix("--") {
                    i += 1;
                    if i < args.len() {
                        run_single(kind, &args[i], &country);
                        ran = true;
                    } else {
                        println!("{}", red(&format!("Flag '{flag}' needs a value.")));
                    }
                } else {
                    println!("{}", red(&format!("Unknown flag: {flag}. Use --help.")));
                }
            }
        }
        i += 1;
    }

    if !ran { demo_all(); }
}
