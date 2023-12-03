// Import necessary libraries
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::fs;
use std::io::Read;
use std::path::Path;

// Define the main function
#[tokio::main]
async fn main() {
    // Create a new Http server
    let addr = ([127, 0, 0, 1], 3000).into();
    let make_svc = make_service_fn(|_conn| {
        async {
            Ok::<_, hyper::Error>(service_fn(handle_request))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

// Define the service function that will handle requests
async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Read the file from the given path
    let mut file = fs::File::open("./../eutiveumsonho/0.1.0").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Return the file content as the response
    Ok(Response::new(Body::from(contents)))
}