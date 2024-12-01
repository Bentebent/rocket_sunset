mod routes {
    use rocket::get;
    use rocket_sunset::{
        deprecation,
        DeprecatedResponder,
    };

    #[get("/ts_only")]
    #[deprecation("2024-12-31T23:59:59Z")]
    pub fn ts_only() -> &'static str {
        "Hello, world!"
    }

    #[get("/with_link")]
    #[deprecation("2024-12-31T23:59:59Z", link = "https://api.example.com/docs")]
    pub fn with_link() -> &'static str {
        "Hello, world!"
    }

    #[get("/with_sunset")]
    #[deprecation("2024-12-31T23:59:59Z", sunset = "2025-12-31T23:59:59Z")]
    pub fn with_sunset() -> &'static str {
        "Hello, world!"
    }

    #[get("/full")]
    #[deprecation(
        "2024-12-31T23:59:59Z",
        link = "https://api.example.com/docs",
        sunset = "2025-12-31T23:59:59Z"
    )]
    pub fn full() -> &'static str {
        "Hello, world!"
    }
}

#[cfg(test)]
mod test {
    use rocket::{
        local::blocking::Client,
        routes,
    };

    use crate::routes;

    #[test]
    fn test_timestamp_only() {
        let rocket = rocket::build().mount("/", routes![routes::ts_only]);
        let client = Client::untracked(rocket).expect("valid rocket instance");

        let response = client.get("/ts_only").dispatch();
        assert_eq!(response.status(), rocket::http::Status::Ok);
        assert_eq!(response.headers().get_one("deprecation").unwrap(), "@1735689599");
        assert!(response.headers().get_one("link").is_none());
        assert!(response.headers().get_one("sunset").is_none());
    }

    #[test]
    fn test_with_link() {
        let rocket = rocket::build().mount("/", routes![routes::with_link]);
        let client = Client::untracked(rocket).expect("valid rocket instance");

        let response = client.get("/with_link").dispatch();
        assert_eq!(response.status(), rocket::http::Status::Ok);
        assert_eq!(response.headers().get_one("deprecation").unwrap(), "@1735689599");
        assert_eq!(
            response.headers().get_one("link").unwrap(),
            r#"<https://api.example.com/docs>; rel="deprecation"; type="text/html""#
        );
        assert!(response.headers().get_one("sunset").is_none());
    }

    #[test]
    fn test_with_sunset() {
        let rocket = rocket::build().mount("/", routes![routes::with_sunset]);
        let client = Client::untracked(rocket).expect("valid rocket instance");

        let response = client.get("/with_sunset").dispatch();
        assert_eq!(response.status(), rocket::http::Status::Ok);
        assert_eq!(response.headers().get_one("deprecation").unwrap(), "@1735689599");
        assert!(response.headers().get_one("link").is_none());
        assert_eq!(
            response.headers().get_one("sunset").unwrap(),
            "Wed, 31 Dec 2025 23:59:59 GMT"
        );
    }

    #[test]
    fn test_full_deprecation() {
        let rocket = rocket::build().mount("/", routes![routes::full]);
        let client = Client::untracked(rocket).expect("valid rocket instance");

        let response = client.get("/full").dispatch();
        assert_eq!(response.status(), rocket::http::Status::Ok);
        assert_eq!(response.headers().get_one("deprecation").unwrap(), "@1735689599");
        assert_eq!(
            response.headers().get_one("link").unwrap(),
            r#"<https://api.example.com/docs>; rel="deprecation"; type="text/html""#
        );
        assert_eq!(
            response.headers().get_one("sunset").unwrap(),
            "Wed, 31 Dec 2025 23:59:59 GMT"
        );
    }
}
