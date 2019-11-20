extern crate iron;
extern crate http;

use std::io::net::ip::Ipv4Addr;
use iron::{Iron, Server, Chain, Request, Response, Alloy, Status, Unwind, FromFn};
use http::status;

fn hello_world(_: &mut Request, res: &mut Response, _: &mut Alloy) -> Status {
    res.serve(status::Ok, "Hello, world!");
    Unwind
}

fn main() {
    let mut server: Server = Iron::new();
    server.chain.link(FromFn::new(hello_world));
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}