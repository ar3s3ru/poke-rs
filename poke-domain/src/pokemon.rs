pub enum Type {
    Normal,
}

pub enum Sex {
    Male,
    Female,
}

pub struct Pokemon {
    pub name: String,
    pub typ: Type,
    pub sex: Sex,
}

pub trait Repository {
    type Error;

    fn get(num: u32) -> Result<Option<Pokemon>, Self::Error>;
}
