use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use eventually::command::Dispatcher;

use poke_domain::pokemon;

pub fn api<R, D>(repository: R, dispatcher: D) -> BoxedFilter<(impl Reply,)>
where
    R: pokemon::Repository + Send + Sync + Clone + 'static,
    D: Dispatcher + Send + Sync + Clone + 'static,
    <D as Dispatcher>::CommandHandler: eventually::command::Handler<
        Command = poke_domain::trainer::TrainerCommand,
        Aggregate = eventually::versioned::AsAggregate<
            eventually::optional::AsAggregate<poke_domain::trainer::Trainer>,
        >,
        Error = poke_domain::trainer::TrainerError,
    >,
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
        .and(warp::path!("adventure" / "start" / "name" / String))
        .and(with_dispatcher(dispatcher))
        .and_then(start_adventure_trainer);

    warp::any()
        .and(get_pokemon_by_id)
        .or(get_pokemon_by_name)
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

use eventually::{aggregate, aggregate::AggregateExt, command, Store as EventStore};

async fn start_adventure_trainer<D>(
    name: String,
    mut dispatcher: D,
) -> Result<warp::reply::Json, warp::Rejection>
where
    D: Dispatcher,
    <D as Dispatcher>::CommandHandler: eventually::command::Handler<
        Command = poke_domain::trainer::TrainerCommand,
        Aggregate = eventually::versioned::AsAggregate<
            eventually::optional::AsAggregate<poke_domain::trainer::Trainer>,
        >,
        Error = poke_domain::trainer::TrainerError,
    >,
{
    let result = dispatcher
        .dispatch(poke_domain::trainer::TrainerCommand::StartAdventure {
            name,
            sex: poke_domain::trainer::Sex::Male,
        })
        .await
        .map_err(|_err| warp::reject())?;

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
    <D as Dispatcher>::CommandHandler: eventually::command::Handler<
        Command = poke_domain::trainer::TrainerCommand,
        Aggregate = eventually::versioned::AsAggregate<
            eventually::optional::AsAggregate<poke_domain::trainer::Trainer>,
        >,
        Error = poke_domain::trainer::TrainerError,
    >,
{
    warp::any().map(move || dispatcher.clone())
}
