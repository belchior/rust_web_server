use mongodb::bson;

pub mod organization;
pub mod repository;
pub mod user;
pub mod utils;

pub type Pipeline = Vec<bson::Document>;

#[cfg(test)]
mod utils_spec;
