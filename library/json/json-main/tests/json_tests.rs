#[cfg(test)]
mod tests {
  use json_main::builder::{main::JsonBuilder, types::JsonNull, array::JsonArray};
  use json_main::Json;

  use logger_main::Logger;

  #[test]
  fn json_object_get_test() {
    let mut json_object = Json::object();
    json_object.insert("key1", "string2");
    json_object.insert("key2", 456);
    json_object.insert("key3", false);
    json_object.insert("key4", JsonNull::new());

    let json_value_key1_string: String = json_object.get("key1").unwrap().into();
    let json_value_key2_i32: i32 = json_object.get("key2").unwrap().into();
    let json_value_key3_bool: bool = json_object.get("key3").unwrap().into();
    let json_value_key4_null: Option<String> = json_object.get("key4").unwrap().into();

    Logger::console(format!("STRING: {}", json_value_key1_string));
    Logger::console(format!("NUMBER: {}", json_value_key2_i32));
    Logger::console(format!("BOOL:   {}", json_value_key3_bool));
    Logger::console(format!("NULL:   {:?}", json_value_key4_null));

    let json = Json::build(json_object);
    println!("--- Generated Json ---");
    println!("{}", json);
    println!("--- Generated Json ---");
  }

  #[test]
  fn json_dt_array_test() {
    let mut json_object_2 = Json::object();
    json_object_2.insert("key1", "string2");
    json_object_2.insert("key2", 456);
    json_object_2.insert("key3", false);
    json_object_2.insert("key4", JsonNull::new());

    let mut json_object_3 = Json::object();
    json_object_3.insert("key1", "string2");
    json_object_3.insert("key2", 456);
    json_object_3.insert("key3", false);
    json_object_3.insert("key4", JsonNull::new());

    let mut json_array = Json::array();
    json_array.append(json_object_2).append(json_object_3);

    let json = Json::build(json_array);
    println!("--- Generated Json ---");
    println!("{}", json);
    println!("--- Generated Json ---");
  }

  #[test]
  fn json_dt_object_test() {
    let mut object_1 = Json::object();
    object_1.insert("key1", "string1");
    object_1.insert("key2", 456);
    object_1.insert("key3", false);
    object_1.insert("key4", JsonNull::new());

    let mut json_object_2 = Json::object();
    json_object_2.insert("key1", "string2");
    json_object_2.insert("key2", 456);
    json_object_2.insert("key3", false);
    json_object_2.insert("key4", JsonNull::new());

    let mut json_object_3 = Json::object();
    json_object_3.insert("key1", "string2");
    json_object_3.insert("key2", 456);
    json_object_3.insert("key3", false);
    json_object_3.insert("key4", JsonNull::new());

    let mut array_1 = Json::array();
    array_1.append(json_object_2);
    array_1.append(json_object_3);

    let array_2_vec_1 = vec![1, 2, 3];
    let array_2_vec_2 = vec!["1", "2", "3"];
    let mut array_2_array_1 = JsonArray::new();
    array_2_array_1.append(3.1415);
    array_2_array_1.append(100);
    array_2_array_1.append("Hello");
    array_2_array_1.append(false);
    array_2_array_1.append(JsonNull::new());

    let mut array_2 = Json::array();
    array_2.append(array_2_vec_1);
    array_2.append(array_2_vec_2);
    array_2.append(array_2_array_1);

    let mut json_object = Json::object();
    json_object
      .insert("key1", "string0")
      .insert("key2", 123)
      .insert("key3", true)
      .insert("key4", JsonNull::new())
      .insert("object_1", object_1)
      .insert("array_1", array_1)
      .insert("array_2", array_2);

    let json = Json::build(json_object);
    println!("--- Generated Json ---");
    println!("{}", json);
    println!("--- Generated Json ---");
  }
}