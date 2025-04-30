use std::collections::HashMap;

use crate::enums::app_enums::HttpMethod;
use crate::logger::app_logger::Logger;
use crate::router::extra_routes::Extra;
use crate::router::get_routes::Get;
use crate::hashmap;

pub struct RouterHandler {
  pub map: HashMap<HttpMethod, HashMap<&'static str, fn() -> String>>,
}

impl RouterHandler {
  pub fn new() -> Self {
    RouterHandler { map: RouterHandler::route_map() }
  }
}

impl RouterHandler {
  fn route_map() -> HashMap<HttpMethod, HashMap<&'static str, fn() -> String>> {
    hashmap! {
      HttpMethod::GET => hashmap! { "/" => Get::home as fn() -> String }
    }
  }
}

impl RouterHandler {
  pub fn exec(&self, method: &HttpMethod, path: &str) -> &fn() -> String {
    Logger::debug(format!("Route to [METHOD: {} | PATH: {}]", method.as_string(), path));
    match method {
        HttpMethod::DELETE      => self.delete(path),
        HttpMethod::GET         => self.get(path),
        HttpMethod::POST        => self.post(path),
        HttpMethod::UPDATE      => self.update(path),
        _                       => self.method_not_allowed(),
    }
  }
}

trait HttpMethodTrait {
  fn get(&self, path: &str)     -> &fn() -> String;
  fn post(&self, path: &str)    -> &fn() -> String;
  fn update(&self, path: &str)  -> &fn() -> String;
  fn delete(&self, path: &str)  -> &fn() -> String;
}

impl HttpMethodTrait for RouterHandler {
  fn get(&self, path: &str) -> &fn() -> String {
    self.map.get(&HttpMethod::GET).unwrap().get(path).unwrap_or(self.not_found())
  }

  fn post(&self, path: &str) -> &fn() -> String {
    self.map.get(&HttpMethod::POST).unwrap().get(path).unwrap_or(self.not_found())
  }

  fn update(&self, path: &str) -> &fn() -> String {
    self.map.get(&HttpMethod::UPDATE).unwrap().get(path).unwrap_or(self.not_found())
  }

  fn delete(&self, path: &str) -> &fn() -> String {
    self.map.get(&HttpMethod::DELETE).unwrap().get(path).unwrap_or(self.not_found())
  }
}

trait ExtraHttpMethodTrait {
  fn not_found(&self) -> &fn() -> String;
  fn method_not_allowed(&self) -> &fn() -> String;
}

impl ExtraHttpMethodTrait for RouterHandler {
  fn method_not_allowed(&self) -> &fn() -> String {
    &(Extra::method_not_allowed as fn() -> String)
  }
  
  fn not_found(&self) -> &fn() -> String {
    &(Extra::not_found as fn() -> String)
  }
}