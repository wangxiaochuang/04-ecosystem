use core::fmt;
use std::str::FromStr;

use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chacha20poly1305::{
    aead::{Aead, OsRng},
    AeadCore, ChaCha20Poly1305, KeyInit,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    name: String,
    #[serde(rename = "private_age")]
    age: u8,
    date_of_birth: DateTime<Utc>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    skills: Vec<String>,
    state: WorkState,
    #[serde(serialize_with = "b64_encode", deserialize_with = "b64_decode")]
    data: Vec<u8>,
    // #[serde(
    //     serialize_with = "serialize_encrypt",
    //     deserialize_with = "deserialize_decrypt"
    // )]
    // sensitive: String,
    #[serde_as(as = "DisplayFromStr")]
    sensitive: SensitiveData,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    url: Vec<http::Uri>,
}

#[derive(Debug)]
struct SensitiveData(String);

impl fmt::Display for SensitiveData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let encrypted = encrypt(self.0.as_bytes()).unwrap();
        write!(f, "{}", encrypted)
    }
}

impl FromStr for SensitiveData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let decrypted = decrypt(s)?;
        Ok(SensitiveData(String::from_utf8(decrypted)?))
    }
}

impl SensitiveData {
    fn new(data: impl Into<String>) -> Self {
        Self(data.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
#[serde(rename_all = "camelCase", tag = "type", content = "details")]
enum WorkState {
    Working(String),
    OnLeave(DateTime<Utc>),
    Terminated,
}

fn main() -> Result<()> {
    // let state = WorkState::Working("Rust Engineer".to_string());
    let state1 = WorkState::OnLeave(Utc::now());
    let user = User {
        name: "John Doe".to_string(),
        age: 42,
        date_of_birth: Utc::now(),
        // skills: vec!["Rust".to_string(), "Serde".to_string()],
        skills: vec![],
        state: state1,
        data: vec![1, 2, 3, 4, 5],
        sensitive: SensitiveData::new("secret"),
        url: vec!["https://www.rust-lang.org".parse()?],
    };

    let json = serde_json::to_string(&user)?;
    println!("{}", json);

    let user1: User = serde_json::from_str(&json)?;
    println!("{:?}", user1);
    Ok(())
}

fn b64_encode<S>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encoded = URL_SAFE_NO_PAD.encode(data);
    serializer.serialize_str(&encoded)
}

fn b64_decode<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encoded = String::deserialize(deserializer)?;
    let decoded = URL_SAFE_NO_PAD
        .decode(encoded)
        .map_err(serde::de::Error::custom)?;
    Ok(decoded)
}

#[allow(dead_code)]
fn serialize_encrypt<S>(data: &String, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encrypted = encrypt(data.as_bytes()).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&encrypted)
}

#[allow(dead_code)]
fn deserialize_decrypt<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encoded = String::deserialize(deserializer)?;
    let decrypted = decrypt(&encoded).map_err(serde::de::Error::custom)?;
    let decoded = String::from_utf8(decrypted).map_err(serde::de::Error::custom)?;
    Ok(decoded)
}

const KEY: &[u8; 32] = b"01234567890123456789012345678901";
/// encrypt with chacha20poly1305 and then encode with base64
fn encrypt(data: &[u8]) -> Result<String> {
    let cipher = ChaCha20Poly1305::new(KEY.into());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, data).unwrap();
    let nonce_ciphertext: Vec<_> = nonce.iter().copied().chain(ciphertext).collect();
    let encoded = URL_SAFE_NO_PAD.encode(nonce_ciphertext);
    Ok(encoded)
}

/// decode with base64 and then decrypt with chacha20poly1305
/// the input is base64 encoded ciphertext
fn decrypt(encoded: &str) -> Result<Vec<u8>> {
    let decoded = URL_SAFE_NO_PAD.decode(encoded.as_bytes())?;
    let cipher = ChaCha20Poly1305::new(KEY.into());
    let nonce = decoded[..12].into();
    let decrypted = cipher.decrypt(nonce, &decoded[12..]).unwrap();
    Ok(decrypted)
}
