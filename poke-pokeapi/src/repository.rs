use std::fmt::{Display, Formatter, Result as FmtResult};

use futures::future::BoxFuture;

use poke_domain::{pokemon, pokemon::Pokemon};

use crate::client::Client;

#[derive(Clone, Default)]
pub struct PokemonRepository(Client);

impl pokemon::Repository for PokemonRepository {
    type Error = RepositoryError;

    fn get<'a>(&'a self, num: u32) -> BoxFuture<'a, Result<Option<Pokemon>, Self::Error>>
    where
        Self: Sync + 'a,
    {
        Box::pin(async move {
            Ok(self
                .0
                .get_pokemon_by_id(num)
                .await
                .map_err(RepositoryError::from)?
                .map(Pokemon::from))
        })
    }
}

#[derive(Debug)]
pub enum RepositoryError {
    InternalServerError { inner: reqwest::Error },
}

impl std::error::Error for RepositoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RepositoryError::InternalServerError { inner } => Some(inner),
        }
    }
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use RepositoryError::*;

        match self {
            InternalServerError { inner } => {
                write!(f, "pokeapi.co client failed unrecoverably: {}", inner)
            }
        }
    }
}

impl From<reqwest::Error> for RepositoryError {
    fn from(error: reqwest::Error) -> RepositoryError {
        if error.is_status() {
            unimplemented!()
        } else {
            RepositoryError::InternalServerError { inner: error }
        }
    }
}
