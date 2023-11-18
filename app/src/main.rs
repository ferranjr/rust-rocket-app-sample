mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use rocket::shield::Shield;
use rocket_prometheus::PrometheusMetrics;
use api::user_api::{create_user, delete_user, get_user, update_user, get_all_users};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let prometheus = PrometheusMetrics::new();
    let db = MongoRepo::init();
    let shield = Shield::default();
    rocket::build()
        .attach(shield)
        .attach(prometheus.clone())
        .manage(db)
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])
        .mount("/private/metrics", prometheus)
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn get_prometheus_metrics() {
        let client = Client::tracked(rocket()).expect("Valid Rocket instance");
        let response = client.get("/private/metrics").dispatch();
        assert_eq!(response.status(), Status::Ok);
        // assert_eq!(response.body_string(), Some("test_me".into()));
    }
}