//! Sentinel config parser — reads `.sentinel.toml` from the contract repo root.

use serde::{Deserialize, Serialize};

/// Top-level config loaded from `.sentinel.toml`.
#[derive(Debug, Serialize, Deserialize)]
pub struct SentinelConfig {
    pub version: String,
    pub contracts: Vec<ContractConfig>,
}

/// Per-contract configuration block.
#[derive(Debug, Serialize, Deserialize)]
pub struct ContractConfig {
    /// Contract crate name (must match `[package].name` in the crate's Cargo.toml).
    pub name: String,
    /// Path to the contract crate relative to workspace root.
    pub path: String,
    /// Invariant templates to apply. Options: "balance_conservation", "access_control", "no_overflow".
    pub invariants: Vec<String>,
    /// Maximum fuzzing duration in seconds per harness.
    #[serde(default = "default_fuzz_timeout")]
    pub fuzz_timeout_secs: u64,
    /// Maximum number of fuzzing iterations (0 = unlimited within timeout).
    #[serde(default)]
    pub fuzz_iterations: u64,
}

fn default_fuzz_timeout() -> u64 {
    60
}

impl SentinelConfig {
    /// Parse config from a TOML string.
    pub fn from_str(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example_config() {
        let raw = r#"
version = "1"

[[contracts]]
name = "sentinel-token"
path = "contracts/token"
invariants = ["balance_conservation", "no_overflow"]
fuzz_timeout_secs = 120

[[contracts]]
name = "sentinel-vault"
path = "contracts/vault"
invariants = ["access_control", "no_overflow"]
"#;
        let cfg = SentinelConfig::from_str(raw).unwrap();
        assert_eq!(cfg.contracts.len(), 2);
        assert_eq!(cfg.contracts[0].name, "sentinel-token");
        assert_eq!(cfg.contracts[1].invariants, vec!["access_control", "no_overflow"]);
    }
}
