use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub enum ClientIdentifier {
    /// Chrome Profiles
    #[serde(rename = "chrome_103")]
    Chrome103,
    #[serde(rename = "chrome_104")]
    Chrome104,
    #[serde(rename = "chrome_105")]
    Chrome105,
    #[serde(rename = "chrome_106")]
    Chrome106,
    #[serde(rename = "chrome_107")]
    Chrome107,
    #[serde(rename = "chrome_108")]
    Chrome108,
    #[serde(rename = "chrome_109")]
    Chrome109,
    #[serde(rename = "chrome_110")]
    Chrome110,
    #[serde(rename = "chrome_111")]
    Chrome111,
    #[serde(rename = "chrome_112")]
    Chrome112,
    #[serde(rename = "chrome_116_PSK")]
    Chrome116Psk,
    #[serde(rename = "chrome_116_PSK_PQ")]
    Chrome116PskPq,
    #[serde(rename = "chrome_117")]
    Chrome117,
    #[default]
    #[serde(rename = "chrome_120")]
    Chrome120,

    /// Safari Profiles (incl. IOS)
    #[serde(rename = "safari_15_6_1")]
    Safari1561,
    #[serde(rename = "safari_16_0")]
    Safari160,
    #[serde(rename = "safari_ios_15_5")]
    SafariIOS155,
    #[serde(rename = "safari_ios_15_6")]
    SafariIOS156,
    #[serde(rename = "safari_ios_16_0")]
    SafariIOS160,
    #[serde(rename = "safari_ios_15_6")]
    SafariIPadOS156,

    /// Firefox profiles
    #[serde(rename = "firefox_102")]
    Firefox102,
    #[serde(rename = "firefox_104")]
    Firefox104,
    #[serde(rename = "firefox_105")]
    Firefox105,
    #[serde(rename = "firefox_106")]
    Firefox106,
    #[serde(rename = "firefox_108")]
    Firefox108,
    #[serde(rename = "firefox_110")]
    Firefox110,
    #[serde(rename = "firefox_117")]
    Firefox117,

    /// Opera profiles
    #[serde(rename = "opera_89")]
    Opera89,
    #[serde(rename = "opera_90")]
    Opera90,
    #[serde(rename = "opera_91")]
    Opera91,

    /// Custom profiles
    #[serde(rename = "zalando_ios_mobile")]
    ZalandoIOSMobile,
    #[serde(rename = "nike_ios_mobile")]
    NikeIOSMobile,
    #[serde(rename = "Cloudscraper")]
    Cloudscraper,
    #[serde(rename = "mms_ios")]
    //#[serde(rename = "mms_ios_1")]
    MmsIOS1,
    #[serde(rename = "mms_ios_2")]
    MmsIOS2,
    #[serde(rename = "mms_ios_3")]
    MmsIOS3,
    #[serde(rename = "mesh_ios")]
    //#[serde(rename = "mesh_ios_1")]
    MeshIOS1,
    #[serde(rename = "confirmed_ios")]
    ConfirmedIOS,
}

impl Into<String> for ClientIdentifier {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}


// TODO: CHECK GPT GENERATED CODE:

// H2Settings
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
pub enum H2Setting {
    #[serde(rename = "HEADER_TABLE_SIZE")]
    HeaderTableSize,
    #[serde(rename = "ENABLE_PUSH")]
    EnablePush,
    #[serde(rename = "MAX_CONCURRENT_STREAMS")]
    MaxConcurrentStreams,
    #[serde(rename = "INITIAL_WINDOW_SIZE")]
    InitialWindowSize,
    #[serde(rename = "MAX_FRAME_SIZE")]
    MaxFrameSize,
    #[serde(rename = "MAX_HEADER_LIST_SIZE")]
    MaxHeaderListSize,
}

// Supported Versions
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum SupportedVersion {
    #[serde(rename = "GREASE")]
    Grease,
    #[serde(rename = "1.3")]
    V1_3,
    #[serde(rename = "1.2")]
    V1_2,
    #[serde(rename = "1.1")]
    V1_1,
    #[serde(rename = "1.0")]
    V1_0,
}

