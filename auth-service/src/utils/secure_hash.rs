
use argon2::password_hash::{PasswordHash, PasswordVerifier};
use argon2::password_hash::Salt;
use argon2::{Argon2, PasswordHasher};
use getrandom::getrandom;


#[allow(dead_code)]
fn create_and_verify_password() {
	let password = b"super_secret_password";

	// 1. Generate a random 16-byte salt manually
	let mut salt_bytes = [0u8; 16];
	getrandom(&mut salt_bytes).expect("Failed to generate secure random bytes");

	// 2. Create a Salt from raw bytes
	let b64_salt = "base64saltstringhere"; // your Base64 string
	let salt     = Salt::from_b64(b64_salt).expect("Invalid base64 salt");	

	// 3. Create the hasher
	let argon2 = Argon2::default();

	// 4. Hash the password
	let hashed_password = argon2
		.hash_password(password, salt)
		.expect("Password hashing failed")
		.to_string();

	println!("Password hash: {}", hashed_password);

	// 5. Verify
	let parsed_hash   = PasswordHash::new(&hashed_password).expect("Failed to parse password hash");
	let verify_result = argon2.verify_password(password, &parsed_hash).is_ok();
	assert!(verify_result, "Password verification failed");
	println!("Password verified successfully!");
}

#[test]
fn test_it() {
	create_and_verify_password();
}
