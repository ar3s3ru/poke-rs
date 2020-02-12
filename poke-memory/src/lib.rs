use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::Mutex;

use poke_domain::{pokemon, pokemon::Pokemon};

#[derive(Clone, Default)]
pub struct InMemoryRepository {
    backend: Arc<Mutex<Vec<Pokemon>>>,
}

impl From<Vec<Pokemon>> for InMemoryRepository {
    fn from(value: Vec<Pokemon>) -> Self {
        InMemoryRepository {
            backend: Arc::new(Mutex::new(value)),
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
            Ok(match num {
                0 => None,
                _ => self.backend.lock().await.get((num as usize) - 1).cloned(),
            })
        })
    }
}
