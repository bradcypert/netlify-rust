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
pub struct SiteSetup {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "custom_domain", skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<String>,
    #[serde(rename = "domain_aliases", skip_serializing_if = "Option::is_none")]
    pub domain_aliases: Option<Vec<String>>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "notification_email", skip_serializing_if = "Option::is_none")]
    pub notification_email: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "ssl_url", skip_serializing_if = "Option::is_none")]
    pub ssl_url: Option<String>,
    #[serde(rename = "admin_url", skip_serializing_if = "Option::is_none")]
    pub admin_url: Option<String>,
    #[serde(rename = "screenshot_url", skip_serializing_if = "Option::is_none")]
    pub screenshot_url: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "session_id", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "ssl", skip_serializing_if = "Option::is_none")]
    pub ssl: Option<bool>,
    #[serde(rename = "force_ssl", skip_serializing_if = "Option::is_none")]
    pub force_ssl: Option<bool>,
    #[serde(rename = "managed_dns", skip_serializing_if = "Option::is_none")]
    pub managed_dns: Option<bool>,
    #[serde(rename = "deploy_url", skip_serializing_if = "Option::is_none")]
    pub deploy_url: Option<String>,
    #[serde(rename = "published_deploy", skip_serializing_if = "Option::is_none")]
    pub published_deploy: Option<Box<crate::models::Deploy>>,
    #[serde(rename = "account_name", skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "account_slug", skip_serializing_if = "Option::is_none")]
    pub account_slug: Option<String>,
    #[serde(rename = "git_provider", skip_serializing_if = "Option::is_none")]
    pub git_provider: Option<String>,
    #[serde(rename = "deploy_hook", skip_serializing_if = "Option::is_none")]
    pub deploy_hook: Option<String>,
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "processing_settings", skip_serializing_if = "Option::is_none")]
    pub processing_settings: Option<Box<crate::models::SiteProcessingSettings>>,
    #[serde(rename = "build_settings", skip_serializing_if = "Option::is_none")]
    pub build_settings: Option<Box<crate::models::RepoInfo>>,
    #[serde(rename = "id_domain", skip_serializing_if = "Option::is_none")]
    pub id_domain: Option<String>,
    #[serde(rename = "default_hooks_data", skip_serializing_if = "Option::is_none")]
    pub default_hooks_data: Option<Box<crate::models::SiteDefaultHooksData>>,
    #[serde(rename = "build_image", skip_serializing_if = "Option::is_none")]
    pub build_image: Option<String>,
    #[serde(rename = "prerender", skip_serializing_if = "Option::is_none")]
    pub prerender: Option<String>,
    #[serde(rename = "repo", skip_serializing_if = "Option::is_none")]
    pub repo: Option<Box<crate::models::RepoInfo>>,
}

impl SiteSetup {
    pub fn new() -> SiteSetup {
        SiteSetup {
            id: None,
            state: None,
            plan: None,
            name: None,
            custom_domain: None,
            domain_aliases: None,
            password: None,
            notification_email: None,
            url: None,
            ssl_url: None,
            admin_url: None,
            screenshot_url: None,
            created_at: None,
            updated_at: None,
            user_id: None,
            session_id: None,
            ssl: None,
            force_ssl: None,
            managed_dns: None,
            deploy_url: None,
            published_deploy: None,
            account_name: None,
            account_slug: None,
            git_provider: None,
            deploy_hook: None,
            capabilities: None,
            processing_settings: None,
            build_settings: None,
            id_domain: None,
            default_hooks_data: None,
            build_image: None,
            prerender: None,
            repo: None,
        }
    }
}


