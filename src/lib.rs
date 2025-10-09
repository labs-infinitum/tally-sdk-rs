pub mod errors;
pub mod config;
pub mod xml_builder;
pub mod models;
pub mod client;

pub use crate::client::TallyClient;
pub use crate::errors::*;
pub use crate::models::*;

// Re-export commonly used items
pub use crate::client::voucher_parser;
