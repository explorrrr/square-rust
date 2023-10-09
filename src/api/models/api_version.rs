//! Square API version enum

/// Square API version
#[derive(Debug, Clone)]
pub enum SquareApiVersion {
    V20230925,
}

impl SquareApiVersion {
    /// Convert the SquareApiVersion to a string
    pub fn to_string(&self) -> String {
        match self {
            SquareApiVersion::V20230925 => "2023-09-25".to_string(),
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
