//! Square API version enum
use std::fmt;

/// Square API version
#[derive(Debug, Clone)]
pub enum SquareApiVersion {
    V20230925,
}

impl fmt::Display for SquareApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SquareApiVersion::V20230925 => write!(f, "2023-09-25"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(SquareApiVersion::V20230925.to_string(), "2023-09-25".to_string());
    }
}
