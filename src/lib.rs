pub mod error;

pub mod models {
    pub mod users;
    pub mod tweets;
    pub mod follows;
}

pub mod repositories {
    pub mod user_repository;
}

pub mod services {
    pub mod user_service;
}