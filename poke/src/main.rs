use structopt::StructOpt;

use warp::Filter;

use poke_cli::{App, Subcommand};
use poke_domain::pokemon::{Element, Pokemon, Stats, Type};

#[tokio::main]
async fn main() {
    env_logger::init();

    match App::from_args().subcommand {
        Subcommand::Web { port } => {
            let logger = warp::log("poke");

            // let in_memory = poke_memory::InMemoryRepository::from(in_mem_pokedex());
            let poke_api = poke_pokeapi::repository::PokemonRepository::default();
            let repository = poke_api; // poke_memory::SecondLevelRepository::from((poke_api, in_memory));

            let routes = poke_http::api(repository).with(logger);

            warp::serve(routes).run(([0, 0, 0, 0], port)).await;
        }
    }
}

// fn in_mem_pokedex() -> Vec<Pokemon> {
//     vec![Pokemon {
//         name: String::from("Bulbasaur"),
//         typ: Type::Single(Element::Grass),
//         stats: Stats {
//             speed: 48,
//             special_defense: 48,
//             special_attack: 48,
//             defense: 48,
//             attack: 48,
//             hit_points: 48,
//         },
//     }]
// }
