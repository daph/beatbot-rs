extern crate beats;
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate serde_derive;

use beats::Beat;
use std::env;
use actix_web::{middleware, server, App, Result, HttpResponse, http, http::Method, State, Form};

struct BeatbotState {
    token: String
}

#[derive(Deserialize)]
struct BeatbotParams {
    token: String,
}


fn beats((state, params): (State<BeatbotState>, Form<BeatbotParams>)) -> Result<HttpResponse> {
    println!("{:?}, {:?}, {:?}", state.token, params.token, state.token == params.token);
    if state.token == params.token {
        Ok(HttpResponse::build(http::StatusCode::OK)
            .content_type("application/json")
            .body(format!("{{ \"response_type\": \"in_channel\",\"text\": \"{}\" }}", Beat::now())))
    } else {
        Ok(HttpResponse::build(http::StatusCode::UNAUTHORIZED)
           .body("Wrong token"))
    }
}


fn main() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    server::new(|| {
        App::with_state(BeatbotState { token: env::var("BEATBOT_RS_TOKEN").unwrap() })
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(Method::POST).with(beats))
    }).bind("127.0.0.1:3005")
        .unwrap()
        .run()
}
