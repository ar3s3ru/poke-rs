use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use poke_domain::pokemon;

pub fn api<R, Store, Handler>(
    repository: R,
    event_store: Store,
    command_handler: Handler,
) -> BoxedFilter<(impl Reply,)>
where
    R: pokemon::Repository + Send + Sync + Clone + 'static,
    Store: eventually::Store<
            SourceId = String,
            Event = eventually::versioned::Versioned<poke_domain::trainer::TrainerEvent>,
        > + Send
        + Sync
        + Clone,
    Handler: eventually::CommandHandler<
            Command = poke_domain::trainer::TrainerCommand,
            Aggregate = eventually::versioned::AsAggregate<
                eventually::optional::AsAggregate<poke_domain::trainer::Trainer>,
            >,
            Error = poke_domain::trainer::TrainerError,
        > + Send
        + Sync
        + Clone,
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
        .and(with_store(event_store));

    warp::any()
        .and(get_pokemon_by_id)
        .or(get_pokemon_by_name)
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

async fn start_adventure_trainer<Store, Handler>(
    name: String,
    sex: poke_domain::trainer::Sex,
    event_store: Store,
    command_handler: Handler,
) -> Result<warp::reply::Json, warp::Rejection>
where
    Store: eventually::Store<
            SourceId = String,
            Event = eventually::versioned::Versioned<poke_domain::trainer::TrainerEvent>,
        > + Send
        + Sync,
    Handler: eventually::CommandHandler<
            Command = poke_domain::trainer::TrainerCommand,
            Aggregate = eventually::versioned::AsAggregate<
                eventually::optional::AsAggregate<poke_domain::trainer::Trainer>,
            >,
            Error = poke_domain::trainer::TrainerError,
        > + Send
        + Sync,
    <Store as EventStore>::SourceId: Clone + Eq + Send,
    <Store as EventStore>::Offset: Default + Send,
    <Store as EventStore>::Stream: Send,
    <Store as EventStore>::Error: std::error::Error + Send + 'static,
    command::AggregateOf<Handler>: AggregateExt<Event = <Store as EventStore>::Event> + Send,
    command::CommandOf<Handler>: eventually::command::dispatcher::Identifiable<SourceId = <Store as EventStore>::SourceId>
        + Send,
    aggregate::EventOf<command::AggregateOf<Handler>>: Clone + Send,
    aggregate::StateOf<command::AggregateOf<Handler>>: Default + Send,
    aggregate::ErrorOf<command::AggregateOf<Handler>>: std::error::Error + Send + 'static,
    command::ErrorOf<Handler>: std::error::Error + Send + 'static,
{
    let result = Dispatcher::new(event_store, command_handler)
        .dispatch(poke_domain::trainer::TrainerCommand::StartAdventure { name, sex })
        .await
        .unwrap();

    log::debug!("Returned state: {:?}", result);

    Ok(warp::reply::json(&true))
}

fn with_repository<R>(
    repository: R,
) -> impl Filter<Extract = (R,), Error = std::convert::Infallible> + Clone
where
    R: pokemon::Repository + Send + Clone,
{
    warp::any().map(move || repository.clone())
}

use eventually::command::dispatcher::Dispatcher;

fn with_store<Store>(
    store: Store,
) -> impl Filter<Extract = (Store,), Error = std::convert::Infallible> + Clone
where
    Store: eventually::Store<
            SourceId = String,
            Event = eventually::versioned::Versioned<poke_domain::trainer::TrainerEvent>,
        > + Send
        + Sync
        + Clone,
{
    warp::any().map(move || store.clone())
}

fn with_command_handler<Handler>(
    handler: Handler,
) -> impl Filter<Extract = (Handler,), Error = std::convert::Infallible> + Clone
where
    Handler: eventually::CommandHandler<
            Command = poke_domain::trainer::TrainerCommand,
            Aggregate = eventually::versioned::AsAggregate<
                eventually::optional::AsAggregate<poke_domain::trainer::Trainer>,
            >,
            Error = poke_domain::trainer::TrainerError,
        > + Send
        + Sync
        + Clone,
{
    warp::any().map(move || handler.clone())
}
