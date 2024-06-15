use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims
{
	pub exp:   usize,
	pub iat:   usize,
	pub email: String,
	pub id:    i32
}

pub fn encode_jwt(email: String, id: i32) -> Result<String, jsonwebtoken::errors::Error>
{
	let now = Utc::now();
	let expiry = Duration::minutes(60);

	let claims = Claims { exp: (now + expiry).timestamp() as usize,
	                      iat: now.timestamp() as usize,
	                      id,
	                      email };

	dotenvy::dotenv().ok();

	let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not found");

	encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error>
{
	// println!("Decoding JWT");
	// println!("JWT is {}", jwt);
	dotenvy::dotenv().ok();

	let secret = std::env::var("JWT_SECRET").unwrap();
	// println!("Secret is {}", secret);
	let claim_data: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode(&jwt, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());

	claim_data
}
