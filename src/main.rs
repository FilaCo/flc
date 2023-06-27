use axum::Router;
use std::env::var;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

// ENV
const FLC_HTTP_PORT_ENV: &str = "FLC_HTTP_PORT";
const FLC_HTTP_PORT_DEFAULT: &str = "8080";

const FLC_HTTP_ADDR_ENV: &str = "FLC_HTTP_ADDR";
const FLC_HTTP_ADDR_DEFAULT: &str = "127.0.0.1";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = ConfigBuilder::new();

    // let addr = config
    //     .get::<SocketAddr>()
    //     .unwrap_or(Arc::new(SocketAddr::from([127, 0, 0, 1], 8080)));
    //
    // let app = config.get::<Router>().expect("Router not found");
    //
    // axum::Server::bind(addr.as_ref())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}
