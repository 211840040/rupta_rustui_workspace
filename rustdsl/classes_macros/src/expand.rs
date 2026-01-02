mod function_helper;
use std::collections::HashSet;

use indexmap::IndexMap;
use proc_macro2::extra::DelimSpan;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote_spanned};
use quote::{quote, quote_each_token, quote_each_token_spanned};
use syn::Token;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

use crate::expand::function_helper::{
    FN_CTOR_HELPER_KEY, FN_HELPER_KEY, expand_ctor_impls_in_data_mod, expand_helper_impls,
};
use crate::syntax::*;

use SuperTy::*;

macro_rules! unreachable {
    () => {
        ::core::unreachable!("{}:{}", file!(), line!())
    };
    ($($arg:tt)+) => {
        ::core::unreachable!("{} {}:{}", format_args!($($arg)*), file!(), line!())
    };
}

#[derive(Debug, thiserror::Error)]
enum CheckError<'a> {
    #[error("`super: {super_field}` duplicated with `extends {extends}`")]
    DuplicatedSuper {
        extends: &'a str,
        super_field: &'a str,
    },
    #[error("`{field}` is duplicated")]
    DuplicatedField { field: &'a syn::Ident },
    #[error("`{field}` first defined here, consider remove it")]
    FieldDefinedHere { field: &'a syn::Ident },
    #[error("`extends {extends}` declared here, consider remove it")]
    ExtendsDeclared { extends: &'a str },
    #[error("`with` must be used with `extends`")]
    ExpectExtends,
    #[error("expect `&self`")]
    ExpectSelfReceiver,
    #[error("field cannot be both `{0}` and `{1}`")]
    InvalidFieldKind(FieldKind, &'a str),
    #[error("unexpected `override` with no `extends` nor `implements`")]
    InvalidOverrideNoSuperNorInterface,
    #[error(
        "ambigious `override` with no `extends` but multiple `implements`, consider use `override fn {0}::{1}"
    )]
    AmbiguiousOverrideNoSuperMultiInterface(&'a syn::Ident, &'a syn::Ident),
    #[error("missing super class in `override` in `mixin`")]
    MissingSuperclass,
    #[error("unexpected super class `Super` in `mixin`")]
    UnexpectedSuper,
    #[error("unexpected super class `{0}` in `mixin`")]
    UnexpectedSuperclass(&'a syn::Ident),
    #[error("expect `override`")]
    ExpectOverride,
    #[error("constructor in mixin is not supported now")]
    UnsupportedCtor,
    #[error("`{0} fn` is not supported yet")]
    UnsupportedKwFn(&'a str),
    #[error(
        "unsupported receiver type, supported receiver types are {}",
        ALL_SELF_KINDS
    )]
    UnsupportedSelfReceiver,
    #[error("no such field `{0}`")]
    NoSuchField(&'a syn::Ident),
    #[error("missing field `{0}` in initializer of `Self`")]
    MissingFieldExpr(&'a syn::Ident),
}

#[derive(Debug, Default)]
struct Config {}

impl Config {
    fn from_attrs(_attrs: &[syn::Attribute]) -> Self {
        Config::default()
    }
}

#[derive(Debug, Default)]
struct ClassConfig {
    no_super: bool,
}

impl ClassConfig {
    fn from_attrs(attrs: &[syn::Attribute]) -> Self {
        let mut config = ClassConfig::default();
        for attr in attrs {
            if attr.path().is_ident("no_super") {
                config.no_super = true;
            }
        }
        config
    }
}

pub fn expand(mut classes: Classes) -> TokenStream {
    let config = Config::from_attrs(&classes.inner_attrs);
    let classes_uses = classes
        .classes_or_extern_mixins()
        .map(|class_or_mixin| class_or_mixin.expand_use());
    let classes_uses = quote! { #( #classes_uses )* };
    let items = classes.items();
    let items = quote! { #( #items )* };
    let classes_or_extern_mixins = classes
        .classes_or_extern_mixins_mut()
        .map(|class_or_mixin_mut| class_or_mixin_mut.expand(&config));
    quote! {
        #classes_uses

        use ::classes::prelude::*;

        // #[cfg(debug_assertions)]
        const MODULE_PATH: &str = ::core::module_path!();

        #[allow(unused_macros)]
        mod _classes {
            use super::*;

            #( #classes_or_extern_mixins )*
        }
        #items
    }
}

pub fn expand_class(mut class: ClassWithAttrs) -> TokenStream {
    let config = Config::from_attrs(&class.attrs);
    class
        .class
        .expand_with_module_path(&config)
        .into_token_stream()
}

#[derive(Clone, Copy)]
enum ClassIdent<'a> {
    Class(&'a syn::Ident),
    Symbol,
}

impl<'a> From<&'a syn::Ident> for ClassIdent<'a> {
    fn from(ident: &'a syn::Ident) -> Self {
        ClassIdent::Class(ident)
    }
}

impl<'a> From<&'static str> for ClassIdent<'a> {
    fn from(s: &'static str) -> Self {
        assert_eq!(s, "$class");
        ClassIdent::Symbol
    }
}

impl ToTokens for ClassIdent<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        match self {
            ClassIdent::Class(ident) => ident.to_tokens(tokens),
            ClassIdent::Symbol => {
                quote_each_token!(tokens $class);
            }
        }
    }
}

enum ExpandClassOrMixinUses<'a> {
    Class(ExpandClassUses<'a>),
    Mixin(ExpandMixinUses<'a>),
}

struct ExpandClassUses<'a> {
    class: &'a syn::Ident,
    mixin_withs: Punctuated<MixinWith, Token![,]>,
    vis: &'a Visibility,
}

struct ExpandMixinUses<'a> {
    mixin: &'a syn::Ident,
    mixin_withs: Punctuated<MixinWith, Token![,]>,
}

impl ToTokens for ExpandClassOrMixinUses<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let (vis, class, mixin_withs) = match self {
            Self::Class(ExpandClassUses {
                class,
                vis,
                mixin_withs,
            }) => {
                quote_each_token! { tokens
                    #[allow(unused_imports)]
                    #vis use _classes::#class;
                }
                (vis.expand::<0>().to_token_stream(), class, mixin_withs)
            }
            Self::Mixin(ExpandMixinUses { mixin, mixin_withs }) => {
                (quote!(pub), mixin, mixin_withs)
            }
        };
        if !mixin_withs.is_empty() {
            let mixin_withs = mixin_withs.iter().map(|mixin_with| {
                let instance = mixin_with.to_instance(class);
                instance.class
            });
            quote_each_token! { tokens
                #[allow(unused_imports)]
                #vis use _classes::{#(#mixin_withs),*};
            }
        }
    }
}

struct MixinWith {
    idents: Punctuated<syn::Ident, Token![/]>,
}

impl MixinWith {
    fn to_instance<'a>(&'a self, mixin: &'a syn::Ident) -> MixinInstance<'a> {
        use std::fmt::Write;
        let mut ident = String::new();
        for pair in self.idents.pairs() {
            write!(&mut ident, "{}", pair.value()).unwrap();
            if pair.punct().is_some() {
                ident.push('_');
            }
        }
        let span = self.idents.span();
        let class = format_ident!("{ident}_{mixin}", span = span);
        MixinInstance {
            super_ty: MixinSuper {
                idents: &self.idents,
            },
            mixin,
            class,
        }
    }
    fn is_mixin_with(&self, mixin: &syn::Ident) -> bool {
        self.idents.iter().skip(1).any(|ident| ident == mixin)
    }
}

impl syn::parse::Parse for MixinWith {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let idents = input.call(Punctuated::parse_separated_nonempty)?;
        Ok(Self { idents })
    }
}

impl ToTokens for MixinWith {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.idents.to_tokens(tokens);
    }
}

struct ExpandClass<'a, const EXPAND_MODULE_PATH: bool> {
    config: &'a Config,
    class_config: ClassConfig,
    class: &'a Class,
    mixin_withs: Punctuated<MixinWith, Token![,]>,
    errors: Vec<syn::Error>,
}

impl<const EXPAND_MODULE_PATH: bool> ToTokens for ExpandClass<'_, EXPAND_MODULE_PATH> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        self.errors
            .iter()
            .for_each(|error| error.to_compile_error().to_tokens(tokens));

        let (super_attrs, super_ty) = self.class.super_ty(&self.class_config, tokens);
        let class_kind = self.class.class_kind();
        let mixin_instances = self
            .mixin_withs
            .iter()
            .map(|mixin_with| mixin_with.to_instance(&self.class.name))
            .collect::<Vec<_>>();
        let fields = self.class.field_entries(super_attrs, super_ty, tokens);
        let mixin_withs = self.class.kw_mixin.is_some().then_some(&self.mixin_withs);
        let ons = self.class.ons(mixin_withs);
        let fns = self
            .class
            .fn_entries(class_kind, super_ty, ons.as_deref(), mixin_withs, tokens);
        let consts = self
            .class
            .class_items
            .iter()
            .filter_map(|item| item.as_item_const())
            .collect::<Vec<_>>();
        let entries = self.class.class_entries(
            self.config,
            class_kind,
            ons.as_deref(),
            &mixin_instances,
            &fields,
            &fns,
            &consts,
        );

        if entries.kind.is_mixin() {
            entries
                .expand_mixin::<EXPAND_MODULE_PATH>()
                .to_tokens(tokens);
            return;
        }
        debug_assert!(!entries.kind.is_mixin_instance());
        debug_assert!(!entries.kind.is_mixin());
        let class = &self.class.name;

        let vis = &self.class.vis.expand::<1>();
        let mod_name = format_ident!("_{}", class);

        let module_path = EXPAND_MODULE_PATH.then(|| {
            quote! {
                // #[cfg(debug_assertions)]
                const MODULE_PATH: &str = ::core::module_path!();
            }
        });

        let def_class = quote_spanned!(class.span()=>
            ::classes::_def_class! { class #class }
        );
        let def_class_extends = match super_ty {
            Specific(ident) => quote_spanned! {ident.span() =>
                type Super = #ident;
                ::classes::_def_class_extends! { #class : #ident }
            },
            Mixined(ident, withs) => {
                let withs = withs.idents.pairs().map(|pair| pair.into_value());
                let span = ident.span();
                quote_spanned!(span=>
                    type Super<T = ::classes::class::ClassMarker, V = ::classes::class::Virtual> = ::classes::mixin!(<T, V> #ident, #(#withs,)*);
                    ::classes::_def_class_extends! { #class : Super (mixin_instance) }
                )
            }
            Object(span) => {
                let object = format_ident!("Object", span = span);
                let object_ty = quote_spanned!(span=> ::classes::object::Object);
                quote_spanned! (span=>
                    type Super = #object_ty;
                    ::classes::_def_class_extends! { #class : #object }
                )
            }
            MixinNoSuper(_) | NoSuper(_) => quote! {},
        };
        let item_struct = entries
            .fields
            .expand_item_struct(&self.class.name, entries.kind);
        let gets_sets = entries.fields.expand_gets_sets(entries.kind);

        let vtable_struct_fields = entries.vtable_struct_fields();
        let assert_layout_eq = vtable_struct_fields.expand_assert_layout_eq();
        let debug_vtable_layout = vtable_struct_fields.expand_debug_vtable_layout();

        let vtable_struct = entries.expand_vtable_struct::<false>();
        let vtable_struct_opt = entries.expand_vtable_struct::<true>();

        let impls = entries.impls().collect::<Vec<_>>();
        let super_ty = ExpandSuperTy {
            super_ty: entries.super_ty(),
            kind: entries.kind,
        };

        let new_class = if self.class.kw_abstract.is_some() {
            format_ident!("new_abstract_class")
        } else {
            format_ident!("new_concrete_class")
        };

        let delegate_ctor = entries.expand_delegate_ctor();
        let fn_impls = entries.expand_fn_impls();
        let builders = expand_helper_impls(self.class, entries.fns);
        let ctor_builders = expand_ctor_impls_in_data_mod(self.class, entries.fns);
        let vtable_impl = entries.expand_vtable_impl();

        let item_consts = entries.consts;
        let item_const_refs = item_consts
            .iter()
            .map(|item| item.expand_ref::<Token![self]>());
        let item_const_refs_in_data = item_consts
            .iter()
            .map(|item| item.expand_ref::<Token![super]>());

        let static_vtable = entries.expand_static_vtable();
        let pub_api = entries.expand_pub_apis::<true>();
        let pub_api_non_virtual = entries.expand_pub_apis::<false>();

        let num_impls = impls.len();
        quote_each_token! {
            tokens

            #module_path
            use ::classes::prelude::*;

            #[allow(unused_imports)]
            #vis use #mod_name::#class;

            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            #[allow(unused_imports)]
            #[allow(dead_code)]
            mod #mod_name {
                ::classes::_mod_uses! { mod class #class }
                #def_class
                #def_class_extends
                #( ::classes::_def_class_impl! { #class : #impls } )*

                mod data {
                    ::classes::_mod_uses! { mod data }

                    #item_struct

                    impl #class {
                        #delegate_ctor
                        #( #fn_impls )*
                        #( #item_const_refs_in_data )*
                    }

                    #( #ctor_builders )*
                }

                #( #builders )*

                mod vtable {
                    ::classes::_mod_uses! { mod vtable }

                    #vtable_struct
                    #debug_vtable_layout

                    pub(super) mod opt {
                        ::classes::_mod_uses! { mod vtable::opt }

                        #vtable_struct_opt

                        #vtable_impl
                    }

                    pub static TYPE: ::classes::vtable::TypeInfo<#num_impls> =
                        ::classes::vtable::TypeInfo::#new_class::<super::#class>(
                            #super_ty
                            [ #( (
                                ::classes::prelude::CVtable::<#impls>::TYPE,
                                ::core::mem::offset_of!(vtable::#class, #impls),
                            ), )* ],
                            // #[cfg(debug_assertions)]
                            MODULE_PATH,
                            // #[cfg(debug_assertions)]
                            stringify!(#class),
                        );
                }

                #assert_layout_eq

                #static_vtable

                #( #item_consts )*

                impl #class<::classes::ptr::RcDyn<#class>> {
                    #( #pub_api )*
                    #( #item_const_refs )*
                }
                impl #class<::classes::ptr::RcDyn<#class>, ::classes::class::NonVirtual> {
                    #( #pub_api_non_virtual )*
                }
                impl #class<::classes::ptr::RcDyn<#class>> {
                    #( #gets_sets )*
                }
            }
        }
    }
}

struct ExpandMixin<'a, const EXPAND_MODULE_PATH: bool> {
    class: &'a ClassEntries<'a>,
}

