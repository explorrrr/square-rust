pub mod create_payment;

use crate::api::models::payments::requests::create_payment::versions::v20230923::CreatePaymentRequestV20230925;

pub enum CreatePaymentRequest {
    CreatePaymentRequestV20230923(CreatePaymentRequestV20230925),

}
