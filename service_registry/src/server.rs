use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

mod generated;
mod model;
mod service;

use model::store::{Registry};

use generated::service_registry::service_registry_server::{ServiceRegistryServer};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();


    let add: SocketAddr = "[::1]:50051".parse()?;
    let registry = Registry {
        store: Arc::new(RwLock::new(HashMap::new())),
    };
    let service = ServiceRegistryServer::new(registry);
    log::info!("Starting server at {}", add);

    Server::builder().add_service(service).serve(add).await?;

    Ok(())
}
