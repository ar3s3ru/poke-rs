use serde::{Deserialize, Serialize};

use poke_domain::pokemon;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub abilities: Vec<Ability>,
    #[serde(rename = "base_experience")]
    pub base_experience: i64,
    pub forms: Vec<Form>,
    #[serde(rename = "game_indices")]
    pub game_indices: Vec<Index>,
    pub height: i64,
    pub id: i64,
    #[serde(rename = "is_default")]
    pub is_default: bool,
    #[serde(rename = "location_area_encounters")]
    pub location_area_encounters: String,
    pub moves: Vec<Mfe>,
    pub name: String,
    pub order: i64,
    pub species: Species,
    pub stats: Vec<Stat>,
    pub types: Vec<Type>,
    pub weight: i64,
}

impl From<Root> for pokemon::Pokemon {
    fn from(value: Root) -> Self {
        pokemon::Pokemon {
            name: value.name,
            typ: pokemon::Type::Single(pokemon::Element::Normal),
            stats: pokemon::Stats {
                speed: 0,
                special_defense: 0,
                special_attack: 0,
                defense: 0,
                attack: 0,
                hit_points: 0,
            },
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    pub ability: Ability2,
    #[serde(rename = "is_hidden")]
    pub is_hidden: bool,
    pub slot: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability2 {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    #[serde(rename = "game_index")]
    pub game_index: i64,
    pub version: Version,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mfe {
    #[serde(rename = "move")]
    pub move_field: Move,
    #[serde(rename = "version_group_details")]
    pub version_group_details: Vec<VersionGroupDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Move {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionGroupDetail {
    #[serde(rename = "level_learned_at")]
    pub level_learned_at: i64,
    #[serde(rename = "move_learn_method")]
    pub move_learn_method: MoveLearnMethod,
    #[serde(rename = "version_group")]
    pub version_group: VersionGroup,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveLearnMethod {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionGroup {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Species {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    #[serde(rename = "base_stat")]
    pub base_stat: i64,
    pub effort: i64,
    pub stat: Stat2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat2 {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub slot: i64,
    #[serde(rename = "type")]
    pub type_field: Type2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type2 {
    pub name: String,
    pub url: String,
}
