use std::fmt::{Display, Formatter, Result as FmtResult};

use async_trait::async_trait;

use eventually::optional::{Aggregate, CommandHandler, EventOf, StateOf};
use eventually::{command, command::dispatcher::Identifiable};

#[derive(Debug, Clone, PartialEq)]
pub struct Trainer {
    name: String,
    sex: Sex,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Clone, PartialEq)]
pub enum TrainerCommand {
    StartAdventure { name: String, sex: Sex },
}

impl Identifiable for TrainerCommand {
    type SourceId = String;

    fn source_id(&self) -> Self::SourceId {
        use TrainerCommand::*;

        match self {
            StartAdventure { name, .. } => name.clone(),
        }
    }
}

#[derive(Clone)]
pub struct TrainerCommandHandler;
#[async_trait]
impl CommandHandler for TrainerCommandHandler {
    type Command = TrainerCommand;
    type Aggregate = Trainer;
    type Error = TrainerError;

    async fn handle_first(
        &self,
        command: Self::Command,
    ) -> command::Result<EventOf<Self::Aggregate>, Self::Error> {
        use TrainerCommand::*;
        use TrainerEvent::*;

        match command {
            StartAdventure { name, sex } => Ok(vec![AdventureStarted { name, sex }]),
        }
    }

    async fn handle_next(
        &self,
        _state: &StateOf<Self::Aggregate>,
        command: Self::Command,
    ) -> command::Result<EventOf<Self::Aggregate>, Self::Error> {
        use TrainerCommand::*;
        use TrainerError::*;

        match command {
            StartAdventure { name, .. } => Err(AdventureAlreadyStarted { name }),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum TrainerEvent {
    AdventureStarted { name: String, sex: Sex },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrainerError {
    AdventureAlreadyStarted { name: String },
}

impl std::error::Error for TrainerError {}

impl Display for TrainerError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use TrainerError::*;

        match self {
            AdventureAlreadyStarted { name } => {
                write!(f, "adventure already started for trainer {}", name)
            }
        }
    }
}

impl Aggregate for Trainer {
    type State = Trainer;
    type Event = TrainerEvent;
    type Error = TrainerError;

    fn apply_first(event: Self::Event) -> Result<Self::State, Self::Error> {
        use TrainerEvent::*;

        match event {
            AdventureStarted { name, sex } => Ok(Trainer { name, sex }),
        }
    }

    fn apply_next(_state: Self::State, event: Self::Event) -> Result<Self::State, Self::Error> {
        use TrainerError::*;
        use TrainerEvent::*;

        match event {
            AdventureStarted { name, .. } => Err(AdventureAlreadyStarted { name }),
        }
    }
}
