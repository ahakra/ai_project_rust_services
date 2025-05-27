use hyper::body::Incoming;
use hyper_util::client::legacy::connect::HttpConnector as LegacyHttpConnector;
use hyper_util::client::legacy::Client as LegacyClient;
use crate::generated::service_registry::service_registry_client::ServiceRegistryClient;
use crate::generated::service_registry::ServiceRegisterRequest;

pub type BoxedGrpcService = tower::util::BoxCloneService<
    http::Request<tonic::body::Body>,
    http::Response<Incoming>,
    tonic::Status,
>;
pub type GrpcResult<T> = Result<T, Box<dyn std::error::Error>>;
pub struct ServiceRegistrar {
    pub client: ServiceRegistryClient<BoxedGrpcService>,
    pub service_id: String,
    pub category: String,
    pub subcategory: String,
    pub registration_data: ServiceRegisterRequest,
}
