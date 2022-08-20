use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Serialize, Deserialize, Builder, Default)]
#[builder(setter(into))]
pub struct User {
    pub id: Option<ObjectId>,
    pub name: String,
    pub location: String,
    pub title: String
}

// impl User {
//     fn get_id(&mut self) -> String { return self.name; }
//     fn get_name(&mut self) -> String 
// }