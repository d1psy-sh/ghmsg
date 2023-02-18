use tokio;

use crate::env;
use reqwest::header::HeaderMap;

#[tokio::main]
pub async fn get_notifications() -> Result<String, Box<dyn std::error::Error>> {
    // TODO: add the right url here
    let token = env::get_token();
    let mut header_map = HeaderMap::new();
    // curl \
    // -H "Accept: application/vnd.github+json" \
    // -H "Authorization: Bearer <YOUR-TOKEN>"\
    // -H "X-GitHub-Api-Version: 2022-11-28" \
    // https://api.github.com/notifications
    header_map.insert("Accept", "application/vnd.github+json".parse()?);
    header_map.insert("Authorization", format!("Bearer {token}").parse()?);
    header_map.insert("X-GitHub-Api-Version", "2022-11-28".parse()?);
    header_map.insert("User-Agent", "rust web-api-client demo".parse()?);
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.github.com/notifications")
        .headers(header_map)
        .send()
        .await?;
    let sync_res = res.text().await?;
    Ok(sync_res)
}
