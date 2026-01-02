use crate::{
    expand::{FnEntry, FnEntryKind},
    syntax::{Class, Param},
};
use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::visit_mut::VisitMut;
pub const FN_HELPER_KEY: &str = "helper";
pub const FN_CTOR_HELPER_KEY: &str = "builder";

pub fn expand_helper_impls<'a>(
    ty: &'a Class,
    fn_entries: &'a [FnEntry],
) -> impl Iterator<Item = ExpandHelperImpl<'a>> {
    fn_entries
        .iter()
        .filter_map(|f| <ExpandHelperImpl>::new(ty, f))
}

pub fn expand_ctor_impls_in_data_mod<'a>(
    ty: &'a Class,
    fn_entries: &'a [FnEntry],
) -> impl Iterator<Item = ExpandHelperImpl<'a, true>> {
    fn_entries
        .iter()
        .filter_map(|f| <ExpandHelperImpl<true>>::new(ty, f))
}

// `CTOR_OF_DATA` is true if the helper is for a constructor in `data` mod
pub struct ExpandHelperImpl<'a, const CTOR_OF_DATA: bool = false> {
    class: &'a syn::Ident,
    entry: &'a FnEntry<'a>,
    helper_ty: syn::Ident,
    helper_new_fn: syn::Ident,
}

impl<'a, const CTOR_OF_DATA: bool> ExpandHelperImpl<'a, CTOR_OF_DATA> {
    fn new(ty: &'a Class, entry: &'a FnEntry<'a>) -> Option<Self> {
        // filter non-constructor APIs for class data
        if CTOR_OF_DATA && !entry.is_ctor() {
            return None;
        }
        // filter constructor APIs for abstract classes
        if !CTOR_OF_DATA && entry.is_ctor() && ty.kw_abstract.is_some() {
            return None;
        }
        if entry
            .params
            .iter()
            .all(|param| param.default_value.is_none())
        {
            None
        } else {
            let (helper_ty, helper_new_fn) = helper_name(ty, entry);
            Some(Self {
                class: &ty.name,
                entry,
                helper_ty,
                helper_new_fn,
            })
        }
    }

