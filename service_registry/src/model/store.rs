use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::generated::service_registry::ServiceInfoResponse;

pub type ServiceList = Vec<ServiceInfoResponse>;
pub type RegistryMap = HashMap<String, ServiceList>;

pub struct Registry {
    pub store: Arc<RwLock<RegistryMap>>,
}
