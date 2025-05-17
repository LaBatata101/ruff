use std::fmt::{Display, Formatter};

// FIX: duplicated with ruff_rule_flake8_bandit::bad_file_permissions
#[derive(Debug, PartialEq, Eq)]
pub enum Reason {
    Permissive(u16),
    Invalid,
}

// FIX: duplicated with ruff_rule_flake8_bandit::weak_cryptographic_key
#[derive(Debug, PartialEq, Eq)]
pub enum CryptographicKey {
    Dsa { key_size: u16 },
    Ec { algorithm: String },
    Rsa { key_size: u16 },
}

impl CryptographicKey {
    pub const fn minimum_key_size(&self) -> u16 {
        match self {
            Self::Dsa { .. } | Self::Rsa { .. } => 2048,
            Self::Ec { .. } => 224,
        }
    }

    pub fn is_vulnerable(&self) -> bool {
        match self {
            Self::Dsa { key_size } | Self::Rsa { key_size } => key_size < &self.minimum_key_size(),
            Self::Ec { algorithm } => {
                matches!(algorithm.as_str(), "SECP192R1" | "SECT163K1" | "SECT163R2")
            }
        }
    }
}

impl Display for CryptographicKey {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        match self {
            CryptographicKey::Dsa { .. } => fmt.write_str("DSA"),
            CryptographicKey::Ec { .. } => fmt.write_str("EC"),
            CryptographicKey::Rsa { .. } => fmt.write_str("RSA"),
        }
    }
}

// FIX: duplicated with ruff_rule_flake8_bandit::shell_injection
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Safety {
    SeemsSafe,
    Unknown,
}