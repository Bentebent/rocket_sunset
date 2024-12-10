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
                    let sunset_ts = DateTime::parse_from_rfc3339(&sunset_raw.value()).map_err(|_| {
                        syn::Error::new(sunset_raw.span(), "Sunset timestamp is not a valid ISO8601 timestamp")
                    })?;

                    if sunset_ts.timestamp() < timestamp {
                        return Err(syn::Error::new(
                            sunset_raw.span(),
                            "Sunset timestamp must not be earlier than deprecation timestamp",
                        ));
                    }

                    sunset = Some(sunset_ts.format("%a, %d %b %Y %H:%M:%S GMT").to_string());
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
    let args = parse_macro_input!(args as DeprecationMacroInput);

    let ItemFn { attrs, vis, sig, block } = parse_macro_input!(input as ItemFn);

    let fn_name = &sig.ident;
    let fn_args = &sig.inputs;

    let output = match &sig.output {
        ReturnType::Type(_, ty) => {
            quote! { -> ::rocket_sunset::DeprecatedResponder<#ty> }
        }
        ReturnType::Default => {
            quote! { -> ::rocket_sunset::DeprecatedResponder<()> }
        }
    };

    let timestamp = args.timestamp;
    let link = args.link;
    let sunset = args.sunset;

    let expanded = quote! {
        #(#attrs)*
        #vis fn #fn_name(#fn_args) #output {
            ::rocket_sunset::DeprecatedResponder::new(
                #block,
                #timestamp,
                #link,
                #sunset
            )
        }
    };

    expanded.into()
}
