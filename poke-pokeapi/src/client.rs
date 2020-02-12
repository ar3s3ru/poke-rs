use crate::model;

pub const POKEAPI_URL: &str = "https://pokeapi.co/api/v2";

#[derive(Clone)]
pub struct Client(reqwest::Client);

impl Default for Client {
    fn default() -> Self {
        Client(reqwest::Client::new())
    }
}

impl Client {
    pub async fn get_pokemon_by_id(
        &self,
        id: u32,
    ) -> Result<Option<model::Root>, std::convert::Infallible> {
        let url = format!("{}/pokemon/{}", POKEAPI_URL, id);

        Ok(self
            .0
            .get(&url)
            .send()
            .await
            .unwrap()
            .json::<Option<model::Root>>()
            .await
            .unwrap())
    }
}
