use rocket::{
    self,
    http::Header,
    response::{
        self,
        Responder,
    },
    Request,
};
pub use rocket_sunset_macro::deprecation;

pub struct DeprecatedResponder<T: for<'r> Responder<'r, 'static>> {
    pub inner: T,
    pub deprecation: Header<'static>,
    pub link: Option<Header<'static>>,
    pub sunset: Option<Header<'static>>,
}

impl<T: for<'r> Responder<'r, 'static>> DeprecatedResponder<T> {
    pub fn new(inner: T, deprecated_timestamp: &str, link: Option<&str>, sunset: Option<&str>) -> Self {
        DeprecatedResponder {
            inner,
            deprecation: Header::new("deprecation", format!("@{}", deprecated_timestamp)),
            link: link.map(|l| Header::new("link", format!(r#"<{}>; rel="deprecation"; type="text/html""#, l))),
            sunset: sunset.map(|s| Header::new("sunset", s.to_owned())),
        }
    }
}

impl<'r, 'o: 'r, T> Responder<'r, 'o> for DeprecatedResponder<T>
where
    T: for<'any> Responder<'any, 'static>,
{
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        self.inner.respond_to(req).map(|mut response| {
            response.set_header(self.deprecation);
            if let Some(header) = self.link {
                response.set_header(header);
            }
            if let Some(header) = self.sunset {
                response.set_header(header);
            }
            response
        })
    }
}
