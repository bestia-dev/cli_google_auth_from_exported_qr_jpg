// cli_google_auth_from_exported_qr_jpg/src/bin/gajpg/main.rs

//! Command line executable binary to run from bash terminal
//! It uses the library cli_google_auth_from_exported_qr_jpg.

// This `main.rs` is the code for the CLI application.
// The build of this project will create the CLI application.
// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`
// This `lib.rs` can be used as dependency crate for other projects.

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.

use cli_google_auth_from_exported_qr_jpg as lib;
// Linux terminal colors
use lib::{RED, RESET, YELLOW};

/// entry point into the bin-executable
fn main() {
    // logging is essential for every project
    pretty_env_logger::init();

    // super simple argument parsing. There are crates that can parse more complex arguments.
    match std::env::args().nth(1).as_deref() {
        // calling like: gajpg --help
        None | Some("--help") | Some("-h") => print_help(),
        // calling like: gajpg image1.jpg
        Some(image_file_name) => generate_otp(image_file_name),
    }
}

/// print help
fn print_help() {
    println!(
        r#"
    {YELLOW}Welcome to gajpg !
    The CLI gajpg generates 2FA OTP codes from exported QR jpg from google authenticator.
    The program is written in Rust.{RESET}

gajpg --help
gajpg image1.jpg
  
    © 2023 bestia.dev  MIT License github.com/automation-tasks-rs/cargo-auto
"#
    );
}

/// generate otp from image_file_name
fn generate_otp(image_file_name: &str) {
    /// I divided this function into two, so I can use the ? error propagation syntax
    /// this internal function returns errors that must be dealt with in the caller code
    fn generate_otp_returns_err(image_file_name: &str) -> anyhow::Result<()> {
        // the function from `lib.rs`, can return error
        let qr_text = lib::get_qr_text_from_jpg(image_file_name)?;
        let otp_token = lib::generate_otp_token_from_qr_text(&qr_text)?;
        println!("{}", otp_token);

        Ok(())
    }

    // all errors must be dealt with inside this function
    match generate_otp_returns_err(image_file_name) {
        // do nothing
        Ok(()) => (),
        // log error from anyhow
        Err(err) => println!("{RED}Error: {err}{RESET}"),
    }
}
