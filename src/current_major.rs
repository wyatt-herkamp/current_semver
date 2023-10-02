use proc_macro2::{TokenStream};
use quote::quote;
use syn::parse::Parse;
use syn::Type;
use crate::parse_int;

pub struct CurrentMajor{
    pub as_type: Type
}
impl Parse for CurrentMajor{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        println!("Parsing CurrentMajor with input: {:?}", input);
        return if input.peek(syn::Token![as]) {
            input.parse::<syn::Token![as]>()?;
            let as_type: Type = input.parse()?;
            Ok(Self { as_type })
        } else {
            // Default to usize
            Ok(Self { as_type: syn::parse_str("usize")? })
        }
    }
}
impl CurrentMajor {
    pub fn output(self) ->TokenStream{
        let Self { as_type } = self;
        let major = parse_int(std::env::var("CARGO_PKG_VERSION_MAJOR").ok());

        quote! {
            #major as #as_type
        }
    }
}