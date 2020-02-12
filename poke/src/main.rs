use structopt::StructOpt;

use warp::Filter;

use poke_cli::{App, Subcommand};
use poke_domain::pokemon::{Element, Pokemon, Stats, Type};

#[tokio::main]
async fn main() {
    env_logger::init();
    let app = App::from_args();

    match app.subcommand {
        Subcommand::Web { port } => {
            let logger = warp::log("poke");

            let repository = poke_memory::InMemoryRepository::from(in_mem_pokedex());
            let routes = poke_http::api(repository).with(logger);

            warp::serve(routes).run(([127, 0, 0, 1], port)).await;
        }
    }
}

fn in_mem_pokedex() -> Vec<Pokemon> {
    vec![Pokemon {
        name: String::from("Bulbasaur"),
        typ: Type::Single(Element::Grass),
        stats: Stats {
            speed: 48,
            special_defense: 48,
            special_attack: 48,
            defense: 48,
            attack: 48,
            hit_points: 48,
        },
    }]
}
