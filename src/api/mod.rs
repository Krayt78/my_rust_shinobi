//! API module containing server functions
//! 
//! Server functions are called from the client but execute on the server,
//! giving access to the database and other server-side resources.

mod player;

pub use player::*;

