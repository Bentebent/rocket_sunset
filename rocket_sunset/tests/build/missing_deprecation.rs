use rocket::{
    get,
    routes,
};
use rocket_sunset::{
    deprecation,
    DeprecatedResponder,
};

#[get("/")]
#[deprecation(sunset = "2024-01-01T00:00:00Z")]
pub fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let _ = rocket::build().mount("/", routes![index]);
}
