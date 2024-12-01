use rocket::{
    get,
    routes,
};
use rocket_sunset::{
    deprecation,
    DeprecatedResponder,
};

#[get("/")]
#[deprecation("2024-12-31 23:59:59")]
pub fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let _ = rocket::build().mount("/", routes![index]);
}
