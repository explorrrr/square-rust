//! Models for the CreateCustomerRequestV20230925 endpoint.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::models::objects::address::Address;
use crate::api::models::objects::customer_tax_ids::CustomerTaxIds;

/// Creates a new customer for a business. For api version 2023-09-25

/// You must provide at least one of the following values in your request to this endpoint:

/// - given_name
/// - family_name
/// - company_name
/// - email_address
/// - phone_number
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerRequestV20230925 {
    /// The idempotency key for the request. For more information, see [Idempotency](https://developer.squareup.com/docs/build-basics/common-api-patterns/idempotency).
    pub idempotency_key: Option<String>,
    /// The given name (that is, the first name) associated with the customer profile.
    /// The maximum length for this value is 300 characters.
    pub given_name: Option<String>,
    /// The family name (that is, the last name) associated with the customer profile.
    /// The maximum length for this value is 300 characters.
    pub family_name: Option<String>,
    /// A business name associated with the customer profile.
    /// The maximum length for this value is 500 characters.
    pub company_name: Option<String>,
    /// A nickname for the customer profile.
    /// The maximum length for this value is 100 characters.
    pub nickname: Option<String>,
    /// The email address associated with the customer profile.
    /// The maximum length for this value is 254 characters.
    pub email_address: Option<String>,
    /// The physical address associated with the customer profile. For maximum length constraints, see
    /// [Customer addresses](https://developer.squareup.com/docs/customers-api/use-the-api/keep-records#address). The first_name and last_name fields are ignored if they are present in the request.
    pub address: Option<Address>,
    /// The phone number associated with the customer profile. The phone number must be valid and can contain 9–16 digits,
    ///  with an optional + prefix and country code. For more information, see [Customer phone numbers](https://developer.squareup.com/docs/customers-api/use-the-api/keep-records#phone-number).
    pub phone_number: Option<String>,
    /// An optional second ID used to associate the customer profile with an entity in another system.
    /// The maximum length for this value is 100 characters.
    pub reference_id: Option<String>,
    /// A custom note associated with the customer profile.
    pub note: Option<String>,
    /// The birthday associated with the customer profile, in YYYY-MM-DD or MM-DD format. For example, specify
    /// 1998-09-21 for September 21, 1998, or 09-21 for September 21. Birthdays are returned in YYYY-MM-DD
    ///  format, where YYYY is the specified birth year or 0000 if a birth year is not specified.
    pub birthday: Option<String>,
    /// The tax ID associated with the customer profile. This field is available only for customers of sellers in EU countries
    ///  or the United Kingdom. In other countries, this field is ignored when included in a CreateCustomer request. For
    /// more information, see [Customer tax IDs](https://developer.squareup.com/docs/customers-api/what-it-does#customer-tax-ids).
    pub tax_ids: Option<Vec<CustomerTaxIds>>,
}

impl CreateCustomerRequestV20230925 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        idempotency_key: Option<String>,
        given_name: Option<String>,
        family_name: Option<String>,
        company_name: Option<String>,
        nickname: Option<String>,
        email_address: Option<String>,
        address: Option<Address>,
        phone_number: Option<String>,
        reference_id: Option<String>,
        note: Option<String>,
        birthday: Option<String>,
        tax_ids: Option<Vec<CustomerTaxIds>>,
    ) -> CreateCustomerRequestV20230925 {
        CreateCustomerRequestV20230925::validate_required_keys(
            given_name.clone(),
            family_name.clone(),
            company_name.clone(),
            email_address.clone(),
            phone_number.clone(),
        );
        CreateCustomerRequestV20230925 {
            idempotency_key,
            given_name,
            family_name,
            company_name,
            nickname,
            email_address,
            address,
            phone_number,
            reference_id,
            note,
            birthday,
            tax_ids,
        }
    }

    fn validate_required_keys(
        given_name: Option<String>,
        family_name: Option<String>,
        company_name: Option<String>,
        email_address: Option<String>,
        phone_number: Option<String>,
    ) {
        if given_name.is_none()
            && family_name.is_none()
            && company_name.is_none()
            && email_address.is_none()
            && phone_number.is_none()
        {
            panic!("At least one of the following values must be provided: given_name, family_name, company_name, email_address, phone_number");
        }
    }

    /// Converts the CreateCustomerRequestV20230925 into a JSON string.
    pub fn to_body_string(&self) -> String {
        let mut json_value: Value = serde_json::to_value(self).unwrap();
        CreateCustomerRequestV20230925::remove_nulls(&mut json_value);
        json_value.to_string()
    }

    fn remove_nulls(value: &mut Value) {
        match value {
            Value::Object(map) => {
                map.retain(|_, v| !v.is_null());
                for v in map.values_mut() {
                    CreateCustomerRequestV20230925::remove_nulls(v);
                }
            }
            Value::Array(arr) => {
                for v in arr.iter_mut() {
                    CreateCustomerRequestV20230925::remove_nulls(v);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::api::models::enums::country::Country;

    #[test]
    fn test_full_serialization() {
        let request = CreateCustomerRequestV20230925 {
            idempotency_key: Some("key".to_string()),
            given_name: Some("John".to_string()),
            family_name: Some("Doe".to_string()),
            company_name: Some("Company".to_string()),
            nickname: Some("JD".to_string()),
            email_address: Some("john.doe@example.com".to_string()),
            address: Some(Address {
                address_line_1: Some("123 Main St".to_string()),
                address_line_2: Some("Apt 4".to_string()),
                address_line_3: None,
                locality: Some("Smalltown".to_string()),
                sublocality: None,
                sublocality_2: None,
                sublocality_3: None,
                administrative_district_level_1: Some("CA".to_string()),
                administrative_district_level_2: None,
                administrative_district_level_3: None,
                postal_code: Some("12345".to_string()),
                country: Some(Country::Us),
                first_name: Some("John".to_string()),
                last_name: Some("Doe".to_string()),
            }),
            phone_number: Some("1234567890".to_string()),
            reference_id: Some("ref123".to_string()),
            note: Some("This is a note.".to_string()),
            birthday: Some("1990-01-01".to_string()),
            tax_ids: Some(vec![CustomerTaxIds {
                eu_vat: Some("EU123456".to_string()),
            }]),
        };

        let serialized = request.to_body_string();
        let expected = json!({
            "idempotency_key": "key",
            "given_name": "John",
            "family_name": "Doe",
            "company_name": "Company",
            "nickname": "JD",
            "email_address": "john.doe@example.com",
            "address": {
                "address_line_1": "123 Main St",
                "address_line_2": "Apt 4",
                "locality": "Smalltown",
                "administrative_district_level_1": "CA",
                "postal_code": "12345",
                "country": "US",
                "first_name": "John",
                "last_name": "Doe"
            },
            "phone_number": "1234567890",
            "reference_id": "ref123",
            "note": "This is a note.",
            "birthday": "1990-01-01",
            "tax_ids": [
                {
                    "eu_vat": "EU123456"
                }
            ]
        })
        .to_string();

        assert_eq!(serialized, expected);
    }
}