impl<const EXPAND_MODULE_PATH: bool> ToTokens for ExpandMixin<'_, EXPAND_MODULE_PATH> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        debug_assert!(self.class.kind.is_mixin());
        let class = &self.class.class;

        let vis = &self.class.vis.expand::<1>();
        let mod_name = format_ident!("_{}", class);

        let module_path = EXPAND_MODULE_PATH.then(|| {
            quote! { const MODULE_PATH: &str = ::core::module_path!(); }
        });

        let def_class = quote_spanned!(class.span()=>
            ::classes::_def_class! { mixin #class }
        );

        let item_struct = self
            .class
            .fields
            .expand_item_struct(self.class.class, self.class.kind);
        let gets_sets = self.class.fields.expand_gets_sets(self.class.kind);

        let vtable_struct_fields = self.class.vtable_struct_fields();
        let assert_layout_eq = vtable_struct_fields.expand_assert_layout_eq();
        let debug_vtable_layout = vtable_struct_fields.expand_debug_vtable_layout();

        let vtable_struct = self.class.expand_vtable_struct::<false>();
        let vtable_struct_opt = self.class.expand_vtable_struct::<true>();

        let ons = self.class.ons().collect::<Vec<_>>();
        let impls = self.class.impls().collect::<Vec<_>>();

        let fn_impls = self.class.expand_fn_impls();
        let pub_api = self.class.expand_pub_apis::<true>();

        let item_consts = self.class.consts;
        let item_const_refs = item_consts
            .iter()
            .map(|item| item.expand_ref::<Token![self]>());
        let item_const_refs_in_data = item_consts
            .iter()
            .map(|item| item.expand_ref::<Token![super]>());

        let num_impls = ons.len() + impls.len();

        let mixin_macro = self.class.mixin_macro();
        let instances = self.class.mixin_instances.iter().map(|instance| {
            let MixinInstance {
                mixin,
                class,
                super_ty,
            } = instance;
            let _mod = format_ident!("_{}", class);
            quote_spanned!(class.span()=> #mixin! { #_mod, #class, #super_ty })
        });
        let instance_idents = self
            .class
            .mixin_instances
            .iter()
            .map(|instance| &instance.class);

        let on_ons = ons.iter().map(|on| {
            if let MixinOn::Mixin(on) = on {
                quote! { #[mixin] #on }
            } else {
                quote! { #[class] #on }
            }
        });
        let on_ons = (!ons.is_empty())
            .then(|| quote! { on #(#on_ons),* })
            .unwrap_or_default();
        let impl_impls = (!impls.is_empty())
            .then(|| quote! { implements #(#impls),* })
            .unwrap_or_default();

        quote_each_token! {
            tokens

            #module_path
            use ::classes::prelude::*;

            #[allow(unused_imports)]
            #vis use #mod_name::#class;

            #[allow(unused_imports)]
            #vis use #mod_name::{#(#instance_idents),*};

            #[allow(non_snake_case)]
            #[allow(unused_variables)]
            #[allow(unused_imports)]
            #[allow(dead_code)]
            mod #mod_name {
                ::classes::_mod_uses! { mod class #class }
                #def_class
                ::classes::_def_mixin! { #class #on_ons #impl_impls }

                mod data {
                    ::classes::_mod_uses! { mod data }

                    #item_struct

                    impl #class {
                        #( #fn_impls )*
                        #( #item_const_refs_in_data )*
                    }
                }
                mod vtable {
                    ::classes::_mod_uses! { mod vtable }

                    #vtable_struct
                    #debug_vtable_layout

                    pub(super) mod opt {
                        ::classes::_mod_uses! { mod vtable::opt }

                        #vtable_struct_opt
                    }

                    pub static TYPE: ::classes::vtable::TypeInfo<#num_impls> =
                        ::classes::vtable::TypeInfo::new_mixin::<super::#class>(
                            [
                                #( ::classes::prelude::CVtable::<#ons>::TYPE,)*
                                #( ::classes::prelude::CVtable::<#impls>::TYPE,)*
                            ],
                            // #[cfg(debug_assertions)]
                            MODULE_PATH,
                            // #[cfg(debug_assertions)]
                            stringify!(#class),
                        );
                }

                #assert_layout_eq

                #( #item_consts )*

                impl #class<::classes::ptr::RcDyn<#class>> {
                    #( #pub_api )*
                    #( #item_const_refs )*
                }
                impl #class<::classes::ptr::RcDyn<#class>> {
                    #( #gets_sets )*
                }

                #mixin_macro
                #( #instances )*
            }
        }
    }
}

impl Class {
    fn super_ty(
        &self,
        config: &ClassConfig,
        mut tokens: &mut TokenStream,
    ) -> (&[syn::Attribute], SuperTy<'_>) {
        let super_field = self.item_struct.as_ref().and_then(ItemStruct::super_field);
        match (&self.extends, super_field, &self.withs) {
            (None, None, Some(withs)) => {
                let error = syn::Error::new(withs.kw_with.span, CheckError::ExpectExtends);
                error.into_compile_error().to_tokens(tokens);
                (&[], Object(self.name.span()))
            }
            (None, None, None) if self.kw_mixin.is_some() => (&[], MixinNoSuper(self.name.span())),
            (None, None, None) if config.no_super => (&[], NoSuper(self.name.span())),
            (None, None, None) => (&[], Object(self.name.span())),
            (None, Some(super_field), _) => {
                let span = super_field.span();
                let class = &self.name;
                let super_ident = &super_field.ident;
                let warning = format!(
                    "{super_ident}, consider declare as `{class} extends {super_ident}` instead"
                );
                quote_each_token_spanned! {tokens span
                    const _: () = {
                        struct #class { #[deprecated = #warning] _super: () }
                        #class { _super: () };
                    };
                };
                (&super_field.attrs, Specific(&super_field.ident))
            }
            (Some(extends), None, None) => (&[], Specific(&extends.ident)),
            (Some(extends), None, Some(withs)) => (&[], Mixined(&extends.ident, withs)),
            (Some(extends_decl), Some(super_field_decl), _) => {
                let extends = &extends_decl.ident.to_token_stream().to_string();
                let super_field = &super_field_decl.ident.to_token_stream().to_string();
                let mut error = syn::Error::new_spanned(
                    extends_decl,
                    CheckError::DuplicatedSuper {
                        extends,
                        super_field,
                    },
                );
                error.combine(syn::Error::new_spanned(
                    super_field_decl,
                    CheckError::ExtendsDeclared { extends },
                ));
                error.into_compile_error().to_tokens(tokens);
                (&super_field_decl.attrs, Specific(&super_field_decl.ident))
            }
        }
    }
    fn field_entries<'a>(
        &'a self,
        super_attrs: &'a [syn::Attribute],
        super_ty: SuperTy<'a>,
        tokens: &mut TokenStream,
    ) -> FieldEntries<'a> {
        let (attrs, fields) =
            self.item_struct
                .as_ref()
                .map_or_else(Default::default, |item_struct| {
                    let mut fields = IndexMap::new();
                    for pair in item_struct.fields.pairs() {
                        let field = pair.into_value();
                        if let Some(old_field) = fields.insert(&field.name, field.entry(tokens)) {
                            let mut error = syn::Error::new(
                                field.name.span(),
                                CheckError::DuplicatedField { field: &field.name },
                            );
                            error.combine(syn::Error::new(
                                old_field.name.span(),
                                CheckError::FieldDefinedHere {
                                    field: &old_field.name,
                                },
                            ));
                            error.into_compile_error().to_tokens(tokens);
                            continue;
                        }
                    }
                    (&item_struct.attrs[..], fields)
                });
        FieldEntries {
            kw_struct: self
                .item_struct
                .as_ref()
                .map(|item_struct| item_struct.kw_struct),
            brace: self
                .item_struct
                .as_ref()
                .map_or(self.brace, |item_struct| item_struct.brace),
            attrs,
            super_attrs,
            super_ty,
            fields,
        }
    }
    fn ons(
        &self,
        mixin_withs: Option<&Punctuated<MixinWith, Token![,]>>,
    ) -> Option<Vec<MixinOn<'_>>> {
        let ons = self.ons.as_ref()?;
        let mixin_withs = mixin_withs?;
        Some(
            ons.idents
                .iter()
                .map(|on| {
                    if mixin_withs
                        .iter()
                        .any(|mixin_with| mixin_with.is_mixin_with(on))
                    {
                        MixinOn::Mixin(on)
                    } else {
                        MixinOn::Class(on)
                    }
                })
                .collect(),
        )
    }
    fn fn_entries<'a>(
        &'a self,
        kind: ClassKind,
        super_ty: SuperTy<'a>,
        ons: Option<&[MixinOn<'a>]>,
        mixin_withs: Option<&Punctuated<MixinWith, Token![,]>>,
        tokens: &mut TokenStream,
    ) -> Vec<FnEntry<'a>> {
        self.class_items
            .iter()
            .filter_map(ClassItem::as_item_fn)
            .filter_map(|item_fn| {
                item_fn.fn_entry(
                    kind,
                    &self.name,
                    super_ty,
                    ons,
                    self.extends.as_ref(),
                    self.withs.as_ref(),
                    self.implements.as_ref(),
                    mixin_withs,
                    tokens,
                )
            })
            .collect()
    }
    fn class_kind(&self) -> ClassKind {
        match (self.kw_abstract, self.kw_mixin, self.kw_class) {
            (_, None, None) => unreachable!(),
            (_, Some(_), None) => ClassKind::AbstractMixin,
            (Some(_), None, Some(_)) => ClassKind::AbstractClass,
            (Some(_), Some(_), Some(_)) => ClassKind::AbstractMixinClass,
            (None, Some(_), Some(_)) => ClassKind::ConcreteMixinClass,
            (None, None, Some(_)) => ClassKind::ConcreteClass,
        }
    }
    fn class_entries<'a>(
        &'a self,
        config: &'a Config,
        kind: ClassKind,
        ons: Option<&'a [MixinOn<'a>]>,
        mixin_instances: &'a [MixinInstance<'a>],
        fields: &'a FieldEntries<'a>,
        fns: &'a [FnEntry<'a>],
        consts: &'a [&'a syn::ItemConst],
    ) -> ClassEntries<'a> {
        ClassEntries {
            config,
            vis: &self.vis,
            class: &self.name,
            kind,
            mixin_instances,
            fields,
            fns,
            ons,
            impls: self.implements.as_ref().map(|impls| &impls.idents),
            consts,
        }
    }
    fn parse_mixin_withs(
        kw_mixin: Option<kw::mixin>,
        attrs: &[syn::Attribute],
    ) -> Punctuated<MixinWith, Token![,]> {
        let mut mixin_withs = Punctuated::new();
        if kw_mixin.is_none() {
            return mixin_withs;
        }
        for attr in attrs {
            if !attr.path().is_ident("with") {
                continue;
            }
            if let Ok(parsed) = attr.parse_args_with(Punctuated::parse_terminated) {
                mixin_withs.extend(parsed.into_pairs());
            }
        }
        mixin_withs
    }
    fn parse_mixin_withs_mut(
        kw_mixin: Option<kw::mixin>,
        attrs: &mut Vec<syn::Attribute>,
    ) -> (Punctuated<MixinWith, Token![,]>, Vec<syn::Error>) {
        let mut mixin_withs = Punctuated::new();
        let mut errors = Vec::new();
        if kw_mixin.is_none() {
            return (mixin_withs, errors);
        }
        attrs.retain(|attr| {
            if !attr.path().is_ident("with") {
                return true;
            }
            match attr.parse_args_with(Punctuated::parse_terminated) {
                Ok(parsed) => mixin_withs.extend(parsed.into_pairs()),
                Err(err) => errors.push(err),
            }
            false
        });
        (mixin_withs, errors)
    }
    fn expand<'a>(&'a mut self, config: &'a Config) -> ExpandClass<'a, false> {
        let (mixin_withs, errors) = Self::parse_mixin_withs_mut(self.kw_mixin, &mut self.attrs);
        let class_config = ClassConfig::from_attrs(&mut self.attrs);
        ExpandClass {
            class: self,
            config,
            class_config,
            mixin_withs,
            errors,
        }
    }
    fn expand_with_module_path<'a>(&'a mut self, config: &'a Config) -> ExpandClass<'a, true> {
        let (mixin_withs, errors) = Self::parse_mixin_withs_mut(self.kw_mixin, &mut self.attrs);
        let class_config = ClassConfig::from_attrs(&mut self.attrs);
        ExpandClass {
            class: self,
            config,
            class_config,
            mixin_withs,
            errors,
        }
    }
}

impl<'a> ClassOrExternMixin<'a> {
    fn expand_use(self) -> ExpandClassOrMixinUses<'a> {
        match self {
            ClassOrExternMixin::Class(class) => {
                let mixin_withs = Class::parse_mixin_withs(class.kw_mixin, &class.attrs);
                ExpandClassOrMixinUses::Class(ExpandClassUses {
                    class: &class.name,
                    vis: &class.vis,
                    mixin_withs,
                })
            }
            ClassOrExternMixin::ExternMixin(mixin) => {
                let mixin_withs = Class::parse_mixin_withs(Some(mixin.kw_mixin), &mixin.attrs);
                ExpandClassOrMixinUses::Mixin(ExpandMixinUses {
                    mixin: &mixin.name,
                    mixin_withs,
                })
            }
        }
    }
}

impl<'a> ClassOrExternMixinMut<'a> {
    fn expand(self, config: &'a Config) -> ExpandClassOrExternMixin<'a> {
        match self {
            ClassOrExternMixinMut::Class(class) => {
                ExpandClassOrExternMixin::Class(class.expand(config))
            }
            ClassOrExternMixinMut::ExternMixin(mixin) => {
                ExpandClassOrExternMixin::ExternMixin(mixin.expand(config))
            }
        }
    }
}

enum ExpandClassOrExternMixin<'a> {
    Class(ExpandClass<'a, false>),
    ExternMixin(ExpandExternMixin<'a>),
}

impl ToTokens for ExpandClassOrExternMixin<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ExpandClassOrExternMixin::Class(class) => class.to_tokens(tokens),
            ExpandClassOrExternMixin::ExternMixin(mixin) => mixin.to_tokens(tokens),
        }
    }
}

struct ExpandExternMixin<'a> {
    mixin: &'a ExternMixin,
    #[allow(dead_code)]
    config: &'a Config,
    mixin_withs: Punctuated<MixinWith, Token![,]>,
    errors: Vec<syn::Error>,
}

impl ExternMixin {
    fn expand<'a>(&'a mut self, config: &'a Config) -> ExpandExternMixin<'a> {
        let (mixin_withs, errors) =
            Class::parse_mixin_withs_mut(Some(self.kw_mixin), &mut self.attrs);
        ExpandExternMixin {
            mixin: self,
            config,
            mixin_withs,
            errors,
        }
    }
}

impl ToTokens for ExpandExternMixin<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        self.errors
            .iter()
            .for_each(|error| error.to_compile_error().to_tokens(tokens));

        let uses = self
            .mixin_withs
            .iter()
            .flat_map(|mixin_with| mixin_with.idents.pairs().map(|pair| pair.into_value()))
            .map(|ident| quote_spanned!(ident.span()=> #ident as _));
        let ExternMixin {
            krate, sep, name, ..
        } = &self.mixin;
        let instances = self.mixin_withs.iter().map(|mixin_with| {
            let instance = mixin_with.to_instance(name);
            let MixinInstance {
                class,
                super_ty,
                mixin,
            } = instance;
            let _mod = format_ident!("_{}", class);
            quote_spanned!(class.span()=> #krate #sep #mixin! { #_mod, #class, #super_ty })
        });
        quote_each_token! { tokens
            #[allow(unused_imports)]
            use { #(#uses,)* };
            #( #instances )*
        };
    }
}

struct ExpandSuperTy<'a> {
    super_ty: SuperTy<'a>,
    kind: ClassKind,
}

impl ToTokens for ExpandSuperTy<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let span = self.super_ty.span();
        if self.super_ty.has_super(self.kind) {
            quote_each_token_spanned! {tokens span
                ::core::option::Option::Some(Super::TYPE),
            };
        } else {
            quote_each_token_spanned! {tokens span
                ::core::option::Option::None,
            };
        }
    }
}

struct ExpandStaticVtable<'a> {
    kind: ClassKind,
    class: &'a syn::Ident,
}

impl ToTokens for ExpandStaticVtable<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let class = self.class;
        let span = self.class.span();
        if self.kind.is_concrete() {
            quote_each_token_spanned!(tokens span ::classes::_def_concrete_class! { #class });
        }
    }
}

impl ToTokens for Extends {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.kw_extends.to_tokens(tokens);
        self.ident.to_tokens(tokens);
    }
}

impl ToTokens for SuperField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.kw_super.to_tokens(tokens);
        self.colon.to_tokens(tokens);
        self.ident.to_tokens(tokens);
    }
}

struct ExpandItemStruct<'a> {
    class: ClassIdent<'a>,
    entries: &'a FieldEntries<'a>,
    kind: ClassKind,
}

impl ToTokens for ExpandItemStruct<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let FieldEntries {
            super_attrs, attrs, ..
        } = *self.entries;
        let class = self.class;
        let super_field = if self.entries.super_ty.has_super(self.kind) {
            quote_spanned!(class.span()=> pub(super) _super: Super, )
        } else {
            quote!()
        };
        let fields = self
            .entries
            .fields
            .values()
            .map(|field| field.expand(self.kind));
        let struct_class = quote_spanned!(class.span()=> pub struct #class);
        if self.entries.super_ty.has_super(self.kind) {
            let span = self.entries.super_ty.span();
            quote_each_token_spanned! {tokens span
                pub(super) type Super = ::classes::prelude::CData<super::Super>;
            }
        }
        quote_each_token! {
            tokens
            #(#attrs)*
            #[repr(C)]
            #struct_class {
                #(#super_attrs)*
                #super_field
                #(#fields,)*
            }
        };
    }
}

struct ExpandFieldGetSet<'a> {
    field: &'a FieldEntry<'a>,
    class_kind: ClassKind,
}

