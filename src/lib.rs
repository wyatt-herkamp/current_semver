mod current_major;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::parse::Parse;
use quote::quote;
use syn::{LitInt, LitStr};

/// Generates a int type value of the current major version
///
/// # Example
/// ```rust,no_run
/// use current_semver::current_major;
/// let major:usize = current_major!();
/// assert_eq!(major.to_string(), env!("CARGO_PKG_VERSION_MAJOR"));
///
///let major:u64 = current_major!(u64);
///assert_eq!(major.to_string(), env!("CARGO_PKG_VERSION_MAJOR"));
/// ```
///
#[proc_macro]
pub fn current_major(input: TokenStream) -> TokenStream {
    let current_major = syn::parse_macro_input!(input with current_major::CurrentMajor::parse);
    current_major.output().into()
}
fn parse_int(version: Option<String>) -> LitInt {
    version
        .and_then(|s| s.parse::<i64>().ok())
        .map(|i| LitInt::new(&i.to_string(), Span::call_site()))
        .unwrap_or(LitInt::new("0", Span::call_site()))
}
/// Creates the semver::Version Object without having to parse it at runtime
#[proc_macro]
pub fn current_semver(_input: TokenStream) -> TokenStream {
    let major = parse_int(std::env::var("CARGO_PKG_VERSION_MAJOR").ok());
    let minor = parse_int(std::env::var("CARGO_PKG_VERSION_MINOR").ok());
    let patch = parse_int(std::env::var("CARGO_PKG_VERSION_PATCH").ok());
    let pre = std::env::var("CARGO_PKG_VERSION_PRE")
        .ok()
        .map(|s| LitStr::new(&s, Span::call_site()))
        .map(|lit| {
            quote! {
                semver::Prerelease::new(#lit).unwrap_or_default()
            }
        })
        .unwrap_or(quote! {
            semver::Prerelease::default()
        });
    let output = quote! {
        semver::Version{
            major: #major,
            minor: #minor,
            patch: #patch,
            pre: #pre,
            build: semver::BuildMetadata::default(),
        }
    };

    output.into()
}
