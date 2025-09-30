// Declare the submodule
pub mod auth;

// Re-export the public functions so they are accessible via `groups::auth`
pub use auth::{init_db, register_user, login_user};
