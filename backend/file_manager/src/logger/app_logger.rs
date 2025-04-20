use std::{error::Error, time::SystemTime};

use time::{format_description, OffsetDateTime};

use crate::config::utility::construct_logger_dtf;

pub struct Logger;

// TODO: ADD FEATURE CONFIG FILE LOADER
#[allow(dead_code)]
impl Logger {
  pub fn info(content: &str) {
    println!("{} {} '{}'", Logger::current_formatted_dt(), "INFO ", content);
  } 
  
  pub fn warn(content: &str) {
    println!("{} {} '{}'", Logger::current_formatted_dt(), "WARN ", content);
  } 

  pub fn debug(content: &str) {
    println!("{} {} '{}'", Logger::current_formatted_dt(), "DEBUG", content);
  } 

  pub fn error(content: &str, e: Option<Box<dyn Error>>) {
    println!("{} {} '{}'", Logger::current_formatted_dt(), "ERROR", content);
    if let Some(err) = e {
      panic!("{:?}", err);
    }
  }
}

impl Logger {
  fn current_formatted_dt() -> String {
    let dtf = construct_logger_dtf();
    let format = format_description::parse(dtf.as_str()).unwrap();
    let current_dt: OffsetDateTime = SystemTime::now().into();
    format!("{}", current_dt.format(&format).unwrap())
  }
}