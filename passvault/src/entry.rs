use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct ServiceInfo {
    pub service: String,
    pub username: String,
    pub password: String,
}
impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> Self {
        Self { service, username, password }
    }
} 