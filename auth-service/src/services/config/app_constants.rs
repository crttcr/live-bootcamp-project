
pub const DEFAULT_REDIS_HOSTNAME:  &str = "127.0.0.1";
pub const JWT_COOKIE_NAME:         &str = "jwt";
pub const ACTIVE_TOKEN_KEY_PREFIX: &str = "2FA:Tokens:Active";
pub const BANNED_TOKEN_KEY_PREFIX: &str = "2FA:Tokens:Banned";
pub const TOKEN_TTL_SECONDS:       i64  = 600; // 10 minutes