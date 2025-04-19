use std::time::SystemTime;

pub struct Logger;

// TODO: ADD FEATURE CONFIG FILE LOADER

impl Logger {
  pub fn info(content: &str) {
    println!("{} {} '{}'", Logger::current_formatted_dt(), "INFO ", content)
  } 

  pub fn debug(content: &str) {
    println!("{} {} '{}'", Logger::current_formatted_dt(), "DEBUG", content)
  } 

  pub fn error(content: &str) {
    println!("{} {} '{}'", Logger::current_formatted_dt(), "ERROR", content)
  }
}

impl Logger {
  fn current_formatted_dt() -> String {
    format!("{:?}", SystemTime::now())
  }
}