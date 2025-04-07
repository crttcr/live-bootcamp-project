
// Define routes sub-modules
pub mod login;
pub mod signup;
pub mod verify_2fa;
pub mod verify_token;
pub mod logout;

// Re-export items from sub-modules
pub use login::*;
pub use logout::*;
pub use signup::*;
pub use verify_2fa::*;
pub use verify_token::*;
