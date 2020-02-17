// Gotta use a Box<Pin<Future<Result>>> for returning an async result
// from a trait for now, until we have Higher-kinded Types in stable...
use futures::future::BoxFuture;

use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
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
#[serde(untagged)]
pub enum Type {
    Single(Element),
    Double(Element, Element),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Stats {
    pub speed: u16,
    pub special_defense: u16,
    pub special_attack: u16,
    pub defense: u16,
    pub attack: u16,
    pub hit_points: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Pokemon {
    pub dex_id: u32,
    pub name: String,

    #[serde(rename = "type")]
    pub typ: Type,
    pub height: u32,
    pub weight: u32,
    pub base_experience: u32,
    pub stats: Stats,
}

pub trait Repository {
    type Error: std::error::Error;

    fn get<'a>(&'a self, num: u32) -> BoxFuture<'a, Result<Option<Pokemon>, Self::Error>>
    where
        Self: Sync + 'a;
}
