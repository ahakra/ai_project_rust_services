use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Menu {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub menu_items: Vec<MenuItem>,
    pub static_flag: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuItem {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuItemChild {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub parent_id: ObjectId,
    pub static_flag: bool,
}
