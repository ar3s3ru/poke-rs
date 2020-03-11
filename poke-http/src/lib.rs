use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use eventually::command::{Dispatcher, Handler};
use eventually::optional::AsAggregate as OptionalAggregate;
use eventually::versioned::AsAggregate as VersionedAggregate;

use poke_domain::pokemon;
use poke_domain::trainer::{Trainer, TrainerCommand};

pub fn api<R, D>(repository: R, dispatcher: D) -> BoxedFilter<(impl Reply,)>
where
    R: pokemon::Repository + Send + Sync + Clone + 'static,
    D: Dispatcher + Send + Sync + Clone + 'static,
    <D as Dispatcher>::CommandHandler: Handler<
        Command = TrainerCommand,
        Aggregate = VersionedAggregate<OptionalAggregate<Trainer>>,
    >,
    <D as Dispatcher>::Error: std::error::Error,
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

    let start_adventure = api
        .and(warp::post())
        .and(warp::path!("adventure" / "start" / "name" / String))
        .and(with_dispatcher(dispatcher.clone()))
        .and_then(start_adventure_trainer);

    let add_pokemon = api
        .and(warp::post())
        .and(warp::path!("adventure" / String / "team" / "add" / u32))
        .and(with_dispatcher(dispatcher))
        .and_then(add_pokemon_to_team);

    warp::any()
        .and(get_pokemon_by_id)
        .or(get_pokemon_by_name)
        .or(add_pokemon)
        .or(start_adventure)
        .boxed()
}

async fn get_pokemon_by_id<R>(id: u32, repository: R) -> Result<warp::reply::Json, warp::Rejection>
where
    R: pokemon::Repository + Send + Sync,
{
    let result = repository.get(id).await.map_err(|err| {
        log::error!("Error received while calling repository: {}", err);
        warp::reject()
    })?;

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

async fn start_adventure_trainer<D>(
    name: String,
    mut dispatcher: D,
) -> Result<warp::reply::Json, warp::Rejection>
where
    D: Dispatcher,
    <D as Dispatcher>::CommandHandler: Handler<
        Command = TrainerCommand,
        Aggregate = VersionedAggregate<OptionalAggregate<Trainer>>,
    >,
{
    let result = dispatcher
        .dispatch(TrainerCommand::StartAdventure {
            name,
            sex: poke_domain::trainer::Sex::Male,
        })
        .await
        .map_err(|_err| warp::reject())?;

    log::debug!("Returned state: {:?}", result);

    Ok(warp::reply::json(&(result.take())))
}

async fn add_pokemon_to_team<D>(
    name: String,
    pokemon_id: u32,
    mut dispatcher: D,
) -> Result<warp::reply::Json, warp::Rejection>
where
    D: Dispatcher,
    <D as Dispatcher>::CommandHandler: Handler<
        Command = TrainerCommand,
        Aggregate = VersionedAggregate<OptionalAggregate<Trainer>>,
    >,
    <D as Dispatcher>::Error: std::error::Error,
{
    log::debug!("banana");

    let result = dispatcher
        .dispatch(TrainerCommand::AddPokemonToTeam { name, pokemon_id })
        .await
        .map_err(|err| {
            log::error!("failed to add pokemon to team: {}", err);
            warp::reject()
        })?;

    log::debug!("Returned state: {:?}", result);

    Ok(warp::reply::json(&(result.take())))
}

fn with_repository<R>(
    repository: R,
) -> impl Filter<Extract = (R,), Error = std::convert::Infallible> + Clone
where
    R: pokemon::Repository + Send + Clone,
{
    warp::any().map(move || repository.clone())
}

fn with_dispatcher<D>(
    dispatcher: D,
) -> impl Filter<Extract = (D,), Error = std::convert::Infallible> + Clone
where
    D: Dispatcher + Send + Sync + Clone,
    <D as Dispatcher>::CommandHandler: Handler<
        Command = TrainerCommand,
        Aggregate = VersionedAggregate<OptionalAggregate<Trainer>>,
    >,
{
    warp::any().map(move || dispatcher.clone())
}
