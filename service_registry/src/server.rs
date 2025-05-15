use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

mod generated;
use generated::service_registry::service_registry_server::{
    ServiceRegistry, ServiceRegistryServer,
};
use generated::service_registry::{
    ServiceDeleteRequest, ServiceDeleteResponse, ServiceInfoRequest, ServiceInfoResponse,
    ServiceRegisterRequest, ServiceUpdateRequest, ServicesByCategoryRequest,
    ServicesByCategoryResponse,
};

type ServiceList = Vec<ServiceInfoResponse>;
type RegistryMap = HashMap<String, ServiceList>;

pub struct Registry {
    store: Arc<RwLock<RegistryMap>>,
}

#[tonic::async_trait]
impl ServiceRegistry for Registry {
    async fn get_service_info(
        &self,
        request: Request<ServiceInfoRequest>,
    ) -> Result<Response<ServiceInfoResponse>, Status> {
        let req = request.into_inner();
        let store = self.store.read().await;

        let category = req.category;
        let subcategory = req.subcategory;
        let service_id = req.service_id;

        if let Some(services) = store.get(&category) {
            for service in services {
                if service.service_id == service_id
                    && (subcategory.is_empty() || service.subcategory == subcategory)
                {
                    return Ok(Response::new(service.clone())); // Return matching service
                }
            }
        }

        Err(Status::not_found("Service not found"))
    }

    async fn register_service(
        &self,
        request: Request<ServiceRegisterRequest>,
    ) -> Result<Response<ServiceInfoResponse>, Status> {
        let req = request.into_inner();

        let mut store = self.store.write().await;

        let service_info = ServiceInfoResponse {
            service_id: req.service_id.clone(),
            service_name: req.service_name,
            category: req.category.clone(),
            subcategory: req.subcategory,
            r#type: req.r#type,
            task: req.task,
            version: req.version,
            status: req.status,
            health_endpoint: req.health_endpoint,
            metadata: req.metadata,
        };

        // Check if the category already has services
        if let Some(services) = store.get_mut(&req.category) {
            // Remove any existing service with the same service_id
            services.retain(|service| service.service_id != req.service_id);

            // Push the new service
            services.push(service_info.clone());
        } else {
            // If no services exist for the category, create a new vector and add the service
            store.insert(req.category, vec![service_info.clone()]);
        }

        Ok(Response::new(service_info))
    }

    async fn update_service(
        &self,
        request: Request<ServiceUpdateRequest>,
    ) -> Result<Response<ServiceInfoResponse>, Status> {
        let req = request.into_inner();
        let service_id = req.service_id;
        let category = req.category;
        let subcategory = req.subcategory;

        let mut store = self.store.write().await;

        if let Some(services) = store.get_mut(&category) {
            for service in services.iter_mut() {
                if service.service_id == service_id
                    && (subcategory.is_empty() || service.subcategory == subcategory)
                {
                    service.status = req.status;
                    service.version = req.version;
                    service.health_endpoint = req.health_endpoint;
                    service.metadata = req.metadata;

                    return Ok(Response::new(service.clone()));
                }
            }
        }

        Err(Status::not_found("Service not found in category"))
    }

    async fn delete_service(
        &self,
        request: Request<ServiceDeleteRequest>,
    ) -> Result<Response<ServiceDeleteResponse>, Status> {
        let req = request.into_inner();
        let service_id = req.service_id;
        let category = req.category;
        let subcategory = req.subcategory; // Optional if you use subcategory filtering

        let mut store = self.store.write().await;

        // Check if the category exists in the store
        if let Some(services) = store.get_mut(&category) {
            // Find the service by its ID
            if let Some(pos) = services
                .iter()
                .position(|service| service.service_id == service_id)
            {
                // If a subcategory is provided, ensure that it matches
                if !subcategory.is_empty() && services[pos].subcategory != subcategory {
                    return Err(Status::not_found("Service subcategory does not match"));
                }

                // Remove the service from the list
                services.remove(pos);

                // If the category has no more services, remove the category
                if services.is_empty() {
                    store.remove(&category);
                }

                // Return a success response
                let response = ServiceDeleteResponse {
                    service_id: service_id.clone(),
                    message: "Service deleted successfully".to_string(),
                };
                return Ok(Response::new(response));
            }
        }

        // If the service is not found in the category
        Err(Status::not_found(
            "Service not found in the specified category",
        ))
    }

    async fn get_services_by_category(
        &self,
        request: Request<ServicesByCategoryRequest>,
    ) -> Result<Response<ServicesByCategoryResponse>, Status> {
        let req = request.into_inner();
        let store = self.store.read().await;

        let services = store
            .get(&req.category)
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|service| req.subcategory.is_empty() || service.subcategory == req.subcategory)
            .cloned()
            .collect::<Vec<_>>();

        Ok(Response::new(ServicesByCategoryResponse { services }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let add: SocketAddr = "[::1]:50051".parse()?;
    let registry = Registry {
        store: Arc::new(RwLock::new(HashMap::new())),
    };
    let service = ServiceRegistryServer::new(registry);
    println!("Starting server at {}", add);
    Server::builder().add_service(service).serve(add).await?;

    Ok(())
}
