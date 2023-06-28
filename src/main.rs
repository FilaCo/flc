use axum::Router;
use flc::{Config, FlcResult, FromConfig};
use std::env::var;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

// ENV
const FLC_HTTP_PORT_ENV: &str = "FLC_HTTP_PORT";
const FLC_HTTP_PORT_DEFAULT: &str = "8080";

const FLC_HTTP_ADDR_ENV: &str = "FLC_HTTP_ADDR";
const FLC_HTTP_ADDR_DEFAULT: &str = "127.0.0.1";

#[derive(Debug)]
struct FlcSocketAddr(SocketAddr);

impl FromConfig for FlcSocketAddr {
    fn from_config(_: &Config) -> Arc<Self> {
        Arc::new(FlcSocketAddr(
            SocketAddr::from_str(
                [
                    var(FLC_HTTP_ADDR_ENV).unwrap_or(FLC_HTTP_ADDR_DEFAULT.to_string()),
                    var(FLC_HTTP_PORT_ENV).unwrap_or(FLC_HTTP_PORT_DEFAULT.to_string()),
                ]
                .join(":")
                .as_str(),
            )
            .expect("unable to create FlcSocketAddr"),
        ))
    }
}

#[tokio::main]
async fn main() -> FlcResult<()> {
    tracing_subscriber::fmt::init();
    let config = Config::builder()
        .add_singleton::<FlcSocketAddr>(None)
        .build()?;

    println!("{:?}", config.get::<FlcSocketAddr>(None));
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

    Ok(())
}
