use secrecy::Secret;
use secrecy::ExposeSecret;

pub trait MaskSecret {
	fn masked(&self) -> String;
}


impl MaskSecret for Secret<String> {
	fn masked(&self) -> String {
		let secret = self.expose_secret();
		let len = secret.len();

		if len <= 4 {
			"*".repeat(len)
		} else {
			format!("{}{}", &secret[..4], "*".repeat(len - 4))
		}
	}
}