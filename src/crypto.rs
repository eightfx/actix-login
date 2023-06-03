use bcrypt::{hash, DEFAULT_COST, verify};
use crate::models::User;

pub trait HashPassword{
	fn hash_password(password: &str) -> String;
	fn verify_password(&self, password: &str) -> bool;
}


impl HashPassword for User{
	fn hash_password(password: &str) -> String{
		hash(password, DEFAULT_COST).unwrap()
	}

	fn verify_password(&self, password: &str) -> bool{
		verify(password, &self.password_hash).unwrap_or(false)
	}
}


