use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use crate::model::registry::{ServiceRegistrar, GrpcResult, BoxedGrpcService};
use hyper_util::rt::TokioExecutor;
use tonic::Code;
use crate::generated::service_registry::service_registry_client::ServiceRegistryClient;
use crate::generated::service_registry::ServiceUpdateRequest;

impl ServiceRegistrar {
    
    pub async fn register_with_retry(&mut self) -> Result<(), tonic::Status> {
        loop {
            let request = tonic::Request::new(self.registration_data.clone());

            match self.client.register_service(request).await {
                Ok(response) => {
                    println!("Service registered: {:?}", response.into_inner());
                    break;
                }
                Err(err) => {
                    eprintln!("Failed to register service: {err}, retrying in 5s...");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }

        Ok(())
    }

    pub async fn start_heartbeat(mut self: Arc<Self>, md:HashMap<String,String>) {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            let update = ServiceUpdateRequest {
                service_id: self.service_id.clone(),
                category: self.category.clone(),
                subcategory: self.subcategory.clone(),
                status: "healthy".into(),
                version: "1.0.0".into(),
                health_endpoint: "".into(),
                metadata: md.clone(),
            };

            let request = tonic::Request::new(update);

            match self.client.update_service(request).await {
                Ok(_) => println!("Service heartbeat sent."),
                Err(e) => {
                    if e.code() == tonic::Code::NotFound {
                        eprintln!("Service not found. Attempting re-registration...");

                        if let Err(reg_err) = &&self.register_with_retry().await {
                            eprintln!("Re-registration failed: {reg_err}");
                        } else {
                            println!("Re-registration succeeded.");
                        }
                    } else {
                        eprintln!("Failed to send heartbeat: {e}");
                    }
                }
            }
        }
    }

}
