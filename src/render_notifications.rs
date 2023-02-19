use serde_derive::{Deserialize, Serialize};
use serde_json;

pub type Notifications = Vec<Notification>;

// NOTE: problem here some values are nullable in the api but i don't know which so i recon that
// subject and title are mandatory
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: Option<String>,
    pub repository: Option<Repository>,
    pub subject: Option<Subject>,
    pub reason: Option<String>,
    pub unread: Option<bool>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(rename = "last_read_at")]
    pub last_read_at: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "subscription_url")]
    pub subscription_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub id: Option<i64>,
    #[serde(rename = "node_id")]
    pub node_id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "full_name")]
    pub full_name: Option<String>,
    pub owner: Option<Owner>,
    pub private: Option<bool>,
    #[serde(rename = "html_url")]
    pub html_url: Option<String>,
    pub description: Option<String>,
    pub fork: Option<bool>,
    pub url: Option<String>,
    #[serde(rename = "archive_url")]
    pub archive_url: Option<String>,
    #[serde(rename = "assignees_url")]
    pub assignees_url: Option<String>,
    #[serde(rename = "blobs_url")]
    pub blobs_url: Option<String>,
    #[serde(rename = "branches_url")]
    pub branches_url: Option<String>,
    #[serde(rename = "collaborators_url")]
    pub collaborators_url: Option<String>,
    #[serde(rename = "comments_url")]
    pub comments_url: Option<String>,
    #[serde(rename = "commits_url")]
    pub commits_url: Option<String>,
    #[serde(rename = "compare_url")]
    pub compare_url: Option<String>,
    #[serde(rename = "contents_url")]
    pub contents_url: Option<String>,
    #[serde(rename = "contributors_url")]
    pub contributors_url: Option<String>,
    #[serde(rename = "deployments_url")]
    pub deployments_url: Option<String>,
    #[serde(rename = "downloads_url")]
    pub downloads_url: Option<String>,
    #[serde(rename = "events_url")]
    pub events_url: Option<String>,
    #[serde(rename = "forks_url")]
    pub forks_url: Option<String>,
    #[serde(rename = "git_commits_url")]
    pub git_commits_url: Option<String>,
    #[serde(rename = "git_refs_url")]
    pub git_refs_url: Option<String>,
    #[serde(rename = "git_tags_url")]
    pub git_tags_url: Option<String>,
    #[serde(rename = "git_url")]
    pub git_url: Option<String>,
    #[serde(rename = "issue_comment_url")]
    pub issue_comment_url: Option<String>,
    #[serde(rename = "issue_events_url")]
    pub issue_events_url: Option<String>,
    #[serde(rename = "issues_url")]
    pub issues_url: Option<String>,
    #[serde(rename = "keys_url")]
    pub keys_url: Option<String>,
    #[serde(rename = "labels_url")]
    pub labels_url: Option<String>,
    #[serde(rename = "languages_url")]
    pub languages_url: Option<String>,
    #[serde(rename = "merges_url")]
    pub merges_url: Option<String>,
    #[serde(rename = "milestones_url")]
    pub milestones_url: Option<String>,
    #[serde(rename = "notifications_url")]
    pub notifications_url: Option<String>,
    #[serde(rename = "pulls_url")]
    pub pulls_url: Option<String>,
    #[serde(rename = "releases_url")]
    pub releases_url: Option<String>,
    #[serde(rename = "ssh_url")]
    pub ssh_url: Option<String>,
    #[serde(rename = "stargazers_url")]
    pub stargazers_url: Option<String>,
    #[serde(rename = "statuses_url")]
    pub statuses_url: Option<String>,
    #[serde(rename = "subscribers_url")]
    pub subscribers_url: Option<String>,
    #[serde(rename = "subscription_url")]
    pub subscription_url: Option<String>,
    #[serde(rename = "tags_url")]
    pub tags_url: Option<String>,
    #[serde(rename = "teams_url")]
    pub teams_url: Option<String>,
    #[serde(rename = "trees_url")]
    pub trees_url: Option<String>,
    #[serde(rename = "hooks_url")]
    pub hooks_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub login: Option<String>,
    pub id: Option<i64>,
    #[serde(rename = "node_id")]
    pub node_id: Option<String>,
    #[serde(rename = "avatar_url")]
    pub avatar_url: Option<String>,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "html_url")]
    pub html_url: Option<String>,
    #[serde(rename = "followers_url")]
    pub followers_url: Option<String>,
    #[serde(rename = "following_url")]
    pub following_url: Option<String>,
    #[serde(rename = "gists_url")]
    pub gists_url: Option<String>,
    #[serde(rename = "starred_url")]
    pub starred_url: Option<String>,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: Option<String>,
    #[serde(rename = "organizations_url")]
    pub organizations_url: Option<String>,
    #[serde(rename = "repos_url")]
    pub repos_url: Option<String>,
    #[serde(rename = "events_url")]
    pub events_url: Option<String>,
    #[serde(rename = "received_events_url")]
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "site_admin")]
    pub site_admin: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub title: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "latest_comment_url")]
    pub latest_comment_url: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

impl Notification {
    pub fn text(&self) -> String {
        let mut text = String::new();
        if let Some(ref subject) = self.subject {
            if let Some(ref title) = subject.title {
                text.push_str(title);
            }
        }
        if let Some(ref repository) = self.repository {
            if let Some(ref full_name) = repository.full_name {
                text.push_str(" - ");
                text.push_str(full_name);
            }
        }
        text
    }
}

pub fn serialize(json: &str) -> Result<Notifications, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str::<Notifications>(json)?)
}
