pub mod cache;

use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::RwLock;

use poke_domain::{pokemon, pokemon::Pokemon};

#[derive(Clone, Default)]
pub struct InMemoryRepository {
    pub(crate) backend: Arc<RwLock<Vec<Pokemon>>>,
}

impl From<Vec<Pokemon>> for InMemoryRepository {
    #[inline]
    fn from(value: Vec<Pokemon>) -> Self {
        InMemoryRepository {
            backend: Arc::new(RwLock::new(value)),
        }
    }
}

impl pokemon::Repository for InMemoryRepository {
    type Error = std::convert::Infallible;

    fn get<'a>(&'a self, num: u32) -> BoxFuture<'a, Result<Option<Pokemon>, Self::Error>>
    where
        Self: Sync + 'a,
    {
        Box::pin(async move {
            let data = self.backend.read().await;
            let position = data.iter().position(|pokemon| pokemon.dex_id == num);

            Ok(position.and_then(|idx| data.get(idx).cloned()))
        })
    }
}
