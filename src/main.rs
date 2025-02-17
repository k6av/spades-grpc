use std::io::BufReader;
use tonic::transport::Server;
use serde_json;
use std::error::Error as AnyError;
use std::fs::File;
use proto::{servers_server::ServersServer,modes_server::ModesServer,maps_server::MapsServer};

mod servers; use servers::ServersSvc;
mod modes; use modes::ModesSvc;
mod maps; use maps::MapsSvc;

pub mod proto {
    tonic::include_proto!("spades_master");
}
use proto::*;

const NET_ADDR: &str = "[::1]:50051";

#[tokio::main]
async fn main() -> Result<(), Box<dyn AnyError>> {
    let addr = NET_ADDR.parse()?;

    let modes: Vec<Mode> = serde_json::from_reader(
        BufReader::new(File::open("modes.json").expect("failed to open modes.json"))
    ).expect("failed to parse modes.json");
    let maps: Vec<Map> = serde_json::from_reader(
        BufReader::new(File::open("maps.json").expect("failed to open maps.json"))
    ).expect("failed to parse maps.json");

    let modes = modes.iter().map(|x| (x.uid, x.clone())).collect();
    let maps = maps.iter().map(|x| (x.uid, x.clone())).collect();

    println!("Serving at {}", NET_ADDR);
    Server::builder()
        .add_service(ServersServer::new(ServersSvc::new()))
        .add_service(ModesServer::new(ModesSvc::new(modes)))
        .add_service(MapsServer::new(MapsSvc::new(maps)))
        .serve(addr)
        .await?;

    Ok(())
}

