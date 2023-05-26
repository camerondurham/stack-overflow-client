use serde::{Deserialize, Serialize};

use crate::{
    common::{ApiVersion, Response, StackSite, STACK_APP_API},
    user::User,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub tagged: Option<String>,
    pub owner: User,
    pub is_answered: bool,
    pub view_count: u32,
    pub favorite_count: Option<u32>,
    pub down_vote_count: Option<u32>,
    pub answer_count: u32,
    pub score: u32,
    pub last_activity_date: u32,
    pub creation_date: u32,
    pub question_id: u32,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionRequest {
    pub tag: Option<String>,
    pub site: StackSite,
    pub api_version: ApiVersion,
}

#[deprecated]
pub fn get_client() -> reqwest::Client {
    reqwest::Client::builder()
        .gzip(true)
        .build()
        .expect("unable to create client")
}

#[deprecated]
pub async fn get_featured_questions(
    client: &reqwest::Client,
    req: QuestionRequest,
) -> Result<Response<Question>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/{}/questions/featured?tagged={}&site={}",
        STACK_APP_API,
        req.api_version.to_string(),
        req.tag.unwrap_or_default(),
        req.site.to_string()
    );
    println!("{:#?}", &url);

    let resp = client.get(&url).send().await?.text().await?;
    let deserialized: Response<Question> = serde_json::from_str(&resp.as_str())
        .expect(format!("Failed to deserialized response: {}", resp).as_str());
    Ok(deserialized)
}

#[cfg(test)]
mod tests {
    use crate::common::Response;

    use super::Question;

