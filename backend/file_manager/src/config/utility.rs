use crate::config::constants::{HOST_DEFAULT_PORT, HOST_IP_ADDRESS};

pub fn construct_app_url() -> String {
    format!("{}:{}", HOST_IP_ADDRESS, HOST_DEFAULT_PORT)
}
