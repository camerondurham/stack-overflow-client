use reqwest::Client;

use crate::{
    answers::Answer,
    common::{ApiVersion, Info, Response, StackSite, STACK_APP_API},
    questions::Question,
};

#[derive(Debug)]
pub struct StackClient {
    client: Client,
    api_version: ApiVersion,
    stack_site: StackSite,
}

/// StackClient is a client for the Stack Exchange API
///
/// Construct a default client for StackOverflow site, API version 2.3 (https://meta.stackexchange.com/questions/366977/api-2-3-release)
/// ```rust
/// use stack_overflow_client::client::StackClient;
/// let client = StackClient::new();
/// ```
///
/// Construct a e client for a specific site and API version
/// ```rust
/// use stack_overflow_client::client::StackClientBuilder;
/// use stack_overflow_client::common::{ApiVersion, StackSite};
/// let client = StackClientBuilder::new()
///                 .stack_site(StackSite::StackOverflow)
///                 .version(ApiVersion::V2_3)
///                 .build();
/// ```
impl StackClient {
    /// StackClient defaults to StackOverflow as the site and api version 2.3
    pub fn new() -> Self {
        StackClientBuilder::new()
            .version(ApiVersion::V2_3)
            .stack_site(StackSite::StackOverflow)
            .build()
    }

    /// Get info about the client's Stack site. Returns a collection of statistics about the site.
    /// API Docs: https://api.stackexchange.com/docs/info
    pub async fn get_info(
        &self,
    ) -> Result<Response<Info>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/{}/info?site={}",
            STACK_APP_API,
            self.api_version.to_string(),
            self.stack_site.to_string()
        );
        let resp = self.client.get(&url).send().await?.text().await?;
        let deserialized: Response<Info> = serde_json::from_str(&resp.as_str())
            .expect("unable to deserialize response");
        Ok(deserialized)
    }

    /// Get featured questions for a tag.
    /// Note that this only retrieves the first page of results.
    /// API Docs: https://api.stackexchange.com/docs/featured-questions
    pub async fn get_featured_questions(
        &self,
        tag: &str,
    ) -> Result<Response<Question>, Box<dyn std::error::Error>> {
        self.get_featured_questions_paginated(tag, 1).await
    }

    /// Get featured questions for a tag and explicitly provide a page.
    /// Note that pagination is not supported beyond 25 pages because authentication is required.
    /// API Docs: https://api.stackexchange.com/docs/featured-questions
    pub async fn get_featured_questions_paginated(
        &self,
        tag: &str,
        page: u32,
    ) -> Result<Response<Question>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/{}/questions/featured?tagged={}&site={}&page={}",
            STACK_APP_API,
            self.api_version.to_string(),
            tag,
            self.stack_site.to_string(),
            page
        );

        let resp = self.client.get(&url).send().await?.text().await?;
        let deserialized: Response<Question> = serde_json::from_str(
            &resp.as_str(),
        )
        .expect(format!("Failed to deserialized response: {}", resp).as_str());
        Ok(deserialized)
    }

    /// Get answers provided by a user
    /// API Docs: https://api.stackexchange.com/docs/answers-on-users
    pub async fn get_answers_for_user(
        &self,
        user_id: u32,
        page: Option<u32>,
    ) -> Result<Response<Answer>, Box<dyn std::error::Error>> {
        let url: String = format!(
            "{}/{}/users/{}/answers?order=desc&sort=activity&site={}&page={}",
            STACK_APP_API,
            self.api_version.to_string(),
            user_id.to_string(),
            self.stack_site.to_string(),
            page.unwrap_or(1),
        );

        let resp = self.client.get(&url).send().await?.text().await?;
        let deserialized: Response<Answer> = serde_json::from_str(
            resp.as_str(),
        )
        .expect(format!("Failed to deserialize response: {}", resp).as_str());
        Ok(deserialized)
    }
}
pub struct StackClientBuilder {
    api_version: Option<ApiVersion>,
    stack_site: Option<StackSite>,
}

impl StackClientBuilder {
    pub fn new() -> Self {
        StackClientBuilder {
            api_version: None,
            stack_site: None,
        }
    }

    pub fn version(mut self, version: ApiVersion) -> Self {
        self.api_version = Some(version);
        self
    }

    pub fn stack_site(mut self, site: StackSite) -> Self {
        self.stack_site = Some(site);
        self
    }

    pub fn build(self) -> StackClient {
        StackClient {
            api_version: self.api_version.unwrap_or(ApiVersion::V2_3),
            stack_site: self.stack_site.unwrap_or(StackSite::StackOverflow),
            client: reqwest::Client::builder()
                .gzip(true) // Stack api responses are gzip encoded: https://api.stackexchange.com/docs/compression
                .build()
                .expect("unable to create reqwest http client"),
        }
    }
}
