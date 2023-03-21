pub mod config;
pub mod router;
pub mod message;
pub mod like;
pub mod error;
pub mod user;

pub use error::{Error, ResultExt};

pub type Result<T, E = Error> = std::result::Result<T, E>;

use tower_http::trace::TraceLayer;