impl ToTokens for ExpandFieldGetSet<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        use FieldKind::*;
        use FieldTypeKind::*;
        let &FieldEntry {
            vis,
            name,
            ty,
            init,
            ..
        } = self.field;
        let FieldType { kind, ty } = ty;
        let wrapped_ty = self.field.ty.wrap_type();
        let raw_get_name = format_ident!("raw_get_{}", name);
        // _classes::_$class
        let vis = vis.expand::<2>();
        let get_name = format_ident!("get_{}", name);
        let get_mut_name = format_ident!("get_mut_{}", name);
        let set_name = format_ident!("set_{}", name);
        let replace_name = format_ident!("replace_{}", name);
        let update_name_with = format_ident!("update_{}_with", name);
        let take_name = format_ident!("take_{}", name);
        let self_0 = if self.class_kind.is_mixin() {
            quote_spanned!(name.span()=> self.0.vtable().data_without_super(&self.0))
        } else {
            quote_spanned!(name.span()=> self.0)
        };
        let ref_self_dot_name = quote_spanned!(name.span()=> &#self_0 .#name);
        let as_get_set_ty = quote_spanned!(ty.span()=> #ty as ::classes::get_set::GetSet);
        let ty_get = quote_spanned!(ty.span()=> <#as_get_set_ty>::Get);
        let ty_set = quote_spanned!(ty.span()=> <#as_get_set_ty>::Set);
        let ty_option_get = quote_spanned!(ty.span()=> <#as_get_set_ty>::OptionGet);
        let into_set_ty = quote_spanned!(ty.span()=> ::core::convert::Into<<#as_get_set_ty>::Set>);
        let into_ty = quote_spanned!(ty.span()=> ::core::convert::Into<#ty>);
        match kind {
            Default(kind) => {
                let get_fn;
                let set_fn;
                let (ty_get, ty_set) = match kind {
                    Rc => {
                        get_fn = quote_spanned!(name.span()=> ::classes::get_set::GetSet::cell_get);
                        set_fn = quote_spanned!(name.span()=> ::classes::get_set::GetSet::cell_set);
                        (ty_get, into_set_ty)
                    }
                    NonRc => {
                        get_fn =
                            quote_spanned!(name.span()=> ::classes::get_set::GetSetCopy::cell_get);
                        set_fn =
                            quote_spanned!(name.span()=> ::classes::get_set::GetSetCopy::cell_set);
                        (ty.to_token_stream(), into_ty)
                    }
                };
                quote_each_token! { tokens
                    #[inline]
                    #vis fn #get_name(&self) -> #ty_get {
                        #get_fn(#ref_self_dot_name)
                    }
                    #[inline]
                    #vis fn #set_name<_T: #ty_set>(&self, #name: _T) {
                        #set_fn(#ref_self_dot_name, #name.into());
                    }
                    #[inline]
                    #[must_use]
                    #vis fn #replace_name<_T: #ty_set>(&self, #name: _T) -> #ty_get {
                        let old = self.#get_name();
                        self.#set_name(#name);
                        old
                    }
                    #[inline]
                    #vis fn #update_name_with<_T: #ty_set, _F: ::core::ops::FnOnce(#ty_get) -> _T>(&self, f: _F) {
                        self.#set_name(f(self.#get_name()));
                    }
                    #[inline] #vis fn #raw_get_name(&self) -> &#wrapped_ty { #ref_self_dot_name }
                }
            }
            Raw | Final => {
                quote_each_token! { tokens
                    #[inline] #vis fn #get_name(&self) -> &#ty { #ref_self_dot_name }
                    #[inline] #vis fn #raw_get_name(&self) -> &#wrapped_ty { #ref_self_dot_name }
                }
            }
            TakeCell => {
                quote_each_token! { tokens
                    #[inline]
                    #vis fn #get_name(&self) -> ::core::option::Option<::classes::cell::TakeRef<'_, #ty>> {
                        #self_0 .#name.borrow()
                    }
                    #[inline]
                    #vis fn #get_mut_name(&self) -> ::core::option::Option<::classes::cell::TakeRefMut<'_, #ty>> {
                        #self_0 .#name.borrow_mut()
                    }
                    #[inline]
                    #vis fn #set_name<_T: ::core::convert::Into<::core::option::Option<#ty>>>(&self, #name: _T) {
                        #self_0 .#name.set(#name.into());
                    }
                    #[inline]
                    #[must_use]
                    #vis fn #take_name(&self) -> ::core::option::Option<#ty> {
                        #self_0 .#name.take()
                    }
                    #[inline]
                    #[must_use]
                    #vis fn #replace_name<_T: ::core::convert::Into<::core::option::Option<#ty>>>(&self, #name: _T) -> ::core::option::Option<#ty> {
                        #self_0 .#name.replace(#name.into())
                    }
                    #[inline] #vis fn #raw_get_name(&self) -> &#wrapped_ty { #ref_self_dot_name }
                }
            }
            Mutable => {
                quote_each_token! { tokens
                    #[inline]
                    #vis fn #get_name(&self) -> ::core::cell::Ref<'_, #ty> {
                        #self_0 .#name.borrow()
                    }
                    #[inline]
                    #vis fn #get_mut_name(&self) -> ::core::cell::RefMut<'_, #ty> {
                        #self_0 .#name.borrow_mut()
                    }
                    #[inline]
                    #vis fn #set_name(&self, #name: #ty) {
                        #self_0 .#name.replace(#name);
                    }
                    #[inline]
                    #[must_use]
                    #vis fn #replace_name(&self, #name: #ty) -> #ty {
                        #self_0 .#name.replace(#name)
                    }
                    #[inline] #vis fn #raw_get_name(&self) -> &#wrapped_ty { #ref_self_dot_name }
                }
            }
            Late(kind) => match init {
                Some(init) => {
                    let get_fn;
                    let set_fn;
                    let (ty_get, ty_set) = match kind {
                        Rc => {
                            get_fn = quote_spanned!(name.span()=>
                                ::classes::get_set::GetSet::cell_option_get_or_init_with
                            );
                            set_fn = quote_spanned!(name.span()=>
                                ::classes::get_set::GetSet::cell_option_set
                            );
                            (ty_option_get, ty_set)
                        }
                        NonRc => {
                            get_fn = quote_spanned!(name.span()=>
                                ::classes::get_set::GetSetCopy::cell_option_get_or_init_with
                            );
                            set_fn = quote_spanned!(name.span()=>
                                ::classes::get_set::GetSetCopy::cell_option_set
                            );
                            (ty.to_token_stream(), ty.to_token_stream())
                        }
                    };
                    quote_each_token! { tokens
                        #[inline]
                        #vis fn #get_name(&self) -> #ty_get {
                            #get_fn(#ref_self_dot_name, || #init)
                        }
                        #[inline]
                        #vis fn #set_name(&self, #name: #ty_set) {
                            #set_fn(#ref_self_dot_name, #name);
                        }
                        #[inline]
                        #[must_use]
                        #vis fn #replace_name(&self, #name: #ty_set) -> #ty_get {
                            let old = self.#get_name();
                            self.#set_name(#name);
                            old
                        }
                        #[inline]
                        #vis fn #update_name_with<_F: ::core::ops::FnOnce(#ty_get) -> #ty_set>(&self, f: _F) {
                            self.#set_name(f(self.#get_name()));
                        }
                        #[inline] #vis fn #raw_get_name(&self) -> &#wrapped_ty { #ref_self_dot_name }
                    }
                }
                None => {
                    let get_fn;
                    let get_forced_fn;
                    let set_fn;
                    let (ty_get_forced, ty_set, ty_get) = match kind {
                        Rc => {
                            get_forced_fn = quote_spanned!(name.span()=> ::classes::get_set::GetSet::cell_option_get);
                            get_fn = quote_spanned!(name.span()=> ::classes::get_set::GetSet::try_cell_option_get);
                            set_fn = quote_spanned!(name.span()=> ::classes::get_set::GetSet::cell_option_set);
                            let option_ty_get = quote_spanned!(ty.span()=>
                                <::core::option::Option<#ty> as ::classes::get_set::GetSet>::Get
                            );
                            (ty_option_get, ty_set, option_ty_get)
                        }
                        NonRc => {
                            get_forced_fn = quote_spanned!(name.span()=> ::classes::get_set::GetSetCopy::cell_option_get);
                            get_fn = quote_spanned!(name.span()=> ::classes::get_set::GetSetCopy::cell_get);
                            set_fn = quote_spanned!(name.span()=> ::classes::get_set::GetSetCopy::cell_option_set);
                            let ty_get = quote_spanned!(ty.span()=> ::core::option::Option<#ty>);
                            (ty.to_token_stream(), ty.to_token_stream(), ty_get)
                        }
                    };
                    quote_each_token! { tokens
                        #[inline]
                        #vis fn #get_name(&self) -> #ty_get_forced {
                            #get_forced_fn(#ref_self_dot_name)
                        }
                        #[inline]
                        #vis fn #set_name(&self, #name: #ty_set) {
                            #set_fn(#ref_self_dot_name, #name);
                        }
                        #[inline]
                        #[must_use]
                        #vis fn #replace_name(&self, #name: #ty_set) -> #ty_get {
                            let old = #get_fn(#ref_self_dot_name);
                            #set_fn(#ref_self_dot_name, #name);
                            old
                        }
                        #[inline]
                        #vis fn #update_name_with<_F: ::core::ops::FnOnce(#ty_get) -> #ty_set>(&self, f: _F) {
                            #set_fn(#ref_self_dot_name, f(#get_fn(#ref_self_dot_name)));
                        }
                        #[inline] #vis fn #raw_get_name(&self) -> &#wrapped_ty { #ref_self_dot_name }
                    };
                }
            },
            LateFinal(kind) => {
                match init {
                    Some(init) => {
                        let (ty_get, get_fn) = match kind {
                            Rc => {
                                let get_fn = quote_spanned!(name.span()=>
                                    ::classes::get_set::GetSet::once_cell_get_or_init_with
                                );
                                (ty_option_get, get_fn)
                            }
                            NonRc => {
                                let get_fn = quote_spanned!(name.span()=>
                                    ::classes::get_set::GetSetOnce::get_or_init_with
                                );
                                (quote_spanned!(ty.span()=> &#ty), get_fn)
                            }
                        };
                        quote_each_token! { tokens
                            #[inline]
                            #vis fn #get_name(&self) -> #ty_get {
                                #get_fn(#ref_self_dot_name, || #init)
                            }
                            #[inline] #vis fn #raw_get_name(&self) -> &#wrapped_ty { #ref_self_dot_name }
                        }
                    }
                    None => {
                        let get_fn;
                        let set_fn;
                        let (ty_option_get, ty_set) = match kind {
                            Rc => {
                                get_fn = quote_spanned!(name.span()=> ::classes::get_set::GetSet::once_cell_get);
                                set_fn = quote_spanned!(name.span()=> ::classes::get_set::GetSet::once_cell_set);
                                (ty_option_get, ty_set)
                            }
                            NonRc => {
                                get_fn = quote_spanned!(name.span()=> ::classes::get_set::GetSetOnce::get);
                                set_fn = quote_spanned!(name.span()=> ::classes::get_set::GetSetOnce::set);
                                (quote_spanned!(ty.span()=> &#ty), ty.to_token_stream())
                            }
                        };
                        quote_each_token! { tokens
                            #[inline]
                            #[track_caller]
                            #vis fn #get_name(&self) -> #ty_option_get {
                                #get_fn(#ref_self_dot_name)
                            }
                            #[inline]
                            #[track_caller]
                            #vis fn #set_name(&self, #name: #ty_set) {
                                #set_fn(#ref_self_dot_name, #name);
                            }
                            #[inline] #vis fn #raw_get_name(&self) -> &#wrapped_ty { #ref_self_dot_name }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum FieldTypeKind {
    NonRc,
    Rc,
}

#[derive(Clone, Copy)]
enum FieldKind {
    Default(FieldTypeKind),
    Raw,
    Mutable,
    TakeCell,
    Final,
    Late(FieldTypeKind),
    LateFinal(FieldTypeKind),
}

#[derive(Clone, Copy)]
struct FieldType<'a> {
    ty: &'a syn::Type,
    kind: FieldKind,
}

impl std::fmt::Debug for FieldKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default(FieldTypeKind::NonRc) => f.write_str("{default}(copy)"),
            Self::Default(FieldTypeKind::Rc) => f.write_str("{default}(rc)"),
            Self::Raw => f.write_str("raw"),
            Self::Mutable => f.write_str("mutable"),
            Self::TakeCell => f.write_str("takecell"),
            Self::Final => f.write_str("final"),
            Self::Late(FieldTypeKind::NonRc) => f.write_str("late(copy)"),
            Self::Late(FieldTypeKind::Rc) => f.write_str("late(rc)"),
            Self::LateFinal(FieldTypeKind::NonRc) => f.write_str("late final(non-rc)"),
            Self::LateFinal(FieldTypeKind::Rc) => f.write_str("late final(rc)"),
        }
    }
}
impl std::fmt::Display for FieldKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default(_) => Ok(()),
            Self::Raw => f.write_str("raw"),
            Self::Mutable => f.write_str("mutable"),
            Self::TakeCell => f.write_str("takecell"),
            Self::Final => f.write_str("final"),
            Self::Late(_) => f.write_str("late"),
            Self::LateFinal(_) => f.write_str("late final"),
        }
    }
}

impl FieldType<'_> {
    fn wrap_type(&self) -> TokenStream {
        use FieldKind::*;
        let ty = self.ty;
        match self.kind {
            Default(_) => {
                quote_spanned!(ty.span()=> ::core::cell::Cell<#ty>)
            }
            Final | Raw => ty.to_token_stream(),
            TakeCell => {
                quote_spanned!(ty.span()=> ::classes::cell::TakeCell<#ty>)
            }
            Mutable => {
                quote_spanned!(ty.span()=> ::core::cell::RefCell<#ty>)
            }
            Late(_) => {
                quote_spanned!(ty.span()=> ::core::cell::Cell<::core::option::Option<#ty>>)
            }
            LateFinal(_) => {
                quote_spanned!(ty.span()=> ::core::cell::OnceCell<#ty>)
            }
        }
    }
}

impl Field {
    fn entry(&self, tokens: &mut TokenStream) -> FieldEntry<'_> {
        use CheckError::*;
        use FieldKind::*;
        let mut new_err = |span, kind, err| {
            syn::Error::new(span, InvalidFieldKind(kind, err))
                .into_compile_error()
                .to_tokens(tokens);
            kind
        };
        let mut check_late_final = |kind, span| {
            let err = match (self.kw_late, self.kw_final) {
                (None, None) => return kind,
                (None, Some(_)) => "final",
                (Some(_), None) => "late",
                (Some(_), Some(_)) => "late final",
            };
            new_err(span, kind, err)
        };
        let kind = match (self.kw_raw, self.kw_mutable, self.kw_takecell) {
            (None, None, None) => match (self.kw_late, self.kw_final) {
                (None, None) => Default(self.ty.ty_kind()),
                (None, Some(_)) => Final,
                (Some(_), None) => Late(self.ty.ty_kind()),
                (Some(_), Some(_)) => LateFinal(self.ty.ty_kind()),
            },
            (Some(raw), Some(_), None) => new_err(raw.span, Raw, "raw mutable"),
            (Some(raw), Some(_), Some(_)) => new_err(raw.span, Raw, "raw mutable takecell"),
            (None, Some(mutable), Some(_)) => new_err(mutable.span, Mutable, "mutable takecell"),
            (Some(raw), None, Some(_)) => new_err(raw.span, Raw, "raw takecell"),
            (Some(raw), None, None) => check_late_final(Raw, raw.span),
            (None, Some(mutable), None) => check_late_final(Mutable, mutable.span),
            (None, None, Some(takecell)) => check_late_final(TakeCell, takecell.span),
        };
        FieldEntry {
            attrs: self.attrs.as_slice(),
            vis: &self.vis,
            name: &self.name,
            colon: self.colon,
            ty: FieldType { ty: &self.ty, kind },
            init: self.init.as_ref().map(|init| &init.expr),
        }
    }
}

struct FieldEntries<'a> {
    kw_struct: Option<Token![struct]>,
    brace: syn::token::Brace,
    attrs: &'a [syn::Attribute],
    super_attrs: &'a [syn::Attribute],
    super_ty: SuperTy<'a>,
    fields: IndexMap<&'a syn::Ident, FieldEntry<'a>>,
}

impl FieldEntries<'_> {
    fn expand_item_struct<'a>(
        &'a self,
        class: impl Into<ClassIdent<'a>>,
        kind: ClassKind,
    ) -> ExpandItemStruct<'a> {
        ExpandItemStruct {
            entries: self,
            class: class.into(),
            kind,
        }
    }
    fn expand_gets_sets(
        &self,
        class_kind: ClassKind,
    ) -> impl Iterator<Item = ExpandFieldGetSet<'_>> {
        self.fields
            .values()
            .map(move |field| ExpandFieldGetSet { field, class_kind })
    }
    fn get_field<'a>(&'a self, field: &'a syn::Ident) -> syn::Result<&'a FieldEntry<'a>> {
        match self.fields.get(&field) {
            Some(entry) => Ok(entry),
            None => Err(syn::Error::new(
                field.span(),
                CheckError::NoSuchField(field),
            )),
        }
    }

    fn expand_write_fields<'a>(
        &'a self,
        seen_fields: Option<&'a HashSet<&'a syn::Ident>>,
    ) -> impl Iterator<Item = (&'a syn::Ident, TokenStream)> + use<'a> {
        self.fields
            .values()
            .filter(move |field| {
                seen_fields.is_none_or(|seend_fields| !seend_fields.contains(field.name))
            })
            .map(|field| {
                let field_ident = field.name;
                let expr = field
                    .wrap_init(InitKind::<syn::Expr>::NoInit(field_ident.span()))
                    .unwrap_or_else(syn::Error::into_compile_error);
                (field_ident, write_field(field_ident, expr))
            })
    }
}

struct FieldEntry<'a> {
    attrs: &'a [syn::Attribute],
    vis: &'a Visibility,
    name: &'a syn::Ident,
    colon: Token![:],
    ty: FieldType<'a>,
    init: Option<&'a syn::Expr>,
}

enum InitKind<'a, T: ?Sized = syn::Expr> {
    Init(&'a T),
    DefaultInit(&'a T),
    /// use the span of `..`
    NoInit(Span),
}

impl<'a, T: ToTokens + ?Sized> From<&'a T> for InitKind<'a, T> {
    fn from(init: &'a T) -> Self {
        Self::Init(init)
    }
}

impl<T: ToTokens + ?Sized> From<Span> for InitKind<'_, T> {
    fn from(dot_dot_span: Span) -> Self {
        Self::NoInit(dot_dot_span)
    }
}

impl<'a, T: ToTokens> InitKind<'a, T> {
    fn use_or_default(self, default: Option<&'a syn::Expr>) -> InitKind<'a, dyn ToTokens + 'a> {
        match (self, default) {
            (InitKind::Init(expr), _) => InitKind::Init(expr),
            (InitKind::DefaultInit(default), _) => InitKind::DefaultInit(default),
            (InitKind::NoInit(_), Some(default)) => InitKind::DefaultInit(default),
            (InitKind::NoInit(dot_dot_span), None) => InitKind::NoInit(dot_dot_span),
        }
    }
}

