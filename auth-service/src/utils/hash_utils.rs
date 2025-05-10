use argon2::password_hash::PasswordHash;
use argon2::password_hash::PasswordHasher;
use argon2::password_hash::PasswordVerifier;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, Version};
use std::error::Error;


type HashResult   = Result<String, Box<dyn Error + Send + Sync>>;
type VerifyResult = Result<(),     Box<dyn Error + Send + Sync>>;


#[tracing::instrument(name = "Hash password(sync)", skip_all)]
pub fn hash_password_sync(password: String) -> HashResult
{
	let bytes  = password.as_bytes();
	let salt   = SaltString::generate(&mut rand::thread_rng());
	let params = Params::new(15000, 2, 1, None)?;
	let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
	let hash   = argon2.hash_password(bytes, &salt)?;
	Ok(hash.to_string())
}

#[tracing::instrument(name = "Hash password(async)", skip_all)]
pub async fn hash_password_async(password: String) -> HashResult 
{
	tokio::task::spawn_blocking(move || hash_password_sync(password)).await?
}

#[tracing::instrument(name = "Verify password hash(sync)", skip_all)]
pub fn verify_password_sync(existing_hash: String, password_candidate: String) -> VerifyResult 
{
	let existing_hash: PasswordHash<'_> = PasswordHash::new(existing_hash.as_str())?;
	Argon2::default()
		.verify_password(password_candidate.as_bytes(), &existing_hash)
		.map_err(|e| e.into())
}

#[tracing::instrument(name = "Verify password hash(async)", skip_all)]
pub async fn verify_password_async(existing: String, password: String) -> VerifyResult
{
	tokio::task::spawn_blocking(move || verify_password_sync(existing, password)).await?
}

