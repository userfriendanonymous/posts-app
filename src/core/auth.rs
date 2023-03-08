use jsonwebtoken::{Header, encode, decode, Algorithm, EncodingKey, DecodingKey, Validation};
use rand::Rng;
use serde::{Serialize, Deserialize};
use pwhash::bcrypt;

#[derive(Debug)]
pub struct Tokens {
    pub access: String,
    pub key: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    pub name: String,
    pub key: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct KeyClaims {
    pub key: String,
    pub exp: usize,
}

#[derive(Debug)]
pub struct Info {
    pub name: String,
}

pub enum Auth {
    Valid {
        info: Info
    },

    Invalid(String)
}

impl Auth {
    pub fn unwrap_invalid_message(&self) -> String {
        let Auth::Invalid(message) = self else {
            panic!("Called unwrap_invalid on Auth::Valid");
        };

        message.clone()
    }
}

mod keys {
    use std::sync::Mutex;

    #[derive(Clone, Debug)] // remove debug!
    pub struct Object {
        pub access: String,
        pub key: String,
    }

    pub static OBJECT: Mutex<Option<Object>> = Mutex::new(None);
}

pub fn get_keys() -> keys::Object {
    match &*keys::OBJECT.lock().unwrap() {
        Some(keys) => keys.clone(),
        None => {
            dotenv::dotenv().ok();
            keys::Object {
                access: std::env::var("ACCESS_KEY").unwrap(),
                key: std::env::var("KEY_KEY").unwrap()
            }
        }
    }
}

pub fn get_tokens(info: Info) -> Result<Tokens, String> {
    let exp = chrono::Utc::now()
    .checked_add_signed(chrono::Duration::days(30))
    .expect("valid timestamp")
    .timestamp() as usize;

    let mut rng = rand::thread_rng();
    let key: String = (0..20).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();

    let access_claims = AccessClaims {
        name: info.name.clone(),
        key: key.clone(),
        exp,
    };

    let key_claims = KeyClaims {
        key,
        exp,
    };

    let keys = get_keys();
    let header = Header::new(Algorithm::HS512);

    let access_token = encode(&header, &access_claims, &EncodingKey::from_secret(&keys.access.as_bytes()))
    .or_else(|error| Err(error.to_string()))?;

    let key_token = encode(&header, &key_claims, &EncodingKey::from_secret(&keys.key.as_bytes()))
    .or_else(|error| Err(error.to_string()))?;

    println!("get tokens.: access - {}, key - {}", access_token.clone(), key_token.clone());

    Ok(Tokens {
        access: access_token,
        key: key_token
    })
}

pub fn verify_tokens(tokens: &Tokens) -> Auth {
    let tokens = Tokens {
        access: String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJuYW1lIjoibG9saSIsImtleSI6IjJnR3FqR01XMHlkMFZ5MFUwbVBBIiwiZXhwIjoxNjgwODYzNjY0LCJzdWIiOiJsb2xpIn0.67rCj2lEKzkdCCPYucaKDfF-rESdO6mI2DH3HDZ4yTRZMxEpCaee59nqokIGWVCGSxe-sbeWrb_HbOpMh20WPw"),
        key: String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJrZXkiOiIyZ0dxakdNVzB5ZDBWeTBVMG1QQSIsImV4cCI6MTY4MDg2MzY2NCwic3ViIjoibG9saSA6a2V5In0.zHlbesYBo859dYjAl_aZnYI6ZudCLHhhlQ69Eda3CdknKYBRSice-FtYTxzIahsgbv5oOv1jTK4SKXN0NTF2lg")
    };
    let keys = get_keys();
    println!("trying to auth: KEYS - {keys:?}, TOKENS - {tokens:?}");

    let access_claims = match decode::<AccessClaims>(
        &tokens.access,
        &DecodingKey::from_secret(&keys.access.as_bytes()),
        &Validation::new(Algorithm::HS512)
    ) {
        Ok(claims) => claims,
        Err(error) => return Auth::Invalid(format!("access token - {}", error.to_string()))
    }.claims;

    let key_claims = match decode::<KeyClaims>(
        &tokens.key,
        &DecodingKey::from_secret(&keys.key.as_bytes()),
        &Validation::new(Algorithm::HS512)
    ) {
        Ok(claims) => claims,
        Err(error) => return Auth::Invalid(format!("key token - {}", error.to_string()))
    }.claims;

    if key_claims.key != access_claims.key {
        return Auth::Invalid("Keys don't match".to_string());
    }

    Auth::Valid {
        info: Info {
            name: access_claims.name
        }
    }
}

pub fn hash_password(password: String) -> Result<String, String> {
    bcrypt::hash(password)
    .map_err(|error| error.to_string())
}

pub fn compare_password(password: String, hash: String) -> bool {
    bcrypt::verify(password.as_str(), hash.as_str())
}