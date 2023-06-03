use std::time::SystemTime;
use jsonwebtoken::{EncodingKey, DecodingKey, encode, decode, Header, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use std::result::Result;

use sqlx::{MySql, Pool};
use actix_web::{web, HttpRequest, http::header};
use crate::models::User;
use crate::error::AuthError;
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
	sub: String,
	exp: u64,
}

pub struct TokenService {
	encoding_key: EncodingKey,
	decoding_key: DecodingKey,
}

impl TokenService {

	pub fn new() -> Self {
		let binding = &std::env::var("AUTH_SECRET").unwrap();
		let secret = binding.as_bytes();
		Self {
			encoding_key: EncodingKey::from_secret(secret),
			decoding_key: DecodingKey::from_secret(secret),
		}
	}

	pub fn create_jwt(&self, user_id: &str) -> Result<String, AuthError> {
		let expiration = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH).unwrap()
			.as_secs() + 604800; // 1 weeks
		
		let claims = Claims {
			sub: user_id.to_owned(),
			exp: expiration,
		};
		
		encode(&Header::default(), &claims, &self.encoding_key).map_err(AuthError::from)
	}

	pub fn verify_jwt(&self, header: &str) -> Result<String, AuthError> {
		let token = Self::strip_scheme(header);
		let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::new(Algorithm::HS256))
			.map_err(AuthError::from)?;
		Ok(token_data.claims.sub)
	}

	fn strip_scheme(header_value: &str) -> &str{
		let parts: Vec<&str> = header_value.splitn(2, ' ').collect();
		if parts.len() == 2 {
			parts[1]
		} else {
			header_value
		}
	}

	pub async fn get_user(db_pool: &web::Data<Pool<MySql>>, req:&HttpRequest) -> Result<User, AuthError>{
		let headers = req.headers();
		match headers.get(header::AUTHORIZATION) {
			Some(header_value) => {
				let token = header_value.to_str().unwrap_or("");
				// Verify the token
				match TokenService::new().verify_jwt(token) {
					Ok(user_id) => {
						// Fetch the user from the database
						dbg!(&user_id);
						let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_id = ?")
							.bind(&user_id)
							.fetch_one(&***db_pool)
							.await;
						match user {
							Ok(user) => Ok(user),
							Err(_) => Err(AuthError::InvalidCredentials),
						}
					},
					Err(_) => Err(AuthError::TokenInvalid)
				}
			},
			None => Err(AuthError::TokenMissing)
		}
		
	}



}

