use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: u32,
    name: String,
    email: String, 
    pass: String,
} 

#[derive(Debug, Serialize, Deserialize)]
pub struct Product{
    id: u32,
    name: u32,
    image: String,
    brand: String,
    price: f32,
    rating: f32,
    rating_count: u32,
    description: String,
}
