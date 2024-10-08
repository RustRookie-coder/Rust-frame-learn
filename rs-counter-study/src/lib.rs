pub mod db {
    pub mod cache;
    pub mod database;
    pub mod kafka;
    pub mod mq;
}

pub mod dto {
    pub mod dto;
}

pub mod error {
    pub mod errors;
}

pub mod handlers {
    pub mod auth;
    pub mod health;
    pub mod log;
    pub mod users;
}

pub mod models {
    pub mod model;
    pub mod msg;
}

pub mod repository {
    pub mod users;
    pub mod log;
    pub mod message;
}

pub mod routes {
    pub mod counter;
    pub mod counter_record;

    pub mod jwt;
}

pub mod service {
    pub mod logs_impl;
    pub mod users_impl;
}

pub mod utils {
    pub mod password;
    pub mod token;
}

pub mod config;
pub mod middleware {
    pub mod auth_middleware;
}

pub mod result;
pub mod app_state;

pub mod kafka {
    pub mod consumer;
    pub mod productor;
}
