use serde::{Deserialize, Serialize};

pub const STACK_APP_API: &str = "https://api.stackexchange.com";

/// The Stack Exchange websites. Full list here: https://stackexchange.com/sites
#[derive(Debug, Deserialize, Serialize)]
pub enum StackSite {
    AskUbuntu,
    Meta,
    ServerFault,
    StackOverflow,
    SuperUser,
    Unix,
}

/// API Version isn't really supported and I should probably remove.
/// The latest version as of 2023-05-25 is 2.3.
/// You can check the latest version manually here: https://api.stackexchange.com/
#[derive(Debug, Deserialize, Serialize)]
pub enum ApiVersion {
    V2_2,
    V2_3,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub items: Vec<T>,
    pub has_more: bool,
    pub quota_max: u32,
    pub quota_remaining: u32,
}

impl ToString for StackSite {
    fn to_string(&self) -> String {
        match self {
            StackSite::AskUbuntu => "askubuntu".to_string(),
            StackSite::Meta => "meta".to_string(),
            StackSite::ServerFault => "serverfault".to_string(),
            StackSite::StackOverflow => "stackoverflow".to_string(),
            StackSite::SuperUser => "superuser".to_string(),
            StackSite::Unix => "unix".to_string(),
        }
    }
}

impl ToString for ApiVersion {
    fn to_string(&self) -> String {
        match self {
            ApiVersion::V2_2 => "2.2".to_string(),
            ApiVersion::V2_3 => "2.3".to_string(),
        }
    }
}

/// Stack API info object type describes a site in the Stack Exchange network.
/// https://api.stackexchange.com/docs/types/info
#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub site: Option<Site>,
    pub new_active_users: u32,
    pub total_users: u32,
    pub badges_per_minute: f32,
    pub total_badges: u32,
    pub total_votes: u32,
    pub total_comments: u32,
    pub answers_per_minute: f32,
    pub questions_per_minute: f32,
    pub total_answers: u32,
    pub total_accepted: u32,
    pub total_unanswered: u32,
    pub total_questions: u32,
    pub api_revision: String,
}

/// This type represents a site in the Stack Exchange network.
/// https://api.stackexchange.com/docs/types/site
#[derive(Debug, Deserialize, Serialize)]
pub struct Site {
    pub name: String,
    pub launch_date: u32,
    pub open_beta_date: Option<u32>,
    pub aliases: Option<Vec<String>>,
    pub logo_url: String,
    pub site_type: String,
    pub related_sites: Option<Vec<RelatedSite>>,
}

/// This type represents a related site in the Stack Exchange network.
/// https://api.stackexchange.com/docs/types/related-site
#[derive(Debug, Deserialize, Serialize)]
pub struct RelatedSite {
    pub name: String,
    pub relation: String,
    pub api_site_parameter: String,
    pub site_url: String,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_info() {
        let info_str = include_str!("../../api-examples/info.json");
        let info: Info = serde_json::from_str(info_str).unwrap();
        println!("{:?}", info);
        assert_eq!(info.site.unwrap().name, "Example Site");
        assert_eq!(info.new_active_users, 10);
    }

    #[test]
    fn test_site() {
        let site_str = include_str!("../../api-examples/site.json");
        let site: Site = serde_json::from_str(site_str).unwrap();
        println!("{:?}", site);
        assert_eq!(site.name, "Example Site");
        assert_eq!(site.launch_date, 1684474623);
        assert_eq!(site.open_beta_date, Some(1684388223));
    }

    #[test]
    pub fn test_related_site() {
        let related_site_str =
            include_str!("../../api-examples/related-site.json");
        let related_site: RelatedSite =
            serde_json::from_str(related_site_str).unwrap();
        println!("{:?}", related_site);
        assert_eq!(related_site.name, "Example Site");
        assert_eq!(related_site.relation, "parent");
    }
}
