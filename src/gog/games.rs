use reqwest::{Client, Url};
use serde::Deserialize;
use crate::integrations::GOGTokensData;

#[derive(Deserialize)]
struct LibraryResponse {
    total_count: u32,
    limit: u32,
    next_page_token: Option<String>,
    items: Vec<GameEntry>
}

#[derive(Deserialize, Debug, Clone)]
pub struct GameEntry{
    pub platform_id: String,
    pub external_id: String,
    pub certificate: String,
    pub owned: bool
}

pub async fn load_games(client: Client, credentials: &GOGTokensData) -> Vec<GameEntry> {
    let mut accumulator: Vec<GameEntry> = Vec::new();
    let response = get_gog_games(&client, &credentials.user_id, &credentials.access_token, None).await.expect("Failed to get games");

    let mut page_token: Option<String> = response.next_page_token;
    accumulator.extend(response.items);

    while page_token.is_some() {
        let response = get_gog_games(&client, &credentials.user_id, &credentials.access_token, None).await.expect("Failed to get games");
        accumulator.extend(response.items);
        page_token = response.next_page_token;
    }

    accumulator.iter().filter(|game| game.platform_id == "gog").map(|game| game.to_owned()).collect()
}

async fn get_gog_games(client: &Client, user_id: &String, access_token: &String, page_token: Option<String>) -> Result<LibraryResponse, reqwest::Error> {
    let mut url = Url::parse(&format!("https://galaxy-library.gog.com/users/{}/releases", user_id)).unwrap();
    if let Some(token) = page_token {
        url.query_pairs_mut().append_pair("page_token", &token);
    }

    let response = client.get(url.as_str()).header("Authorization", format!("Bearer {}", access_token)).send().await?;
    let games: LibraryResponse = response.json().await?;
    Ok(games)
}
