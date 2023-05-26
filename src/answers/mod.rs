// Example API call https://api.stackexchange.com/docs/answers-on-users#order=desc&sort=activity&ids=4676641&filter=default&site=stackoverflow&run=true
// Answer model: https://api.stackexchange.com/docs/types/answer

use serde::{Deserialize, Serialize};

use crate::user::User;

/// Represents an answer to a quesetion on one of the Stack Exchange sites.
/// As on the question page it is possible to fetch the comments on an answer as part of the call; though this is not done by default.
/// The upvoted, downvoted, and accepted fields are not supported in this initial API implementation but they are available in the StackExchange API.
/// Docs: https://api.stackexchange.com/docs/types/answer
///
/// Example answer:
/// ```json
/// {
///   "owner": {
///     "account_id": 5947562,
///     "reputation": 4229,
///     "user_id": 4676641,
///     "user_type": "registered",
///     "profile_image": "https://i.stack.imgur.com/8Pzxd.jpg?s=256&g=1",
///     "display_name": "cam",
///     "link": "https://stackoverflow.com/users/4676641/cam"
///   },
///   "is_accepted": false,
///   "score": 1,
///   "last_activity_date": 1616609814,
///   "creation_date": 1616609814,
///   "answer_id": 66786986,
///   "question_id": 66786311,
///   "content_license": "CC BY-SA 4.0"
/// }
/// ```
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    answer_id: u32,
    content_license: String,
    is_accepted: bool,
    last_activity_date: u32,
    score: u32,
    owner: User,
    question_id: u32,
    title: Option<String>,
    body: Option<String>,
}
