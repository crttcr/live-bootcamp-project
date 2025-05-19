
use secrecy::{ExposeSecret, Secret};
use crate::services::config::smoker_private;


pub trait Smoker: smoker_private::Helper {
	const STARS: &'static str = "****************************************";
	
	fn raw(&self) -> &str;
	
	/// Let the smoker see the secret value
	fn stars_exact_length(&self) -> String {
		let raw = self.raw();
		"*".repeat(raw.len())
	}
	
	fn stars_standard(&self) -> String {
		Self::STARS.to_string()
	}

	fn stars_leading(&self) -> String {
		let raw  = self.raw();
		let show = self.number_of_characters_to_reveal(raw);
		let hide = Self::STARS.len() - show;
		let from = raw.len() - show;
		let open = &raw[from..];
		format!("{}{}", "*".repeat(hide + 1), open)
	}
	
	fn stars_trailing(&self) -> String {
		let raw  = self.raw();
		let show = self.number_of_characters_to_reveal(raw);
		let hide = Self::STARS.len() - show;
		let open = &raw[..show];
		format!("{}{}", open, "*".repeat(hide + 1))
	}
}

impl Smoker for Secret<String> {
	fn raw(&self) -> &str {
		self.expose_secret().as_str()
	}
}
