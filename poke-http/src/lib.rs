use warp::filters::BoxedFilter;
use warp::http::{Response, StatusCode};
use warp::{Filter, Reply};

pub fn api() -> BoxedFilter<(impl Reply,)> {
    let api = warp::path("pokemons");

    let get_pokemon_by_id = api
        .and(warp::get())
        .and(warp::path::param())
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

async fn get_pokemon_by_id(id: u32) -> Result<impl Reply, std::convert::Infallible> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(format!("Requested pokemon #{}", id)))
}

async fn get_pokemon_by_name(name: String) -> Result<impl Reply, std::convert::Infallible> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(format!("Requested pokemon '{}'", name)))
}
