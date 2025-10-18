// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::ContentDto;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: ContentDto = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateArticleDto {
    pub title: String,
    pub body: String,
}
