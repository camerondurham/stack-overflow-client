use stack_overflow::{
    client::StackClient,
    common::{ApiVersion, StackSite},
    questions::QuestionRequest,
};

#[tokio::main]
async fn main() {
    let question: QuestionRequest = QuestionRequest {
        tag: Some("docker".to_string()),
        site: StackSite::StackOverflow,
        api_version: ApiVersion::V2_3,
    };
    println!("{:?}", question);

    let client = StackClient::new();
    let results = client
        .get_featured_questions("docker")
        .await
        .expect("Unable to fetch featured docker questions");
    dbg!(&results);
    /*
    QuestionRequest { tag: Some("rust"), site: StackOverflow, api_version: V2_3 }
    "https://api.stackexchange.com/2.3/questions/featured?tagged=rust&site=stackoverflow"
    {"items":[{"tags":["rust"],"owner":{"account_id":7063007,"reputation":13,"user_id":12276541,"user_type":"registered","profile_image":"https://www.gravatar.com/avatar/90649aebcc0cccc983abf5b461007176?s=256&d=identicon&r=
    PG&f=1","display_name":"Rafaelo","link":"https://stackoverflow.com/users/12276541/rafaelo"},"is_answered":true,"view_count":109,"bounty_amount":150,"bounty_closes_date":1683583494,"answer_count":1,"score":0,"last_activ
    ity_date":1682978694,"creation_date":1682714190,"question_id":76133120,"content_license":"CC BY-SA 4.0","link":"https://stackoverflow.com/questions/76133120/can-a-memory-pool-allocate-things-with-a-static-lifetime","ti
    tle":"Can a memory pool allocate things with a `&#39;static` lifetime?"}],"has_more":false,"quota_max":300,"quota_remaining":296}

    "{\"items\":[{\"tags\":[\"docker\",\"docker-compose\",\"ngrok\",\"vapor\"],\"owner\":{\"account_id\":14409195,\"reputation\":2356,\"user_id\":10408494,\"user_type\":\"registered\",\"profile_image\":\"https://i.stack.imgur.com/6Clbj.jpg?s=256&g=1\",\"display_name\":\"Roland
    Lariotte\",\"link\":\"https://stackoverflow.com/users/10408494/roland-lariotte\"},\"is_answered\":true,\"view_count\":52,\"bounty_amount\":50,\"bounty_closes_date\":1684225877,\"accepted_answer_id\":76224966,\"answer_count\":2,\"score\":2,\"last_activity_date\":1683789088,\"creation_date\":1682626252,\"last_edit_date\":1683635969,\"question_id\":76124274,\"content_license\":\"CC
    BY-SA
    4.0\",\"link\":\"https://stackoverflow.com/questions/76124274/how-can-i-set-ngrok-on-vapor-to-work-locally-with-an-owned-domain\",\"title\":\"How
    can I set ngrok on Vapor to work locally with an owned
    domain?\"},{\"tags\":[\"docker\",\"security\",\"github\",\"containers\",\"github-actions\"],\"owner\":{\"account_id\":2319125,\"reputation\":485,\"user_id\":2035417,\"user_type\":\"registered\",\"accept_rate\":75,\"profile_image\":\"https://www.gravatar.com/avatar/be22c7b705fa45820b740d9256e1593e?s=256&d=identicon&r=PG\",\"display_name\":\"ruckc\",\"link\":\"https://stackoverflow.com/users/2035417/ruckc\"},\"is_answered\":false,\"view_count\":159,\"bounty_amount\":50,\"bounty_closes_date\":1683835712,\"answer_count\":1,\"score\":2,\"last_activity_date\":1683555302,\"creation_date\":1666965308,\"question_id\":74236426,\"content_license\":\"CC
    BY-SA
    4.0\",\"link\":\"https://stackoverflow.com/questions/74236426/github-actions-scheduled-container-rebuild-on-latest-release-tag-only\",\"title\":\"GitHub
    Actions - Scheduled Container Rebuild On Latest Release Tag
    only\"},{\"tags\":[\"docker\"],\"owner\":{\"account_id\":1312027,\"reputation\":4629,\"user_id\":1259842,\"user_type\":\"registered\",\"accept_rate\":49,\"profile_image\":\"https://www.gravatar.com/avatar/d23f0b2bb6cb82b239c03a31beb625a2?s=256&d=identicon&r=PG&f=y&so-version=2\",\"display_name\":\"miltone\",\"link\":\"https://stackoverflow.com/users/1259842/miltone\"},\"is_answered\":false,\"view_count\":40,\"bounty_amount\":50,\"bounty_closes_date\":1683972718,\"answer_count\":1,\"score\":1,\"last_activity_date\":1683368008,\"creation_date\":1683187049,\"last_edit_date\":1683368008,\"question_id\":76170819,\"content_license\":\"CC
    BY-SA
    4.0\",\"link\":\"https://stackoverflow.com/questions/76170819/docker-compose-build-generate-failed-to-fetch-oauth-token-post-https-auth-do\",\"title\":\"docker-compose
    build generate failed to fetch oauth token: Post
    &quot;https://auth.docker.io/token&quot;: proxyconnect tcp: dial tcp
    127.0.0.1:7080\"},{\"tags\":[\"docker\",\"heroku\",\"docker-compose\",\"fastapi\",\"streamlit\"],\"owner\":{\"account_id\":20813,\"reputation\":21665,\"user_id\":50065,\"user_type\":\"registered\",\"accept_rate\":93,\"profile_image\":\"https://i.stack.imgur.com/ZbEgH.jpg?s=256&g=1\",\"display_name\":\"BioGeek\",\"link\":\"https://stackoverflow.com/users/50065/biogeek\"},\"is_answered\":true,\"view_count\":106,\"bounty_amount\":200,\"bounty_closes_date\":1683883040,\"answer_count\":1,\"score\":2,\"last_activity_date\":1683331610,\"creation_date\":1683031857,\"last_edit_date\":1683278209,\"question_id\":76155032,\"content_license\":\"CC
    BY-SA
    4.0\",\"link\":\"https://stackoverflow.com/questions/76155032/connectionerror-on-multi-docker-app-streamlit-fastapi-when-deployed-on-herok\",\"title\":\"ConnectionError
    on multi-Docker app (Streamlit + FastAPI) when deployed on
    Heroku\"}],\"has_more\":false,\"quota_max\":300,\"quota_remaining\":297}"
           */

    let user_id = 4676641;
    let result = client
        .get_answers_for_user(user_id, None)
        .await
        .expect("unable to fetch answers for user 4676641");
    dbg!(&result);

    let mut counter = 2;
    loop {
        if result.has_more {
            let result =
                client.get_answers_for_user(user_id, Some(counter)).await;
            dbg!(&result);
            counter += 1;
        } else {
            break;
        }
    }

    /*
    [src/main.rs:47] result = Response {
        items: [
            Question {
                tagged: None,
                owner: User {
                    reputation: 547,
                    account_id: 3863408,
                    user_id: 3200552,
                    user_type: "registered",
                    display_name: "merhoo",
                },
                is_answered: true,
                view_count: 96,
                favorite_count: None,
                down_vote_count: None,
                answer_count: 1,
                score: 3,
                last_activity_date: 1684111790,
                creation_date: 1678979778,
                question_id: 75758174,
                title: "How to make profile default for docker-compose?",
            },
            ...
        ],
        has_more: true,
        quota_max: 300,
        quota_remaining: 281,
    }
         */
}
