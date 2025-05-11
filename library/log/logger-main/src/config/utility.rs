use crate::config::constants::{LOGGER_FORMAT_DATE, LOGGER_FORMAT_TIME};

pub fn logger_dt_format() -> String {
  format!("{} {}", LOGGER_FORMAT_DATE, LOGGER_FORMAT_TIME)
}