#[derive(Debug, Clone)]
pub struct TallyConfig {
    pub host: String,
    pub port: u16,
    pub timeout_secs: u64,
    pub retry_attempts: u32,
    pub tally_net_account: Option<String>,
    pub tally_net_password: Option<String>,
}

impl Default for TallyConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 9000,
            timeout_secs: 30,
            retry_attempts: 3,
            tally_net_account: None,
            tally_net_password: None,
        }
    }
}
