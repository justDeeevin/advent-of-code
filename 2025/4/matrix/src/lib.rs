use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use std::path::PathBuf;
use syn::{Error, Ident, LitStr, Token, parse::Parse, parse_macro_input};

struct Matrix<T> {
    name: Ident,
    lit: LitStr,
    value: T,
}

impl Parse for Matrix<String> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Self::parse_inner(input, |lit| Ok(lit.value()))
    }
}

impl ToTokens for Matrix<String> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.lines_to_matrix(
            self.value
                .lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            tokens,
        );
    }
}

impl Parse for Matrix<PathBuf> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Self::parse_inner(input, |lit| {
            Ok(
                PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").map_err(|e| {
                    Error::new_spanned(lit, format!("couldn't get crate root path: {e}"))
                })?)
                .join(lit.value()),
            )
        })
    }
}

impl ToTokens for Matrix<PathBuf> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let contents = match std::fs::read_to_string(&self.value) {
            Ok(contents) => contents,
            Err(err) => {
                Error::new_spanned(&self.lit, format!("couldn't read file: {err}"))
                    .into_compile_error()
                    .to_tokens(tokens);
                return;
            }
        };
        self.lines_to_matrix(
            contents
                .lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            tokens,
        );
    }
}

impl<T> Matrix<T> {
    fn parse_inner(
        input: syn::parse::ParseStream,
        value_getter: impl Fn(&LitStr) -> syn::Result<T>,
    ) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let lit = input.parse::<LitStr>()?;
        Ok(Self {
            name,
            value: value_getter(&lit)?,
            lit,
        })
    }
    fn lines_to_matrix(&self, lines: Vec<Vec<char>>, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let width = lines.first().map(|l| l.len()).unwrap_or(0);
        for (i, line) in lines.iter().enumerate().skip(1) {
            let len = line.len();
            if len != width {
                Error::new_spanned(&self.lit, format!("all lines must be the same length; line {i} is length {len}, expected {width}"))
                        .into_compile_error().to_tokens(tokens);
                return;
            }
        }
        let len = lines.len();
        let width = lines.first().map(|l| l.len()).unwrap_or(0);

        quote! {
            const #name: [[char; #width]; #len] = [
                #(
                    [#(#lines,)*],
                )*
            ];
        }
        .to_tokens(tokens);
    }
}

#[proc_macro]
/// Create a constant matrix of characters from a string literal.
pub fn matrix_str(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as Matrix<String>)
        .into_token_stream()
        .into()
}

#[proc_macro]
/// Create a constant matrix of characters from a file at the given path.
///
/// **The path is relative to the crate root.**
pub fn matrix_file(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as Matrix<PathBuf>)
        .into_token_stream()
        .into()
}
