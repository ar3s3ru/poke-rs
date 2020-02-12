use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use poke_domain::pokemon;

pub fn api<R>(repository: R) -> BoxedFilter<(impl Reply,)>
where
    R: pokemon::Repository + Send + Sync + Clone + 'static,
{
    let api = warp::path("pokemons");

    let get_pokemon_by_id = api
        .and(warp::get())
        .and(warp::path::param())
        .and(with_repository(repository))
        .and_then(get_pokemon_by_id);

    let get_pokemon_by_name = api
        .and(warp::get())
        .and(warp::path!("name" / String))
        .and_then(get_pokemon_by_name);

    warp::any()
        .and(get_pokemon_by_id)
        .or(get_pokemon_by_name)
        .boxed()
}

fn with_repository<R>(
    repository: R,
) -> impl Filter<Extract = (R,), Error = std::convert::Infallible> + Clone
where
    R: pokemon::Repository + Send + Clone,
{
    warp::any().map(move || repository.clone())
}

async fn get_pokemon_by_id<R>(id: u32, repository: R) -> Result<warp::reply::Json, warp::Rejection>
where
    R: pokemon::Repository + Send + Sync,
{
    let result = repository.get(id).await.map_err(|_err| warp::reject())?;

    match result {
        None => Err(warp::reject::not_found()),
        Some(pokemon) => Ok({
            log::debug!("Pokemon found: {:?}", pokemon);
            warp::reply::json(&pokemon)
        }),
    }
}

async fn get_pokemon_by_name(_name: String) -> Result<warp::reply::Json, warp::Rejection> {
    Err(warp::reject::not_found())
}
