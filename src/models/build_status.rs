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
pub struct BuildStatus {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<i32>,
    #[serde(rename = "pending_concurrency", skip_serializing_if = "Option::is_none")]
    pub pending_concurrency: Option<i32>,
    #[serde(rename = "enqueued", skip_serializing_if = "Option::is_none")]
    pub enqueued: Option<i32>,
    #[serde(rename = "build_count", skip_serializing_if = "Option::is_none")]
    pub build_count: Option<i32>,
    #[serde(rename = "minutes", skip_serializing_if = "Option::is_none")]
    pub minutes: Option<Box<crate::models::BuildStatusMinutes>>,
}

impl BuildStatus {
    pub fn new() -> BuildStatus {
        BuildStatus {
            active: None,
            pending_concurrency: None,
            enqueued: None,
            build_count: None,
            minutes: None,
        }
    }
}


