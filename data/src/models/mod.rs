pub mod validate;

pub mod confirmation;
pub mod group;
pub mod product;
pub mod service;
pub mod user;

pub use confirmation::*;
pub use group::*;
pub use product::*;
pub use service::*;
pub use user::*;

pub type KeyType = i32;
