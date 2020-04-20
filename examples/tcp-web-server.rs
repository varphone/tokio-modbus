#[cfg(feature = "tcp-server-unstable")]
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use futures::future;
    use hyper::{
        service::{make_service_fn, service_fn},
        Body, Server,
    };
    use std::{convert::Infallible, net::SocketAddr, thread};
    use tokio_modbus::{
        prelude::*,
        server::{self, Service},
    };

    async fn hello_world(_req: hyper::Request<Body>) -> Result<hyper::Response<Body>, Infallible> {
        Ok(hyper::Response::new("Hello, World".into()))
    }

    struct MbServer;

    impl Service for MbServer {
        type Request = Request;
        type Response = Response;
        type Error = std::io::Error;
        type Future = future::Ready<Result<Self::Response, Self::Error>>;

        fn call(&self, req: Self::Request) -> Self::Future {
            match req {
                Request::ReadCoils(_addr, cnt) => {
                    let mut coils = vec![false; cnt as usize];
                    coils[0] = true;
                    future::ready(Ok(Response::ReadCoils(coils)))
                }
                Request::ReadHoldingRegisters(_addr, cnt) => {
                    let mut registers = vec![0; cnt as usize];
                    registers[1] = 0x77;
                    future::ready(Ok(Response::ReadHoldingRegisters(registers)))
                }

                _ => unimplemented!(),
            }
        }
    }

    let mb_saddr = SocketAddr::from(([0, 0, 0, 0], 502));
    let web_saddr = SocketAddr::from(([0, 0, 0, 0], 8080));

    println!("Starting up server...");

    let _server = thread::spawn(move || {
        server::tcp::Server::new(mb_saddr).serve(|| Ok(MbServer));
    });

    let web_service =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello_world)) });

    let web_server = Server::bind(&web_saddr).serve(web_service);

    if let Err(e) = web_server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}

#[cfg(not(feature = "tcp-server-unstable"))]
pub fn main() {
    println!("`tcp-server-unstable` feature is required to run this example");
    std::process::exit(1);
}