    fn helper(&self) -> TokenStream {
        let Self {
            class,
            helper_ty,
            entry,
            ..
        } = self;
        let mut lifetime_visitor = LifetimeReplacer::new();
        let (mut struct_fields, mod_fns): (Vec<_>, Vec<_>) = self
            .entry
            .params
            .iter()
            .map(|param| {
                let Param { name, ty, .. } = param;
                let mut ty = ty.clone();
                lifetime_visitor.try_replace_lifetime(&mut ty);

                let struct_field = quote! { #name: #ty };
                let modify_fn = quote! {
                    pub fn #name(self, #name: #ty) -> Self {
                        Self { #name, ..self }
                    }
                };
                (struct_field, modify_fn)
            })
            .unzip();
        let helper_fn_impl = self.helper_fn_impl();
        if entry.is_method() {
            struct_fields.insert(0, quote! { __this: #class<::classes::ptr::RcDyn<#class>> });
        } else if CTOR_OF_DATA {
            assert!(entry.is_ctor());
            struct_fields.insert(0, quote! { __this: ::classes::prelude::CRcUninit<#class> });
        }
        let lifetime_params = lifetime_visitor
            .generate_lifetime_params()
            .map_or(quote!(), |lp| quote! { #lp });

        quote! {
            #[must_use]
            pub struct #helper_ty #lifetime_params {
                #(#struct_fields),*
            }

            impl #lifetime_params #helper_ty #lifetime_params {
                #(#mod_fns)*
                #helper_fn_impl
            }
        }
    }

    fn helper_fn_impl(&self) -> TokenStream {
        let Self { class, entry, .. } = self;

        let fields = entry
            .params
            .iter()
            .map(|param| {
                let Param { name, .. } = param;
                quote! { #name }
            })
            .collect::<Vec<_>>();
        let fn_name = entry.name;
        let final_fn = if self.entry.is_ctor() {
            syn::Ident::new("build", fn_name.span())
        } else {
            syn::Ident::new("call", fn_name.span())
        };
        match entry.kind {
            FnEntryKind::Method { ret, .. } => {
                let ret = match ret {
                    Some(ty) => quote! { -> #ty },
                    None => quote! {},
                };
                quote! {
                    pub fn #final_fn(self) #ret {
                        let Self { __this, #(#fields),* } = self;
                        __this.#fn_name(#(#fields),*)
                    }
                }
            }
            FnEntryKind::Function { ret, .. } => {
                let ret = match ret {
                    Some(ty) => quote! { -> #ty },
                    None => quote! {},
                };
                quote! {
                    pub fn #final_fn(self) #ret {
                        let Self { #(#fields),* } = self;
                        #class::<::classes::ptr::RcDyn<#class>>::#fn_name(#(#fields),*)
                    }
                }
            }
            FnEntryKind::Ctor { .. } if CTOR_OF_DATA => {
                quote! {
                    pub fn #final_fn(self) -> ::classes::prelude::CRc<#class> {
                        let Self { __this, #(#fields),* } = self;
                        #class::#fn_name(__this, #(#fields),*)
                    }
                }
            }
            FnEntryKind::Ctor { .. } => {
                let build_superclass = quote::quote_spanned!(fn_name.span()=> build_superclass);
                let build_supertype = quote::quote_spanned!(fn_name.span()=> build_supertype);
                quote! {
                    pub fn #final_fn(self) -> #class<::classes::ptr::RcDyn<#class>> {
                        let Self { #(#fields),* } = self;
                        #class::<::classes::ptr::RcDyn<#class>>::#fn_name(#(#fields),*)
                    }
                    pub fn #build_superclass<A>(self) -> A
                    where
                        A: ::classes::class::ClassRc,
                        for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
                    {
                        self.build().into_superclass::<A>()
                    }
                    pub fn #build_supertype<A>(self) -> A
                    where
                        A: ::classes::class::ClassRc,
                        for<'a> &'a A: From<&'a ::classes::ptr::RcDyn<A::Class>>,
                    {
                        self.build().into_supertype::<A>()
                    }
                }
            }
        }
    }

    fn class_helper_impl(&self) -> TokenStream {
        let Self {
            class,
            helper_ty,
            helper_new_fn,
            entry,
        } = self;
        let mut lifetime_visitor = LifetimeReplacer::new();
        let required_params = self
            .entry
            .params
            .iter()
            .filter_map(|param| {
                let Param { name, ty, .. } = param;
                let mut ty = ty.clone();
                lifetime_visitor.try_replace_lifetime(&mut ty);
                param.default_value.is_none().then(|| quote! { #name: #ty })
            })
            .collect::<Vec<_>>();
        let init_fields = self.entry.params.iter().map(|param| {
            let Param {
                name,
                default_value,
                ..
            } = param;
            if let Some(default_value) = default_value.as_ref() {
                quote! { #name: #default_value }
            } else {
                quote! { #name }
            }
        });
        let lifetime_params = lifetime_visitor
            .generate_lifetime_params()
            .map_or(quote!(), |lp| quote! { #lp });

        if entry.is_method() {
            quote! {
                impl #class<classes::ptr::RcDyn<#class>> {
                    pub fn #helper_new_fn #lifetime_params (&self, #(#required_params),*) -> #helper_ty #lifetime_params {
                        #helper_ty {
                            __this: self.clone(),
                            #(#init_fields),*
                        }
                    }
                }
            }
        } else if CTOR_OF_DATA {
            assert!(entry.is_ctor());
            quote! {
                impl #class {
                    pub fn #helper_new_fn #lifetime_params (
                        __this: ::classes::prelude::CRcUninit<#class>,
                        #(#required_params),*
                    ) -> #helper_ty #lifetime_params {
                        #helper_ty {
                            __this,
                            #(#init_fields),*
                        }
                    }
                }
            }
        } else {
            quote! {
                impl #class<classes::ptr::RcDyn<#class>> {
                    pub fn #helper_new_fn #lifetime_params (#(#required_params),*) -> #helper_ty #lifetime_params {
                        #helper_ty {
                            #(#init_fields),*
                        }
                    }
                }
            }
        }
    }
}

impl<const CTOR_OF_DATA: bool> ToTokens for ExpandHelperImpl<'_, CTOR_OF_DATA> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let helper_impl = self.helper();
        let class_helper_impl = self.class_helper_impl();

        quote::quote_each_token! {
            tokens

            #helper_impl

            #class_helper_impl
        }
    }
}

fn helper_name(ty: &Class, entry: &FnEntry) -> (syn::Ident, syn::Ident) {
    let try_extract_name = |attr: &syn::Attribute, name: &str| -> Option<syn::Ident> {
        let named_value = attr.meta.require_name_value().ok()?;
        if named_value.path.is_ident(name) {
            let syn::Expr::Lit(syn::ExprLit { lit, .. }) = &named_value.value else {
                panic!("Expected literal expression for attribute `{}`", name)
            };
            let syn::Lit::Str(s) = lit else {
                panic!("Expected string literal for attribute `{}`", name)
            };
            Some(format_ident!("{}", s.value(), span = s.span()))
        } else {
            None
        }
    };
    let fn_suffix = if entry.is_ctor() {
        FN_CTOR_HELPER_KEY
    } else {
        FN_HELPER_KEY
    };
    let fn_name = entry
        .attrs
        .iter()
        .find_map(|attr| try_extract_name(attr, fn_suffix))
        .unwrap_or_else(|| {
            if entry.name == "new" {
                format_ident!("builder", span = entry.name.span())
            } else {
                format_ident!("{}_{fn_suffix}", entry.name)
            }
        });

    let helper_ty = format_ident!(
        "{}{}",
        ty.name,
        fn_name.to_string().to_case(Case::Pascal),
        span = fn_name.span()
    );
    (helper_ty, fn_name)
}

struct LifetimeReplacer {
    next: usize,
    lifetimes: Vec<syn::Lifetime>,
    generic_param_depth: u32,
}
impl LifetimeReplacer {
    fn new() -> Self {
        Self {
            next: 1,
            lifetimes: Vec::new(),
            generic_param_depth: 0,
        }
    }

    fn new_lifetime(&mut self) -> syn::Lifetime {
        let lifetime = syn::Lifetime::new(&format!("'__{}", self.next), Span::call_site());
        self.lifetimes.push(lifetime.clone());
        self.next += 1;
        lifetime
    }

    fn try_replace_lifetime(&mut self, ty: &mut syn::Type) {
        self.visit_type_mut(ty);
    }

    fn generate_lifetime_params(self) -> Option<syn::PathArguments> {
        if self.lifetimes.is_empty() {
            return None;
        }
        let lifetimes = self
            .lifetimes
            .into_iter()
            .map(syn::GenericArgument::Lifetime);
        let generic_args = syn::parse_quote! {
            <#(#lifetimes),*>
        };
        Some(syn::PathArguments::AngleBracketed(generic_args))
    }
}

impl VisitMut for LifetimeReplacer {
    fn visit_fn_arg_mut(&mut self, _: &mut syn::FnArg) {
        // Don't change function types
    }
    fn visit_item_fn_mut(&mut self, _: &mut syn::ItemFn) {
        // Don't change function types
    }
    fn visit_type_bare_fn_mut(&mut self, _: &mut syn::TypeBareFn) {
        // Don't change function types
    }
    fn visit_lifetime_mut(&mut self, i: &mut syn::Lifetime) {
        if i.ident == "_" {
            *i = self.new_lifetime();
        } else if i.ident != "static" {
            self.lifetimes.push(i.clone());
        }
    }
    fn visit_parenthesized_generic_arguments_mut(
        &mut self,
        i: &mut syn::ParenthesizedGenericArguments,
    ) {
        self.generic_param_depth += 1;
        syn::visit_mut::visit_parenthesized_generic_arguments_mut(self, i);
        self.generic_param_depth -= 1;
    }
    fn visit_type_reference_mut(&mut self, i: &mut syn::TypeReference) {
        if i.lifetime.is_none() && self.generic_param_depth == 0 {
            // Will be replaced in `visit_lifetime_mut`
            i.lifetime = Some(syn::Lifetime::new("'_", Span::call_site()));
        }
        syn::visit_mut::visit_type_reference_mut(self, i);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_replace_implicit_lifetime() {
        let mut visitor = LifetimeReplacer::new();
        macro_rules! test_case {
            ($visitor:expr, $item:ty, $expected:ty $(,)?) => {{
                let mut item: syn::Type = syn::parse_quote!($item);
                let expected: syn::Type = syn::parse_quote!($expected);
                $visitor.try_replace_lifetime(&mut item);
                assert_eq!(
                    item.to_token_stream().to_string(),
                    expected.to_token_stream().to_string(),
                );
            }};
        }
        test_case!(visitor, &i32, &'__1 i32);
        test_case!(visitor, Foo<'a, 'b>, Foo<'a, 'b>);
        test_case!(visitor, Foo<'_>, Foo<'__2>);
        test_case!(visitor, &Foo<'a, 'b>, &'__3 Foo<'a, 'b>);
        test_case!(visitor, fn(&i32) -> &i64, fn(&i32) -> &i64);
        test_case!(visitor, dyn Fn(&i32) -> &i64, dyn Fn(&i32) -> &i64);
        test_case!(
            visitor,
            dyn Fn(&dyn Fn(&i32), &dyn Fn(&i32)) -> &i64,
            dyn Fn(&dyn Fn(&i32), &dyn Fn(&i32)) -> &i64,
        );
        test_case!(visitor, &dyn Fn(&i32) -> &i64, &'__4 dyn Fn(&i32) -> &i64);
        test_case!(visitor, dyn Fn(&i32) + '_, dyn Fn(&i32) + '__5);
        test_case!(
            visitor,
            dyn Fn(&dyn Fn(&i32), &dyn Fn(&i32)) + '_,
            dyn Fn(&dyn Fn(&i32), &dyn Fn(&i32)) + '__6,
        );
    }
}
