use crate::integrations::GOGTokensData;
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod constants;
mod gog;
mod integrations;

slint::include_modules!();

struct JudyState {
    tokens: Option<GOGTokensData>,
}

impl JudyState {
    pub fn new() -> Self {
        JudyState { tokens: None }
    }

    pub fn tokens(&self) -> &Option<GOGTokensData> {
        &self.tokens
    }

    pub fn set_tokens(&mut self, tokens: GOGTokensData) {
        self.tokens = Some(tokens)
    }
}

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let state = Arc::new(Mutex::new(JudyState::new()));
    let client = reqwest::ClientBuilder::new()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    let window = MainWindow::new()?;

    let weak = window.as_weak();
    let mut state_handle = state.clone();
    tokio::spawn(async move {
        let token = integrations::load_refresh_token().await;
        let new_tokens = gog::auth::get_new_tokens(client.clone(), token.unwrap()).await;

        {
            let mut state = state_handle.lock().unwrap();
            state.set_tokens(new_tokens.clone());
        }

        let games = gog::games::load_games(client.clone(), &new_tokens).await;
        slint::invoke_from_event_loop(move || {
            let game_ids: Vec<slint::SharedString> = games.iter().map(|game| slint::SharedString::from(&game.external_id)).collect();
            let window = weak.unwrap();
                window.set_games_loading(false);
                window.set_games(slint::ModelRc::from(std::rc::Rc::new(
                    slint::VecModel::from(game_ids),
                )));
        })
    });

    window.run()
}
