pub trait JsonParser {
  fn parser(json: String) -> Self;
  fn parse(&mut self) -> &mut Self;
  fn get<T: 'static>(&mut self) -> &T;
  fn get_mut<T: 'static>(&mut self) -> &mut T;
}