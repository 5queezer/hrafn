use anyhow::{Result, anyhow};
use chrono::Utc;
use rand::RngExt;

/// Session identifier: `YYYYMMDD_HHMMSS_<6hex>`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(String);

impl SessionId {
    /// Generate a new ID from the current UTC time plus 24 bits of randomness.
    #[must_use]
    pub fn generate() -> Self {
        let now = Utc::now();
        let stamp = now.format("%Y%m%d_%H%M%S");
        let suffix: u32 = rand::rng().random_range(0..0x0100_0000);
        Self(format!("{stamp}_{suffix:06x}"))
    }

    /// Parse an existing ID, validating the shape.
    pub fn parse(s: &str) -> Result<Self> {
        if s.len() != 22
            || s.as_bytes()[8] != b'_'
            || s.as_bytes()[15] != b'_'
            || !s[..8].bytes().all(|b| b.is_ascii_digit())
            || !s[9..15].bytes().all(|b| b.is_ascii_digit())
            || !s[16..].bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(anyhow!("invalid session id: {s}"));
        }
        Ok(Self(s.to_string()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Short display form: the first 10 characters of the ID (date portion).
    #[must_use]
    pub fn short(&self) -> &str {
        &self.as_str()[..10.min(self.as_str().len())]
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_has_correct_shape() {
        let id = SessionId::generate();
        let s = id.as_str();
        assert_eq!(s.len(), 22, "want len 22, got {} ({})", s.len(), s);
        assert_eq!(&s[8..9], "_");
        assert_eq!(&s[15..16], "_");
        assert!(s[..8].chars().all(|c| c.is_ascii_digit()));
        assert!(s[9..15].chars().all(|c| c.is_ascii_digit()));
        assert!(s[16..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn parse_accepts_valid() {
        let id = SessionId::parse("20260417_205355_53b1e8").unwrap();
        assert_eq!(id.as_str(), "20260417_205355_53b1e8");
    }

    #[test]
    fn parse_rejects_bad_format() {
        assert!(SessionId::parse("").is_err());
        assert!(SessionId::parse("20260417205355_53b1e8").is_err());
        assert!(SessionId::parse("20260417_205355_53b1eZ").is_err());
        assert!(SessionId::parse("20260417_205355_53b1e").is_err());
    }

    #[test]
    fn generate_many_produces_distinct() {
        let mut set = std::collections::HashSet::new();
        for _ in 0..100 {
            assert!(set.insert(SessionId::generate()));
        }
    }
}