    #[test]
    fn test_parse_response() {
        let response = "{\"items\":[{\"tags\":[\"docker\",\"docker-compose\",\"ngrok\",\"vapor\"],\"owner\":{\"account_id\":14409195,\"reputation\":2356,\"user_id\":10408494,\"user_type\":\"registered\",\"profile_image\":\"https://i.stack.imgur.com/6Clbj.jpg?s=256&g=1\",\"display_name\":\"Roland Lariotte\",\"link\":\"https://stackoverflow.com/users/10408494/roland-lariotte\"},\"is_answered\":true,\"view_count\":52,\"bounty_amount\":50,\"bounty_closes_date\":1684225877,\"accepted_answer_id\":76224966,\"answer_count\":2,\"score\":2,\"last_activity_date\":1683789088,\"creation_date\":1682626252,\"last_edit_date\":1683635969,\"question_id\":76124274,\"content_license\":\"CC BY-SA 4.0\",\"link\":\"https://stackoverflow.com/questions/76124274/how-can-i-set-ngrok-on-vapor-to-work-locally-with-an-owned-domain\",\"title\":\"How can I set ngrok on Vapor to work locally with an owned domain?\"},{\"tags\":[\"docker\",\"security\",\"github\",\"containers\",\"github-actions\"],\"owner\":{\"account_id\":2319125,\"reputation\":485,\"user_id\":2035417,\"user_type\":\"registered\",\"accept_rate\":75,\"profile_image\":\"https://www.gravatar.com/avatar/be22c7b705fa45820b740d9256e1593e?s=256&d=identicon&r=PG\",\"display_name\":\"ruckc\",\"link\":\"https://stackoverflow.com/users/2035417/ruckc\"},\"is_answered\":false,\"view_count\":160,\"bounty_amount\":50,\"bounty_closes_date\":1683835712,\"answer_count\":1,\"score\":2,\"last_activity_date\":1683555302,\"creation_date\":1666965308,\"question_id\":74236426,\"content_license\":\"CC BY-SA 4.0\",\"link\":\"https://stackoverflow.com/questions/74236426/github-actions-scheduled-container-rebuild-on-latest-release-tag-only\",\"title\":\"GitHub Actions - Scheduled Container Rebuild On Latest Release Tag only\"},{\"tags\":[\"docker\"],\"owner\":{\"account_id\":1312027,\"reputation\":4629,\"user_id\":1259842,\"user_type\":\"registered\",\"accept_rate\":49,\"profile_image\":\"https://www.gravatar.com/avatar/d23f0b2bb6cb82b239c03a31beb625a2?s=256&d=identicon&r=PG&f=y&so-version=2\",\"display_name\":\"miltone\",\"link\":\"https://stackoverflow.com/users/1259842/miltone\"},\"is_answered\":false,\"view_count\":40,\"bounty_amount\":50,\"bounty_closes_date\":1683972718,\"answer_count\":1,\"score\":1,\"last_activity_date\":1683368008,\"creation_date\":1683187049,\"last_edit_date\":1683368008,\"question_id\":76170819,\"content_license\":\"CC BY-SA 4.0\",\"link\":\"https://stackoverflow.com/questions/76170819/docker-compose-build-generate-failed-to-fetch-oauth-token-post-https-auth-do\",\"title\":\"docker-compose build generate failed to fetch oauth token: Post &quot;https://auth.docker.io/token&quot;: proxyconnect tcp: dial tcp 127.0.0.1:7080\"},{\"tags\":[\"docker\",\"heroku\",\"docker-compose\",\"fastapi\",\"streamlit\"],\"owner\":{\"account_id\":20813,\"reputation\":21665,\"user_id\":50065,\"user_type\":\"registered\",\"accept_rate\":93,\"profile_image\":\"https://i.stack.imgur.com/ZbEgH.jpg?s=256&g=1\",\"display_name\":\"BioGeek\",\"link\":\"https://stackoverflow.com/users/50065/biogeek\"},\"is_answered\":true,\"view_count\":106,\"bounty_amount\":200,\"bounty_closes_date\":1683883040,\"answer_count\":1,\"score\":2,\"last_activity_date\":1683331610,\"creation_date\":1683031857,\"last_edit_date\":1683278209,\"question_id\":76155032,\"content_license\":\"CC BY-SA 4.0\",\"link\":\"https://stackoverflow.com/questions/76155032/connectionerror-on-multi-docker-app-streamlit-fastapi-when-deployed-on-herok\",\"title\":\"ConnectionError on multi-Docker app (Streamlit + FastAPI) when deployed on Heroku\"}],\"has_more\":false,\"quota_max\":300,\"quota_remaining\":285}";
        let deseralized: Response<Question> =
            serde_json::from_str(&response).unwrap();
        println!("{:#?}", deseralized);
        assert_eq!(deseralized.has_more, false);
        let question = deseralized.items.first().unwrap();
        assert_eq!(question.question_id, 76124274);
        assert_eq!(question.title, "How can I set ngrok on Vapor to work locally with an owned domain?");
        assert_eq!(question.view_count, 52);
        assert_eq!(question.answer_count, 2);
        /*
        Example parsed response:
        {
            items: [
                Question {
                    tagged: None,
                    owner: User {
                        reputation: 2356,
                        account_id: 14409195,
                        user_id: 10408494,
                        user_type: "registered",
                        display_name: "Roland Lariotte",
                    },
                    is_answered: true,
                    view_count: 52,
                    answer_count: 2,
                    score: 2,
                    last_activity_date: 1683789088,
                    creation_date: 1682626252,
                    question_id: 76124274,
                    title: "How can I set ngrok on Vapor to work locally with an owned domain?",
                },
                Question {
                    tagged: None,
                    owner: User {
                        reputation: 485,
                        account_id: 2319125,
                        user_id: 2035417,
                        user_type: "registered",
                        display_name: "ruckc",
                    },
                    is_answered: false,
                    view_count: 160,
                    answer_count: 1,
                    score: 2,
                    last_activity_date: 1683555302,
                    creation_date: 1666965308,
                    question_id: 74236426,
                    title: "GitHub Actions - Scheduled Container Rebuild On Latest Release Tag only",
                },
                Question {
                    tagged: None,
                    owner: User {
                        reputation: 4629,
                        account_id: 1312027,
                        user_id: 1259842,
                        user_type: "registered",
                        display_name: "miltone",
                    },
                    is_answered: false,
                    view_count: 40,
                    answer_count: 1,
                    score: 1,
                    last_activity_date: 1683368008,
                    creation_date: 1683187049,
                    question_id: 76170819,
                    title: "docker-compose build generate failed to fetch oauth token: Post &quot;https://auth.docker.io/token&quot;: proxyconnect tcp: dial tcp 127.0.0.1:7080",
                },
                Question {
                    tagged: None,
                    owner: User {
                        reputation: 21665,
                        account_id: 20813,
                        user_id: 50065,
                        user_type: "registered",
                        display_name: "BioGeek",
                    },
                    is_answered: true,
                    view_count: 106,
                    answer_count: 1,
                    score: 2,
                    last_activity_date: 1683331610,
                    creation_date: 1683031857,
                    question_id: 76155032,
                    title: "ConnectionError on multi-Docker app (Streamlit + FastAPI) when deployed on Heroku",
                },
            ],
        }
         */
    }
}
