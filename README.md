[//]: # (auto_md_to_doc_comments segment start A)

# cli_google_auth_from_exported_qr_jpg

[//]: # (auto_cargo_toml_to_md start)

**CLI generates 2FA OTP codes from exported QR jpg images from google authenticator**  
***version: 1.0.71 date: 2023-03-26 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/cli_google_auth_from_exported_qr_jpg)***  

[//]: # (auto_cargo_toml_to_md end)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-311-green.svg)](https://github.com/bestia-dev/cli_google_auth_from_exported_qr_jpg/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-150-blue.svg)](https://github.com/bestia-dev/cli_google_auth_from_exported_qr_jpg/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-58-purple.svg)](https://github.com/bestia-dev/cli_google_auth_from_exported_qr_jpg/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/cli_google_auth_from_exported_qr_jpg/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/cli_google_auth_from_exported_qr_jpg/)

[//]: # (auto_lines_of_code end)

[![crates.io](https://img.shields.io/crates/v/cli_google_auth_from_exported_qr_jpg.svg)](https://crates.io/crates/cli_google_auth_from_exported_qr_jpg) [![Documentation](https://docs.rs/cli_google_auth_from_exported_qr_jpg/badge.svg)](https://docs.rs/cli_google_auth_from_exported_qr_jpg/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/cli_google_auth_from_exported_qr_jpg.svg)](https://web.crev.dev/rust-reviews/crate/cli_google_auth_from_exported_qr_jpg/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/cli_google_auth_from_exported_qr_jpg/) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cli_google_auth_from_exported_qr_jpg/blob/master/LICENSE) [![Rust](https://github.com/bestia-dev/cli_google_auth_from_exported_qr_jpg/actions/workflows/fmt-build-test.yml/badge.svg)](https://github.com/bestia-dev/cli_google_auth_from_exported_qr_jpg/actions/)  

Hashtags: #rust #rustlang #tutorial  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  
Generated documentation on github: <https://bestia-dev.github.io/cli_google_auth_from_exported_qr_jpg/>  

## Motivation

After my friend lost his phone on vacation I realized how 2FA can be a gargantuan pain in the ass. I know by heart all of my primary passwords, but it is not enough anymore.  
Most of the serious internet services use some kind of 2 factor authentication (2FA) today. My 2FA is mostly linked to my phone and I am screwed if I loose it. First it used to be that the internet services sent me an SMS upon login. This was a terrible idea. If I loose my phone, I can buy a new one in a couple of hours, but to get a SIM from my provider with the same telephone number can be tricky. Especially if I am on the other side of the planet in a foreign country. Not to mention how many times the SMS simply does not work for any reason. Scammers allegedly can get around SMS-based 2FA by using social engineering.  

Later all services started to adopt the concept of OTP (one-time password) for 2FA, mostly TOTP (time-based one-time password). Time-based one-time password (TOTP) is a computer algorithm that generates a one-time password (OTP) that uses the current time as a source of uniqueness. TOTP is the cornerstone of Initiative for Open Authentication (OATH), and is used in a number of two-factor authentication (2FA) systems. It has been adopted as an Internet Engineering Task Force (IETF) standard.  

I use the Google Authenticator app on my android phone to generate 2FA TOTP for many internet services. It is alright I guess. TOTP is a standard, so I don't expect many differences between apps. But sadly google authenticator works only on android. If I loose my phone I cannot use a Windows or Linux computer to generate my TOTPs. And this is exactly what I want.

There are many TOTP programs for Windows and Linux, but I just need an excuse to make my own. I am a hobbyist programming teacher/tutor after all.

Google authenticator can "transfer" or "export" single TOTP definitions as a QR code. On android I cannot take a screenshot of this QR code, but I can make a photo of it using another phones camera. Silly old school tricks! So now I have a jpg image with all the seed data needed to generator TOTPs on another device. The text inside the QR code is something like:

```bash
otpauth-migration://offline?data=...
```

I want to save this jpg somewhere safe. It is a kind of little silly obfuscation idea to have the data inside a qr jpg and not in a text format. Security is never 100%, it is just a series of steps to make it harder for the hackers to understand what is going on.  

In case of need I want a simple one-file CLI program to read this QR inside the jpg and generate my OTP. No secrets are ever typed in the terminal, only the jpg file name.

Because of the tutorial nature of this project, sure everything must be coded in Rust.  

## Short name

The development project has a very long and expressive name `cli_google_auth_from_exported_qr_jpg`. For the CLI I want to have a short name like `gajpg`. To achieve that, I just need to change the name of the directory inside the `/src/bin/` directory. I will then call the CLI with a line like this:

```bash
gajpg image1.jpg
```

## Test data

I created a TOTP seed just for testing on <https://totp-online.tobythe.dev/>.

```json
{
    "account":"test",
    "issuer":"tester",
    "secret":"w5eq3367bie6ux4onpi3vh2mr3k3lgu27sn",
    "period":30,
    "digits":6,
    "algorithm":"SHA-1"
}
```
  
Then I used Google authenticator on Android to create a new auth item. Then I "transferred" or "exported" this item and got a qr code on the screen of my phone. I cannot take a screenshot, because of "security". But I can take a photo of the screen with another camera.  

I saved this testing image inside this project as `image1.jpg`.  

## First step read the qr from a jpg

To read the qr from the jpg I will use the crate `bardecoder`. It didn't decode my QR until I edited the image to be clean, black and white with good contrast and around 700x700 pixels. More manual work that I wanted to accept, but it is not impossible and it is done only once.  
The qr text inside this jpg image is:

```text
otpauth-migration://offline?data=CioKFbdJDe%2FfCgnqX45r0bqfTI7VtZqa%2FBILdGVzdGVyOnRlc3QgASgBMAIQARgBIAAo0tfy2Qc%3D
```

## Second step to get secret from migration code

Inside the `otpauth-migration` google has put the account, the issuer and the secret. I need to extract these. I tried some existing crates, to no avail.

**NOTE THAT THE 2FA CODES ARE SECRETS THAT YOU SHOULD TREAT AS SUCH!**  
<https://alexbakker.me/post/parsing-google-auth-export-qr-code.html>

First we need to decode base64. That is not difficult.
But then we have a brand new "protobuf" protocol buffer message.

## Protobuf

Protocol Buffers are a language-neutral, platform-neutral extensible mechanism for serializing structured data created by google. They claim it is performant and efficient, more than json and xml. Instead of a generic coder/decoder, they prepare optimized code for every specific proto message. They created a code generator for 20 languages, but not for Rust. There is the `prost` crate that looks it generates rust code from the `.proto` definition. I created a sub-project called `code_generator` to generate Rust code from the schema.  
The resulting Rust code is copied into `gauth_migration_proto_mod.rs`.  

## Third step to generate OTP

When I have the seed secret it is easy to generate the TOTP using the crate `totp-rs`.

## automation helper

I use my crate `cargo-auto` to code automation tasks like build, release, doc, test,...
All is done in Rust code. Fancy. Start with:  

```bash
cargo auto
```

And follow the detailed instructions. Then continue with the unambiguous command:

```bash
cargo auto build
```

## open-source and free as a beer

My open-source projects are free as a beer (MIT license).

I just love programming.

But I need also to drink. If you find my projects and tutorials helpful,please buy me a beer donating on my [paypal](https://paypal.me/LucianoBestia).

You know the price of a beer in your local bar ;-) So I can drink a free beer for your health :-)

[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//]: # (auto_md_to_doc_comments segment end A)
