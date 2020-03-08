use eventually::command::dispatcher::DirectDispatcher;
use eventually::optional::CommandHandler;
use eventually::versioned::{CommandHandlerExt, Versioned};
use eventually_memory::Store;

use structopt::StructOpt;
use warp::Filter;

use poke_cli::{App, Subcommand};
use poke_domain::trainer::{TrainerCommandHandler, TrainerEvent};

#[tokio::main]
async fn main() {
    env_logger::init();

    match App::from_args().subcommand {
        Subcommand::Web { port } => web(port).await,
    }
}

async fn web(port: u16) {
    let logger = warp::log("poke");

    let event_store = Store::<String, Versioned<TrainerEvent>>::default();
    let handler = TrainerCommandHandler.as_handler().versioned();
    let dispatcher = DirectDispatcher::new(event_store, handler);

    let poke_api = poke_pokeapi::repository::PokemonRepository::default();
    let repository = poke_memory::cache::CacheLayer::from(poke_api);

    let routes = poke_http::api(repository, dispatcher).with(logger);

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
