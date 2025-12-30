//! Database module for PostgreSQL connection and queries
//!
//! This module is only available on the server side (ssr feature).
//!
//! ## Structure
//!
//! - `pool` - Database connection pool management
//! - `player` - Player accounts and authentication
//! - `character` - Characters, stats, and state tracking  
//! - `location` - Towns, locations, and actions
//! - `item` - Items and inventory
//! - `skill` - Skills and abilities
//! - `guild` - Guilds and memberships

#[cfg(feature = "ssr")]
mod pool;

#[cfg(feature = "ssr")]
pub mod player;

#[cfg(feature = "ssr")]
pub mod character;

#[cfg(feature = "ssr")]
pub mod location;

#[cfg(feature = "ssr")]
pub mod item;

#[cfg(feature = "ssr")]
pub mod skill;

#[cfg(feature = "ssr")]
pub mod guild;

// Re-export pool utilities at the top level
#[cfg(feature = "ssr")]
pub use pool::*;

// Re-export all models and queries for convenience
#[cfg(feature = "ssr")]
pub use player::*;

#[cfg(feature = "ssr")]
pub use character::{*, types::CharacterClass};

#[cfg(feature = "ssr")]
pub use location::*;

#[cfg(feature = "ssr")]
pub use item::*;

#[cfg(feature = "ssr")]
pub use skill::*;

#[cfg(feature = "ssr")]
pub use guild::*;


