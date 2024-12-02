# Rocket Sunset

[![Build Status](https://github.com/sd2k/rocket_prometheus/workflows/Rust/badge.svg)](https://github.com/Bentebent/rocket_sunset/actions)
[![docs.rs](https://docs.rs/rocket_sunset/badge.svg)](https://docs.rs/rocket_sunset)
[![crates.io](https://img.shields.io/crates/v/rocket_sunset.svg)](https://crates.io/crates/rocket_sunset)

Proc macros for handling HTTP deprecation headers for Rocket applications. Based on the draft proposal [The Deprecation HTTP Header Field](https://github.com/ietf-wg-httpapi/deprecation-header).

## Usage

Add this crate to your `Cargo.toml` alongside Rocket 0.5:

```toml
[dependencies]
rocket = "0.5.1"
rocket_sunset = "0.1.0"
```

Define your endpoints as usual in Rocket. Apply the macro `deprecation`with the relevant information.

```rust
use rocket_sunset::{
    deprecation,
    DeprecatedResponder,
};

#[get("/ts_only")]
#[deprecation("2024-12-31T23:59:59Z")]
pub fn ts_only() -> &'static str {
    "Hello, timestamp!"
}

#[get("/with_link")]
#[deprecation("2024-12-31T23:59:59Z", link = "https://api.example.com/docs")]
pub fn with_link() -> &'static str {
    "Hello, link!"
}

#[get("/with_sunset")]
#[deprecation("2024-12-31T23:59:59Z", sunset = "2025-12-31T23:59:59Z")]
pub fn with_sunset() -> &'static str {
    "Hello, sunset!"
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
```

This will wrap your endpoint response in a DeprecatedResponder that adds HTTP headers for deprecation, depreaction link and sunset. This showcases the added headers for the `/full` endpoint above.

| Name        | Value                                                                |
| ----------- | -------------------------------------------------------------------- |
| deprecation | "@1735689599"                                                        |
| link        | "<https://api.example.com/docs>; rel="deprecation"; type="text/html" |
| sunset      | "Wed, 31 Dec 2025 23:59:59 GMT"                                      |

## TODO

- [x] Deprecation proc macro support
- [ ] Sunset proc macro support (automatically disable endpoints past sunset timestamp)
- [ ] Documentation
- [x] Proc macro build unit tests
- [ ] Tests

## License

You can choose between [MIT License](https://opensource.org/licenses/MIT) or [Apache License 2.0](http://www.apache.org/licenses/LICENSE-2.0).

### MIT License

Copyright (c) 2024 Tim Henriksson

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice (including the next paragraph) shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

### Apache License 2.0

Copyright 2024 Tim Henriksson

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
