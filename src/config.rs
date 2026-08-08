//! Connection settings for [`crate::TallyClient`].
//!
//! Defaults target a local TallyPrime HTTP listener on port `9000`.

/// Configuration for connecting to TallyPrime over HTTP.
///
/// Use [`Default`] for `localhost:9000` with a 30s timeout and 3 retries.
/// Set [`Self::current_company`] when multiple companies are available or when
/// no company is loaded in the Tally UI.
#[derive(Debug, Clone)]
pub struct TallyConfig {
    /// Hostname or IP of the Tally HTTP server.
    pub host: String,
    /// Tally HTTP port (commonly `9000`).
    pub port: u16,
    /// Per-request timeout in seconds.
    pub timeout_secs: u64,
    /// How many times to retry a failed connection before giving up.
    pub retry_attempts: u32,
    /// Company to target in export/import requests.
    ///
    /// When `None`, the client tries to discover the active company in Tally.
    pub current_company: Option<String>,
    /// Optional Tally.NET account name for remote/authenticated endpoints.
    pub tally_net_account: Option<String>,
    /// Optional Tally.NET password paired with [`Self::tally_net_account`].
    pub tally_net_password: Option<String>,
}

impl Default for TallyConfig {
    /// `localhost:9000`, 30s timeout, 3 retries, no company or Tally.NET credentials.
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 9000,
            timeout_secs: 30,
            retry_attempts: 3,
            current_company: None,
            tally_net_account: None,
            tally_net_password: None,
        }
    }
}
