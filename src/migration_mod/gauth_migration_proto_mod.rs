// auto-generated from schema gauth_migration.proto 2023-03-25
// Don't edit, just regenerate if needed. Use this Rust project:
// ~/rustprojects/cli_google_auth_from_exported_qr_jpg/code_generator/

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationPayload {
    #[prost(message, repeated, tag = "1")]
    pub otp_parameters: ::prost::alloc::vec::Vec<migration_payload::OtpParameters>,
    #[prost(int32, tag = "2")]
    pub version: i32,
    #[prost(int32, tag = "3")]
    pub batch_size: i32,
    #[prost(int32, tag = "4")]
    pub batch_index: i32,
    #[prost(int32, tag = "5")]
    pub batch_id: i32,
}
/// Nested message and enum types in `MigrationPayload`.
pub mod migration_payload {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OtpParameters {
        #[prost(bytes = "vec", tag = "1")]
        pub secret: ::prost::alloc::vec::Vec<u8>,
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub issuer: ::prost::alloc::string::String,
        #[prost(enumeration = "Algorithm", tag = "4")]
        pub algorithm: i32,
        #[prost(enumeration = "DigitCount", tag = "5")]
        pub digits: i32,
        #[prost(enumeration = "OtpType", tag = "6")]
        pub r#type: i32,
        #[prost(int64, tag = "7")]
        pub counter: i64,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Algorithm {
        Unspecified = 0,
        Sha1 = 1,
        Sha256 = 2,
        Sha512 = 3,
        Md5 = 4,
    }
    impl Algorithm {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Algorithm::Unspecified => "ALGORITHM_UNSPECIFIED",
                Algorithm::Sha1 => "ALGORITHM_SHA1",
                Algorithm::Sha256 => "ALGORITHM_SHA256",
                Algorithm::Sha512 => "ALGORITHM_SHA512",
                Algorithm::Md5 => "ALGORITHM_MD5",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ALGORITHM_UNSPECIFIED" => Some(Self::Unspecified),
                "ALGORITHM_SHA1" => Some(Self::Sha1),
                "ALGORITHM_SHA256" => Some(Self::Sha256),
                "ALGORITHM_SHA512" => Some(Self::Sha512),
                "ALGORITHM_MD5" => Some(Self::Md5),
                _ => None,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DigitCount {
        Unspecified = 0,
        Six = 1,
        Eight = 2,
    }
    impl DigitCount {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DigitCount::Unspecified => "DIGIT_COUNT_UNSPECIFIED",
                DigitCount::Six => "DIGIT_COUNT_SIX",
                DigitCount::Eight => "DIGIT_COUNT_EIGHT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DIGIT_COUNT_UNSPECIFIED" => Some(Self::Unspecified),
                "DIGIT_COUNT_SIX" => Some(Self::Six),
                "DIGIT_COUNT_EIGHT" => Some(Self::Eight),
                _ => None,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OtpType {
        Unspecified = 0,
        Hotp = 1,
        Totp = 2,
    }
    impl OtpType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OtpType::Unspecified => "OTP_TYPE_UNSPECIFIED",
                OtpType::Hotp => "OTP_TYPE_HOTP",
                OtpType::Totp => "OTP_TYPE_TOTP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OTP_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "OTP_TYPE_HOTP" => Some(Self::Hotp),
                "OTP_TYPE_TOTP" => Some(Self::Totp),
                _ => None,
            }
        }
    }
}
