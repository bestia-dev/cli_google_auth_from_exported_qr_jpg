// cli_google_auth_from_exported_qr_jpg/src/lib.rs

#![doc=include_str!("../README.md")]

// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`

// The `lib.rs` does not have any real code. All the code is in modules in separate files.
// The `lib.rs` has just the list of modules, it publishes module's functions or class for the caller
// and it has some global stuff like the Error enum.

// access to modules
mod migration_mod;

// `pub use` allows the caller of the lib to access modules functions, structs or all(*)
pub use migration_mod::generate_otp_token_from_qr_text;
pub use migration_mod::get_qr_text_from_jpg;

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.

/// all possible library errors for `thiserror`
use thiserror::Error;
#[derive(Error, Debug)]
pub enum LibraryError {
    // TODO: 2023-03-26 wrote Issue to totp_rs to add traits Error and Display to SecretParseError
    //#[error(transparent)]
    //SecretParseError(#[from] totp_rs::SecretParseError),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error(transparent)]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    DecodeError(#[from] base64::DecodeError),
    #[error(transparent)]
    ProstDecodeError(#[from] prost::DecodeError),
    #[error(transparent)]
    TotpUrlError(#[from] totp_rs::TotpUrlError),

    #[error("QRCode empty")]
    QrCodeEmpty,
    #[error("QRCode contains more than one result")]
    QrCodeMoreThanOneResult,

    #[error("No data found in URI")]
    NoDataFoundInUri,

    #[error("Unknown error.")]
    Unknown,
}

// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";
