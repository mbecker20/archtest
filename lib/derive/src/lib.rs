use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Type};

#[proc_macro_derive(Resolver, attributes(resolver_target))]
pub fn derive_resolver(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let Data::Enum(e) = input.data {
        e.variants.into_iter().map(|v| v.ident).collect::<Vec<_>>()
    } else {
        panic!("expected request enum")
    };

    let attr = input
        .attrs
        .into_iter()
        .find(|attr| attr.path().is_ident("resolver_target"))
        .expect("did not find resolver_target attribute");

    let target: Type = attr
        .parse_args()
        .expect("should pass struct to implement resolve_request on, eg. AppState");

    quote! {
        impl #target {
            pub async fn resolve_request(&self, request: #name) -> anyhow::Result<String> {
                match request {
                    #(#name::#variants(req) => self.resolve_response(req).await),*
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(Response, attributes(response))]
pub fn has_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let req = input.ident;

    let attr = input
        .attrs
        .into_iter()
        .find(|attr| attr.path().is_ident("response"))
        .expect("did not find response attribute");

    let res: Type = attr.parse_args().expect("should pass response type");

    quote! {
        impl crate::api::HasResponse for  #req {
            type Response = #res;
            fn req_type() -> &'static str {
                stringify!(#res)
            }
        }
    }
    .into()
}