impl FieldEntry<'_> {
    fn expand(&self, kind: ClassKind) -> ExpandFieldDef<'_> {
        ExpandFieldDef { entry: self, kind }
    }
    fn wrap_init<'a, T: ToTokens + 'a>(
        &self,
        init_kind: impl Into<InitKind<'a, T>>,
    ) -> syn::Result<TokenStream> {
        use FieldKind::*;
        use FieldTypeKind::*;
        use InitKind::*;
        Ok(
            match (self.ty.kind, init_kind.into().use_or_default(self.init)) {
                (Default(NonRc), DefaultInit(expr) | Init(expr)) => {
                    quote_spanned!(expr.span()=> ::classes::get_set::NewCopy::new_cell(#expr))
                }
                (Default(Rc), DefaultInit(expr) | Init(expr)) => {
                    quote_spanned!(expr.span()=> ::classes::get_set::New::new_cell(#expr))
                }
                (Final | Raw, DefaultInit(expr) | Init(expr)) => expr.to_token_stream(),
                (TakeCell, NoInit(dot_dot_span)) => {
                    quote_spanned!(dot_dot_span=> ::core::default::Default::default())
                }
                (TakeCell, DefaultInit(expr) | Init(expr)) => {
                    quote_spanned!(expr.span()=> ::classes::cell::TakeCell::new(#expr))
                }
                (Mutable, DefaultInit(expr) | Init(expr)) => {
                    quote_spanned!(expr.span()=> ::core::cell::RefCell::new(#expr))
                }
                (Default(_) | Final | Raw | Mutable, NoInit(dot_dot_span)) => {
                    return Err(syn::Error::new(
                        dot_dot_span,
                        CheckError::MissingFieldExpr(self.name),
                    ));
                }
                (Late(NonRc), Init(expr)) => {
                    quote_spanned!(expr.span()=> ::classes::get_set::NewCopy::new_cell(#expr))
                }
                (Late(Rc), Init(expr)) => {
                    quote_spanned!(expr.span()=> ::classes::get_set::New::new_cell(#expr))
                }
                (Late(_), DefaultInit(_) | NoInit(_)) => {
                    quote!(::core::cell::Cell::new(::core::option::Option::None))
                }
                (LateFinal(NonRc), Init(expr)) => {
                    quote_spanned!(expr.span()=> ::classes::get_set::NewOnce::new(#expr))
                }
                (LateFinal(Rc), Init(expr)) => {
                    quote_spanned!(expr.span()=> ::classes::get_set::New::new_once_cell(#expr))
                }
                (LateFinal(_), DefaultInit(_) | NoInit(_)) => {
                    quote!(::core::cell::OnceCell::new())
                }
            },
        )
    }
}

struct ExpandFieldDef<'a> {
    entry: &'a FieldEntry<'a>,
    kind: ClassKind,
}

impl ToTokens for ExpandFieldDef<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let FieldEntry {
            attrs,
            vis,
            name,
            colon,
            ty,
            ..
        } = self.entry;
        let vis = if self.kind.is_mixin() {
            quote!(pub)
        } else {
            vis.to_token_stream()
        };
        let ty = ty.wrap_type();
        let vis = quote_spanned!(vis.span()=> pub(super));
        quote_each_token! {
            tokens
            // #(#attrs)* #vis #name : #ty,
            #(#attrs)* #vis #name #colon #ty
        };
    }
}

struct ExpandVtableStruct<'a, const OPT: bool> {
    class: ClassIdent<'a>,
    header: ExpandSuperOrHeader<'a, FieldDef<OPT>>,
    entries: &'a [FnEntry<'a>],
    impls: Option<&'a Punctuated<syn::Ident, Token![,]>>,
    kind: ClassKind,
}

impl<const OPT: bool> ToTokens for ExpandVtableStruct<'_, OPT> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let Self {
            class,
            header,
            entries,
            impls,
            kind,
        } = *self;
        let fields = entries
            .iter()
            .filter_map(|entry| entry.expand_vtable_field_def::<OPT>(kind));
        let impls = impls
            .iter()
            .flat_map(|idents| idents.pairs())
            .map(|pair| pair.into_value());

        if self.header.super_ty.has_super(kind) {
            let span = self.header.super_ty.span();
            if OPT {
                quote_each_token_spanned! {tokens span
                    pub(in super::super) type Super = ::classes::prelude::CVtableOpt<super::super::Super>;
                }
            } else {
                quote_each_token_spanned! {tokens span
                    pub(super) type Super = ::classes::prelude::CVtable<super::Super>;
                }
            }
        }
        quote_each_token! {
            tokens
            #[repr(C)]
            #[derive(Clone, Copy)]
        }
        if OPT {
            quote_each_token! { tokens #[derive(Default)] }
        }
        let vtable_wrapper = if OPT {
            format_ident!("CVtableOpt")
        } else {
            format_ident!("CVtable")
        };
        let struct_class = quote_spanned!(class.span()=> pub struct #class);
        quote_each_token! {
            tokens
            #struct_class {
                #header
                #(#fields)*
                #(pub #impls: ::classes::prelude::#vtable_wrapper<#impls>,)*
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum SuperTy<'a> {
    /// `B` of `class A extends B`
    Specific(&'a syn::Ident),
    /// `mixin!(B, M)` of `class A extends B with M`
    Mixined(&'a syn::Ident, &'a Withs),
    /// implicit super class `Object`
    Object(Span),
    /// `mixin M` which has no super class
    MixinNoSuper(Span),
    /// No super class
    NoSuper(Span),
}

impl<'a> SuperTy<'a> {
    fn is_mixin(&self) -> bool {
        matches!(self, MixinNoSuper(_))
    }
    fn span(&self) -> Span {
        match *self {
            Specific(ident) | Mixined(ident, _) => ident.span(),
            Object(span) | MixinNoSuper(span) | NoSuper(span) => span,
        }
    }
    fn has_super(&self, kind: ClassKind) -> bool {
        kind.is_mixin_instance() || matches!(self, Specific(_) | Mixined(..) | Object(_))
    }
}

#[derive(Clone, Copy)]
struct ExpandSuperOrHeader<'a, Mode: Copy> {
    super_ty: SuperTy<'a>,
    kind: ClassKind,
    mode: Mode,
}

impl<'a, Mode: Copy + Default> ExpandSuperOrHeader<'a, Mode> {
    fn new(super_ty: SuperTy<'a>, kind: ClassKind) -> Self {
        Self {
            super_ty,
            kind,
            mode: Mode::default(),
        }
    }
}

impl<'a, Mode: Copy> ExpandSuperOrHeader<'a, Mode> {
    fn with_mode(super_ty: SuperTy<'a>, kind: ClassKind, mode: Mode) -> Self {
        Self {
            super_ty,
            kind,
            mode,
        }
    }
}

#[derive(Default, Clone, Copy)]
struct FieldDef<const OPT: bool>;
#[derive(Default, Clone, Copy)]
struct DefaultExpr;
#[derive(Default, Clone, Copy)]
struct InitHeader;

#[derive(Clone, Copy)]
struct InitMixinHeader<'a> {
    class_and_mixin: Option<(ClassIdent<'a>, &'a syn::Ident)>,
}

impl<'a> InitMixinHeader<'a> {
    fn new(class_and_mixin: Option<(impl Into<ClassIdent<'a>>, &'a syn::Ident)>) -> Self {
        let class_and_mixin = class_and_mixin.map(|(class, mixin)| (class.into(), mixin));
        Self { class_and_mixin }
    }
}

#[derive(Default, Clone, Copy)]
struct Init;
#[derive(Default, Clone, Copy)]
struct AssertInit;

impl<const OPT: bool> ToTokens for ExpandSuperOrHeader<'_, FieldDef<OPT>> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let span = self.super_ty.span();
        let vis = if OPT {
            quote!(pub(in super::super))
        } else {
            quote!(pub(super))
        };
        if self.super_ty.has_super(self.kind) {
            quote_each_token_spanned! {tokens span
                #vis _super: Super,
            }
        } else if !self.kind.is_mixin() {
            if OPT {
                quote_each_token_spanned! { tokens span
                    header: ::classes::vtable::VtableHeaderOpt,
                }
            } else {
                quote_each_token_spanned! { tokens span
                    header: ::classes::vtable::VtableHeader,
                }
            }
        }
    }
}

impl ToTokens for ExpandSuperOrHeader<'_, DefaultExpr> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let span = self.super_ty.span();
        if self.super_ty.has_super(self.kind) {
            quote_each_token_spanned! { tokens span
                _super: Super::DEFAULT,
            }
        } else if !self.kind.is_mixin() {
            quote_each_token_spanned! { tokens span
                header: ::classes::vtable::VtableHeaderOpt::DEFAULT,
            }
        }
    }
}

impl ToTokens for ExpandSuperOrHeader<'_, InitHeader> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let span = self.super_ty.span();
        if self.super_ty.has_super(self.kind) {
            quote_each_token_spanned! { tokens span self._super.init_header(::core::option::Option::Some(ty), offset); }
        } else if !self.kind.is_mixin() {
            quote_each_token_spanned! { tokens span
                self.header = ::classes::vtable::VtableHeaderOpt::new(ty, offset);
            }
        }
    }
}

impl ToTokens for ExpandSuperOrHeader<'_, InitMixinHeader<'_>> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        if let Some((class, mixin)) = self.mode.class_and_mixin {
            let span = class.span();
            quote_each_token_spanned! { tokens span
                let (first, rest) = mixin_header.split_first_mut().expect("mixin header is empty");
                Super::init_mixin_header(rest);
                first.write(::classes::vtable::MixinVtableHeader::new::<super::#class>(
                    ::core::mem::size_of::<data::#class>()
                        - ::core::mem::size_of::<::classes::prelude::CDataBase<#mixin>>(),
                    super::#class::MIXIN_HEADER_ENTRIES
                        * ::core::mem::size_of::<::classes::vtable::MixinVtableHeader>()
                        + ::core::mem::size_of::<vtable::#class>()
                        - ::core::mem::size_of::<::classes::prelude::CVtableBase<#mixin>>(),
                ));
            }
        } else {
            let span = self.super_ty.span();
            if self.super_ty.has_super(self.kind) {
                quote_each_token_spanned! { tokens span Super::init_mixin_header(mixin_header); }
            } else if !self.kind.is_mixin() {
                quote_each_token_spanned! { tokens span assert!(mixin_header.is_empty()); }
            }
        }
    }
}

impl ToTokens for ExpandSuperOrHeader<'_, Init> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        if self.super_ty.has_super(self.kind) {
            let span = self.super_ty.span();
            quote_each_token_spanned! { tokens span Super::init(_self); }
        }
    }
}

impl ToTokens for ExpandSuperOrHeader<'_, AssertInit> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let span = self.super_ty.span();
        if self.super_ty.has_super(self.kind) {
            quote_each_token_spanned! { tokens span _super: self._super.assert_init(), }
        } else if !self.kind.is_mixin() {
            quote_each_token_spanned! { tokens span header: self.header.assert_init(), }
        }
    }
}

#[derive(Clone, Copy)]
enum OverrideMode<'a> {
    Single(OverrideSingle<'a>),
    Multi {
        base: OverrideBase<'a>,
        interfaces: &'a Punctuated<InterfaceMaybeSuperclass, Token![as]>,
    },
}

