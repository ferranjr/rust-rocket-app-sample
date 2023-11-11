mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use rocket_prometheus::PrometheusMetrics;
use api::user_api::{create_user, delete_user, get_user, update_user, get_all_users};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let prometheus = PrometheusMetrics::new();
    let db = MongoRepo::init();
    rocket::build()
        .attach(prometheus.clone())
        .manage(db)
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])
        .mount("/private/metrics", prometheus)
}