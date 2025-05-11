pub(crate) struct Logic;

impl Logic {
  pub fn format(json: &str, tab_size: i32) -> String {
	  let mut formatted = String::new();
	  let mut temp = String::new();
	  let mut is_quoted = false;
	  let mut tab = 0;

	  let mut value_queue = vec![false];
	  let mut colon_queue = vec![];

	  let paint = |temp: &mut String, formatted: &mut String| {
		  if let Some(last_char) = temp.pop() {
			  match ("span", &temp) {
				  (tag, x) if x.starts_with('"') && x.ends_with('"') => {
					  formatted.push_str(format!(r#"<{tag} style="color: green">{x}</{tag}>"#).as_str());
				  },
				  (tag, x) if x.as_str() == "null" => {
					  formatted.push_str(format!(r#"<{tag} style="color: gray">{x}</{tag}>"#).as_str());
				  },
				  (tag, x) if ["true", "false"].contains(&x.as_str()) => {
					  formatted.push_str(format!(r#"<{tag} style="color: orange">{x}</{tag}>"#).as_str());
				  },
				  (tag, x) => {
					  formatted.push_str(format!(r#"<{tag} style="color: red">{x}</{tag}>"#).as_str());
				  }
			  }

			  formatted.push(last_char);
			  temp.clear();
		  }
	  };

	  for ch in json.chars() {
		  match ch {
			  '"' => is_quoted = !is_quoted,
			  ' ' | '\n' => if !is_quoted { continue },
			  _ => {}
		  }

		  match value_queue.last().unwrap() {
			  true => temp.push(ch),
			  false => formatted.push(ch),
		  } 

		  match ch {
			  '{' | '[' => {
				  tab += tab_size;
				  formatted.push_str(&temp);
				  temp.clear();
				  formatted.push('\n');
				  formatted.push_str(&" ".repeat(tab as usize));
				  if ch == '[' { 
					  value_queue.push(true); 
					  colon_queue.push(false);
				  } else { 
					  value_queue.push(false);
					  colon_queue.push(true);
				  }
			  },
			  '}' | ']' => {
				  tab -= tab_size;
				  paint(&mut temp, &mut formatted);

				  let last_char = formatted.pop().unwrap();
				  match formatted.chars().nth_back((tab + tab_size) as usize).unwrap() != '\n' { 
					  true => {
						  formatted.push('\n'); 
						  formatted.push_str(&" ".repeat(tab as usize));
					  }
					  false => for _ in tab_size..tab { formatted.pop(); }
				  }

				  formatted.push(last_char);
				  value_queue.pop();
				  colon_queue.pop();
			  },
			  ',' => {
				  paint(&mut temp, &mut formatted);
				  formatted.push('\n');
				  formatted.push_str(&" ".repeat(tab as usize));
				  if *colon_queue.last().unwrap() {
					  value_queue.insert(value_queue.len(), false);
				  }
			  }
			  ':' => {
				  formatted.push(' ');
				  value_queue.insert(value_queue.len(), true);
			  },
			  _ => {}
		  }
	  }

	  formatted
  }
}

#[cfg(test)]
mod tests {
  use crate::Logic;

	#[test]
	fn json_format_logic_test() {
		let query = String::from(
			r#"{"key2":123,"array_1":[{"key4":null,"key2":456,"key1":"string2","key3":false},{"key2":456,"key4":null,"key3":false,"key1":"string2"}],"object_1":{"key2":456,"key3":false,"key4":null,"key1":"string1"},"key1":"string0","key4":null,"array_2":[[1,2,3],["1","2","3"],[3.1415,100,"Hello",false,null]],"key3":true}"#
		);
		let result = String::from(
			r#"{
  "key2": <span style="color: red">123</span>,
  "array_1": [
    {
      "key4": <span style="color: gray">null</span>,
      "key2": <span style="color: red">456</span>,
      "key1": <span style="color: green">"string2"</span>,
      "key3": <span style="color: orange">false</span>
    },
    {
      "key2": <span style="color: red">456</span>,
      "key4": <span style="color: gray">null</span>,
      "key3": <span style="color: orange">false</span>,
      "key1": <span style="color: green">"string2"</span>
    }
  ]<span style="color: red"></span>,
  "object_1": {
    "key2": <span style="color: red">456</span>,
    "key3": <span style="color: orange">false</span>,
    "key4": <span style="color: gray">null</span>,
    "key1": <span style="color: green">"string1"</span>
  },
  "key1": <span style="color: green">"string0"</span>,
  "key4": <span style="color: gray">null</span>,
  "array_2": [
    [
      <span style="color: red">1</span>,
      <span style="color: red">2</span>,
      <span style="color: red">3</span>
    ]<span style="color: red"></span>,
    [
      <span style="color: green">"1"</span>,
      <span style="color: green">"2"</span>,
      <span style="color: green">"3"</span>
    ]<span style="color: red"></span>,
    [
      <span style="color: red">3.1415</span>,
      <span style="color: red">100</span>,
      <span style="color: green">"Hello"</span>,
      <span style="color: orange">false</span>,
      <span style="color: gray">null</span>
    ]<span style="color: red"></span>
  ]<span style="color: red"></span>,
  "key3": <span style="color: orange">true</span>
}"#
		);

		let formatted_json = Logic::format(&query, 2);
		assert_eq!(formatted_json, result)
	}
}