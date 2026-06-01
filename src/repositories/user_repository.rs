use crate::models::users::Entity;
use aestivate::repository;

#[repository(Entity)]
pub trait UserRepository {}
