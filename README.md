# 🛡️ Input Validation Engine

> A reusable, zero-dependency Rust library for validating common input types — emails, phone numbers, URLs, dates, UUIDs, credit cards, IP addresses, postal codes, usernames, and passwords.

---

## Features

| Validator | Description |
|-----------|-------------|
| `EmailValidator` | RFC 5322-style email validation |
| `PhoneValidator` | E.164 international phone numbers |
| `UrlValidator` | HTTP/HTTPS/FTP URLs with scheme checking |
| `DateValidator` | ISO 8601 dates with leap-year awareness |
| `UuidValidator` | RFC 4122 UUIDs v1–v5, hyphenated and compact |
| `CreditCardValidator` | Luhn algorithm + network detection (Visa, MC, Amex…) |
| `IpValidator` | IPv4 and IPv6 addresses |
| `PostalCodeValidator` | Country-specific codes (US, UK, CA, DE, AU, KE) |
| `UsernameValidator` | Configurable length, chars, reserved words |
| `PasswordValidator` | Configurable strength rules |

---

## Quick Start

```rust
use input_validation_engine::Validator;
use input_validation_engine::validators::email::{EmailValidator, EmailConfig};

let result = EmailValidator::validate("alice@example.com", &EmailConfig::default());
assert!(result.is_ok());
```

---

## API Usage

### Email
```rust
use input_validation_engine::validators::email::{EmailValidator, EmailConfig};
use input_validation_engine::Validator;

EmailValidator::validate("user@example.com", &EmailConfig::default())?;
```

### Phone (E.164)
```rust
use input_validation_engine::validators::phone::{PhoneValidator, PhoneConfig};
use input_validation_engine::Validator;

PhoneValidator::validate("+254712345678", &PhoneConfig::default())?;
```

### URL
```rust
use input_validation_engine::validators::url::{UrlValidator, UrlConfig};
use input_validation_engine::Validator;

UrlValidator::validate("https://rust-lang.org", &UrlConfig::default())?;
```

### Date
```rust
use input_validation_engine::validators::date::{DateValidator, DateConfig};
use input_validation_engine::Validator;

DateValidator::validate("2024-02-29", &DateConfig::default())?; // leap year ✅
```

### UUID
```rust
use input_validation_engine::validators::uuid::{UuidValidator, UuidConfig};
use input_validation_engine::Validator;

UuidValidator::validate("550e8400-e29b-41d4-a716-446655440000", &UuidConfig::default())?;
```

### Credit Card
```rust
use input_validation_engine::validators::credit_card::{CreditCardValidator, CreditCardConfig};
use input_validation_engine::Validator;

CreditCardValidator::validate("4111 1111 1111 1111", &CreditCardConfig::default())?; // Visa ✅
```

### IP Address
```rust
use input_validation_engine::validators::ip_address::{IpValidator, IpConfig};
use input_validation_engine::Validator;

IpValidator::validate("192.168.1.1", &IpConfig::default())?;
IpValidator::validate("::1",         &IpConfig::default())?;
```

### Postal Code
```rust
use input_validation_engine::validators::postal_code::{PostalCodeValidator, PostalCodeConfig, Country};
use input_validation_engine::Validator;

let cfg = PostalCodeConfig { country: Country::UK };
PostalCodeValidator::validate("SW1A 1AA", &cfg)?;
```

### Username
```rust
use input_validation_engine::validators::username::{UsernameValidator, UsernameConfig};
use input_validation_engine::Validator;

UsernameValidator::validate("nick_99", &UsernameConfig::default())?;
```

### Password
```rust
use input_validation_engine::validators::password::{PasswordValidator, PasswordConfig};
use input_validation_engine::Validator;

// Default: 8+ chars, uppercase, lowercase, digit required
PasswordValidator::validate("Secure1pass", &PasswordConfig::default())?;

// Strict: also requires a symbol
PasswordValidator::validate("Secure1pass!", &PasswordConfig::strict())?;
```

---

## Error Handling

All validators return `Result<(), ValidationError>`. Errors are descriptive:

```rust
use input_validation_engine::ValidationError;

match result {
    Ok(())                                    => println!("Valid!"),
    Err(ValidationError::MissingAtSign)       => println!("Email is missing @"),
    Err(ValidationError::TooShort { min, actual }) =>
        println!("Too short: need {min}, got {actual}"),
    Err(e)                                    => println!("Error: {e}"),
}
```

---

## CLI Tool

```bash
# Run the demo (validates sample inputs for all types)
cargo run --bin input-validator

# Validate specific inputs
cargo run --bin input-validator -- --email "user@example.com"
cargo run --bin input-validator -- --phone "+12025550173"
cargo run --bin input-validator -- --url "https://rust-lang.org"
cargo run --bin input-validator -- --date "2024-02-29"
cargo run --bin input-validator -- --uuid "550e8400-e29b-41d4-a716-446655440000"
cargo run --bin input-validator -- --card "4111111111111111"
cargo run --bin input-validator -- --ip "::1"
cargo run --bin input-validator -- --postal "SW1A 1AA" --country UK
cargo run --bin input-validator -- --username "nick_99"
cargo run --bin input-validator -- --password "Secure1pass!"

# Interactive REPL
cargo run --bin input-validator -- --interactive
```

---

## Running Tests

```bash
cargo test
```

Expected output: all tests pass across all 10 validators.

---

## Design Patterns Used

| Pattern | Usage |
|---------|-------|
| **Strategy** | `Validator` trait — all 10 validators share one interface |
| **Builder** | Config structs with `Default` + custom constructors |
| **Error as Data** | `ValidationError` enum with structured variants |
| **Layered Architecture** | Domain → lib → CLI, each layer isolated |

---

## License

MIT
