use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use std::path::PathBuf;
use syn::{Error, Ident, LitStr, Token, parse::Parse, parse_macro_input};

struct ConstMatrix<T>(Matrix<T>);

impl<T> Parse for ConstMatrix<T>
where
    Matrix<T>: Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(Matrix::parse(input)?))
    }
}

struct MutMatrix<T>(Matrix<T>);

impl<T> Parse for MutMatrix<T>
where
    Matrix<T>: Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(Matrix::parse(input)?))
    }
}

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

impl ToTokens for ConstMatrix<String> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens_inner(tokens, false);
    }
}

impl ToTokens for MutMatrix<String> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens_inner(tokens, true);
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

impl ToTokens for ConstMatrix<PathBuf> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens_inner(tokens, false);
    }
}

impl ToTokens for MutMatrix<PathBuf> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens_inner(tokens, true);
    }
}

impl Matrix<String> {
    fn to_tokens_inner(&self, tokens: &mut proc_macro2::TokenStream, mutable: bool) {
        self.lines_to_matrix(
            self.value
                .lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            tokens,
            mutable,
        );
    }
}

impl Matrix<PathBuf> {
    fn to_tokens_inner(&self, tokens: &mut proc_macro2::TokenStream, mutable: bool) {
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
            mutable,
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
    fn lines_to_matrix(
        &self,
        lines: Vec<Vec<char>>,
        tokens: &mut proc_macro2::TokenStream,
        mutable: bool,
    ) {
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
        let decl = if mutable {
            quote!(let mut)
        } else {
            quote!(const)
        };

        quote! {
            #decl #name: [[char; #width]; #len] = [
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
    parse_macro_input!(input as ConstMatrix<String>)
        .into_token_stream()
        .into()
}

#[proc_macro]
/// Create a mutable matrix of characters from a string literal.
pub fn mut_matrix_str(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as MutMatrix<String>)
        .into_token_stream()
        .into()
}

#[proc_macro]
/// Create a constant matrix of characters from a file at the given path.
///
/// **The path is relative to the crate root.**
pub fn matrix_file(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as ConstMatrix<PathBuf>)
        .into_token_stream()
        .into()
}

#[proc_macro]
/// Create a mutable matrix of characters from a file at the given path.
///
/// **The path is relative to the crate root.**
pub fn mut_matrix_file(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as MutMatrix<PathBuf>)
        .into_token_stream()
        .into()
}
