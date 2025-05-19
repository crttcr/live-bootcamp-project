use std::fmt;
use std::fmt::{Display, Formatter};
use secrecy::Secret;
use config::{Config, ConfigError, File, Environment};
use secrecy::ExposeSecret;
use serde::Deserialize;
use std::sync::LazyLock;
use crate::services::config::smoker::Smoker;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
	pub app_name:    String,
	pub port:        u16,
	pub jwt_secret: Secret<String>,
}

impl AppConfig {
	pub fn format(&self) -> FormattedConfig<'_> {
		FormattedConfig(self)
	}
}

pub struct FormattedConfig<'a>(&'a AppConfig);

impl<'a> Display for FormattedConfig<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		writeln!(f, "Configuration Summary:")?;
		writeln!(f, "  App Name   : {}", self.0.app_name)?;
		writeln!(f, "  Port       : {}", self.0.port)?;
		writeln!(f, "  JWT Secret : {}", self.0.jwt_secret.stars_leading())?;
		writeln!(f, "  JWT Secret : {}", mask_secret(self.0.jwt_secret.expose_secret()))
	}
}

fn mask_secret(secret: &str) -> String {
	if secret.len() <= 4 {
		"*".repeat(secret.len())
	} else {
		format!("{}{}", "*".repeat(secret.len() - 4), &secret[secret.len() - 4..])
	}
}

pub static SETTINGS: LazyLock<AppConfig> = LazyLock::new(|| {
	load_config().expect("Failed to load configuration")
});

fn load_config() -> Result<AppConfig, ConfigError> {
	let builder = Config::builder()
		.add_source(File::with_name("Config").required(false))
		.add_source(Environment::default().separator("_"));

	builder.build()?.try_deserialize()
}

pub fn get() -> &'static AppConfig {
	&SETTINGS
}
