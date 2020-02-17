use futures::future::BoxFuture;
use tokio::sync::mpsc;

use poke_domain::{pokemon, pokemon::Pokemon};

use crate::InMemoryRepository;

#[derive(Clone)]
pub struct CacheLayer<R> {
    upstream: R,
    tx: mpsc::Sender<Pokemon>,
    inmemory: InMemoryRepository,
}

impl<R> From<R> for CacheLayer<R>
where
    R: pokemon::Repository,
{
    fn from(upstream: R) -> CacheLayer<R> {
        let inmemory = InMemoryRepository::default();
        let cache = inmemory.clone();

        let (tx, mut rx) = mpsc::channel::<Pokemon>(1);

        // Spawn a thread for background update
        tokio::spawn(async move {
            while let Some(pokemon) = rx.recv().await {
                log::debug!("Updating cache with Pokemon #{}", pokemon.dex_id);
                cache.backend.write().await.push(pokemon);
            }
        });

        CacheLayer {
            upstream,
            tx,
            inmemory,
        }
    }
}

impl<R> pokemon::Repository for CacheLayer<R>
where
    R: pokemon::Repository + Sync + Send,
{
    type Error = R::Error;

    fn get<'a>(&'a self, num: u32) -> BoxFuture<'a, Result<Option<Pokemon>, Self::Error>>
    where
        Self: Sync + 'a,
    {
        Box::pin(async move {
            if let Some(pokemon) = self.inmemory.get(num).await.unwrap() {
                log::debug!("Got Pokemon #{} from cache", pokemon.dex_id);
                return Ok(Some(pokemon));
            }

            let result = self.upstream.get(num).await?;
            log::debug!("Got Pokemon #{} from upstream", num);

            if let Some(ref pokemon) = result {
                log::debug!("Sending Pokemon #{} to background thread", pokemon.dex_id);
                self.tx.clone().send(pokemon.clone()).await.unwrap();
            }

            Ok(result)
        })
    }
}
