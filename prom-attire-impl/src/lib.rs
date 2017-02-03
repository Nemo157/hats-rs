#![recursion_limit = "128"]

#[macro_use]
extern crate error_chain;
extern crate syn;
#[macro_use]
extern crate quote;

mod dissect;
mod expand;
mod errors;
mod tmp;

pub use errors::*;
use tmp::TryInto;

pub struct Config<'a> {
    pub scope: Option<&'a str>,
}

pub fn derive(input: &str, config: Config) -> Result<String> {
    let ast = syn::parse_derive_input(input)?;
    let strukt = (&ast).try_into()?;
    let expanded = expand::expand(&strukt, &config);
    Ok(expanded.to_string())
}
