//! # Environment for Square API calls

/// The Square API environment
#[derive(Clone, Debug, PartialEq)]
pub enum Environment {
    Mock,
    Sandbox,
    Production,
}

impl Default for Environment {
    /// The default Square API environment
    fn default() -> Self {
        Environment::Sandbox
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(Environment::default(), Environment::Sandbox);
    }
}
