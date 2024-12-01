use rocket::{
    get,
    routes,
};
use rocket_sunset::{
    deprecation,
    DeprecatedResponder,
};

#[get("/ts_only")]
#[deprecation("2024-12-31T23:59:59Z")]
pub fn ts_only() -> &'static str {
    "Hello, world!"
}

#[get("/ts_link")]
#[deprecation("2024-12-31T23:59:59Z", link = "https://developer.example.com/deprecation")]
pub fn ts_link() -> &'static str {
    "Hello, world!"
}

#[get("/ts_sunset")]
#[deprecation("2024-12-31T23:59:59Z", sunset = "2025-01-01T00:00:00Z")]
pub fn ts_sunset() -> &'static str {
    "Hello, world!"
}

#[get("/all")]
#[deprecation(
    "2024-12-31T23:59:59Z",
    link = "https://developer.example.com/deprecation",
    sunset = "2024-12-31T23:59:59-02:00"
)]
pub fn all() -> &'static str {
    "Hello, world!"
}

#[get("/order")]
#[deprecation(
    "2024-12-31T23:59:59Z",
    sunset = "2024-12-31T23:59:59-02:00",
    link = "https://developer.example.com/deprecation"
)]
pub fn order() -> &'static str {
    "Hello, world!"
}

fn main() {
    let _ = rocket::build().mount("/", routes![ts_only, ts_link, ts_sunset, all, order]);
}
