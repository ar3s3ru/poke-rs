// Gotta use a Box<Pin<Future<Result>>> for returning an async result
// from a trait for now, until we have Higher-kinded Types in stable...
use futures::future::BoxFuture;

use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Element {
    Normal,
    Fight,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Type {
    Single(Element),
    Double(Element, Element),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Stats {
    pub speed: u8,
    pub special_defense: u8,
    pub special_attack: u8,
    pub defense: u8,
    pub attack: u8,
    pub hit_points: u8,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Pokemon {
    pub name: String,
    pub typ: Type,
    pub stats: Stats,
}

pub trait Repository {
    type Error: std::error::Error;

    fn get<'a>(&'a self, num: u32) -> BoxFuture<'a, Result<Option<Pokemon>, Self::Error>>
    where
        Self: Sync + 'a;
}
