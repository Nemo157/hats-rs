extern crate prom_attire_impl;

#[macro_use]
extern crate quote;
extern crate syn;

use prom_attire_impl::{Config, ErrorKind};

macro_rules! assert_error_kind {
    ($err:expr, $kind:pat) => {{
        let err = $err;
        match err.kind() {
            &$kind => (),
            _ => {
                panic!(
                    "expected error of kind {}, got: {:?}",
                    stringify!($kind),
                    err)
            }
        }
    }}
}

#[test]
fn enuum() {
    let input = quote! { enum A {} };
    let config = Config {
        scope: None,
        docs: None,
    };
    let result = prom_attire_impl::derive(input.as_str(), config);
    assert_error_kind!(result.unwrap_err(), ErrorKind::StructBody)
}

#[test]
fn tuple_struct() {
    let input = quote! { struct A(); };
    let config = Config {
        scope: None,
        docs: None,
    };
    let result = prom_attire_impl::derive(input.as_str(), config);
    assert_error_kind!(result.unwrap_err(), ErrorKind::StructBody)
}

#[test]
fn bad_docs_type() {
    let input = quote! {
        struct A {
            docs: Vec<u64>,
        }
    };
    let config = Config {
        scope: None,
        docs: Some("docs"),
    };
    let result = prom_attire_impl::derive(input.as_str(), config);
    assert_error_kind!(result.unwrap_err(), ErrorKind::DocsTy(_))
}