impl std::fmt::Debug for OverrideMode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(single) => write!(f, "{:?}", single),
            Self::Multi { base, interfaces } => f
                .debug_list()
                .entry(base)
                .entries(interfaces.pairs().map(|pair| pair.into_value()))
                .finish(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum MixinOn<'a> {
    /// `#[with(A/M1)] mixin M on M1` where `M1` is a mixin
    Mixin(&'a syn::Ident),
    /// `mixin M on A` where `A` is a class
    Class(&'a syn::Ident),
}

impl<'a> MixinOn<'a> {
    fn ident(&self) -> &'a syn::Ident {
        match self {
            MixinOn::Mixin(ident) => ident,
            MixinOn::Class(ident) => ident,
        }
    }
}

impl ToTokens for MixinOn<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            MixinOn::Mixin(ident) => ident.to_tokens(tokens),
            MixinOn::Class(ident) => ident.to_tokens(tokens),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum OverrideSingle<'a> {
    Super,
    /// `override fn A::f(..)`
    Supertype(&'a syn::Ident),
    /// `override fn I::f(..)` of `class A implements I`
    SelfInterface(&'a syn::Ident),
    /// `override fn A::f(..)` of `mixin M on A`
    On(MixinOn<'a>),
    /// `override fn M::f(..)` of `class A extends B with M`
    Mixin(&'a syn::Ident),
}

#[derive(Debug, Clone, Copy)]
enum OverrideBase<'a> {
    /// `override fn <Self as ...>::f(..)` of `class A`
    SelfClass,
    /// `override fn <Super as ...>::f(..)` of `class A extends B` and `class B implements M`
    SuperBase(Span),
    /// `override fn <I as ...>::f(..)` of `class A implements I`
    SelfInterfaceBase(#[expect(dead_code)] &'a syn::Ident),
    /// `override fn <A as ...>::f(..)` of `class C extends B` and `class B extends A`
    SuperclassBase(&'a syn::Ident),
    /// `override fn <M as ...>::f(..)` of `class A extends B with M`
    MixinBase(#[expect(dead_code)] &'a syn::Ident),
}

impl ItemFn {
    fn override_mode<'a>(
        &'a self,
        ons: Option<&[MixinOn<'a>]>,
        extends: Option<&Extends>,
        withs: Option<&'a Withs>,
        impls: Option<&'a Implements>,
        mixin_withs: Option<&Punctuated<MixinWith, Token![,]>>,
        tokens: &mut TokenStream,
    ) -> Option<OverrideMode<'a>> {
        use OverrideBase::*;
        use OverrideMode::*;
        use OverrideSingle::*;
        Some(match (&self.kw_override, &self.overrid, extends, impls) {
            (None, None, _, _) => return None,
            (Some(kw_override), None, None, None) => {
                syn::Error::new(
                    kw_override.span,
                    CheckError::InvalidOverrideNoSuperNorInterface,
                )
                .into_compile_error()
                .to_tokens(tokens);
                return None;
            }
            (Some(kw_override), None, None, Some(impls)) => {
                let first = impls.idents.first().unwrap();
                if impls.idents.len() > 1 {
                    syn::Error::new(
                        kw_override.span,
                        CheckError::AmbiguiousOverrideNoSuperMultiInterface(first, &self.name),
                    )
                    .into_compile_error()
                    .to_tokens(tokens);
                }
                Single(SelfInterface(first))
            }
            (Some(_), None, Some(_), _) => Single(Super),
            (kw_override, Some(overrid), _, _) => {
                if kw_override.is_none() {
                    syn::Error::new(self.kw_fn.span, CheckError::ExpectOverride)
                        .into_compile_error()
                        .to_tokens(tokens);
                }

                match overrid {
                    Override::Superclass(superclass, _) => {
                        Single(match (ons, extends, withs, impls) {
                            (None, None, None, None) if mixin_withs.is_some() => {
                                syn::Error::new(superclass.span(), CheckError::MissingSuperclass)
                                    .into_compile_error()
                                    .to_tokens(tokens);
                                On(MixinOn::Class(superclass))
                            }
                            (None, None, None, None) => {
                                syn::Error::new(
                                    superclass.span(),
                                    CheckError::InvalidOverrideNoSuperNorInterface,
                                )
                                .into_compile_error()
                                .to_tokens(tokens);
                                return None;
                            }
                            (Some(ons), ..) if ons.iter().any(|on| on.ident() == superclass) => {
                                On(ons
                                    .into_iter()
                                    .copied()
                                    .find(|on| on.ident() == superclass)
                                    .unwrap())
                            }
                            (_, Some(_), Some(withs), _) if withs.contains(superclass) => {
                                Mixin(superclass)
                            }
                            (.., Some(impls)) if impls.contains(superclass) => {
                                SelfInterface(superclass)
                            }
                            (_, None, ..) => Supertype(superclass),
                            (_, Some(_), ..) => {
                                debug_assert!(!mixin_withs.is_some());
                                Supertype(superclass)
                            }
                        })
                    }
                    Override::Interface {
                        alias, interfaces, ..
                    } => Multi {
                        base: match alias {
                            OverrideAlias::SelfClass(_) => SelfClass,
                            OverrideAlias::Super(super_ty) => SuperBase(super_ty.span),
                            OverrideAlias::Superclass(mixin)
                                if withs.is_some_and(|withs| {
                                    withs.idents.pairs().any(|pair| pair.into_value() == mixin)
                                }) || mixin_withs.is_some_and(|mixin_withs| {
                                    mixin_withs
                                        .iter()
                                        .all(|mixin_with| mixin_with.is_mixin_with(mixin))
                                }) =>
                            {
                                MixinBase(mixin)
                            }
                            OverrideAlias::Superclass(superclass)
                                if impls.is_some_and(|impls| {
                                    impls
                                        .idents
                                        .pairs()
                                        .any(|pair| pair.into_value() == superclass)
                                }) =>
                            {
                                SelfInterfaceBase(superclass)
                            }
                            OverrideAlias::Superclass(superclass) => SuperclassBase(superclass),
                        },
                        interfaces,
                    },
                }
            }
        })
    }
    fn fn_entry<'a>(
        &'a self,
        kind: ClassKind,
        class: &'a syn::Ident,
        super_ty: SuperTy<'a>,
        ons: Option<&[MixinOn<'a>]>,
        extends: Option<&Extends>,
        withs: Option<&'a Withs>,
        impls: Option<&'a Implements>,
        mixin_withs: Option<&Punctuated<MixinWith, Token![,]>>,
        tokens: &mut TokenStream,
    ) -> Option<FnEntry<'a>> {
        // eprintln!("entry fn {}", self.name);
        let override_mode = self.override_mode(ons, extends, withs, impls, mixin_withs, tokens);
        if let Some((span, kw)) = None
            .or(self.kw_async.map(|kw| (kw.span, "async")))
            .or(self.kw_const.map(|kw| (kw.span, "const")))
            .or(self.kw_final.map(|kw| (kw.span, "final")))
        {
            syn::Error::new(span, CheckError::UnsupportedKwFn(kw))
                .into_compile_error()
                .to_tokens(tokens);
        }
        fn return_type_to_opt_type(return_type: &syn::ReturnType) -> Option<&syn::Type> {
            match return_type {
                syn::ReturnType::Default => None,
                syn::ReturnType::Type(_, ty) => Some(ty),
            }
        }
        let kind = match &self.ret_body {
            FnRetAndBody::Method(ret, body) => FnEntryKind::Method {
                self_kind: self.params.self_kind(tokens),
                override_mode,
                ret: return_type_to_opt_type(ret),
                body: body.body(),
                body_span: body.span(),
            },
            FnRetAndBody::Function(ret, body) => FnEntryKind::Function {
                ret: return_type_to_opt_type(ret),
                body,
            },
            FnRetAndBody::Ctor(_, self_type, brace, ctor_body, method_body) => {
                if kind.is_mixin() | kind.is_mixin_instance() {
                    syn::Error::new(self_type.span, CheckError::UnsupportedCtor)
                        .into_compile_error()
                        .to_tokens(tokens);
                    return None;
                }
                FnEntryKind::Ctor {
                    ret_span: self_type.span,
                    ctor_body,
                    method_body: method_body.as_ref(),
                    body_span: brace.span.join(),
                }
            }
        };
        Some(FnEntry {
            class,
            super_ty,
            attrs: &self.attrs,
            vis: &self.vis,
            kw_unsafe: &self.kw_unsafe,
            abi: &self.abi,
            kw_fn: self.kw_fn,
            name: &self.name,
            param_span: self.params.tk_paren.span,
            params: &self.params.params,
            kind,
        })
    }
}

macro_rules! def_self_kind {
    (enum $name:ident {
        $( $variant:ident = $display:literal, )*
    }) => {
        #[derive(Debug, Clone, Copy)]
        enum $name {
            $( $variant(Span), )*
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $( Self::$variant(_) => f.write_str($display), )*
                }
            }
        }
        static ALL_SELF_KINDS: &str = concat!($("`", $display, "`, ")*);
    }
}

def_self_kind! {
    enum SelfKind {
        RefSelf = "&self",
    }
}

impl FnParams {
    fn self_kind(&self, tokens: &mut TokenStream) -> SelfKind {
        fn is_self_ty(ty: &syn::Type) -> bool {
            matches!(ty, syn::Type::Reference(ty) if matches!(&*ty.elem, syn::Type::Path(path) if path.path.is_ident("Self")))
        }
        if let Some(receiver) = &self.self_param {
            if receiver.reference.is_some()
                && receiver.mutability.is_none()
                && receiver.colon_token.is_none()
                && is_self_ty(&receiver.ty)
            {
                SelfKind::RefSelf(receiver.span())
            } else {
                syn::Error::new(receiver.span(), CheckError::UnsupportedSelfReceiver)
                    .into_compile_error()
                    .to_tokens(tokens);
                SelfKind::RefSelf(receiver.span())
            }
        } else {
            syn::Error::new(self.tk_paren.span.open(), CheckError::ExpectSelfReceiver)
                .into_compile_error()
                .to_tokens(tokens);
            SelfKind::RefSelf(self.tk_paren.span.join())
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ClassKindFlags: u8 {
        const CLASS = 1 << 0;
        const ABSTRACT = 1 << 1;
        const MIXIN = 1 << 2;
        // mixin instance is an abstract class by default.
        const MIXIN_INSTANCE = Self::ABSTRACT.bits() | 1 << 3;

        const CONCRETE_CLASS = Self::CLASS.bits();
        const CONCRETE_MIXIN_CLASS = Self::MIXIN.bits() | Self::CLASS.bits();
        // mixin is treated as an abstract type.
        const ABSTRACT_MIXIN = Self::ABSTRACT.bits() | Self::MIXIN.bits();
        const ABSTRACT_CLASS = Self::ABSTRACT.bits() | Self::CLASS.bits();
        const ABSTRACT_MIXIN_CLASS = Self::ABSTRACT.bits() | Self::MIXIN.bits() | Self::CLASS.bits();
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassKind {
    // concrete types
    ConcreteClass = ClassKindFlags::CONCRETE_CLASS.bits(),
    // we cannot tell the difference between ConcreteMixinClass and ConcreteClass,
    // so we use `ConcreteClass` to represent both.
    ConcreteMixinClass = ClassKindFlags::CONCRETE_MIXIN_CLASS.bits(),
    // abstract types
    AbstractMixin = ClassKindFlags::ABSTRACT_MIXIN.bits(),
    AbstractClass = ClassKindFlags::ABSTRACT_CLASS.bits(),
    AbstractMixinClass = ClassKindFlags::ABSTRACT_MIXIN_CLASS.bits(),
    MixinInstance = ClassKindFlags::MIXIN_INSTANCE.bits(),
}

#[allow(dead_code)]
impl ClassKind {
    pub const fn is_class(self) -> bool {
        ClassKindFlags::from_class_kind(self).is_class()
    }
    pub const fn is_abstract(self) -> bool {
        ClassKindFlags::from_class_kind(self).is_abstract()
    }
    pub const fn is_concrete(self) -> bool {
        ClassKindFlags::from_class_kind(self).is_concrete()
    }
    pub const fn is_mixin(self) -> bool {
        ClassKindFlags::from_class_kind(self).is_mixin()
    }
    pub const fn is_mixin_instance(self) -> bool {
        ClassKindFlags::from_class_kind(self).is_mixin_instance()
    }
}

#[allow(dead_code)]
impl ClassKindFlags {
    const fn from_class_kind(kind: ClassKind) -> Self {
        ClassKindFlags::from_bits_truncate(kind as u8)
    }
    pub const fn is_class(self) -> bool {
        self.contains(Self::CLASS)
    }
    pub const fn is_abstract(self) -> bool {
        self.contains(Self::ABSTRACT)
    }
    pub const fn is_concrete(self) -> bool {
        !self.is_abstract()
    }
    pub const fn is_mixin(self) -> bool {
        self.contains(Self::MIXIN)
    }
    pub const fn is_mixin_instance(self) -> bool {
        let is_mixin_instance = self.contains(Self::MIXIN_INSTANCE);
        debug_assert!(is_mixin_instance == (self.bits() == Self::MIXIN_INSTANCE.bits()));
        is_mixin_instance
    }
}

struct ClassEntries<'a> {
    config: &'a Config,
    vis: &'a Visibility,
    class: &'a syn::Ident,
    kind: ClassKind,
    mixin_instances: &'a [MixinInstance<'a>],
    fields: &'a FieldEntries<'a>,
    fns: &'a [FnEntry<'a>],
    ons: Option<&'a [MixinOn<'a>]>,
    impls: Option<&'a Punctuated<syn::Ident, Token![,]>>,
    consts: &'a [&'a syn::ItemConst],
}

#[derive(Clone, Copy)]
struct MixinSuper<'a> {
    idents: &'a Punctuated<syn::Ident, Token![/]>,
}

impl ToTokens for MixinSuper<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let span = self.idents.span();
        let mut idents = self.idents.pairs().map(|pair| pair.into_value());
        if self.idents.len() == 1 {
            let ident = idents.next().unwrap();
            quote_each_token_spanned! { tokens span #ident<T, V> }
        } else {
            quote_each_token_spanned! { tokens span
                ::classes::mixin!(<T, V> #(#idents,)*)
            }
        }
    }
}

struct MixinInstance<'a> {
    super_ty: MixinSuper<'a>,
    mixin: &'a syn::Ident,
    class: syn::Ident,
}

impl ClassEntries<'_> {
    fn super_ty(&self) -> SuperTy<'_> {
        self.fields.super_ty
    }
    fn ons(&self) -> impl Iterator<Item = MixinOn<'_>> {
        self.ons.iter().copied().flatten().copied()
    }
    fn impls(&self) -> impl Iterator<Item = &syn::Ident> {
        self.impls
            .iter()
            .flat_map(|impls| impls.pairs())
            .map(|pair| pair.into_value())
    }
    fn expand_vtable_struct<const OPT: bool>(&self) -> ExpandVtableStruct<'_, OPT> {
        ExpandVtableStruct {
            class: self.class.into(),
            header: ExpandSuperOrHeader::new(self.fields.super_ty, self.kind),
            entries: &self.fns,
            impls: self.impls,
            kind: self.kind,
        }
    }
    fn expand_static_vtable(&self) -> ExpandStaticVtable<'_> {
        ExpandStaticVtable {
            kind: self.kind,
            class: &self.class,
        }
    }
    fn expand_mixin<const MODULE_PATH: bool>(&self) -> ExpandMixin<'_, MODULE_PATH> {
        ExpandMixin { class: self }
    }
    fn mixin_macro(&self) -> MixinInstanceMacro<'_> {
        MixinInstanceMacro {
            config: self.config,
            vis: self.vis,
            mixin: self.class,
            fields: self.fields,
            fns: self.fns,
            ons: self.ons,
            impls: self.impls,
            consts: self.consts,
        }
    }
    fn expand_delegate_ctor(&self) -> ExpandDelegateCtor<'_> {
        ExpandDelegateCtor {
            self_span: self
                .fields
                .kw_struct
                .map_or(self.class.span(), |kw_struct| kw_struct.span),
            brace: self.fields.brace,
            super_field: self
                .fields
                .super_ty
                .has_super(self.kind)
                .then_some(self.fields.super_ty.span()),
            super_ty: self.fields.super_ty,
            fields: &self.fields,
            kind: self.kind,
        }
    }
    fn expand_fn_impls(&self) -> impl Iterator<Item = ExpandFnImpl<'_>> {
        self.fns.iter().map(|entry| ExpandFnImpl {
            entry,
            fields: &self.fields,
            kind: self.kind,
        })
    }
    fn expand_vtable_impl(&self) -> ExpandVtableImpl<'_> {
        ExpandVtableImpl {
            class: self.class.into(),
            mixin: None,
            fields: self.fields,
            fns: self.fns,
            ons: self.ons,
            impls: self.impls,
            kind: self.kind,
        }
    }
    fn expand_pub_apis<const VIRTUAL: bool>(
        &self,
    ) -> impl Iterator<Item = ExpandPubApi<'_, VIRTUAL>> {
        self.fns
            .iter()
            .filter(|entry| match entry.kind {
                FnEntryKind::Method { body, .. } => VIRTUAL || body.is_some(),
                FnEntryKind::Ctor { .. } => VIRTUAL && self.kind.is_concrete(),
                FnEntryKind::Function { .. } => VIRTUAL,
            })
            .map(|entry| ExpandPubApi {
                config: self.config,
                entry,
                class_kind: self.kind,
                mixin: None,
            })
    }
    fn vtable_struct_fields(&self) -> VtableStructFields<'_> {
        let fields = self
            .fns
            .iter()
            .filter_map(|entry| entry.expand_vtable_field_def::<false>(self.kind))
            .map(|field| field.name)
            .collect();
        VtableStructFields {
            class: self.class.into(),
            kind: self.kind,
            super_ty: self.fields.super_ty,
            impls: self.impls,
            fields,
        }
    }
}

struct VtableStructFields<'a> {
    class: ClassIdent<'a>,
    kind: ClassKind,
    super_ty: SuperTy<'a>,
    fields: Vec<&'a syn::Ident>,
    impls: Option<&'a Punctuated<syn::Ident, Token![,]>>,
}

impl VtableStructFields<'_> {
    fn all_fields(&self) -> impl Iterator<Item = &syn::Ident> + use<'_> {
        self.fields
            .iter()
            .copied()
            .chain(self.impls.into_iter().flatten())
    }
    fn expand_assert_layout_eq(&self) -> ExpandAssertLayoutEq<'_> {
        ExpandAssertLayoutEq { fields: self }
    }
    fn expand_debug_vtable_layout(&self) -> ExpandDebugVtableLayout<'_> {
        ExpandDebugVtableLayout { fields: self }
    }
}

struct ExpandAssertLayoutEq<'a> {
    fields: &'a VtableStructFields<'a>,
}

impl ToTokens for ExpandAssertLayoutEq<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let class = self.fields.class;
        let fields = self.fields.all_fields();

        quote_each_token! { tokens
            ::classes::assert_layout_eq! {
                vtable::#class,
                vtable::opt::#class,
                { #( #fields ),* }
            }
        }
    }
}

struct ExpandDebugVtableLayout<'a> {
    fields: &'a VtableStructFields<'a>,
}

impl ToTokens for ExpandDebugVtableLayout<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let &VtableStructFields {
            class,
            kind,
            super_ty,
            ref fields,
            impls,
        } = self.fields;
        let fields = fields.iter().map(|field| {
            quote_spanned!(field.span()=> dbg.field(stringify!(#field), &offset_of!(#field));)
        });
        let super_ty = if super_ty.has_super(kind) {
            quote_spanned!(super_ty.span()=> dbg.field("super", &self.this._super.debug_vtable_layout(offset_of!(_super)));)
        } else if !kind.is_mixin() {
            quote_spanned!(super_ty.span()=> dbg.field("header", &self.this.header);)
        } else {
            quote!()
        };
        let impls = impls.into_iter().flatten().map(|ident| {
            quote_spanned!(ident.span()=> dbg.field(stringify!(#ident), &self.this.#ident.debug_vtable_layout(offset_of!(#ident)));)
        });
        quote_each_token! { tokens
            impl #class {
                pub const fn debug_vtable_layout(&self, offset: usize) -> self::DebugVtableLayout<'_> {
                    self::DebugVtableLayout { this: self, offset }
                }
            }

            pub struct DebugVtableLayout<'a> {
                this: &'a self::#class,
                offset: usize,
            }

            impl ::core::fmt::Debug for self::DebugVtableLayout<'_> {
                #[allow(unused_macros)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    macro_rules! offset_of {
                        ($field:ident) => {
                            self.offset + ::core::mem::offset_of!(#class, $field)
                        };
                    }
                    let mut dbg = f.debug_struct(stringify!(#class));
                    dbg.field("\'start", &self.offset);
                    #super_ty
                    #( #fields )*
                    #( #impls )*
                    dbg.field("\'end", &(self.offset + ::core::mem::size_of::<#class>()));
                    dbg.finish()
                }
            }
        }
    }
}

struct MixinInstanceMacro<'a> {
    config: &'a Config,
    vis: &'a Visibility,
    mixin: &'a syn::Ident,
    fields: &'a FieldEntries<'a>,
    fns: &'a [FnEntry<'a>],
    ons: Option<&'a [MixinOn<'a>]>,
    impls: Option<&'a Punctuated<syn::Ident, Token![,]>>,
    consts: &'a [&'a syn::ItemConst],
}

