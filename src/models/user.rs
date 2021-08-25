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
pub struct User {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "full_name", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "avatar_url", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "affiliate_id", skip_serializing_if = "Option::is_none")]
    pub affiliate_id: Option<String>,
    #[serde(rename = "site_count", skip_serializing_if = "Option::is_none")]
    pub site_count: Option<i64>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "last_login", skip_serializing_if = "Option::is_none")]
    pub last_login: Option<String>,
    #[serde(rename = "login_providers", skip_serializing_if = "Option::is_none")]
    pub login_providers: Option<Vec<String>>,
    #[serde(rename = "onboarding_progress", skip_serializing_if = "Option::is_none")]
    pub onboarding_progress: Option<Box<crate::models::UserOnboardingProgress>>,
}

impl User {
    pub fn new() -> User {
        User {
            id: None,
            uid: None,
            full_name: None,
            avatar_url: None,
            email: None,
            affiliate_id: None,
            site_count: None,
            created_at: None,
            last_login: None,
            login_providers: None,
            onboarding_progress: None,
        }
    }
}


