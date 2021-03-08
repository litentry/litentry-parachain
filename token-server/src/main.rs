
use std::convert::Infallible;
use std::env;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::collections::HashMap;

const LOCAL_URL: ([u8; 4], u16) = ([127, 0, 0, 1], 4000);

fn read_token() -> String {
    let keys: Vec<&str> = vec![
        "etherscan",
        "infura",
        "blockchain",
    ];

    let mut token_map = HashMap::<&str, String>::new();
    for key in keys.into_iter() {
        let token = match env::var(key) {
            Ok(token) => token,
            Err(_) => "".to_string(),
        };
        token_map.insert(key, token);
    }

    serde_json::to_string(&token_map).unwrap()
}

async fn hello(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("{:?}, {:?}", req.uri().path(), req.method());
    let data = read_token();
    Ok(Response::new(Body::from(data)))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    read_token();
    pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let addr = LOCAL_URL.into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
