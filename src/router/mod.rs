use crate::controller::user;
use axum::{routing::get, Router};

pub fn routes() -> Vec<Router> {
    let mut routes = Vec::new();

    // test controller.
    let router = Router::new().route("/home", get(user::new));
    routes.push(router);

    routes
}
