use rocket::{
    get,
    launch,
    routes,
    Build,
    Rocket,
};
use rocket_sunset::{
    deprecation,
    DeprecatedResponder,
};

#[get("/")]
#[deprecation(
    "2024-12-31T23:59:59Z",
    link = "https://developer.example.com/deprecation",
    sunset = "2024-12-31T23:59:59-02:00"
)]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}
