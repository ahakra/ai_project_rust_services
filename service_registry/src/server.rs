use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tonic_reflection::server::Builder;
use tonic_reflection::pb::v1alpha::FILE_DESCRIPTOR_SET;

mod generated;
mod model;
mod service;

use model::store::{Registry};

use generated::service_registry::service_registry_server::{ServiceRegistryServer,ServiceRegistry};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();


    let add: SocketAddr = "[::1]:50051".parse()?;
    let registry = Registry {
        store: Arc::new(RwLock::new(HashMap::new())),
    };
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;

    let service = ServiceRegistryServer::new(registry);
    log::info!("Starting server at {}", add);

    Server::builder()
        .add_service(service) // <- your gRPC service
        .add_service(reflection_service) // <- reflection comes last
        .serve(add)
        .await?;


    Ok(())
}
