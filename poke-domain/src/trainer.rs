use std::fmt::{Display, Formatter, Result as FmtResult};

use async_trait::async_trait;

use eventually::optional::{Aggregate, CommandHandler, EventOf, StateOf};
use eventually::{command, command::dispatcher::Identifiable};

use serde::Serialize;

use crate::pokemon;
use crate::pokemon::Pokemon;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Trainer {
    name: String,
    sex: Sex,
    pokemons: Vec<Pokemon>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Sex {
    Male,
    Female,
}

#[derive(Clone, PartialEq)]
pub enum TrainerCommand {
    StartAdventure { name: String, sex: Sex },
    AddPokemonToTeam { name: String, pokemon_id: u32 },
}

impl Identifiable for TrainerCommand {
    type SourceId = String;

    fn source_id(&self) -> Self::SourceId {
        use TrainerCommand::*;

        match self {
            StartAdventure { name, .. } => name.clone(),
            AddPokemonToTeam { name, .. } => name.clone(),
        }
    }
}

#[derive(Clone)]
pub struct TrainerCommandHandler<R> {
    poke_repository: R,
}

impl<R> TrainerCommandHandler<R> {
    pub fn new(repository: R) -> Self {
        TrainerCommandHandler {
            poke_repository: repository,
        }
    }
}

#[async_trait]
impl<R> CommandHandler for TrainerCommandHandler<R>
where
    R: pokemon::Repository + Send + Sync,
{
    type Command = TrainerCommand;
    type Aggregate = Trainer;
    type Error = TrainerCommandHandlerError<R::Error>;

    async fn handle_first(
        &self,
        command: Self::Command,
    ) -> command::Result<EventOf<Self::Aggregate>, Self::Error> {
        use TrainerCommand::*;
        use TrainerCommandHandlerError::*;
        use TrainerError::*;
        use TrainerEvent::*;

        match command {
            StartAdventure { name, sex } => Ok(vec![AdventureStarted { name, sex }]),
            AddPokemonToTeam { .. } => Err(InvalidCommand(AdventureNotStarted)),
        }
    }

    async fn handle_next(
        &self,
        _state: &StateOf<Self::Aggregate>,
        command: Self::Command,
    ) -> command::Result<EventOf<Self::Aggregate>, Self::Error> {
        use TrainerCommand::*;
        use TrainerCommandHandlerError::*;
        use TrainerError::*;

        match command {
            StartAdventure { name, .. } => Err(InvalidCommand(AdventureAlreadyStarted { name })),
            AddPokemonToTeam { pokemon_id, .. } => self.add_pokemon_to_team(pokemon_id).await,
        }
    }
}

impl<R> TrainerCommandHandler<R>
where
    R: pokemon::Repository + Send + Sync,
{
    async fn add_pokemon_to_team(
        &self,
        pokemon_id: u32,
    ) -> Result<Vec<TrainerEvent>, TrainerCommandHandlerError<R::Error>> {
        use TrainerCommandHandlerError::*;
        use TrainerEvent::*;

        let pokemon = self
            .poke_repository
            .get(pokemon_id)
            .await
            .map_err(RepositoryError)?
            .ok_or(NoPokemonsFound)?;

        Ok(vec![PokemonAdded { pokemon }])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrainerCommandHandlerError<R> {
    NoPokemonsFound,
    InvalidCommand(TrainerError),
    RepositoryError(R),
}

impl<R> std::error::Error for TrainerCommandHandlerError<R>
where
    R: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use TrainerCommandHandlerError::*;

        match self {
            NoPokemonsFound => None,
            InvalidCommand(inner) => Some(inner),
            RepositoryError(inner) => Some(inner),
        }
    }
}

impl<R> Display for TrainerCommandHandlerError<R>
where
    R: std::error::Error,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use TrainerCommandHandlerError::*;

        match self {
            NoPokemonsFound => write!(f, "no pokemon found"),
            InvalidCommand(inner) => Display::fmt(&inner, f),
            RepositoryError(inner) => Display::fmt(&inner, f),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum TrainerEvent {
    AdventureStarted { name: String, sex: Sex },
    PokemonAdded { pokemon: Pokemon },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrainerError {
    AdventureAlreadyStarted { name: String },
    AdventureNotStarted,
}

impl std::error::Error for TrainerError {}

impl Display for TrainerError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use TrainerError::*;

        match self {
            AdventureAlreadyStarted { name } => {
                write!(f, "adventure already started for trainer {}", name)
            }
            AdventureNotStarted => write!(f, "adventure not started yet"),
        }
    }
}

impl Aggregate for Trainer {
    type State = Trainer;
    type Event = TrainerEvent;
    type Error = TrainerError;

    fn apply_first(event: Self::Event) -> Result<Self::State, Self::Error> {
        use TrainerError::*;
        use TrainerEvent::*;

        match event {
            AdventureStarted { name, sex } => Ok(Trainer {
                name,
                sex,
                pokemons: Vec::default(),
            }),
            PokemonAdded { .. } => Err(AdventureNotStarted),
        }
    }

    fn apply_next(mut state: Self::State, event: Self::Event) -> Result<Self::State, Self::Error> {
        use TrainerError::*;
        use TrainerEvent::*;

        match event {
            AdventureStarted { name, .. } => Err(AdventureAlreadyStarted { name }),
            PokemonAdded { pokemon } => {
                state.pokemons.push(pokemon);
                Ok(state)
            }
        }
    }
}
