use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};
use std::net::SocketAddr;

fn main() {
    //We'll use the SocketAddr struct, which contains both the IpAddr and the u16 for the port number.
    let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();
    // println!("{:?}", addr); -> V4(127.0.0.1:8080)
    let builder = Server::bind(&addr);
    let server = builder.serve(|| service_fn_ok(|_| Response::new(Body::from("Basic GET on /"))));
    let server = server.map_err(drop);
    println!("Server Running at {}:{}", addr.ip(), addr.port());
    hyper::rt::run(server)
}
