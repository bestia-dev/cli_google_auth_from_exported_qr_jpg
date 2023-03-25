// cli_google_auth_from_exported_qr_jpg/src/migration_mod.rs

// functions copied from https://github.com/Levminer/authme/tree/dev/core/crates/google_authenticator_converter

use crate::LibraryError;
use prost::Message;

pub mod gauth_migration_proto_mod;

#[derive(Debug)]
pub struct Account {
    pub name: String,
    pub secret: String,
    pub issuer: String,
}

impl Account {
    pub fn new(name: String, secret: String, mut issuer: String) -> Account {
        // If the issuer is empty, use the name as the issuer.
        if issuer.is_empty() {
            issuer = name.clone()
        }

        Account {
            name,
            secret,
            issuer,
        }
    }
}

/// get qr text from jpg
pub fn get_qr_text_from_jpg(image_file_name: &str) -> Result<String, crate::LibraryError> {
    log::info!("start get_qr_text_from_jpg({})", image_file_name);
    // use the ? syntax to bubble the error up one level or continue (early return)
    let img = image::open(image_file_name)?;

    // Use default decoder
    let decoder = bardecoder::default_decoder();

    let mut results = decoder.decode(&img);
    // I allow only one account in one qr
    if results.len() == 0 {
        return Err(crate::LibraryError::QrCodeEmpty);
    }
    if results.len() > 1 {
        return Err(crate::LibraryError::QrCodeMoreThanOneResult);
    }
    let qr_text = results
        .swap_remove(0)
        .expect("I already checked it has exactly one item in the vector.");

    // return
    Ok(qr_text)
}

/// get qr text from jpg
pub fn generate_otp_token_from_qr_text(qr_text: &str) -> Result<String, crate::LibraryError> {
    log::info!("start generate_otp_token_from_qr_text({})", qr_text);
    // extract seed secret from otpauth_migration format
    let accounts = get_data_from_migration(&qr_text)?;

    // I allow only one account in one qr
    if accounts.len() == 0 {
        return Err(crate::LibraryError::QrCodeEmpty);
    }
    if accounts.len() > 1 {
        return Err(crate::LibraryError::QrCodeMoreThanOneResult);
    }
    let account = accounts
        .get(0)
        .expect("I already checked it has exactly one item in the vector.");

    // println!( "{0}\n{1}\n{2}", account.name, account.secret, account.issuer );
    let secret = account.secret.clone();

    // generate otp token
    // paypal and porkbun have secrets smaller then 128bit !?!
    // This is despite the standard says explicitly that 128 bit is minimal.
    // The standard new() constructor asserts the minimal size of the secret
    // and therefor fails for paypal and porkbun.
    // Because of that I must use the non-standard function new_unchecked()
    let totp = totp_rs::TOTP::new_unchecked(
        totp_rs::Algorithm::SHA1,
        6,
        1,
        30,
        totp_rs::Secret::Encoded(secret).to_bytes().unwrap(),
    );
    let token = totp.generate_current()?;

    // return
    Ok(token)
}

/// Convert a Google Authenticator migration QR code string to a list of accounts
fn get_data_from_migration(uri_string: &str) -> Result<Vec<Account>, LibraryError> {
    let encoded_data = extract_data_from_uri(uri_string)?;

    use base64::Engine;
    let decoded_data = base64::engine::general_purpose::STANDARD.decode(encoded_data)?;

    let migration_payload = gauth_migration_proto_mod::MigrationPayload::decode(
        &mut std::io::Cursor::new(decoded_data),
    )?;

    let otp_parameters = migration_payload.otp_parameters;

    let alphabet = base32::Alphabet::RFC4648 { padding: false };

    let payloads: Vec<Account> = otp_parameters
        .into_iter()
        .map(|p| Account::new(p.name, base32::encode(alphabet, &p.secret), p.issuer))
        .collect();

    return Ok(payloads);
}

fn extract_data_from_uri(raw: &str) -> Result<String, LibraryError> {
    let mut split = raw.split("data=");
    split.next();

    if let Some(encoded_data) = split.next() {
        let s = urlencoding::decode(encoded_data)?;
        Ok(s.to_string())
    } else {
        Err(LibraryError::NoDataFoundInUri)
    }
}
