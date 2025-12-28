//! API module containing server functions
//!
//! Server functions are called from the client but execute on the server,
//! giving access to the database and other server-side resources.

mod location;
mod player;

pub use location::*;
pub use player::*;
