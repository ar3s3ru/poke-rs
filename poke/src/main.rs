use structopt::StructOpt;
use warp::Filter;

use poke_cli::{App, Subcommand};

#[tokio::main]
async fn main() {
    env_logger::init();

    match App::from_args().subcommand {
        Subcommand::Web { port } => {
            let logger = warp::log("poke");

            let poke_api = poke_pokeapi::repository::PokemonRepository::default();
            let repository = poke_memory::cache::CacheLayer::from(poke_api);

            let routes = poke_http::api(repository).with(logger);

            warp::serve(routes).run(([0, 0, 0, 0], port)).await;
        }
    }
}