// Supported Signature Algorithms
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum SignatureAlgorithm {
    #[serde(rename = "PKCS1WithSHA256")]
    PKCS1WithSHA256,
    #[serde(rename = "PKCS1WithSHA384")]
    PKCS1WithSHA384,
    #[serde(rename = "PKCS1WithSHA512")]
    PKCS1WithSHA512,
    #[serde(rename = "PSSWithSHA256")]
    PSSWithSHA256,
    #[serde(rename = "PSSWithSHA384")]
    PSSWithSHA384,
    #[serde(rename = "PSSWithSHA512")]
    PSSWithSHA512,
    #[serde(rename = "ECDSAWithP256AndSHA256")]
    ECDSAWithP256AndSHA256,
    #[serde(rename = "ECDSAWithP384AndSHA384")]
    ECDSAWithP384AndSHA384,
    #[serde(rename = "ECDSAWithP521AndSHA512")]
    ECDSAWithP521AndSHA512,
    #[serde(rename = "PKCS1WithSHA1")]
    PKCS1WithSHA1,
    #[serde(rename = "ECDSAWithSHA1")]
    ECDSAWithSHA1,
    #[serde(rename = "Ed25519")]
    Ed25519,
}

// CertCompressionAlgorithm
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum CertCompressionAlgorithm {
    #[serde(rename = "zlib")]
    Zlib,
    #[serde(rename = "brotli")]
    Brotli,
    #[serde(rename = "zstd")]
    Zstd,
}

// Supported Delegated Credentials
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum DelegatedCredential {
    #[serde(rename = "PKCS1WithSHA256")]
    PKCS1WithSHA256,
    #[serde(rename = "PKCS1WithSHA384")]
    PKCS1WithSHA384,
    #[serde(rename = "PKCS1WithSHA512")]
    PKCS1WithSHA512,
    #[serde(rename = "PSSWithSHA256")]
    PSSWithSHA256,
    #[serde(rename = "PSSWithSHA384")]
    PSSWithSHA384,
    #[serde(rename = "PSSWithSHA512")]
    PSSWithSHA512,
    #[serde(rename = "ECDSAWithP256AndSHA256")]
    ECDSAWithP256AndSHA256,
    #[serde(rename = "ECDSAWithP384AndSHA384")]
    ECDSAWithP384AndSHA384,
    #[serde(rename = "ECDSAWithP521AndSHA512")]
    ECDSAWithP521AndSHA512,
    #[serde(rename = "PKCS1WithSHA1")]
    PKCS1WithSHA1,
    #[serde(rename = "ECDSAWithSHA1")]
    ECDSAWithSHA1,
    #[serde(rename = "Ed25519")]
    Ed25519,
    #[serde(rename = "SHA224_RSA")]
    SHA224RSA,
    #[serde(rename = "SHA224_ECDSA")]
    SHA224ECDSA,
}

// KeyShareCurves
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum KeyShareCurve {
    #[serde(rename = "GREASE")]
    Grease,
    #[serde(rename = "P256")]
    P256,
    #[serde(rename = "P384")]
    P384,
    #[serde(rename = "P521")]
    P521,
    #[serde(rename = "X25519")]
    X25519,
}

// KdfIds
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum KdfId {
    #[serde(rename = "HKDF_SHA256")]
    HKDF_SHA256,
    #[serde(rename = "HKDF_SHA384")]
    HKDF_SHA384,
    #[serde(rename = "HKDF_SHA512")]
    HKDF_SHA512,
}

// AeadIds
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum AeadId {
    #[serde(rename = "AEAD_AES_128_GCM")]
    AEAD_AES_128_GCM,
    #[serde(rename = "AEAD_AES_256_GCM")]
    AEAD_AES_256_GCM,
    #[serde(rename = "AEAD_CHACHA20_POLY1305")]
    AEAD_CHACHA20_POLY1305,
}