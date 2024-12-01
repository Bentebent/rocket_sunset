extern crate proc_macro;

use chrono::DateTime;
use quote::quote;
use syn::{
    parse::{
        Parse,
        ParseStream,
    },
    parse_macro_input,
    Ident,
    ItemFn,
    LitStr,
    Result,
    ReturnType,
    Token,
};

mod quote_option;
use quote_option::QuoteOption;

#[derive(Debug)]
struct DeprecationMacroInput {
    timestamp: String,
    link: QuoteOption<String>,
    sunset: QuoteOption<String>,
}

impl Parse for DeprecationMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let timestamp: LitStr = input
            .parse()
            .map_err(|e| syn::Error::new(input.span(), format!("Missing or invalid deprecation timestamp, {}", e)))?;
        let timestamp: i64 = DateTime::parse_from_rfc3339(&timestamp.value())
            .map_err(|_| {
                syn::Error::new(
                    timestamp.span(),
                    "Deprecation timestamp is not a valid ISO8601 timestamp",
                )
            })?
            .timestamp();

        let mut link = None;
        let mut sunset = None;

        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let ident: Ident = input.parse()?;

            match ident.to_string().as_str() {
                "link" => {
                    input.parse::<Token![=]>()?;
                    link = Some(input.parse::<LitStr>()?.value());
                }
                "sunset" => {
                    input.parse::<Token![=]>()?;
                    let sunset_raw = input.parse::<LitStr>()?;

                    let sunset_epoch: i64 = DateTime::parse_from_rfc3339(&sunset_raw.value())
                        .map_err(|_| {
                            syn::Error::new(sunset_raw.span(), "Sunset timestamp is not a valid ISO8601 timestamp")
                        })?
                        .timestamp();

                    if sunset_epoch < timestamp {
                        return Err(syn::Error::new(
                            sunset_raw.span(),
                            "Sunset timestamp must not be earlier than deprecation timestamp",
                        ));
                    }

                    sunset = Some(sunset_raw.value());
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        "Expected 'link' or 'sunset' identifiers.",
                    ))
                }
            }
        }

        Ok(DeprecationMacroInput {
            timestamp: timestamp.to_string(),
            link: link.into(),
            sunset: sunset.into(),
        })
    }
}

#[proc_macro_attribute]
pub fn deprecation(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    let ItemFn { attrs, vis, sig, block } = input_fn;

    let fn_name = &sig.ident;
    let fn_args = &sig.inputs;

    let input = parse_macro_input!(args as DeprecationMacroInput);

    let output = match &sig.output {
        ReturnType::Type(_, ty) => {
            quote! { -> DeprecatedResponder<#ty> }
        }
        ReturnType::Default => {
            quote! { -> DeprecatedResponder<()> }
        }
    };

    let timestamp = input.timestamp;
    let link = input.link;
    let sunset = input.sunset;

    let expanded = quote! {
        #(#attrs)*
        #vis fn #fn_name(#fn_args) #output {
            DeprecatedResponder::new(
                #block,
                #timestamp,
                #link,
                #sunset
            )
        }
    };

    proc_macro::TokenStream::from(expanded)
}
