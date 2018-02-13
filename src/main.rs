extern crate hyper;
extern crate futures;
extern crate beats;
extern crate url;

use beats::Beat;
use futures::Stream;
use futures::future::Future;
use hyper::{Method, StatusCode};
use hyper::header::ContentType;
use hyper::server::{Http, Request, Response, Service};
use std::collections::HashMap;
use std::env;
use url::form_urlencoded;

struct Beatbot;

impl Service for Beatbot {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {

        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                Box::new(futures::future::ok(
                        Response::new().with_body("hello")
                ))
            },
            (&Method::Post, "/") => {
                Box::new(req.body().concat2().map(|b| {
                    let params = form_urlencoded::parse(b.as_ref())
                                                    .into_owned()
                                                    .collect::<HashMap<String, String>>();

                    let token = if let Some(t) = params.get("token") {
                        t.to_owned()
                    } else {
                        return Response::new().with_status(StatusCode::Unauthorized);
                    };

                    let my_token = env::var("BEATBOT_RS_TOKEN").unwrap();

                    if token == my_token {
                        // Handwritten JSON 'cause fuck it
                        let resp = format!("{{ \"response_type\": \"in_channel\",\"text\": \"{}\" }}", Beat::now());
                        Response::new().with_header(ContentType::json()).with_body(resp)
                    } else {
                        Response::new().with_status(StatusCode::Unauthorized)
                    }
                }))
            },
            _ => {
                Box::new(futures::future::ok(
                        Response::new().with_status(StatusCode::NotFound)

                ))
            },
        }
    }
}

fn main() {
    let addr = "127.0.0.1:3005".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Beatbot)).unwrap();
    server.run().unwrap();
}
