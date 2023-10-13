//! CustomerFilter

use serde::{Deserialize, Serialize};

/// Represents the filtering criteria in a [search query](https://developer.squareup.com/reference/square/objects/CustomerQuery) that defines how to filter customer profiles returned in [SearchCustomers](https://developer.squareup.com/reference/square/customers-api/search-customers) results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerFilter {
    /// A filter to select customers based on their creation source.
    pub creation_source: Option<CustomerCreationSourceFilter>,
    /// A filter to select customers based on when they were created.
    pub created_at: Option<TimeRange>,
    /// A filter to select customers based on when they were last updated.
    pub updated_at: Option<TimeRange>,
    /// A filter to [select customers by their email addresses](https://developer.squareup.com/docs/customers-api/use-the-api/search-customers#search-by-email-address) visible to the seller. This filter is case-insensitive.
    ///
    /// For [exact matching](https://developer.squareup.com/docs/customers-api/use-the-api/search-customers#exact-search-by-email-address), this filter causes the search to return customer profiles whose `email_address` field value are identical to the email address provided in the query.
    ///
    /// For [fizzy matching](https://developer.squareup.com/docs/customers-api/use-the-api/search-customers#fuzzy-search-by-email-address) this filter causes the search to return customer profiles whose `email_address` field value has a token-wise partial match against the filtering expression in the query. For example, with `Steven gmail` provided in a search query, the search returns customers whose email address is `steven.johnson@gmail.com` or `mygmail@stevenbakery.com`. Square removes any punctuation (including periods (`.`), underscores (`_`), and the `@` symbol) and tokenizes the email addresses on spaces. A match is found if a tokenized email address contains all the tokens in the search query, irrespective of the token order.
    pub email_address: Option<CustomerTextFilter>,
    /// A filter to [select customers by their phone numbers](https://developer.squareup.com/docs/customers-api/use-the-api/search-customers#search-by-phone-number) visible to the seller.
    ///
    /// For [exact matching](https://developer.squareup.com/docs/customers-api/use-the-api/search-customers#exact-search-by-phone-number), this filter returns customers whose phone number matches the specified query expression. The number in the query must be of an E.164-compliant form. In particular, it must include the leading `+` sign followed by a country code and then a subscriber number. For example, the standard E.164 form of a US phone number is `+12062223333` and an E.164-compliant variation is `+1 (206) 222-3333`. To match the query expression, stored customer phone numbers are converted to the standard E.164 form.
    ///
    /// For [fizzy matching](https://developer.squareup.com/docs/customers-api/use-the-api/search-customers#fuzzy-search-by-phone-number), this filter returns customers whose phone number matches the token or tokens provided in the query expression. For example, with `415` provided in a search query, the search returns customers with the phone numbers `+1-415-212-1200`, `+1-212-415-1234`, and `+1 (551) 234-1567`. Similarly, a search query of `415 123` returns customers with the phone numbers `+1-212-415-1234` and `+1 (551) 234-1567` but not `+1-212-415-1200`. A match is found if a tokenized phone number contains all the tokens in the search query, irrespective of the token order.
    pub phone_number: Option<CustomerTextFilter>,
    /// A filter to [select customers by their reference IDs](https://developer.squareup.com/docs/customers-api/use-the-api/search-customers#search-by-reference-id). This filter is case-insensitive.
    ///
    /// [Exact matching](https://developer.squareup.com/docs/customers-api/use-the-api/search-customers#exact-search-by-reference-id) of a customer's reference ID against a query's reference ID is evaluated as an exact match between two strings, character by character in the given order.
    ///
    /// [Fuzzy matching](https://developer.squareup.com/docs/customers-api/use-the-api/search-customers#fuzzy-search-by-reference-id) of stored reference IDs against queried reference IDs works exactly the same as fuzzy matching on email addresses. Non-alphanumeric characters are replaced by spaces to tokenize stored and queried reference IDs. A match is found if a tokenized stored reference ID contains all tokens specified in any order in the query. For example, a query of `NYC M` matches customer profiles with the `reference_id` value of `NYC_M_35_JOHNSON` and `NYC_27_MURRAY`.
    pub reference_id: Option<CustomerTextFilter>,
    /// A filter to select customers based on the [groups](https://developer.squareup.com/reference/square/objects/CustomerGroup) they belong to. Group membership is controlled by sellers and developers.
    ///
    /// The `group_ids` filter has the following syntax:
    /// ```
    /// "group_ids": {
    ///  "any":  ["{group_a_id}", "{group_b_id}", ...],
    ///  "all":  ["{group_1_id}", "{group_2_id}", ...],
    ///  "none": ["{group_i_id}", "{group_ii_id}", ...]
    /// }
    /// ```
    /// You can use any combination of the `any`, `all`, and `none` fields in the filter. With `any`, the search returns customers in groups `a` or `b` or any other group specified in the list. With `all`, the search returns customers in groups `1` and `2` and all other groups specified in the list. With `none`, the search returns customers not in groups `i` or `ii` or any other group specified in the list.
    ///
    /// If any of the search conditions are not met, including when an invalid or non-existent group ID is provided, the result is an empty object (`{}`).
    pub group_ids: Option<FilterValue>,
    /// A filter to select customers based on one or more custom attributes.
    ///
    /// This filter can contain up to 10 custom attribute filters. Each custom attribute filter specifies filtering criteria for a target custom attribute. If multiple custom attribute filters are provided, they are combined as an AND operation.
    ///
    /// To be valid for a search, the custom attributes must be visible to the requesting application. For more information, including example queries, see [Search by custom attribute](https://developer.squareup.com/docs/customers-api/use-the-api/search-customers#search-by-custom-attribute).
    ///
    /// Square returns matching customer profiles, which do not contain custom attributes. To retrieve customer-related custom attributes, use the [Customer Custom Attributes API](https://developer.squareup.com/reference/square/customercustomattributes-api). For example, you can call [RetrieveCustomerCustomAttribute](https://developer.squareup.com/reference/square/customercustomattributes-api/retrieve-customer-custom-attribute) using a customer ID from the result set.
    pub custom_attribute: Option<CustomerCustomAttributeFilters>,
    /// A filter to select customers based on the [segments](https://developer.squareup.com/reference/square/objects/CustomerSegment) they belong to. Segment membership is dynamic and adjusts automatically based on whether customers meet the segment criteria.
    ///
    /// You can provide up to three segment IDs in the filter, using any combination of the `all`, `any`, and `none` fields. For the following example, the results include customers who belong to both segment `A` and segment `B` but do not belong to segment `C`.
    ///
    /// ```
    /// "segment_ids": {
    /// "all":  ["{segment_A_id}", "{segment_B_id}"],
    /// "none":  ["{segment_C_id}"]
    /// }
    /// ```
    ///
    /// If an invalid or non-existent segment ID is provided in the filter, Square stops processing the request and returns a `400 BAD_REQUEST` error that includes the segment ID.
    pub segment_ids: Option<FilterValue>,
}