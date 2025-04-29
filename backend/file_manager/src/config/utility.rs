use crate::config::constants::{HOST_DEFAULT_PORT, HOST_IP_ADDRESS, LOGGER_FORMAT_DATE, LOGGER_FORMAT_TIME};

pub fn construct_app_url() -> String {
    format!("{}:{}", HOST_IP_ADDRESS, HOST_DEFAULT_PORT)
}

pub fn logger_dt_format() -> String {
  format!("{} {}", LOGGER_FORMAT_DATE, LOGGER_FORMAT_TIME)
}