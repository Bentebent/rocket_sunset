mod routes {
    use rocket::get;
    use rocket_sunset::{
        deprecation,
        DeprecatedResponder,
    };

    #[get("/index")]
    #[deprecation(
        "2024-12-31T23:59:59Z",
        link = "https://developer.example.com/deprecation",
        sunset = "2024-12-31T23:59:59-02:00"
    )]
    pub fn index() -> &'static str {
        "Hello, world!"
    }
}

#[cfg(test)]
mod test {
    use rocket::{
        local::blocking::Client,
        routes,
    };

    use super::*;

    #[test]
    fn test_index() {
        let rocket = rocket::build().mount("/", routes![routes::index]);
        let client = Client::untracked(rocket).expect("valid rocket instance");

        client.get("/index").dispatch();
    }
}
