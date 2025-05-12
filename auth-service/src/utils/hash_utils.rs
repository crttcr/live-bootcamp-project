use argon2::password_hash::PasswordHash;
use argon2::password_hash::PasswordHasher;
use argon2::password_hash::PasswordVerifier;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, Version};
use color_eyre::eyre;


type HashResult   = eyre::Result<String>;
type VerifyResult = eyre::Result<()>;


#[tracing::instrument(name = "Hash password(sync)", skip_all)]
pub fn hash_password_sync(password: String) -> HashResult
{
	let bytes  = password.as_bytes();
	let salt   = SaltString::generate(&mut rand::thread_rng());
	let params = Params::new(15000, 2, 1, None)?;
	let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
	let hash   = argon2.hash_password(bytes, &salt)?;
	let hash   = hash.to_string();
	Ok(hash)
	// Forcing an error
	//let err = Box::new(std::io::Error::other("oh no!")) as Box<dyn Error + Send + Sync>;
	//Err(err)
	//Err(eyre::eyre!("Heavens no!"))
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
	//	.wrap_err("Failed ot verify password")"
}

#[tracing::instrument(name = "Verify password hash(async)", skip_all)]
pub async fn verify_password_async(existing: String, password: String) -> VerifyResult
{
	tokio::task::spawn_blocking(move || verify_password_sync(existing, password)).await?
}