impl ToTokens for MixinInstanceMacro<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let mixin = self.mixin();

        let macro_export = if self.vis.is_pub() {
            quote!(#[macro_export])
        } else {
            quote!()
        };
        let gets_sets = self.fields.expand_gets_sets(ClassKind::MixinInstance);

        let ons = self.ons().collect::<Vec<_>>();
        let offset_ons = ons.iter().map(|&on| {
            quote_spanned! {on.span()=>
                #[cfg(not(debug_assertions))]
                0,
                #[cfg(debug_assertions)]
                Super::TYPE.const_offset_of(::classes::prelude::CVtable::<#on>::TYPE)
                    .expect(concat!(
                        stringify!($super_ty), " is not a subclass of ", stringify!(#on),
                        ", we only support `mixin` to `on` a superclass yet"
                    ))
                    .offset,
            }
        });
        let impls = self.impls().collect::<Vec<_>>();

        let item_struct = self
            .fields
            .expand_item_struct("$class", ClassKind::MixinInstance);

        let vtable_struct_fields = self.vtable_struct_fields();
        let assert_layout_eq = vtable_struct_fields.expand_assert_layout_eq();
        let debug_vtable_layout = vtable_struct_fields.expand_debug_vtable_layout();

        let vtable_struct = self.expand_vtable_struct::<false>();
        let vtable_struct_opt = self.expand_vtable_struct::<true>();

        let delegate_ctor = self.expand_delegate_ctor();
        let fn_impls = self.expand_fn_impls();
        let vtable_impl = self.expand_vtable_impl();

        let pub_api = self.expand_pub_apis::<true>();
        let pub_api_non_virtual = self.expand_pub_apis::<false>();

        let item_consts = self.consts;
        let item_const_refs = item_consts
            .iter()
            .map(|item| item.expand_ref::<Token![self]>());
        let item_const_refs_in_data = item_consts
            .iter()
            .map(|item| item.expand_ref::<Token![super]>());

        let num_impls = ons.len() + impls.len();
        quote_each_token! {
            tokens

            #macro_export
            macro_rules! #mixin {
                ($mod_name:ident, $class:ident, $super_ty:ty) => {
                    #[allow(unused_imports)]
                    pub use $mod_name::$class;

                    #[allow(non_snake_case)]
                    #[allow(non_camel_case_types)]
                    #[allow(unused_attributes)]
                    mod $mod_name {
                        ::classes::_mod_uses! { mod class $class }
                        ::classes::_def_class! { class $class }
                        type Super<T = ::classes::class::ClassMarker, V = ::classes::class::Virtual> = $super_ty;
                        ::classes::_def_mixin_instance! { $class: Super with #mixin }
                        ::classes::_def_class_extends! { $class : Super (mixin_instance) }
                        #( ::classes::_def_class_impl! { $class : #impls } )*

                        mod data {
                            ::classes::_mod_uses! { mod data }

                            #item_struct

                            impl $class {
                                #delegate_ctor
                                #( #fn_impls )*
                                #( #item_const_refs_in_data )*
                            }
                        }
                        mod vtable {
                            ::classes::_mod_uses! { mod vtable }

                            #vtable_struct
                            #debug_vtable_layout

                            pub(super) mod opt {
                                ::classes::_mod_uses! { mod vtable::opt }

                                #vtable_struct_opt

                                #vtable_impl
                            }

                            pub static TYPE: ::classes::vtable::TypeInfo<#num_impls> =
                                ::classes::vtable::TypeInfo::new_mixin_instance::<super::$class>(
                                    Super::TYPE,
                                    unsafe { ::classes::prelude::CVtable::<#mixin>::TYPE.as_mixin_unchecked() },
                                    [
                                        #(#offset_ons)*
                                        #( ::core::mem::offset_of!(vtable::$class, #impls), )*
                                    ],
                                    // #[cfg(debug_assertions)]
                                    MODULE_PATH,
                                    // #[cfg(debug_assertions)]
                                    stringify!($class),
                                );
                        }

                        #assert_layout_eq

                        #( #item_consts )*

                        impl $class<::classes::ptr::RcDyn<$class>> {
                            #( #pub_api )*
                            #( #item_const_refs )*
                        }
                        impl $class<::classes::ptr::RcDyn<$class>, ::classes::class::NonVirtual> {
                            #( #pub_api_non_virtual )*
                        }
                        impl $class<::classes::ptr::RcDyn<$class>> {
                            #( #gets_sets )*
                        }
                    }
                };
            }
        }
    }
}

impl MixinInstanceMacro<'_> {
    fn mixin(&self) -> &syn::Ident {
        self.mixin
    }
    fn ons(&self) -> impl Iterator<Item = MixinOn<'_>> {
        self.ons.iter().copied().flatten().copied()
    }
    fn impls(&self) -> impl Iterator<Item = &syn::Ident> {
        self.impls
            .iter()
            .flat_map(|impls| impls.pairs())
            .map(|pair| pair.into_value())
    }
    fn expand_delegate_ctor(&self) -> ExpandDelegateCtor<'_> {
        ExpandDelegateCtor {
            self_span: self
                .fields
                .kw_struct
                .map_or(self.mixin.span(), |kw_struct| kw_struct.span),
            brace: self.fields.brace,
            super_field: Some(self.fields.super_ty.span()),
            super_ty: self.fields.super_ty,
            fields: &self.fields,
            kind: ClassKind::MixinInstance,
        }
    }
    fn expand_fn_impls(&self) -> impl Iterator<Item = ExpandFnImpl<'_>> {
        self.fns.iter().map(|entry| ExpandFnImpl {
            entry,
            fields: &self.fields,
            kind: ClassKind::MixinInstance,
        })
    }
    fn expand_vtable_struct<'a, const OPT: bool>(&'a self) -> ExpandVtableStruct<'a, OPT> {
        ExpandVtableStruct {
            class: "$class".into(),
            header: ExpandSuperOrHeader::new(self.fields.super_ty, ClassKind::MixinInstance),
            entries: &self.fns,
            impls: self.impls,
            kind: ClassKind::MixinInstance,
        }
    }
    fn expand_vtable_impl<'a>(&'a self) -> ExpandVtableImpl<'a> {
        ExpandVtableImpl {
            class: "$class".into(),
            mixin: Some(self.mixin),
            fields: self.fields,
            fns: self.fns,
            impls: self.impls,
            ons: self.ons,
            kind: ClassKind::MixinInstance,
        }
    }
    fn expand_pub_apis<const VIRTUAL: bool>(
        &self,
    ) -> impl Iterator<Item = ExpandPubApi<'_, VIRTUAL>> {
        self.fns
            .iter()
            .filter(|entry| match entry.kind {
                FnEntryKind::Method { body, .. } => VIRTUAL || body.is_some(),
                FnEntryKind::Ctor { .. } => false,
                FnEntryKind::Function { .. } => VIRTUAL,
            })
            .map(|entry| ExpandPubApi {
                config: self.config,
                entry,
                class_kind: ClassKind::MixinInstance,
                mixin: Some(self.mixin),
            })
    }
    fn vtable_struct_fields(&self) -> VtableStructFields<'_> {
        let fields = self
            .fns
            .iter()
            .filter_map(|entry| entry.expand_vtable_field_def::<false>(ClassKind::MixinInstance))
            .map(|field| field.name)
            .collect();
        VtableStructFields {
            class: "$class".into(),
            kind: ClassKind::MixinInstance,
            super_ty: self.fields.super_ty,
            fields,
            impls: self.impls,
        }
    }
}

struct ExpandDelegateCtor<'a> {
    self_span: Span,
    brace: syn::token::Brace,
    super_field: Option<Span>,
    super_ty: SuperTy<'a>,
    fields: &'a FieldEntries<'a>,
    kind: ClassKind,
}

impl ExpandDelegateCtor<'_> {
    fn expand_field_existance<'a>(
        &'a self,
        fields: &'a [&'a syn::Ident],
    ) -> ExpandFieldExistance<'a> {
        ExpandFieldExistance {
            self_span: self.self_span,
            brace: self.brace,
            super_field: self.super_field,
            super_ty: self.super_ty,
            fields,
        }
    }
}

impl ToTokens for ExpandDelegateCtor<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let &ExpandDelegateCtor { fields, kind, .. } = self;
        let (attrs, body) = match kind {
            ClassKind::AbstractMixin => return,
            ClassKind::MixinInstance => {
                let (uninit_fields, fields): (Vec<_>, TokenStream) =
                    fields.expand_write_fields(None).unzip();
                let check_fields_existance = self.expand_field_existance(&uninit_fields);
                let body = quote! (
                    #[allow(unused_unsafe)]
                    unsafe { #fields }
                    #check_fields_existance
                    let should_delegate = !::classes::class_const_eq!(
                        <_S::Class as ::classes::class::ClassImpl>::Vtable,
                        ::classes::prelude::CVtable<Super>,
                    );
                    if should_delegate {
                        let _self = Super::_delegate_ctor::<_S, _F>(_self.into_super(), new);
                        unsafe { _self.into_subclass_unchecked::<::classes::prelude::CRc<Self>>() }
                    } else {
                        let _self = new(_self.into_superclass());
                        unsafe { ::classes::class::ClassRc::into_subclass_unchecked::<::classes::prelude::CRc<Self>>(_self) }
                    }
                );
                (quote!(#[inline]), body)
            }
            _ => (
                quote!(#[cold] #[inline(never)]),
                quote!(let _ = new; panic!("unsupported")),
            ),
        };
        quote_each_token! { tokens
            #attrs
            pub fn _delegate_ctor<
                _S: ::classes::class::IsClass,
                _F: FnOnce(::classes::prelude::CRcUninit<_S>) -> ::classes::prelude::CRc<_S>,
            >(mut _self: ::classes::prelude::CRcUninit<Self>, new: _F) -> ::classes::prelude::CRc<Self>
            where
                ::classes::prelude::CRc<_S>: ::classes::class::ClassRc,
                for<'a> &'a ::classes::prelude::CRc<_S>: From<
                    &'a ::classes::ptr::RcDyn<
                        <::classes::prelude::CRc<_S> as ::classes::class::IsClass>::Class,
                    >,
                >,
            {
                #body
            }
        }
    }
}

struct ExpandFnImpl<'a> {
    fields: &'a FieldEntries<'a>,
    entry: &'a FnEntry<'a>,
    kind: ClassKind,
}

impl ToTokens for ExpandFnImpl<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let &FnEntry {
            vis,
            kw_unsafe,
            abi,
            name,
            param_span,
            params,
            ..
        } = self.entry;
        // eprintln!("fn {name}: {:#?}", self.entry.kind);
        let (vis, this, ret, body) = match self.entry.kind {
            FnEntryKind::Method { .. } | FnEntryKind::Ctor { .. } if self.kind.is_mixin() => return,
            FnEntryKind::Method { body: None, .. } => return,
            FnEntryKind::Method {
                self_kind: SelfKind::RefSelf(span),
                override_mode,
                ret,
                body: Some(body),
                body_span,
            } => {
                let this = quote_spanned! {span => _self: &::classes::prelude::CRc<Self>, };
                let ret = ret.map(|ret| quote!( -> #ret));
                let body = body.expand(override_mode);
                let body = quote_spanned!(body_span=> { #body });
                (quote_spanned!(vis.span()=> pub(super)), this, ret, body)
            }
            FnEntryKind::Function { ret, body } => {
                let ret = ret.map(|ret| quote!( -> #ret));
                let body = body.into_token_stream();
                (quote_spanned!(vis.span()=> pub(super)), quote!(), ret, body)
            }
            FnEntryKind::Ctor {
                ret_span,
                ctor_body,
                method_body,
                body_span,
                ..
            } => {
                let ret = Some(quote_spanned! {ret_span=> -> ::classes::prelude::CRc<Self>});
                let body = ctor_body.expand(self.fields);
                let method_body = match method_body {
                    None => quote!(),
                    Some(method_body) => method_body.expand(None).into_token_stream(),
                };
                let body = quote_spanned!(body_span=> {
                    #body
                    #method_body
                });
                let this = quote!(mut _self: ::classes::prelude::CRcUninit<Self>,);
                let vis = vis.expand::<2>().into_token_stream();
                (vis, this, ret, body)
            }
        };
        let params = quote_spanned! {param_span.join()=> (#this #params) };
        quote_each_token! {
            tokens
            #vis #kw_unsafe #abi fn #name #params #ret #body
        }
    }
}

impl ToTokens for Param {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let Self {
            attrs,
            tk_mut,
            name,
            ty,
            ..
        } = self;
        quote_each_token! {
            tokens
            #(#attrs)* #tk_mut #name: #ty
        }
    }
}

struct FnEntry<'a> {
    class: &'a syn::Ident,
    super_ty: SuperTy<'a>,
    attrs: &'a [syn::Attribute],
    vis: &'a Visibility,
    kw_unsafe: &'a Option<Token![unsafe]>,
    abi: &'a Option<syn::Abi>,
    kw_fn: Token![fn],
    name: &'a syn::Ident,
    param_span: DelimSpan,
    params: &'a Punctuated<Param, Token![,]>,
    kind: FnEntryKind<'a>,
}

enum FnEntryKind<'a> {
    Method {
        self_kind: SelfKind,
        override_mode: Option<OverrideMode<'a>>,
        ret: Option<&'a syn::Type>,
        body: Option<&'a MethodFnBodyTokenStream>,
        body_span: Span,
    },
    Function {
        ret: Option<&'a syn::Type>,
        body: &'a syn::Block,
    },
    Ctor {
        ret_span: Span,
        ctor_body: &'a CtorFnBodyTokenStream,
        method_body: Option<&'a MethodFnBodyTokenStream>,
        body_span: Span,
    },
}

impl std::fmt::Debug for FnEntryKind<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FnEntryKind::Method {
                self_kind,
                override_mode,
                ret,
                body,
                body_span,
            } => f
                .debug_struct("Method")
                .field("self_kind", self_kind)
                .field("override_mode", override_mode)
                .field("ret", &ret.map(|ty| ty.span()))
                .field("body", &body.map_or("{}", |_| "{ ... }"))
                .field("body_span", body_span)
                .finish(),
            FnEntryKind::Function { ret, body } => f
                .debug_struct("Function")
                .field("ret", &ret.map(|ty| ty.span()))
                .field("body", &"{ ... }")
                .field("body_span", &body.span())
                .finish(),
            FnEntryKind::Ctor {
                ret_span,
                body_span,
                ..
            } => f
                .debug_struct("Ctor")
                .field("ret_span", ret_span)
                .field("body_span", body_span)
                .field("body", &"{ ... }")
                .finish(),
        }
    }
}

impl FnEntry<'_> {
    #[inline]
    fn override_mode(&self) -> Option<OverrideMode<'_>> {
        match self.kind {
            FnEntryKind::Method { override_mode, .. } => override_mode,
            _ => None,
        }
    }
    #[inline]
    fn is_virtual_of_self(&self) -> bool {
        matches!(
            self.kind,
            FnEntryKind::Method {
                override_mode: None,
                ..
            }
        )
    }
    #[inline]
    fn is_ctor(&self) -> bool {
        matches!(self.kind, FnEntryKind::Ctor { .. })
    }
    #[inline]
    fn is_method(&self) -> bool {
        matches!(self.kind, FnEntryKind::Method { .. })
    }
    #[inline]
    fn expand_vtable_field_def<const OPT: bool>(
        &self,
        class_kind: ClassKind,
    ) -> Option<ExpandVtableFieldDef<'_, OPT>> {
        match self.kind {
            FnEntryKind::Method {
                self_kind,
                override_mode: None,
                ret,
                ..
            } => Some(ExpandVtableFieldDef {
                class: self.class,
                vis: self.vis,
                kw_unsafe: self.kw_unsafe,
                abi: self.abi,
                name: self.name,
                self_kind,
                param_span: self.param_span,
                params: self.params,
                ret,
                class_kind,
            }),
            FnEntryKind::Method { .. }
            | FnEntryKind::Function { .. }
            | FnEntryKind::Ctor { .. } => None,
        }
    }
    #[inline]
    fn expand_vtable_init<'a>(
        &'a self,
        mixin: Option<&'a syn::Ident>,
    ) -> Option<ExpandVtableInit<'a>> {
        match self.kind {
            FnEntryKind::Method {
                body: Some(_), ret, ..
            } => Some(ExpandVtableInit {
                super_ty: self.super_ty,
                mixin,
                override_mode: self.override_mode(),
                name: self.name,
                params: self.params,
                ret,
            }),
            _ => None,
        }
    }
}

struct ExpandVtableFieldDef<'a, const OPT: bool> {
    class: &'a syn::Ident,
    vis: &'a Visibility,
    kw_unsafe: &'a Option<Token![unsafe]>,
    abi: &'a Option<syn::Abi>,
    name: &'a syn::Ident,
    self_kind: SelfKind,
    param_span: DelimSpan,
    params: &'a Punctuated<Param, Token![,]>,
    ret: Option<&'a syn::Type>,
    class_kind: ClassKind,
}

impl<const OPT: bool> ToTokens for ExpandVtableFieldDef<'_, OPT> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let &Self {
            class,
            vis,
            name,
            kw_unsafe,
            abi,
            self_kind,
            param_span,
            params,
            ret,
            class_kind,
        } = self;
        let params = params.iter().map(Param::expand_fn_ptr_param);
        let name_span = name.span();
        let vis = if class_kind.is_mixin() {
            quote!(pub)
        } else if OPT {
            // `mod _{class}::vtable::opt`
            vis.expand::<3>().into_token_stream()
        } else {
            // `mod _{class}::vtable`
            vis.expand::<2>().into_token_stream()
        };
        quote_each_token_spanned! {
            tokens name_span
            #vis #name:
        };
        let self_param = match self_kind {
            SelfKind::RefSelf(span) if class_kind.is_mixin_instance() => {
                if OPT {
                    quote_spanned!(span => &::classes::prelude::CRc<super::super::super::#class>)
                } else {
                    quote_spanned!(span => &::classes::prelude::CRc<super::super::#class>)
                }
            }
            SelfKind::RefSelf(span) => quote_spanned!(span => &::classes::prelude::CRc<Self>),
        };
        let params = quote_spanned!(param_span.join()=> (#self_param, #(#params,)*));
        let ret_type = ret.as_slice();
        if OPT {
            let option_open = quote_spanned!(name_span=> ::core::option::Option<);
            let close_span = ret
                .map(|ret| ret.span())
                .unwrap_or_else(|| param_span.close());
            // let close_span = param_span.close();
            let option_close = quote_spanned!(close_span=> >);
            quote_each_token! {
                tokens
                #option_open #kw_unsafe #abi fn #params #( -> #ret_type )* #option_close,
            };
        } else {
            quote_each_token! {
                tokens
                #kw_unsafe #abi fn #params #( -> #ret_type )*,
            };
        }
    }
}

impl Param {
    fn expand_fn_ptr_param(&self) -> ExpandFnPtrParam<'_> {
        ExpandFnPtrParam { param: self }
    }
}

struct ExpandFnPtrParam<'a> {
    param: &'a Param,
}

impl ToTokens for ExpandFnPtrParam<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let Param { name, ty, .. } = self.param;
        quote_each_token! {
            tokens
            #name: #ty
        };
    }
}

struct ExpandVtableImpl<'a> {
    class: ClassIdent<'a>,
    mixin: Option<&'a syn::Ident>,
    fields: &'a FieldEntries<'a>,
    fns: &'a [FnEntry<'a>],
    ons: Option<&'a [MixinOn<'a>]>,
    impls: Option<&'a Punctuated<syn::Ident, Token![,]>>,
    kind: ClassKind,
}

