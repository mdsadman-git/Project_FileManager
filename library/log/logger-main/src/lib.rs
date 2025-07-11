use std::{error::Error, time::SystemTime};

use time::{format_description, OffsetDateTime};

use crate::config::utility::logger_dt_format;

pub struct Logger;

mod config;

// TODO: ADD FEATURE CONFIG FILE LOADER

#[allow(dead_code)]
impl Logger {
  pub fn console(content: impl Into<String>) {
    println!("{}", content.into());
  } 

  pub fn info(content: impl Into<String>) {
    println!("{} {} '{}'", Logger::current_formatted_dt(), "INFO ", content.into());
  } 
  
  pub fn warn(content: impl Into<String>) {
    println!("{} {} '{}'", Logger::current_formatted_dt(), "WARN ", content.into());
  } 

  pub fn debug(content: impl Into<String>) {
    println!("{} {} '{}'", Logger::current_formatted_dt(), "DEBUG", content.into());
  } 

  pub fn error(content: impl Into<String>, e: Option<Box<dyn Error>>) {
    println!("{} {} '{}'", Logger::current_formatted_dt(), "ERROR", content.into());
    if let Some(err) = e {
      panic!("{:?}", err);
    }
  }
}

impl Logger {
  fn current_formatted_dt() -> String {
    let dtf = logger_dt_format();
    let format = format_description::parse(dtf.as_str()).unwrap();
    let current_dt: OffsetDateTime = SystemTime::now().into();
    format!("{}", current_dt.format(&format).unwrap())
  }
}