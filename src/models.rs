use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    email: String, 
    pass: String,
} 

impl User{
    pub fn get_pass(&self) -> &str{
        return self.pass.as_str();
    }

    pub fn set_pass(&mut self, hash: String) {
        self.pass = hash;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product{
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    image: String,
    brand: String,
    price: f64,
    rating: f64,
    rating_count: u32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Orders{
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    id: Option<ObjectId>,
    user_id: String,
    products: Vec<String>,
}
