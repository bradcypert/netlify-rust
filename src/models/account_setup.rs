/*
 * Netlify's API documentation
 *
 * Netlify is a hosting service for the programmable web. It understands your documents and provides an API to handle atomic deploys of websites, manage form submissions, inject JavaScript snippets, and much more. This is a REST-style API that uses JSON for serialization and OAuth 2 for authentication.  This document is an OpenAPI reference for the Netlify API that you can explore. For more detailed instructions for common uses, please visit the [online documentation](https://www.netlify.com/docs/api/). Visit our Community forum to join the conversation about [understanding and using Netlify’s API](https://community.netlify.com/t/common-issue-understanding-and-using-netlifys-api/160).  Additionally, we have two API clients for your convenience: - [Go Client](https://github.com/netlify/open-api#go-client) - [JS Client](https://github.com/netlify/js-client)
 *
 * The version of the OpenAPI document: 2.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountSetup {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type_id")]
    pub type_id: String,
    #[serde(rename = "payment_method_id", skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
    #[serde(rename = "extra_seats_block", skip_serializing_if = "Option::is_none")]
    pub extra_seats_block: Option<i32>,
}

impl AccountSetup {
    pub fn new(name: String, type_id: String) -> AccountSetup {
        AccountSetup {
            name,
            type_id,
            payment_method_id: None,
            period: None,
            extra_seats_block: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Period {
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "yearly")]
    Yearly,
}

