use serde::{Deserialize, Serialize};

use poke_domain::pokemon;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    // pub abilities: Vec<Ability>,
    #[serde(rename = "base_experience")]
    pub base_experience: i64,
    // pub forms: Vec<Form>,
    // #[serde(rename = "game_indices")]
    // pub game_indices: Vec<Index>,
    pub height: i64,
    pub id: i64,
    // #[serde(rename = "is_default")]
    // pub is_default: bool,
    #[serde(rename = "location_area_encounters")]
    pub location_area_encounters: String,
    // pub moves: Vec<Mfe>,
    pub name: String,
    pub order: i64,
    // pub species: Species,
    pub stats: Vec<Stat>,
    pub types: Vec<Type>,
    pub weight: i64,
}

impl Root {
    fn from_types(types: &[Type]) -> pokemon::Type {
        match types.len() {
            1 => pokemon::Type::Single(types.get(0).unwrap().into()),
            2 => pokemon::Type::Double(types.get(0).unwrap().into(), types.get(1).unwrap().into()),
            _ => panic!("can't have no type or more than 2 types"),
        }
    }

    fn from_stats(stats: &[Stat]) -> pokemon::Stats {
        let mut speed = 0;
        let mut special_defense = 0;
        let mut special_attack = 0;
        let mut defense = 0;
        let mut attack = 0;
        let mut hit_points = 0;

        for stat in stats.iter() {
            match &*(stat.stat.name) {
                "speed" => speed = stat.base_stat as u16,
                "special-defense" => special_defense = stat.base_stat as u16,
                "special-attack" => special_attack = stat.base_stat as u16,
                "defense" => defense = stat.base_stat as u16,
                "attack" => attack = stat.base_stat as u16,
                "hp" => hit_points = stat.base_stat as u16,
                _ => continue,
            }
        }

        pokemon::Stats {
            speed,
            special_defense,
            special_attack,
            defense,
            attack,
            hit_points,
        }
    }
}

impl From<Root> for pokemon::Pokemon {
    fn from(value: Root) -> Self {
        pokemon::Pokemon {
            name: value.name,
            typ: Root::from_types(&value.types),
            stats: Root::from_stats(&value.stats),
        }
    }
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Ability {
//     pub ability: Ability2,
//     #[serde(rename = "is_hidden")]
//     pub is_hidden: bool,
//     pub slot: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Ability2 {
//     pub name: String,
//     pub url: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Form {
//     pub name: String,
//     pub url: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Index {
//     #[serde(rename = "game_index")]
//     pub game_index: i64,
//     pub version: Version,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Version {
//     pub name: String,
//     pub url: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Mfe {
//     #[serde(rename = "move")]
//     pub move_field: Move,
//     #[serde(rename = "version_group_details")]
//     pub version_group_details: Vec<VersionGroupDetail>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Move {
//     pub name: String,
//     pub url: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct VersionGroupDetail {
//     #[serde(rename = "level_learned_at")]
//     pub level_learned_at: i64,
//     #[serde(rename = "move_learn_method")]
//     pub move_learn_method: MoveLearnMethod,
//     #[serde(rename = "version_group")]
//     pub version_group: VersionGroup,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct MoveLearnMethod {
//     pub name: String,
//     pub url: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct VersionGroup {
//     pub name: String,
//     pub url: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Species {
//     pub name: String,
//     pub url: String,
// }

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

impl From<&Type> for pokemon::Element {
    fn from(value: &Type) -> pokemon::Element {
        (&value.type_field).into()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type2 {
    pub name: String,
    pub url: String,
}

impl From<&Type2> for pokemon::Element {
    fn from(value: &Type2) -> pokemon::Element {
        // String implements Deref<Target=str> ;)
        match &*(value.name) {
            "normal" => pokemon::Element::Normal,
            "fight" => pokemon::Element::Fight,
            "flying" => pokemon::Element::Flying,
            "poison" => pokemon::Element::Poison,
            "ground" => pokemon::Element::Ground,
            "rock" => pokemon::Element::Rock,
            "bug" => pokemon::Element::Bug,
            "ghost" => pokemon::Element::Ghost,
            "steel" => pokemon::Element::Steel,
            "fire" => pokemon::Element::Fire,
            "water" => pokemon::Element::Water,
            "grass" => pokemon::Element::Grass,
            "electric" => pokemon::Element::Electric,
            "psychic" => pokemon::Element::Psychic,
            "ice" => pokemon::Element::Ice,
            "dragon" => pokemon::Element::Dragon,
            "dark" => pokemon::Element::Dark,
            "fairy" => pokemon::Element::Fairy,
            _ => panic!("should not happen!"),
        }
    }
}
