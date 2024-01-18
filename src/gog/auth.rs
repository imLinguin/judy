use crate::constants;
use crate::integrations::GOGTokensData;

pub async fn get_new_tokens(client: reqwest::Client, refresh_token: String) -> GOGTokensData {
    let url = reqwest::Url::parse_with_params(
        "https://auth.gog.com/token?without_new_session=1&grant_type=refresh_token",
        [
            ("client_id", constants::CLIENT_ID),
            ("client_secret", constants::CLIENT_SECRET),
            ("refresh_token", &refresh_token)
        ],
    ).unwrap();

    let response = client
        .get(url.as_str())
        .send()
        .await
        .expect("Failed to get new tokens");

    response.json().await.expect("Failed to parse data")
}