impl ToTokens for ExpandVtableImpl<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let class = self.class;
        let super_ty = self.fields.super_ty;
        let default = ExpandSuperOrHeader::<DefaultExpr>::new(super_ty, self.kind);
        let init_header = ExpandSuperOrHeader::<InitHeader>::new(super_ty, self.kind);
        let init_mixin_header = ExpandSuperOrHeader::with_mode(
            super_ty,
            self.kind,
            InitMixinHeader::new(self.mixin.map(|mixin| (class, mixin))),
        );
        let init = ExpandSuperOrHeader::<Init>::new(super_ty, self.kind);
        let init_fields = self
            .fns
            .iter()
            .filter_map(|entry| entry.expand_vtable_init(self.mixin));
        let assert_init = ExpandSuperOrHeader::<AssertInit>::new(super_ty, self.kind);
        let vtable_fields = self.vtable_fields().collect::<Vec<_>>();
        let on_impls = self.ons().map(|on| {
            if self.mixin.is_some() {
                quote_spanned!(on.span()=>
                    ::classes::static_assert_subclass_or_mixin_instance!(
                        super::#class,
                        ::classes::prelude::CVtable<#on>,
                        ", we only support `mixin` to `on` a superclass yet"
                    );
                )
            } else {
                quote!()
            }
        });
        let impls = self.impls().collect::<Vec<_>>();

        let vtable_field_defaults = vtable_fields.iter().map(|field| {
            quote_spanned! { field.span()=> #field: ::core::option::Option::None }
        });

        let class_or_mixin = self.mixin.map_or(self.class.to_token_stream(), |mixin| {
            mixin.to_token_stream()
        });
        let vtable_field_inits = vtable_fields.iter().map(|field| {
            quote_spanned! {
                field.span()=>
                #field: self.#field.expect(concat!(
                    "cannot instantiate because method `",
                    stringify!(#class_or_mixin),
                    "::",
                    stringify!(#field),
                    "` is not implemented",
                ))
            }
        });
        quote_each_token! {
            tokens
            impl #class {
                pub const DEFAULT: Self = Self {
                    #default
                    #(#vtable_field_defaults,)*
                    #(#impls: ::classes::prelude::CVtableOpt::<#impls>::DEFAULT,)*
                };

                pub const fn init_mixin_header(
                    mixin_header: &mut [::core::mem::MaybeUninit<::classes::vtable::MixinVtableHeader>],
                ) {
                    #init_mixin_header
                }

                pub const fn init_header(&mut self, ty: ::core::option::Option<::classes::vtable::Type>, offset: usize) {
                    let ty = match ty {
                        ::core::option::Option::None => Self::TYPE,
                        ::core::option::Option::Some(ty) => ty,
                    };
                    #init_header
                    #( self.#impls.init_header(
                        ::core::option::Option::None,
                        offset + ::core::mem::offset_of!(::classes::prelude::CVtable<Self>, #impls),
                    ); )*
                }

                #[allow(unused_unsafe)]
                pub const fn init<V: ::classes::class::ClassVtableOpt>(_self: &mut V) {
                    #( #on_impls )*
                    #init
                    #( #init_fields )*
                }

                #[track_caller]
                pub const fn assert_init(self) -> ::classes::prelude::CVtable<Self> {
                    ::classes::prelude::CVtable::<Self> {
                        #assert_init
                        #( #vtable_field_inits, )*
                        #( #impls: self.#impls.assert_init(), )*
                    }
                }
            }
        }
    }
}

impl ExpandVtableImpl<'_> {
    fn ons(&self) -> impl Iterator<Item = MixinOn<'_>> {
        self.ons.iter().copied().flatten().copied()
    }
    fn impls(&self) -> impl Iterator<Item = &syn::Ident> {
        self.impls
            .iter()
            .flat_map(|impls| impls.pairs())
            .map(|pair| pair.into_value())
    }
    fn vtable_fields(&self) -> impl Iterator<Item = &syn::Ident> {
        self.fns
            .iter()
            .filter(|entry| entry.is_virtual_of_self())
            .map(|entry| entry.name)
    }
}

struct ExpandVtableInit<'a> {
    super_ty: SuperTy<'a>,
    mixin: Option<&'a syn::Ident>,
    override_mode: Option<OverrideMode<'a>>,
    name: &'a syn::Ident,
    params: &'a Punctuated<Param, Token![,]>,
    ret: Option<&'a syn::Type>,
}

impl ToTokens for ExpandVtableInit<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        // use OverrideBase::*;
        use OverrideMode::*;
        use OverrideSingle::*;
        let &Self {
            super_ty,
            mixin,
            override_mode,
            name,
            params,
            ret,
        } = self;
        let params = params
            .pairs()
            .map(|pair| &pair.into_value().name)
            .collect::<Vec<_>>();

        let mut do_override = |supertype: TokenStream| {
            // If the return type has an implicit lifetime, it should only
            // override the superclass (i.e., offset is zero), where `as_superclass()`
            // is available to get the superclass.
            if ret.is_some_and(|ret| ret.has_implicit_lifetime()) {
                let assert = quote_spanned! { name.span()=>
                    assert!(offset == 0, "cannot override a method with an implicit lifetime on a non-superclass")
                };
                quote_each_token! { tokens
                    let (ptr, offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<#supertype>>(_self);
                    #assert;
                    ptr.#name = ::core::option::Option::Some(|this, #(#params,)* | ::classes::prelude::CData::<Self>::#name(
                        unsafe { this.as_subclass_unchecked() },
                        #(#params,)*
                    ).into());
                }
            } else {
                let fn_ptr = quote_spanned! { name.span()=>
                    ::core::option::Option::Some(|this, #(#params,)* | ::classes::prelude::CData::<Self>::#name(
                        &unsafe { this.try_to_subtype().unwrap_unchecked() },
                        #(#params,)*
                    ).into())
                };
                let upcast_mut = quote_spanned! { supertype.span()=>
                    ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<#supertype>>(_self)
                };
                let upcast_mut_next = quote_spanned! { supertype.span()=>
                    ::classes::vtable::vtable_opt_upcast_mut_next::<_, ::classes::prelude::CVtableOpt<#supertype>>(_self, &mut offset)
                };
                quote_each_token! { tokens
                    {
                        // let (_self, _offset) = ::classes::vtable::vtable_opt_upcast_mut::<_, ::classes::prelude::CVtableOpt<Self>>(_self);
                        let (ptr, mut offset) = #upcast_mut;
                        ptr.#name = #fn_ptr;
                        while let Some(ptr) = #upcast_mut_next {
                            ptr.#name = #fn_ptr;
                        }
                    }
                }
            }
        };
        match override_mode {
            None if super_ty.is_mixin() => {
                do_override(mixin.expect("expected mixin").to_token_stream());
            }
            // None if super_ty.is_mixin() => {}
            None => do_override(quote!(Self)),
            Some(Single(Super)) => do_override(quote!(Super)),
            Some(Single(
                Supertype(supertype)
                | On(MixinOn::Class(supertype) | MixinOn::Mixin(supertype))
                | SelfInterface(supertype)
                | Mixin(supertype),
            )) => {
                do_override(supertype.to_token_stream());
            }
            Some(Multi { interfaces, .. }) => {
                let supertype = interfaces.last().unwrap().interface_superclass();
                do_override(supertype.to_token_stream());
            }
        }
    }
}

impl MethodFnBodyTokenStream {
    fn expand<'a>(&'a self, override_mode: Option<OverrideMode<'a>>) -> ExpandMethodFnBody<'a> {
        ExpandMethodFnBody {
            tokens: &self.tokens,
            override_mode,
        }
    }
}

struct ExpandMethodFnBody<'a> {
    tokens: &'a [MethodFnBodyTokenTree],
    override_mode: Option<OverrideMode<'a>>,
}

impl ToTokens for ExpandMethodFnBody<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        use MethodFnBodyTokenTree::*;
        for token in self.tokens {
            match token {
                Item(item) => item.to_tokens(tokens),
                SelfValue(self_value) => {
                    // `self` => `_self`
                    let span = self_value.span;
                    quote_each_token_spanned!(tokens span _self);
                }
                SuperValue(super_value) => {
                    // `super` => `_self.delegate_super()`
                    let span = super_value.span;
                    quote_each_token_spanned!(tokens span { _self.delegate_super() });
                }
                DotSuper(_, super_value) => {
                    // `.super` => `.as_super()`
                    let span = super_value.span;
                    quote_each_token_spanned!(tokens span .as_super());
                }
                Group(group) => {
                    let mut new_group = proc_macro2::Group::new(
                        group.delimiter,
                        group.tokens.expand(self.override_mode).into_token_stream(),
                    );
                    new_group.set_span(group.span.join());
                    new_group.to_tokens(tokens)
                }
                Ident(ident) => ident.to_tokens(tokens),
                Punct(punct) => punct.to_tokens(tokens),
                Literal(literal) => literal.to_tokens(tokens),
            }
        }
    }
}

impl FnBodyTokenStream<CtorFnBodyTokenTree> {
    fn expand<'a>(&'a self, fields: &'a FieldEntries<'a>) -> ExpandCtorFnBody<'a> {
        ExpandCtorFnBody {
            tokens: &self.tokens,
            fields,
        }
    }
}

struct ExpandCtorFnBody<'a> {
    tokens: &'a [CtorFnBodyTokenTree],
    fields: &'a FieldEntries<'a>,
}

impl ExpandCtorFnBody<'_> {
    fn expand_self_struct_expr<'a>(
        &'a self,
        self_ty: Token![Self],
        brace: syn::token::Brace,
        super_ty: SuperTy<'a>,
        expr: &'a SelfStructExpr,
    ) -> ExpandSelfStructExpr<'a> {
        ExpandSelfStructExpr {
            self_ty,
            brace,
            super_ty,
            expr,
            fields: self.fields,
        }
    }
}

impl ToTokens for ExpandCtorFnBody<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        use CtorFnBodyTokenTree::*;
        for token in self.tokens {
            match token {
                Item(item) => item.to_tokens(tokens),
                &SelfStructExpr(self_ty, brace, ref self_struct_expr) => {
                    let self_struct_expr = self.expand_self_struct_expr(
                        self_ty,
                        brace,
                        self.fields.super_ty,
                        self_struct_expr,
                    );
                    quote_each_token!(tokens unsafe { #self_struct_expr } );
                }
                LetSelfExpr(let_self_expr) => {
                    let &crate::syntax::LetSelfExpr {
                        tk_self_type,
                        tk_let,
                        tk_self,
                        tk_eq,
                        tk_semi,
                        ..
                    } = let_self_expr;
                    let _self = format_ident!("_self", span = tk_self.span());
                    let self_ty =
                        quote_spanned!(tk_self_type.span()=> ::classes::prelude::CRc<Self>);
                    quote_each_token!(tokens #tk_let #_self: #self_ty #tk_eq);
                    match &let_self_expr.kind {
                        &LetSelfExprKind::Struct(brace, ref self_struct_expr) => {
                            let self_struct_expr = self.expand_self_struct_expr(
                                tk_self_type,
                                brace,
                                self.fields.super_ty,
                                self_struct_expr,
                            );
                            let span = brace.span.join();
                            quote_each_token_spanned!(tokens span unsafe { #self_struct_expr });
                        }
                        LetSelfExprKind::Method(path_sep, ident, paren, token_stream) => {
                            quote_each_token!(tokens #tk_self_type #path_sep #ident);
                            let _self = format_ident!("_self", span = tk_self.span);
                            let paren_span = paren.span.join();
                            quote_each_token_spanned!(tokens paren_span ( #_self, #token_stream ));
                        }
                    }
                    quote_each_token!(tokens #tk_semi);
                }
                Group(group) => proc_macro2::Group::new(
                    group.delimiter,
                    group.tokens.expand(self.fields).into_token_stream(),
                )
                .to_tokens(tokens),
                Ident(ident) => ident.to_tokens(tokens),
                Punct(punct) => punct.to_tokens(tokens),
                Literal(literal) => literal.to_tokens(tokens),
            }
        }
    }
}

struct ExpandSelfStructExpr<'a> {
    self_ty: Token![Self],
    brace: syn::token::Brace,
    super_ty: SuperTy<'a>,
    expr: &'a SelfStructExpr,
    fields: &'a FieldEntries<'a>,
}

impl ExpandSelfStructExpr<'_> {
    fn expand_field_existance<'a>(
        &'a self,
        fields: &'a [&'a syn::Ident],
    ) -> ExpandFieldExistance<'a> {
        ExpandFieldExistance {
            self_span: self.self_ty.span,
            brace: self.brace,
            super_field: self
                .expr
                .super_field
                .as_ref()
                .map(|field| field.ident.span()),
            super_ty: self.super_ty,
            fields,
        }
    }
}

fn write_field(field: &syn::Ident, expr: TokenStream) -> TokenStream {
    quote!(
        ::core::ptr::write(
            &raw mut (*_self.as_mut_ptr()).#field,
            #expr,
        );
    )
}

impl ToTokens for ExpandSelfStructExpr<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let mut fields = Vec::new();
        let mut seen_fields = HashSet::new();
        for field in self.expr.fields.pairs().map(|pair| pair.into_value()) {
            let field_ident = &field.ident;
            seen_fields.insert(field_ident);
            let expr = self
                .fields
                .get_field(field_ident)
                .and_then(|entry| match &field.expr {
                    Some(ColonExpr { expr, .. }) => entry.wrap_init(&expr),
                    None => entry.wrap_init(field_ident),
                })
                .unwrap_or_else(syn::Error::into_compile_error);
            fields.push(field_ident);
            write_field(field_ident, expr).to_tokens(tokens);
        }
        // fill in missing fields
        if let Some(_) = self.expr.dot_dot {
            fields.extend(self.fields.expand_write_fields(Some(&seen_fields)).map(
                |(field, expr)| {
                    expr.to_tokens(tokens);
                    field
                },
            ));
        }

        let check_fields_existance = self.expand_field_existance(&fields);
        quote_each_token!(tokens #check_fields_existance);

        if let Some(super_field) = &self.expr.super_field {
            let ColonCallExpr {
                colon: _,
                super_ty,
                tk_path_sep,
                method,
                tk_paren,
                token_stream,
                chained_calls,
            } = &super_field.expr;
            let _super = quote_spanned!(super_field.ident.span => _self.into_super());
            if let Mixined(_super, _) = self.super_ty {
                let delegate_ctor = quote_spanned!(method.span() => _delegate_ctor);
                quote_each_token!(tokens #super_ty #tk_path_sep #delegate_ctor::<#_super, _>(_self.into_super(), |_self| {
                    ::classes::prelude::CData::<#_super>::#method(_self, #token_stream)
                        #( #chained_calls )*
                }) );
            } else {
                quote_each_token!(tokens #super_ty #tk_path_sep #method );
                tk_paren.surround(tokens, |mut tokens| {
                    let super_span = super_field.ident.span;
                    quote_each_token_spanned!(tokens super_span _self.into_super());
                    quote_each_token!(tokens , #token_stream);
                });
                quote_each_token!(tokens #(#chained_calls)*);
            }
            let span = self.self_ty.span;
            quote_each_token_spanned!(tokens span
                .into_subclass_unchecked()
            );
        } else if let Object(span) = self.super_ty {
            quote_each_token_spanned!(tokens span
            ::classes::prelude::CData::<::classes::object::Object>::new(_self.into_super())
                .into_subclass_unchecked()
            );
        } else {
            let span = self.self_ty.span;
            quote_each_token_spanned!(tokens span
                ::classes::prelude::CRc::<Self>::_from_inner(_self.assume_init())
            );
        }
    }
}

struct ExpandFieldExistance<'a> {
    self_span: Span,
    brace: syn::token::Brace,
    super_field: Option<Span>,
    super_ty: SuperTy<'a>,
    fields: &'a [&'a syn::Ident],
}

