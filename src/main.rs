#[macro_use]
extern crate prometheus;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

mod config;
mod tado;

use tokio;
use std::convert::Infallible;
use std::time::Duration;
use hyper::{service::make_service_fn, service::service_fn, Server};
use ticker::Ticker;

use config::loader as config_loader;
use tado::metrics::renderer;
use tado::client::Client as TadoClient;

#[tokio::main]
async fn main() {
    let config = config_loader::load();

    // Start ticker
    run_ticker(config);

    // set up http server
    let addr = ([0, 0, 0, 0], 9898).into();
    println!("starting tado° exporter on address: {:?}", addr);

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(renderer))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // start HTTP server
    if let Err(e) = server.await {
        eprintln!("a server error occured: {}", e);
    }
}

fn run_ticker(config: config_loader::Config) {
    tokio::spawn(async move {
        let mut tado_client = TadoClient::new(config.username, config.password, config.client_secret);

        let ticker = Ticker::new(0.., Duration::from_secs(config.ticker));
        for i in ticker {
            println!("{:?}", i);
            
            tado_client.retrieve().await;
        }
    });
}