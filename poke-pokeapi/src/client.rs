use crate::model;

pub const POKEAPI_URL: &str = "https://pokeapi.co/api/v2";

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            client: reqwest::Client::new(),
        }
    }
}

impl Client {
    pub async fn get_pokemon_by_id(&self, id: u32) -> Result<Option<model::Root>, reqwest::Error> {
        let url = format!("{}/pokemon/{}", POKEAPI_URL, id);

        self.client
            .get(&url)
            .send()
            .await?
            .json::<Option<model::Root>>()
            .await
    }
}
