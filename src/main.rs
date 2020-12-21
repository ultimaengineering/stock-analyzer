extern crate iron;
extern crate time;

use iron::prelude::*;
use iron::{typemap, AfterMiddleware, BeforeMiddleware};
use time::{OffsetDateTime};
use std::{thread};

struct ResponseTime;

struct Health {
    status: String
}

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let t = OffsetDateTime::now_utc() - OffsetDateTime::unix_epoch();
        req.extensions.insert::<ResponseTime>(t.whole_milliseconds() as u64);
        Ok(())
    }
}
impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let t = OffsetDateTime::now_utc() - OffsetDateTime::unix_epoch();
        let delta = t.whole_milliseconds() as u64 - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64));
        Ok(res)
    }
}

fn health(_: &mut Request) -> IronResult<Response> {
    let health = Health {
        status: String::from("up")
    };
    Ok(Response::with((iron::status::Ok, "status: up")))
}

fn main() {
    let mut chain = Chain::new(health);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    Iron::new(chain).http("localhost:3000").unwrap();
}