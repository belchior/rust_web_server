use mongodb::bson;

pub mod organization;
pub mod repository;
pub mod user;
pub mod utils;

pub type Pipeline = Vec<bson::Document>;

#[cfg(test)]
mod organization_spec;

#[cfg(test)]
mod repository_spec;

#[cfg(test)]
mod user_spec;

#[cfg(test)]
mod utils_spec;
