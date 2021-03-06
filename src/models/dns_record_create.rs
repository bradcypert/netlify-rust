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
pub struct DnsRecordCreate {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(rename = "flag", skip_serializing_if = "Option::is_none")]
    pub flag: Option<i64>,
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl DnsRecordCreate {
    pub fn new() -> DnsRecordCreate {
        DnsRecordCreate {
            _type: None,
            hostname: None,
            value: None,
            ttl: None,
            priority: None,
            weight: None,
            port: None,
            flag: None,
            tag: None,
        }
    }
}


