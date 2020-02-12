use futures::future::BoxFuture;

use poke_domain::{pokemon, pokemon::Pokemon};

use crate::client::Client;

#[derive(Clone, Default)]
pub struct PokemonRepository(Client);

impl pokemon::Repository for PokemonRepository {
    type Error = std::convert::Infallible;

    fn get<'a>(&'a self, num: u32) -> BoxFuture<'a, Result<Option<Pokemon>, Self::Error>>
    where
        Self: Sync + 'a,
    {
        Box::pin(async move {
            Ok(self
                .0
                .get_pokemon_by_id(num)
                .await
                .unwrap()
                .map(Pokemon::from))
        })
    }
}
