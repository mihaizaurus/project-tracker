pub mod models;
pub mod factories;
pub mod builders;
pub mod id;

use id::Id;

pub trait EntityType {
    fn prefix() -> &'static str;
}

pub trait HasId {
    type Entity;
    
    fn id(&self) -> Id<Self::Entity>;
}