#[macro_use]
extern crate rocket;

#[get("/status")]
fn index() -> &'static str {
    "ok"
}

#[launch]
fn rocket() -> _ {
    use rocket_prometheus::PrometheusMetrics;
    let prometheus = PrometheusMetrics::new();

    rocket::build()
        .attach(prometheus.clone())
        .mount("/", routes![index])
        .mount("/metrics", prometheus)
}

