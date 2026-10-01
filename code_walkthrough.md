# 📖 Input Validation Engine — Complete Code Walkthrough
### Every file · Every line · Every decision explained

---

## Table of Contents
1. [Project Structure](#1-project-structure)
2. [Cargo.toml — The Project Manifest](#2-cargotoml--the-project-manifest)
3. [src/lib.rs — The Front Door](#3-srclibrs--the-front-door)
4. [src/error.rs — The Error System](#4-srcerrorrs--the-error-system)
5. [src/validators/mod.rs — The Module Hub](#5-srcvalidatorsmodrs--the-module-hub)
6. [src/validators/email.rs](#6-srcvalidatorsemailrs)
7. [src/validators/phone.rs](#7-srcvalidatorsphoners)
8. [src/validators/url.rs](#8-srcvalidatorsurlrs)
9. [src/validators/date.rs](#9-srcvalidatorsdaters)
10. [src/validators/uuid.rs](#10-srcvalidatorsuuidrs)
11. [src/validators/credit_card.rs](#11-srcvalidatorscredit_cardrs)
12. [src/validators/ip_address.rs](#12-srcvalidatorsip_addressrs)
13. [src/validators/postal_code.rs](#13-srcvalidatorspostal_coders)
14. [src/validators/username.rs](#14-srcvalidatorsusernamers)
15. [src/validators/password.rs](#15-srcvalidatorspasswordrs)
16. [src/bin/main.rs — The CLI Tool](#16-srcbinmainrs--the-cli-tool)

---

## 1. Project Structure

```
INPUT VALIDATION ENGINE/
│
├── Cargo.toml              ← Project configuration (like package.json in Node)
│
├── src/
│   ├── lib.rs              ← The library's public front door
│   ├── error.rs            ← One shared error type for the whole project
│   │
│   ├── validators/
│   │   ├── mod.rs          ← Tells Rust "these are the sub-files inside validators/"
│   │   ├── email.rs        ← Email validator
│   │   ├── phone.rs        ← Phone validator
│   │   ├── url.rs          ← URL validator
│   │   ├── date.rs         ← Date validator
│   │   ├── uuid.rs         ← UUID validator
│   │   ├── credit_card.rs  ← Credit card validator (with Luhn algorithm)
│   │   ├── ip_address.rs   ← IPv4 / IPv6 validator
│   │   ├── postal_code.rs  ← Postal code validator (country-aware)
│   │   ├── username.rs     ← Username validator
│   │   └── password.rs     ← Password strength validator
│   │
│   └── bin/
│       └── main.rs         ← The CLI tool (the runnable binary)
│
└── README.md               ← Documentation
```

**Why this structure?**
This is called **Layered Architecture**. Each concern lives in its own place:
- `error.rs` is used by *every* validator — so it lives at the top level
- Each validator is isolated — `email.rs` knows nothing about `phone.rs`
- `lib.rs` is the only "public face" — it controls what the outside world can see
- `main.rs` in `bin/` is completely separate from the library logic

---

## 2. `Cargo.toml` — The Project Manifest

```toml
[package]
name = "input_validation_engine"
version = "0.1.0"
edition = "2021"
authors = ["Sirnickko"]
description = "A reusable, zero-dependency input validation library..."
license = "MIT"
keywords = ["validation", "email", "phone", "uuid", "password"]
categories = ["text-processing", "value-formatting"]
```

| Line | What it does | Why |
|------|-------------|-----|
| `name` | The official name of this crate | Used when other Rust projects add it as a dependency |
| `version` | Semantic version (major.minor.patch) | `0.1.0` means first release, not yet stable |
| `edition = "2021"` | Which version of Rust language rules to use | 2021 is the latest, has the best features |
| `authors` | Who wrote it | Shows up on crates.io if published |
| `license = "MIT"` | Anyone can use this freely | MIT is the most permissive open-source license |
| `keywords` | Search tags for crates.io | Helps people discover the library |

```toml
[[bin]]
name = "input-validator"
path = "src/bin/main.rs"
```

> `[[bin]]` (double brackets) means "this is an executable binary target."
> Without this, Cargo wouldn't know `main.rs` should produce a runnable `.exe`.
> The `name` is what you type after `cargo run --bin`.

```toml
[lib]
name = "input_validation_engine"
path = "src/lib.rs"
```

> This tells Cargo: "there is also a **library** target, and it starts at `src/lib.rs`."
> Having both `[lib]` and `[[bin]]` in one project means you get a library
> (for other Rust code to use) AND a CLI tool — from the same codebase.

```toml
[dependencies]
[dev-dependencies]
```

> Both are empty — this project has **zero external dependencies**.
> Everything is written in pure Rust from scratch. This is intentional:
> it means the library is lightweight, fast to compile, and has no security
> vulnerabilities from third-party packages.

---

## 3. `src/lib.rs` — The Front Door

```rust
//! # Input Validation Engine
//!
//! A reusable, zero-dependency library...
```

> Lines starting with `//!` are **module-level doc comments**.
> The `!` means "document the thing I'm *inside*" (the whole crate).
> When you run `cargo doc`, these become the front page of the HTML documentation.
> Regular `//` comments are just for developers reading the code;
> `///` and `//!` comments become the *published* documentation.

```rust
pub mod error;
pub mod validators;
```

> `mod` tells Rust "there is a module called `error`."
> Rust will look for it in either `src/error.rs` or `src/error/mod.rs`.
> `pub` makes it **public** — accessible to anyone using this library.
> Without `pub`, it would be internal only.

```rust
pub use error::ValidationError;
```

> This **re-exports** `ValidationError` so users can write:
> ```rust
> use input_validation_engine::ValidationError;
> ```
> Instead of the longer:
> ```rust
> use input_validation_engine::error::ValidationError;
> ```
> It's a convenience shortcut — the `pub use` pattern is standard in Rust APIs.

```rust
pub trait Validator {
    type Config: Default;
    fn validate(input: &str, config: &Self::Config) -> Result<(), ValidationError>;
}
```

> This is the **Strategy Pattern** — the heart of the whole design.
>
> - `trait` = an interface (like an `interface` in Java or TypeScript)
> - `type Config: Default` = each validator defines its own config type,
>   but it must implement `Default` (so you can call `Config::default()`)
> - `fn validate(...)` = the one method every validator must implement
> - `&str` = a borrowed string slice (no copying, efficient)
> - `Result<(), ValidationError>` = either success `Ok(())` or a specific error
>
> Every single one of our 10 validators implements this trait.
> That means you can write code that works with *any* validator
> without knowing which one it is — that's the power of traits.

---

## 4. `src/error.rs` — The Error System

```rust
use std::fmt;
```

> We import the `fmt` module from Rust's standard library.
> We need it to implement the `Display` trait (how the error prints itself).

```rust
#[derive(Debug, PartialEq, Clone)]
pub enum ValidationError { ... }
```

> `enum` = a type that can be one of several named variants.
> Think of it like a dropdown menu — it can only be one option at a time.
>
> The `#[derive(...)]` line automatically generates code for three capabilities:
> - `Debug` → lets you print it with `{:?}` for debugging (`println!("{:?}", e)`)
> - `PartialEq` → lets you compare two errors with `==` (needed in tests: `assert_eq!`)
> - `Clone` → lets you duplicate/copy the error value

```rust
EmptyInput,
TooShort { min: usize, actual: usize },
TooLong  { max: usize, actual: usize },
InvalidFormat { field: &'static str, reason: String },
```

> Each line is a **variant** of the error enum. Notice three shapes:
>
> 1. **Unit variant** (`EmptyInput`) — no extra data, just a name
> 2. **Struct variant** (`TooShort { min, actual }`) — carries named fields
>    with the exact numbers, so the error message can say
>    "minimum 8, you gave 4"
> 3. **Tuple variant** (`UnsupportedScheme(String)`) — carries one unnamed value
>
> `&'static str` means a string that lives for the entire program duration
> (like a hardcoded label: `"email"`, `"date"`).
> `String` is a heap-allocated, growable string for dynamic content.
> `usize` is an unsigned integer sized to the machine (64-bit on modern hardware).

```rust
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "Input must not be empty."),
            Self::TooShort { min, actual } =>
                write!(f, "Input is too short: minimum {min} characters, got {actual}."),
            ...
        }
    }
}
```

> `impl Display for ValidationError` means:
> "teach `ValidationError` how to display itself as a human-readable string."
> After this, `println!("{}", my_error)` and `format!("{}", my_error)` work.
>
> `match self` is Rust's pattern matching — it checks which variant the error is
> and runs the corresponding arm. It's exhaustive: the compiler forces you to
> handle *every* variant or it won't compile.
>
> `write!(f, "...")` writes the text into the formatter — this is how
> `Display` puts text into a string or terminal.

```rust
impl std::error::Error for ValidationError {}
```

> This empty `impl` declares that `ValidationError` is an official Rust `Error`.
> It unlocks interoperability with the rest of the Rust ecosystem:
> libraries like `anyhow` and `thiserror` can wrap it,
> and functions returning `Box<dyn Error>` can return it.

---

## 5. `src/validators/mod.rs` — The Module Hub

```rust
pub mod email;
pub mod phone;
pub mod url;
// ... (10 total)
```

> This file is the "table of contents" for the `validators` folder.
> Without these declarations, Rust would not find the individual files.
> Each `pub mod` line means "include this file and make it public."

---

## 6. `src/validators/email.rs`

### The Config Struct
```rust
#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub max_length: usize,
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self { max_length: 254 }
    }
}
```

> `struct` is a named group of fields (like an object/class in other languages).
> `pub` on each field means callers can read/write it directly.
>
> Why 254? That's the maximum email length defined in **RFC 5321** (the official
> email standard). We picked a real-world rule, not a made-up number.
>
> `impl Default` means "implement the `Default` trait" — giving
> `EmailConfig::default()` a sensible starting value.

### The Validator Struct
```rust
pub struct EmailValidator;
```

> This is an **empty struct** — it has no fields.
> Why? Because the validator holds no state. All state (rules) lives in the config.
> The struct exists only to give us a type to hang the `impl Validator` block on.
> This is a common Rust pattern for stateless services.

### The Validation Logic — Step by Step
```rust
let input = input.trim();
```
> `trim()` removes leading and trailing whitespace.
> A user typing `"  alice@example.com  "` should still pass.

```rust
if input.is_empty() {
    return Err(ValidationError::EmptyInput);
}
```
> Early return if there's nothing to validate.
> `return Err(...)` exits the function immediately with an error.
> This "guard clause" pattern keeps the main logic clean.

```rust
let at_count = input.chars().filter(|&c| c == '@').count();
if at_count == 0 { return Err(ValidationError::MissingAtSign); }
if at_count > 1  { return Err(ValidationError::InvalidLocalPart); }
```
> `.chars()` iterates the string character by character (Unicode-safe).
> `.filter(|&c| c == '@')` keeps only `@` characters.
> `.count()` counts how many are left.
> An email needs **exactly one** `@`. Zero or two-plus are both invalid.

```rust
let (local, domain) = input.split_once('@').unwrap();
```
> `split_once('@')` splits at the **first** (and only) `@`, returning a tuple.
> `.unwrap()` is safe here because we already checked `at_count == 1`,
> so we know the split will succeed.
> `(local, domain)` is **destructuring** — pulling both values out at once.

```rust
if local.starts_with('.') || local.ends_with('.') || local.contains("..") {
    return Err(ValidationError::InvalidLocalPart);
}
```
> Real email addresses can't start or end with a dot, and can't have two dots
> in a row (`us..er@`). These are RFC 5321 rules.

```rust
for ch in local.chars() {
    if !matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '_' | '+' | '-') {
        return Err(ValidationError::InvalidLocalPart);
    }
}
```
> `matches!` is a macro that checks if a value matches any of the patterns.
> `'a'..='z'` is a **range pattern** — any lowercase letter.
> This is more readable than a long chain of `||` conditions.
> We reject any character that isn't in our allowed set.

```rust
let labels: Vec<&str> = domain.split('.').collect();
if labels.len() < 2 { return Err(ValidationError::InvalidDomain); }
```
> We split the domain by `.` into **labels** (e.g., `["google", "com"]`).
> A valid domain needs at least two labels (you can't have just `"com"`).

```rust
let tld = labels.last().unwrap();
if !tld.chars().all(|c| c.is_ascii_alphabetic()) {
    return Err(ValidationError::InvalidTLD);
}
```
> `.last()` gets the final label — the TLD (like `"com"`, `"org"`, `"io"`).
> `.all(|c| ...)` returns true only if **every** character passes the check.
> TLDs must be purely alphabetic — `"123"` is not a real TLD.

### The Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Validator;
    fn cfg() -> EmailConfig { EmailConfig::default() }
```
> `#[cfg(test)]` means "only compile this block when running tests."
> It disappears from production builds — zero overhead.
> `mod tests` creates a nested module just for tests.
> `use super::*` imports everything from the parent module (the validator itself).
> `fn cfg()` is a tiny helper so tests don't have to repeat `EmailConfig::default()`.

```rust
#[test]
fn valid_simple() {
    assert!(EmailValidator::validate("user@example.com", &cfg()).is_ok());
}
```
> `#[test]` marks this function as a test case — `cargo test` will find and run it.
> `assert!` panics (fails the test) if the value is `false`.
> `.is_ok()` returns `true` if the `Result` is `Ok(...)`.

---

## 7. `src/validators/phone.rs`

### Config
```rust
pub struct PhoneConfig {
    pub allow_separators: bool,  // +1-800-555-0100 → strip the hyphens
    pub min_digits: usize,       // E.164: min 7 digits
    pub max_digits: usize,       // E.164: max 15 digits
}
```
> E.164 is the international phone number standard (ITU-T E.164).
> 7 digits minimum: the shortest real phone numbers (some Pacific islands).
> 15 digits maximum: the longest allowed by the standard.

### Key Logic
```rust
if !input.starts_with('+') {
    return Err(ValidationError::MissingCountryCode);
}
```
> E.164 requires `+` followed by country code.
> `+254` = Kenya, `+1` = USA/Canada, `+44` = UK.

```rust
let digits_part = &input[1..];
let cleaned: String = if config.allow_separators {
    digits_part.chars().filter(|c| !matches!(c, ' ' | '-' | '(' | ')')).collect()
} else {
    digits_part.to_string()
};
```
> `&input[1..]` slices the string from index 1 onward (skipping the `+`).
> If separators are allowed, we filter them out character by character.
> `.collect()` gathers the filtered chars back into a `String`.

```rust
if !cleaned.chars().all(|c| c.is_ascii_digit()) {
    return Err(ValidationError::InvalidPhoneCharacters);
}
```
> After stripping separators, only digits are allowed.
> `is_ascii_digit()` returns true for `'0'` through `'9'`.

---

## 8. `src/validators/url.rs`

### Config: The AllowedSchemes Enum
```rust
pub enum AllowedSchemes {
    HttpOnly,
    All,
    Custom(Vec<String>),
}
```
> Instead of a plain boolean flag, we use an enum with three meaningful options.
> `Custom(Vec<String>)` lets developers pass their own list: `["sftp", "ssh"]`.
> This is more expressive than a `bool` — a `bool` can only say yes/no,
> while an enum can say "which ones specifically."

### Key Logic
```rust
let scheme_end = input.find("://").ok_or(ValidationError::MissingScheme)?;
```
> `.find("://")` returns `Some(index)` if found, or `None` if not.
> `.ok_or(err)` converts `None` into `Err(err)`.
> The `?` operator propagates the error — if it's an error, the function returns
> it immediately. This is Rust's elegant error propagation shorthand.

```rust
let host_end = after_scheme.find(|c| matches!(c, '/' | '?' | '#')).unwrap_or(after_scheme.len());
let host = &after_scheme[..host_end];
```
> The host ends at the first `/`, `?`, or `#` (start of path, query, or fragment).
> `.unwrap_or(len)` handles the case where there's no path — the host goes to end.

---

## 9. `src/validators/date.rs`

### The Leap Year Algorithm
```rust
fn is_leap(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
```
> The **official Gregorian calendar leap year rule**:
> - Divisible by 4 → leap year... BUT
> - Divisible by 100 → NOT a leap year... UNLESS
> - Divisible by 400 → leap year again
>
> Example: 2000 ÷ 400 = 5 → ✅ leap year
> Example: 1900 ÷ 100 = 19, 1900 ÷ 400 = 4.75 → ❌ NOT a leap year
> This is why we have a test for both 2000 (pass) and 1900 (fail).

### Days Per Month
```rust
fn days_in_month(month: u8, year: u32) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,  // Jan Mar May Jul Aug Oct Dec
        4 | 6 | 9 | 11               => 30,  // Apr Jun Sep Nov
        2 => if is_leap(year) { 29 } else { 28 },
        _ => 0,  // impossible — but Rust needs exhaustive matches
    }
}
```
> `match` with `|` means "or" — months 1, 3, 5, 7, 8, 10, or 12 → 31 days.
> February gets 29 days only in a leap year, 28 otherwise.
> The `_` arm catches anything impossible (months 0 or 13+).

### Parsing the Date
```rust
let year: u32 = parts[0].parse().map_err(|_| ValidationError::InvalidYear)?;
```
> `.parse()` converts a string slice `"2024"` into a `u32`.
> It returns `Result<u32, ParseIntError>`.
> `.map_err(|_| ...)` converts any parse error into our own `ValidationError`.
> The `_` discards the original error (we don't need its details).
> `?` propagates: if parsing fails, the whole `validate()` function returns the error.

---

## 10. `src/validators/uuid.rs`

### What is a UUID?
A UUID looks like: `550e8400-e29b-41d4-a716-446655440000`
It's 32 hex digits arranged in groups: **8-4-4-4-12**, separated by hyphens.

### Validating the Format
```rust
let parts: Vec<&str> = input.split('-').collect();
if parts.len() != 5
    || parts[0].len() != 8
    || parts[1].len() != 4
    || parts[2].len() != 4
    || parts[3].len() != 4
    || parts[4].len() != 12
{
    return Err(ValidationError::InvalidUuidFormat);
}
```
> We split by `-` and check each group is exactly the right length.
> All five conditions must be true simultaneously with `&&`.

### Checking the Version
```rust
let version_char = hex.chars().nth(12).unwrap();
let version = version_char.to_digit(16).unwrap() as u8;
if version < 1 || version > 5 {
    return Err(ValidationError::InvalidUuidVersion);
}
```
> The **version nibble** is always at position 12 (the 13th character) of the
> 32-character hex string. For `550e8400-e29b-41d4-...`, position 12 is `4`.
> `.to_digit(16)` converts a hex char like `'4'` to the number `4`.
> RFC 4122 only defines versions 1 through 5.

### Checking if all chars are hex
```rust
fn is_hex(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_hexdigit())
}
```
> `is_ascii_hexdigit()` returns true for `0-9`, `a-f`, `A-F` — hex alphabet.

---

## 11. `src/validators/credit_card.rs`

### The Luhn Algorithm
```rust
fn luhn_check(digits: &[u8]) -> bool {
    let sum: u32 = digits
        .iter()
        .rev()                          // step 1: work right-to-left
        .enumerate()                    // step 2: get index alongside each digit
        .map(|(i, &d)| {
            let mut n = d as u32;
            if i % 2 == 1 {             // step 3: double every second digit
                n *= 2;
                if n > 9 { n -= 9; }   // step 4: if doubled > 9, subtract 9
            }
            n
        })
        .sum();                         // step 5: sum everything up
    sum % 10 == 0                       // step 6: valid if divisible by 10
}
```
> The **Luhn algorithm** was invented in 1954 by Hans Peter Luhn (IBM).
> It's used to catch single-digit typos in card numbers.
> It's NOT a security feature — it's an error-detection checksum.
>
> Walk-through with `4111 1111 1111 1111` (Visa test card):
> 1. Right-to-left: 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,4
> 2. Double every 2nd (index 1,3,5...): 1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,8
> 3. Sum = 40 → 40 % 10 = 0 ✅

### Network Detection
```rust
let first  = &digits[..1];  // first 1 digit
let first2 = &digits[..2];  // first 2 digits
let first4 = &digits[..4];  // first 4 digits
```
> Card networks are identified by their **IIN (Issuer Identification Number)**,
> the first 6 digits. We check prefix patterns defined by each network:
>
> - Visa: starts with `4`, length 13 or 16
> - Mastercard: starts with `51–55` or `2221–2720`, length 16
> - Amex: starts with `34` or `37`, length 15
> - Discover: starts with `6011` or `65`, length 16

```rust
detect_network(&cleaned).ok_or(ValidationError::UnknownCardNetwork)?;
```
> `detect_network` returns `Option<CardNetwork>`.
> `.ok_or(err)` turns `None` into `Err(...)`.
> `?` propagates the error if detection fails.

---

## 12. `src/validators/ip_address.rs`

### IPv4 Validation
```rust
fn validate_ipv4(input: &str) -> Result<(), ValidationError> {
    let octets: Vec<&str> = input.split('.').collect();
    if octets.len() != 4 { return Err(ValidationError::InvalidIPv4Format); }
    for octet in &octets {
        let val: u32 = octet.parse().map_err(|_| ValidationError::InvalidIPv4Octet)?;
        if val > 255 { return Err(ValidationError::InvalidIPv4Octet); }
        if octet.len() > 1 && octet.starts_with('0') {
            return Err(ValidationError::InvalidIPv4Octet);
        }
    }
    Ok(())
}
```
> An IPv4 address is 4 numbers separated by dots: `192.168.1.1`
> Each number (called an **octet**) must be 0–255.
> We also reject leading zeros like `01` — in some contexts they mean octal
> (base-8), which would be confusing and wrong.
> We parse as `u32` first (not `u8`) to detect values > 255 before casting.

### IPv6 Validation — The `::` Challenge
```rust
let double_colon_count = input.matches("::").count();
if double_colon_count > 1 { return Err(ValidationError::InvalidIPv6Format); }
```
> IPv6 addresses can use `::` as a shorthand for consecutive groups of zeros.
> `::1` = `0000:0000:0000:0000:0000:0000:0000:0001` (the loopback address).
> You can only use `::` **once** per address — otherwise it's ambiguous.

```rust
let (left, right) = if double_colon_count == 1 {
    let idx = input.find("::").unwrap();
    (&input[..idx], &input[idx + 2..])
} else {
    (input, "")
};
```
> We split the address at `::` into a left and right half, then validate each
> group of hex digits independently.

---

## 13. `src/validators/postal_code.rs`

### The Country Enum
```rust
pub enum Country { US, UK, CA, DE, AU, KE }
```
> Instead of accepting a raw string like `"US"` (which could be typo'd as `"us"`
> or `"USA"`), we use an enum. This means invalid country codes are **compile-time
> errors** — you cannot pass `Country::XX` because it doesn't exist.
> This is **type-driven design** in action.

### US Postal Code
```rust
fn validate_us(input: &str) -> bool {
    if input.len() == 5 {
        input.chars().all(|c| c.is_ascii_digit())
    } else if input.len() == 10 {
        let (zip, ext) = input.split_at(5);
        zip.chars().all(|c| c.is_ascii_digit())
            && ext.starts_with('-')
            && ext[1..].chars().all(|c| c.is_ascii_digit())
    } else {
        false
    }
}
```
> US ZIP codes are either 5 digits (`90210`) or ZIP+4 (`90210-1234`).
> `.split_at(5)` splits at exactly position 5 into two slices.

### UK Postal Code
```rust
fn validate_uk(input: &str) -> bool {
    let s = input.trim().to_uppercase();
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() != 2 { return false; }
    let (out, inward) = (parts[0], parts[1]);
    let inward_ok = inward.len() == 3
        && inward.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
        && inward[1..].chars().all(|c| c.is_ascii_alphabetic());
    let outward_ok = (2..=4).contains(&out.len())
        && out.chars().all(|c| c.is_ascii_alphanumeric());
    inward_ok && outward_ok
}
```
> UK postcodes have a complex format like `SW1A 1AA` or `M1 1AE`.
> They always have a space, with an outward code (2–4 chars) and
> an inward code (1 digit + 2 letters).
> We convert to uppercase first (`to_uppercase`) to handle both `sw1a 1aa` and `SW1A 1AA`.

---

## 14. `src/validators/username.rs`

### Start and End Check
```rust
let first = input.chars().next().unwrap();
let last  = input.chars().last().unwrap();
if !first.is_ascii_alphanumeric() {
    return Err(ValidationError::InvalidUsernameCharacter(first));
}
```
> `.next()` gets the first character. `.last()` gets the last.
> The `.unwrap()` is safe because we already checked `is_empty()` above.
> Usernames cannot start or end with `_`, `-`, or `.`
> — they'd look weird and often cause issues in UI displays.

### Consecutive Special Characters
```rust
let special: &[char] = &['_', '-', '.'];
let chars: Vec<char> = input.chars().collect();
for window in chars.windows(2) {
    if special.contains(&window[0]) && special.contains(&window[1]) {
        return Err(...)
    }
}
```
> `.windows(2)` creates overlapping pairs: `["n","i"], ["i","c"], ["c","k"]...`
> We check each pair — if both chars are special (`__`, `--`, `._`, etc.), reject.
> This prevents usernames like `nick__99` or `john-.doe`.

### Reserved Words Check
```rust
let lower = input.to_lowercase();
for reserved in &config.reserved {
    if lower == reserved.to_lowercase() {
        return Err(ValidationError::ReservedUsername(input.to_string()));
    }
}
```
> We compare in lowercase on both sides — `ADMIN` and `admin` are both blocked.
> This is **case-insensitive matching** — important for security so nobody
> tricks the system with `AdMiN`.

---

## 15. `src/validators/password.rs`

### The SYMBOLS Constant
```rust
const SYMBOLS: &str = "!@#$%^&*()-_=+[]{}|;:',.<>?/`~\"\\";
```
> `const` is a compile-time constant — it's baked into the binary, never allocated.
> We use `&str` (a string slice) because it's just a fixed list of characters.
> The `\"` and `\\` are **escape sequences** for literal `"` and `\` inside a string.

### Why We Don't Trim Passwords
```rust
fn validate(input: &str, config: &Self::Config) -> Result<(), ValidationError> {
    // Do NOT trim passwords — leading/trailing spaces can be intentional
    if input.is_empty() { ... }
```
> Every other validator trims whitespace. Passwords don't.
> A user might intentionally have a space at the start of their password.
> Silently removing it would mean their password no longer works on login.
> This is a deliberate security decision.

### The `.any()` Pattern
```rust
if config.require_uppercase && !input.chars().any(|c| c.is_ascii_uppercase()) {
    return Err(ValidationError::MissingUppercase);
}
```
> Short-circuit: if `require_uppercase` is `false`, the `&&` stops and we skip the check.
> `.any(|c| ...)` returns `true` as soon as it finds **one** matching character.
> It stops early — efficient on long passwords.
> `!.any(...)` = "there is NOT even one uppercase letter."

### The `strict()` Constructor
```rust
impl PasswordConfig {
    pub fn strict() -> Self {
        Self { require_symbol: true, ..Self::default() }
    }
}
```
> `..Self::default()` is **struct update syntax** — it fills in all other fields
> from `default()` and only overrides `require_symbol`.
> This is a named "preset" — callers get a shortcut for the most common strong config.

---

## 16. `src/bin/main.rs` — The CLI Tool

### ANSI Colour Helpers
```rust
fn green(s: &str) -> String { format!("\x1b[32m{s}\x1b[0m") }
fn red(s: &str)   -> String { format!("\x1b[31m{s}\x1b[0m") }
fn bold(s: &str)  -> String { format!("\x1b[1m{s}\x1b[0m") }
```
> `\x1b[32m` is an **ANSI escape code** — a special sequence that terminals
> interpret as a colour instruction.
> - `\x1b[32m` = start green text
> - `\x1b[0m`  = reset to default colour
> This is how CLI tools produce coloured output without external libraries.
> `format!` builds a new `String` with the escape codes wrapped around the text.

### The print_result Helper
```rust
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
```
> `impl std::fmt::Display` in the parameter means "any type that can display itself."
> This makes the function generic — it works with any error type, not just ours.
> `if let Err(e) = result` is a **pattern-matching if** —
> it only enters the block if the result is an error, and binds the error to `e`.

### Argument Parsing (No Library Needed)
```rust
let args: Vec<String> = env::args().collect();
```
> `env::args()` returns an iterator of command-line arguments.
> `args[0]` is always the program name itself.
> `args[1..]` are the actual arguments the user typed.

```rust
while i < args.len() {
    match args[i].as_str() {
        "--interactive" | "-i" => { interactive_mode(); ran = true; }
        "--country" => { i += 1; if i < args.len() { country = args[i].clone(); } }
        flag => {
            if let Some(kind) = flag.strip_prefix("--") {
                i += 1;
                if i < args.len() { run_single(kind, &args[i], &country); ran = true; }
            }
        }
    }
    i += 1;
}
```
> `match args[i].as_str()` matches the current flag as a string slice.
> `.strip_prefix("--")` removes `"--"` from the front and returns `Some("email")`
> for `"--email"`, or `None` for anything else.
> The `i += 1` inside the `--country` arm skips the *next* arg (the country value).
> This is a hand-rolled argument parser — no external crate needed.

### Interactive REPL
```rust
fn interactive_mode() {
    let stdin = io::stdin();
    loop {
        print!("{} ", cyan("validator>"));
        io::stdout().flush().unwrap();
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let parts: Vec<&str> = line.trim().splitn(3, ' ').collect();
        match parts.as_slice() {
            ["quit"] | ["exit"] | ["q"] => { println!("Goodbye!"); break; }
            [kind, value] => run_single(kind, value, "US"),
            [kind, value, country] => run_single(kind, value, country),
            _ => println!("Usage: <type> <value> [country]"),
        }
    }
}
```
> `loop` is Rust's infinite loop — runs until `break`.
> `io::stdout().flush()` forces the prompt (`validator>`) to appear
> before we wait for input — otherwise buffering might hide it.
> `stdin.lock()` acquires a lock on stdin for thread-safe reading.
> `.splitn(3, ' ')` splits into at most 3 parts:
> `"postal 90210 US"` → `["postal", "90210", "US"]`.
> `match parts.as_slice()` matches the entire slice against patterns — 
> `["quit"]` matches a slice of exactly one element equal to `"quit"`.

---

## Summary: How It All Connects

```
User types: cargo run --bin input-validator -- --email "alice@gmail.com"
                                ↓
                          src/bin/main.rs
                          parses "--email" → calls run_single("email", "alice@gmail.com", "US")
                                ↓
                          run_single() calls:
                          EmailValidator::validate("alice@gmail.com", &EmailConfig::default())
                                ↓
                          src/validators/email.rs
                          runs all the rules → returns Ok(()) or Err(ValidationError::...)
                                ↓
                          main.rs print_result() formats and prints:
                          Email "alice@gmail.com" → ✅ VALID
```

Every layer does one job:
- `main.rs` → talk to the user
- `lib.rs` → define the contract (`Validator` trait)
- `validators/*.rs` → implement the logic
- `error.rs` → describe what went wrong

That's **Separation of Concerns** — one of the most important principles in software engineering.
