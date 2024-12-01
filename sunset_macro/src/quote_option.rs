use proc_macro2::TokenStream;
use quote::{
    quote,
    ToTokens,
    TokenStreamExt,
};

#[derive(Debug)]
pub struct QuoteOption<T>(Option<T>);

impl<T: std::fmt::Debug> From<Option<T>> for QuoteOption<T> {
    fn from(option: Option<T>) -> Self {
        QuoteOption(option)
    }
}

impl<T: ToTokens> ToTokens for QuoteOption<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.0 {
            Some(t) => tokens.append_all(quote! { ::std::option::Option::Some(#t) }),
            None => tokens.append_all(quote! { ::std::option::Option::None }),
        }
    }
}
