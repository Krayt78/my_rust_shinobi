//! Database module for PostgreSQL connection and queries
//! 
//! This module is only available on the server side (ssr feature).

#[cfg(feature = "ssr")]
mod pool;
#[cfg(feature = "ssr")]
mod models;
#[cfg(feature = "ssr")]
mod queries;

#[cfg(feature = "ssr")]
pub use pool::*;
#[cfg(feature = "ssr")]
pub use models::*;
#[cfg(feature = "ssr")]
pub use queries::*;

