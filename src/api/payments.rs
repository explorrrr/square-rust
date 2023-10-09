use crate::config::SquareApiConfig;

const DEFAULT_API_URL : &str = "/payments";

pub struct PaymentsApi {
    config: SquareApiConfig,
}

impl PaymentsApi {
    pub fn new(config: SquareApiConfig) -> Self {
        PaymentsApi {
            config,
        }
    }

    pub fn create_payment(
        &self
    ) {
        println!("create_payment");
    }
}
