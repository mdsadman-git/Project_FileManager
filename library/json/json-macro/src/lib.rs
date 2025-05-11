use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(JsonParser)]
pub fn json_parser_macro(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse(item).unwrap();
  impl_json_parser_trait(ast)
}

fn impl_json_parser_trait(ast: DeriveInput) -> TokenStream {
  let ident = ast.ident;
  let fields = match ast.data {
    syn::Data::Struct(data) => data.fields,
    syn::Data::Enum(_) => panic!(""),
    syn::Data::Union(_) => panic!(""),
  };

  quote::quote! {
    impl JsonParser for #ident {
      pub fn parse(&mut self, json: String) {

      }
    }
  }.into()
}