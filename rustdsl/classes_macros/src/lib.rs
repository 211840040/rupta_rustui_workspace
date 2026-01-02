use proc_macro::TokenStream;

mod expand;
mod syntax;

#[proc_macro]
pub fn classes(input: TokenStream) -> TokenStream {
    syn::parse(input)
        .map(expand::expand)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
#[doc(hidden)]
pub fn classes_quoted(input: TokenStream) -> TokenStream {
    let tokens = syn::parse(input)
        .map(expand::expand)
        .unwrap_or_else(syn::Error::into_compile_error);
    quote::quote! { ::quote::quote! { #tokens } }.into()
}

#[proc_macro]
pub fn class(input: TokenStream) -> TokenStream {
    syn::parse(input)
        .map(expand::expand_class)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
#[doc(hidden)]
pub fn class_quoted(input: TokenStream) -> TokenStream {
    let tokens = syn::parse(input)
        .map(expand::expand_class)
        .unwrap_or_else(syn::Error::into_compile_error);
    quote::quote! { ::quote::quote! { #tokens } }.into()
}
