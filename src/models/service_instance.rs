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
pub struct ServiceInstance {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
    #[serde(rename = "external_attributes", skip_serializing_if = "Option::is_none")]
    pub external_attributes: Option<serde_json::Value>,
    #[serde(rename = "service_slug", skip_serializing_if = "Option::is_none")]
    pub service_slug: Option<String>,
    #[serde(rename = "service_path", skip_serializing_if = "Option::is_none")]
    pub service_path: Option<String>,
    #[serde(rename = "service_name", skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env: Option<serde_json::Value>,
    #[serde(rename = "snippets", skip_serializing_if = "Option::is_none")]
    pub snippets: Option<Vec<serde_json::Value>>,
    #[serde(rename = "auth_url", skip_serializing_if = "Option::is_none")]
    pub auth_url: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl ServiceInstance {
    pub fn new() -> ServiceInstance {
        ServiceInstance {
            id: None,
            url: None,
            config: None,
            external_attributes: None,
            service_slug: None,
            service_path: None,
            service_name: None,
            env: None,
            snippets: None,
            auth_url: None,
            created_at: None,
            updated_at: None,
        }
    }
}


