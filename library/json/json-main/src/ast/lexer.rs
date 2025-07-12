use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
  ObjectStart, 
  ObjectEnd, 
  ArrayStart, 
  ArrayEnd, 
  ContentSeparator, 
  ElementSeparator, 
  JsonKey, 
  JsonValue,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValueOf {
  Array, Object, None
}

impl Display for TokenType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("{:<20}", format!("{:?}", self)))
  }
}

#[derive(Debug)]
pub struct Token {
  pub tt: TokenType,
  pub vof: ValueOf,
  pub value: String,
  pub quoted: Option<bool>,
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("Token {{ {:?} {:?} {} }}", self.tt, self.quoted, self.value))
  }
}

impl Token {
  pub fn value(&self) -> String {
    self.value.clone()
  }
}

trait QuotedToken {
  fn new(tt: TokenType, vof: ValueOf, value: impl Into<String>, quoted: Option<bool>) -> Self;
}

trait BasicToken {
  fn new(tt: TokenType, vof: ValueOf, value: impl Into<String>) -> Self;
}

impl QuotedToken for Token {
  fn new(tt: TokenType, vof: ValueOf, value: impl Into<String>, quoted: Option<bool>) -> Self {
    Self { tt, vof, value: value.into(), quoted }
  }
}

impl BasicToken for Token {
  fn new(tt: TokenType, vof: ValueOf, value: impl Into<String>) -> Self {
    Self { tt, vof, value: value.into(), quoted: None }
  }
}

pub struct Lexer<'a> {
  iterator: Box<dyn Iterator<Item = char> + 'a>,
  tokens: Vec<Token>,
  is_quoted: bool,
  object_track: Vec<bool>,
  key_track: Vec<bool>,
}

impl<'a> Lexer<'a> {
  pub fn new(json: &'a str) -> Self {
    Self {
      iterator: Box::new(json.chars()), 
      tokens: Vec::new(),
      is_quoted: false, 
      object_track: Vec::new(),
      key_track: Vec::new(),
    }

  }
}

impl Lexer<'_> {
  pub fn tokenize(&mut self) -> &Vec<Token> {
    loop { if !self.next() { break; } }
    &self.tokens
  }

  fn next(&mut self) -> bool {
    if let Some(mut c) = self.iterator.next() {
      'label: loop {
        if c == ' ' || c == '\n' || c == '\t' {
          break 'label; 
        }

        if self.is_quoted || (c != '"' && self.key_track.len() > 0 && !*self.key_track.last().unwrap()) {
          self.add_content(&mut c);
        }

        if c == '"' {
          self.is_quoted = !self.is_quoted;
        } else if c == '{' && !self.is_quoted {
          self.object_track.push(true);
          self.key_track.push(true);
          self.tokens.push(BasicToken::new(TokenType::ObjectStart, ValueOf::None, c));
        } else if c == '}' && !self.is_quoted {
          self.object_track.pop();
          self.key_track.pop();
          self.tokens.push(BasicToken::new(TokenType::ObjectEnd, ValueOf::None, c));
        } else if c == '[' && !self.is_quoted {
          self.object_track.push(false);
          self.key_track.push(false);
          self.tokens.push(BasicToken::new(TokenType::ArrayStart, ValueOf::None, c));
        } else if c == ']' && !self.is_quoted {
          self.object_track.pop();
          self.key_track.pop();
          self.tokens.push(BasicToken::new(TokenType::ArrayEnd, ValueOf::None, c));
        } else if c == ':' && !self.is_quoted {
          self.key_track.pop();
          self.key_track.push(false);
          self.tokens.push(BasicToken::new(TokenType::ContentSeparator, ValueOf::None, c));
        } else if c == ',' && !self.is_quoted {
          self.key_track.pop();
          self.key_track.push(*self.object_track.last().unwrap());
          self.tokens.push(BasicToken::new(TokenType::ElementSeparator, ValueOf::None, c));
        } else {
          // TODO: BETTER TO ADD A PANIC HERE FOR UNKNOWN CHARACTER
        }

        break 'label;
      }

      return true;
    } 

    false
  }

  fn add_content(&mut self, c: &mut char) {
    let checkers = [',','}','{',']','[',' ','\n','\t'];
    if checkers.contains(c) { return; }

    let mut content = String::from(*c);
    let mut is_escape = false;
    let is_quoted = self.is_quoted;
    while let Some(q) = self.iterator.next() {
        if q == '"' && !is_escape {
          self.is_quoted = !self.is_quoted;
          break;
        } else if !self.is_quoted && !is_escape && checkers.contains(&q) {
          *c = q;
          break;
        } else if q == '\\' && !is_escape {
          is_escape = true;
        } else {
          is_escape = false;
        }

        content.push(q);
    }

    let vof: ValueOf = match self.object_track.last().unwrap() {
      true => ValueOf::Object, 
      false => ValueOf::Array, 
    };

    if *self.key_track.last().unwrap() {
      self.tokens.push(BasicToken::new(TokenType::JsonKey, vof, content));
    } else {
      self.tokens.push(QuotedToken::new(TokenType::JsonValue, vof, content, Some(is_quoted)));
    }
  }
}

// TESTS
#[cfg(test)]
mod tests {
  use logger_main::Logger;
  use super::Lexer;

  #[test]
  fn json_object_parser_test() {
    let json = String::from(r#"
      {
        "key4": null,
        "key3": true,
        "key1": "string0",
        "key2": 123
      }
    "#);

    let mut lexer = Lexer::new(json.as_str());
    let tokens = lexer.tokenize();
    Logger::info("Tokenized List");
    for e in tokens {
      println!("Token - {} {:<10} {:<15} '{}'", e.tt, format!("{:?}", e.vof), format!("{:?}", e.quoted), e.value);
    }
  }

  #[test]
  fn json_parser_test() {
    let json = String::from(r#"
      {
        "array_1": [
          {
            "key1": "string2",
            "key2": 456,
            "key3": false,
            "key4": null
          },
          {
            "key3": false,
            "key4": null,
            "key1": "string2",
            "key2": 456
          }
        ],
        "key4": null,
        "array_2": [
          [
            1,
            2,
            3
          ],
          [
            "1",
            "2",
            "3"
          ],
          [
            3.1415,
            100,
            "Hello",
            false,
            null
          ]
        ],
        "key3": true,
        "object_1": {
          "key2": 456,
          "key4": null,
          "key1": "string1",
          "key3": false
        },
        "key1": "string0",
        "key2": 123
      }
    "#);
    let mut lexer = Lexer::new(json.as_str());
    let v = lexer.tokenize();
    Logger::info("Tokenized List");
    for e in v {
      println!("Token - {} {:<10} {:<15} '{}'", e.tt, format!("{:?}", e.vof), format!("{:?}", e.quoted), e.value);
    }
  }
}