impl ToTokens for ExpandFieldExistance<'_> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let self_ty = quote_spanned!(self.self_span => Self);
        let fields = self.fields;
        quote_each_token!(tokens let _ = |#self_ty);
        self.brace.surround(tokens, |mut tokens| {
            if let Some(span) = self.super_field {
                quote_each_token_spanned!(tokens span _super,);
            } else if let Object(span) = self.super_ty {
                quote_each_token_spanned!(tokens span _super,);
            }
            let fields = fields
                .iter()
                .map(|field| quote_spanned!(field.span()=> #field: _,));
            quote_each_token!(tokens #(#fields)*);
        });
        quote_each_token!(tokens : #self_ty| (););
    }
}

struct ExpandPubApi<'a, const VIRTUAL: bool> {
    #[expect(dead_code)]
    config: &'a Config,
    entry: &'a FnEntry<'a>,
    class_kind: ClassKind,
    mixin: Option<&'a syn::Ident>,
}

impl<const VIRTUAL: bool> ToTokens for ExpandPubApi<'_, VIRTUAL> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        let &FnEntry {
            class,
            attrs,
            vis,
            kw_unsafe,
            abi,
            kw_fn,
            name,
            param_span,
            params,
            ..
        } = self.entry;
        let vis = vis.expand::<2>();
        let param_fields = params
            .pairs()
            .map(|pair| &pair.into_value().name)
            .collect::<Vec<_>>();
        let overriden_body = |supertype: TokenStream, ret: Option<&syn::Type>, body_span: Span| {
            if ret.is_some_and(|ret| ret.has_implicit_lifetime()) {
                quote_spanned! {body_span=> {#kw_unsafe {
                    self.as_superclass::<::classes::prelude::CRc<#supertype>>().#name(#(#param_fields,)*)
                }.try_into().unwrap()} }
            } else {
                quote_spanned! {body_span=> {#kw_unsafe {
                    self.to_supertype::<::classes::prelude::CRc<#supertype>>().#name(#(#param_fields,)*)
                }.try_into().unwrap()} }
            }
        };
        let (self_param, ret, body) = if self.class_kind.is_mixin() {
            match &self.entry.kind {
                FnEntryKind::Function { ret, body } => {
                    let body = quote_spanned! {body.span()=> {#kw_unsafe {
                        data::#class::#name(#(#param_fields,)*)
                    }} };
                    (None, ret.map(|ret| quote!(-> #ret)), body)
                }
                &FnEntryKind::Method {
                    self_kind: SelfKind::RefSelf(self_span),
                    override_mode,
                    ret,
                    body_span,
                    ..
                } => {
                    use OverrideBase::*;
                    use OverrideMode::*;
                    use OverrideSingle::*;
                    let self_param = Some(quote_spanned!(self_span=> &self,));
                    debug_assert!(VIRTUAL);
                    let body = match override_mode {
                        None => {
                            quote_spanned! {body_span=> {#kw_unsafe {
                                (self.0.vtable().vtable_without_super().#name)(self, #(#param_fields,)*)
                            }} }
                        }
                        Some(Single(Super | Supertype(_))) => {
                            syn::Error::new(self.entry.name.span(), CheckError::MissingSuperclass)
                                .to_compile_error()
                        }
                        Some(Single(
                            SelfInterface(supertype)
                            | On(MixinOn::Mixin(supertype) | MixinOn::Class(supertype)),
                        )) => overriden_body(supertype.to_token_stream(), ret, body_span),
                        Some(Multi {
                            base: SelfClass | SelfInterfaceBase(_) | MixinBase(_),
                            interfaces,
                            ..
                        }) => {
                            let supertype = interfaces.last().unwrap().interface_superclass();
                            overriden_body(supertype.to_token_stream(), ret, body_span)
                        }
                        Some(Multi {
                            base: SuperBase(span),
                            ..
                        }) => syn::Error::new(span, CheckError::UnexpectedSuper).to_compile_error(),
                        Some(Single(Mixin(_))) => unreachable!(),
                        Some(Multi {
                            base: SuperclassBase(superclass),
                            ..
                        }) => syn::Error::new(
                            superclass.span(),
                            CheckError::UnexpectedSuperclass(superclass),
                        )
                        .to_compile_error(),
                    };
                    let ret = ret.map(|ret| quote!(-> #ret));
                    (self_param, ret, body)
                }
                FnEntryKind::Ctor { .. } => return,
            }
        } else {
            match &self.entry.kind {
                &FnEntryKind::Method {
                    self_kind: SelfKind::RefSelf(self_span),
                    override_mode,
                    ret,
                    body_span,
                    ..
                } => {
                    use OverrideMode::*;
                    use OverrideSingle::*;
                    let self_param = Some(quote_spanned!(self_span=> &self,));
                    let body = {
                        if !VIRTUAL {
                            quote_spanned! {body_span=> {#kw_unsafe {
                                ::classes::prelude::CData::<Self>::#name(self.as_virtual(), #(#param_fields,)*)
                            }} }
                        } else {
                            match override_mode {
                                None if self.class_kind.is_mixin_instance() => {
                                    let mixin = self.mixin.expect("expected mixin");
                                    overriden_body(mixin.to_token_stream(), ret, body_span)
                                }
                                None => {
                                    quote_spanned! {body_span=> {#kw_unsafe {
                                        (self.0.vtable().#name)(self, #(#param_fields,)*)
                                    }} }
                                }
                                Some(Single(Super | On(_) | Mixin(_))) => {
                                    quote_spanned! {body_span=> {#kw_unsafe {
                                        self.as_super().#name(#(#param_fields,)*)
                                    }.try_into().unwrap()} }
                                }
                                Some(Single(Supertype(supertype) | SelfInterface(supertype))) => {
                                    overriden_body(supertype.to_token_stream(), ret, body_span)
                                }
                                Some(Multi { interfaces, .. }) => {
                                    let supertype =
                                        interfaces.last().unwrap().interface_superclass();
                                    overriden_body(supertype.to_token_stream(), ret, body_span)
                                }
                            }
                        }
                    };
                    let ret = ret.map(|ret| quote!(-> #ret));
                    (self_param, ret, body)
                }
                FnEntryKind::Function { .. } if self.class_kind.is_mixin_instance() => return,
                FnEntryKind::Function { ret, body } => {
                    let body = quote_spanned! {body.span()=> {#kw_unsafe {
                        ::classes::prelude::CData::<Self>::#name(#(#param_fields,)*)
                    }} };
                    (None, ret.map(|ret| quote!(-> #ret)), body)
                }
                &FnEntryKind::Ctor {
                    ret_span,
                    body_span,
                    ..
                } => {
                    let body = quote_spanned! {body_span=> {
                        ::classes::prelude::CData::<Self>::#name(
                            ::classes::prelude::CRcUninit::<Self>::new_uninit(),
                            #(#param_fields,)*
                        )
                    }};
                    (None, Some(quote_spanned! (ret_span=> -> Self)), body)
                }
            }
        };
        let params = quote_spanned!(param_span.join() => (#self_param #params ));
        let vis = if self.class_kind.is_mixin() {
            quote!(pub)
        } else {
            vis.to_token_stream()
        };

        let attrs = attrs.iter().filter(|attr| passed_attr(attr));
        quote_each_token! {
            tokens
            #( #attrs )*
            #[inline]
            #vis #kw_unsafe #abi #kw_fn #name #params #ret #body
        }
    }
}

fn passed_attr(attr: &syn::Attribute) -> bool {
    const ELIMINATED_ATTRS: &[&str] = &[FN_CTOR_HELPER_KEY, FN_HELPER_KEY];
    attr.path()
        .get_ident()
        .is_none_or(|p| !ELIMINATED_ATTRS.contains(&p.to_string().as_str()))
}

struct ExpandVisibility<'a, const DEPTH: usize> {
    vis: &'a Visibility,
}

impl<const DEPTH: usize> ToTokens for ExpandVisibility<'_, DEPTH> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        if DEPTH == 0 {
            self.vis.to_tokens(tokens);
            return;
        }
        use VisPathAlone::*;
        use VisPathBegin::*;
        use Visibility::*;
        let prefix = (1..DEPTH).map(|_| quote!(super::));
        match self.vis {
            Pub(_)
            | PubAlone(.., CratePath(_))
            | PubPath(.., Global(_) | Special(CratePath(_), _), _) => {
                self.vis.to_tokens(tokens);
            }
            Inherited if DEPTH == 1 => {
                quote_each_token!(tokens pub (super));
            }
            Inherited => {
                quote_each_token!(tokens pub (in #(#prefix)* super));
            }
            PubAlone(.., tk_in, SelfPath(_)) if DEPTH == 1 => {
                let span = self.vis.span();
                quote_each_token_spanned!(tokens span pub (#tk_in super));
            }
            PubAlone(.., SelfPath(_)) => {
                let span = self.vis.span();
                quote_each_token_spanned!(tokens span pub (in #(#prefix)* super));
            }
            PubAlone(.., SuperPath(_)) => {
                let span = self.vis.span();
                quote_each_token_spanned!(tokens span pub (in #(#prefix)* super::super));
            }
            PubPath(.., Special(SelfPath(_), _) | Local, path) => {
                let span = self.vis.span();
                quote_each_token_spanned!(tokens span pub (in #(#prefix)* super::#path));
            }
            PubPath(.., Special(SuperPath(_), _), path) => {
                let span = self.vis.span();
                quote_each_token_spanned!(tokens span pub (in #(#prefix)* super::super::#path));
            }
        }
    }
}

trait ExpandRef {
    fn expand_ref<T: ToTokens + Default>(&self) -> TokenStream;
}

impl ExpandRef for syn::ItemConst {
    /// Expand `const CONST: Type = expr` to `const CONST: Type = T::CONST`
    /// where `T` is a token of path `self` or `super`.
    fn expand_ref<T: ToTokens + Default>(&self) -> TokenStream {
        let syn::ItemConst {
            attrs,
            vis,
            const_token,
            ident,
            generics: _, // experimental
            colon_token,
            ty,
            eq_token,
            expr: _,
            semi_token,
        } = self;
        let base = T::default();
        let expr = quote_spanned!(ident.span()=> #base::#ident);
        quote! {
            #( #attrs )*
            #vis #const_token #ident #colon_token #ty #eq_token #expr #semi_token
        }
    }
}

trait VisibilityExt {
    /// expand visibility with the depth of the relative module
    fn expand<const DEPTH: usize>(&self) -> ExpandVisibility<'_, DEPTH>;
}

impl VisibilityExt for Visibility {
    fn expand<const DEPTH: usize>(&self) -> ExpandVisibility<'_, DEPTH> {
        ExpandVisibility { vis: self }
    }
}

trait TypeExt {
    // Returns the inner `T` if it matches `outer_path<T>`
    fn inner_ty(&self, outer_path: &[&str]) -> Option<&Self>;
    fn ty_kind(&self) -> FieldTypeKind {
        if self.is_not_rc() && self.is_not_option_rc() {
            FieldTypeKind::NonRc
        } else {
            FieldTypeKind::Rc
        }
    }
    fn is_not_option_rc(&self) -> bool {
        self.inner_ty(&["core", "option", "Option"])
            .is_none_or(Self::is_not_rc)
    }
    fn is_not_rc(&self) -> bool {
        self.inner_ty(&["classes", "prelude", "RcLike"]).is_none()
            && self.inner_ty(&["classes", "prelude", "CRc"]).is_none()
            && self.inner_ty(&["classes", "prelude", "CWeak"]).is_none()
            && self.inner_ty(&["alloc", "rc", "Rc"]).is_none()
            && self.inner_ty(&["alloc", "rc", "Weak"]).is_none()
    }
}

impl TypeExt for syn::Type {
    fn inner_ty(&self, outer_path: &[&str]) -> Option<&Self> {
        let syn::Type::Path(syn::TypePath { path, qself: None }) = self else {
            return None;
        };
        let [prefix @ .., last] = outer_path else {
            return None;
        };
        fn match_crate(krate: &str, path: &syn::PathSegment) -> bool {
            path.arguments.is_none()
                && match krate {
                    "core" | "alloc" => path.ident == krate || path.ident == "std",
                    _ => path.ident == krate,
                }
        }
        fn match_module_path<'a>(
            prefix: &[&str],
            path: &'a Punctuated<syn::PathSegment, Token![::]>,
        ) -> Option<&'a syn::PathSegment> {
            if path.len() != prefix.len() + 1 {
                return None;
            }
            let [krate, rest @ ..] = prefix else {
                return None;
            };
            let mut iter = path.iter();
            if iter.next().is_none_or(|seg| !match_crate(krate, &seg)) {
                return None;
            }
            if iter
                .by_ref()
                .zip(rest)
                .all(|(seg, expected)| seg.arguments.is_none() && seg.ident == expected)
            {
                return iter.next();
            }
            None
        }
        let path = match path.leading_colon {
            _ if path.segments.len() == outer_path.len() => {
                match_module_path(prefix, &path.segments)?
            }
            Some(_) => return None,
            None => (path.segments.len() == 1).then_some(&path.segments.first()?)?,
        };
        if path.ident != last {
            return None;
        }
        if let syn::PathArguments::AngleBracketed(args) = &path.arguments {
            if args.args.len() == 1 {
                if let syn::GenericArgument::Type(ty) = args.args.first().unwrap() {
                    return Some(ty);
                }
            }
        }
        None
    }
}

trait HasImplicitLifetime {
    fn has_implicit_lifetime(&self) -> bool;
}

impl<T: HasImplicitLifetime> HasImplicitLifetime for Box<T> {
    fn has_implicit_lifetime(&self) -> bool {
        (**self).has_implicit_lifetime()
    }
}

impl<T: HasImplicitLifetime> HasImplicitLifetime for Option<T> {
    fn has_implicit_lifetime(&self) -> bool {
        self.as_ref()
            .is_some_and(|item| item.has_implicit_lifetime())
    }
}

impl HasImplicitLifetime for syn::GenericArgument {
    fn has_implicit_lifetime(&self) -> bool {
        match self {
            syn::GenericArgument::Type(ty) => ty.has_implicit_lifetime(),
            syn::GenericArgument::Lifetime(lifetime) => lifetime.ident == "_",
            syn::GenericArgument::Constraint(_) | _ => false,
        }
    }
}

impl HasImplicitLifetime for syn::QSelf {
    fn has_implicit_lifetime(&self) -> bool {
        self.ty.has_implicit_lifetime()
    }
}

impl HasImplicitLifetime for syn::Path {
    fn has_implicit_lifetime(&self) -> bool {
        self.segments
            .iter()
            .any(|segment| segment.has_implicit_lifetime())
    }
}

impl HasImplicitLifetime for syn::PathSegment {
    fn has_implicit_lifetime(&self) -> bool {
        matches!(
            &self.arguments,
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. })
                if args.iter().any(|arg| arg.has_implicit_lifetime())
        )
    }
}

impl HasImplicitLifetime for syn::TypeParamBound {
    fn has_implicit_lifetime(&self) -> bool {
        match self {
            syn::TypeParamBound::Lifetime(lifetime) if (lifetime.ident == "_") => true,
            syn::TypeParamBound::Trait(syn::TraitBound { path, .. }) => {
                path.has_implicit_lifetime()
            }
            _ => false,
        }
    }
}

impl<T: HasImplicitLifetime, P> HasImplicitLifetime for Punctuated<T, P> {
    fn has_implicit_lifetime(&self) -> bool {
        self.iter().any(|item| item.has_implicit_lifetime())
    }
}

impl HasImplicitLifetime for syn::Lifetime {
    fn has_implicit_lifetime(&self) -> bool {
        self.ident == "_"
    }
}

impl HasImplicitLifetime for syn::Type {
    fn has_implicit_lifetime(&self) -> bool {
        use syn::Type::*;
        match self {
            Array(syn::TypeArray { elem, .. })
            | Group(syn::TypeGroup { elem, .. })
            | Paren(syn::TypeParen { elem, .. })
            | Slice(syn::TypeSlice { elem, .. }) => elem.has_implicit_lifetime(),
            ImplTrait(syn::TypeImplTrait { bounds, .. })
            | TraitObject(syn::TypeTraitObject { bounds, .. }) => bounds.has_implicit_lifetime(),
            Path(syn::TypePath { path, qself }) => {
                path.has_implicit_lifetime() || qself.has_implicit_lifetime()
            }
            Reference(syn::TypeReference { lifetime, .. }) => lifetime
                .as_ref()
                .is_none_or(|lft| lft.has_implicit_lifetime()),
            Tuple(syn::TypeTuple { elems, .. }) => elems.has_implicit_lifetime(),
            BareFn(_) | Infer(_) | Macro(_) | Never(_) | Ptr(_) | Verbatim(_) | _ => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_expand_vis() {
        macro_rules! test_case {
            ($depth:literal, ($($vis:tt)*), ($($expected:tt)*)) => {{
                let vis: Visibility = syn::parse_quote!($($vis)*);
                let expected: Visibility = syn::parse_quote!($($expected)*);
                assert_eq!(
                    vis.expand::<$depth>().to_token_stream().to_string(),
                    expected.to_token_stream().to_string(),
                );
            }};
        }

        test_case!(0, (), ());
        test_case!(0, (pub (self)), (pub (self)));
        test_case!(0, (pub (super)), (pub (super)));
        test_case!(0, (pub (crate)), (pub (crate)));
        test_case!(0, (pub (in self)), (pub (in self)));
        test_case!(0, (pub (in super)), (pub (in super)));
        test_case!(0, (pub (in crate)), (pub (in crate)));
        test_case!(0, (pub (in self::path)), (pub (in self::path)));
        test_case!(0, (pub (in super::path)), (pub (in super::path)));
        test_case!(0, (pub (in crate::path)), (pub (in crate::path)));

        test_case!(1, (), (pub (super)));
        test_case!(1, (pub (self)), (pub (super)));
        test_case!(1, (pub (super)), (pub (in super::super)));
        test_case!(1, (pub (crate)), (pub (crate)));
        test_case!(1, (pub (in self)), (pub (in super)));
        test_case!(1, (pub (in super)), (pub (in super::super)));
        test_case!(1, (pub (in crate)), (pub (in crate)));
        test_case!(1, (pub (in self::path)), (pub (in super::path)));
        test_case!(1, (pub (in super::path)), (pub (in super::super::path)));
        test_case!(1, (pub (in crate::path)), (pub (in crate::path)));

        test_case!(2, (), (pub (in super::super)));
        test_case!(2, (pub (self)), (pub (in super::super)));
        test_case!(2, (pub (super)), (pub (in super::super::super)));
        test_case!(2, (pub (crate)), (pub (crate)));
        test_case!(2, (pub (in self)), (pub (in super::super)));
        test_case!(2, (pub (in super)), (pub (in super::super::super)));
        test_case!(2, (pub (in crate)), (pub (in crate)));
        test_case!(2, (pub (in self::path)), (pub (in super::super::path)));
        test_case!(2, (pub (in super::path)), (pub (in super::super::super::path)));
        test_case!(2, (pub (in crate::path)), (pub (in crate::path)));
    }

    #[test]
    fn test_has_implicit_lifetime() {
        macro_rules! test_case {
            ($kind:ty, $result:literal, $($tt:tt)*) => {{
                let item: $kind = syn::parse_quote!($($tt)*);
                assert_eq!(item.has_implicit_lifetime(), $result);
            }};
        }
        test_case!(syn::Type, false, ());
        test_case!(syn::Type, true, &());
        test_case!(syn::Type, true, &());
        test_case!(
            syn::Type,
            true,
            Pin<Box<dyn Future<Output = ImmutableBuffer> + '_>>
        );
    }
}
