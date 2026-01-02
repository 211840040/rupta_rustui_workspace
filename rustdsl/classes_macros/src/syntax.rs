use proc_macro2::{TokenStream, TokenTree};
use syn::{
    parse::{discouraged::Speculative, Lookahead1, Parse, ParseStream},
    punctuated::Punctuated,
    token, Token,
};

pub(crate) mod kw {
    syn::custom_keyword!(class);
    syn::custom_keyword!(mixin);
    syn::custom_keyword!(extends);
    syn::custom_keyword!(implements);
    syn::custom_keyword!(with);
    syn::custom_keyword!(on);
    syn::custom_keyword!(late);
    syn::custom_keyword!(mutable);
    syn::custom_keyword!(takecell);
    syn::custom_keyword!(raw);
    syn::custom_keyword!(Super);
}

#[derive(Debug, thiserror::Error)]
enum ParseError {
    #[error("expect `class` or `mixin`")]
    ExpectClassOrMixin,
    #[error("redundant `abstract` before `mixin` (not `mixin class`)")]
    RedundantAbstractMixin,
    #[error("`on` cannot be used with `class`")]
    InvalidClassOn,
    #[error("`extends` cannot be used with `mixin`")]
    InvalidMixinExtends,
    #[error("`with` cannot be used with `mixin`")]
    InvalidMixinWith,
    #[error("redundant visibility before `struct`")]
    RedundantVisibilityBeforeStruct,
    #[error("expect param")]
    ExpectParam,
    #[error("expect field")]
    ExpectField,
    #[error("expect function body for non-method functions")]
    ExpectBlock,
    #[error("maybe missing a `&self` receiver")]
    MissingSelfReceiver,
}

#[derive(Debug, thiserror::Error)]
#[error("bug: EOF")]
struct EofError;

pub struct Classes {
    pub inner_attrs: Vec<syn::Attribute>,
    pub classes_or_items: Vec<Item>,
}

impl Parse for Classes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut classes_or_items = Vec::new();
        let inner_attrs = input.call(syn::Attribute::parse_inner)?;
        while !input.is_empty() {
            let forked = input.fork();
            if let Ok(item) = forked.parse() {
                input.advance_to(&forked);
                classes_or_items.push(Item::Item(item));
                continue;
            }
            let forked = input.fork();
            if let Ok(mixin) = forked.parse() {
                input.advance_to(&forked);
                classes_or_items.push(Item::ExternMixin(mixin));
                continue;
            }
            classes_or_items.push(Item::Class(input.parse()?));
        }
        Ok(Classes {
            inner_attrs,
            classes_or_items,
        })
    }
}

impl Classes {
    pub fn classes_or_extern_mixins_mut(
        &mut self,
    ) -> impl Iterator<Item = ClassOrExternMixinMut<'_>> {
        self.classes_or_items
            .iter_mut()
            .filter_map(Item::as_class_or_extern_mixin_mut)
    }
    pub fn classes_or_extern_mixins(&self) -> impl Iterator<Item = ClassOrExternMixin<'_>> {
        self.classes_or_items
            .iter()
            .filter_map(Item::as_class_or_extern_mixin)
    }
    pub fn items(&self) -> impl Iterator<Item = &syn::Item> {
        self.classes_or_items.iter().filter_map(Item::as_item)
    }
}

pub enum Item {
    Item(syn::Item),
    Class(Class),
    ExternMixin(ExternMixin),
}

pub enum ClassOrExternMixin<'a> {
    Class(&'a Class),
    ExternMixin(&'a ExternMixin),
}

pub enum ClassOrExternMixinMut<'a> {
    Class(&'a mut Class),
    ExternMixin(&'a mut ExternMixin),
}

impl Item {
    pub fn as_class_or_extern_mixin_mut(&mut self) -> Option<ClassOrExternMixinMut<'_>> {
        match self {
            Item::Item(_) => None,
            Item::Class(class) => Some(ClassOrExternMixinMut::Class(class)),
            Item::ExternMixin(mixin) => Some(ClassOrExternMixinMut::ExternMixin(mixin)),
        }
    }
    pub fn as_class_or_extern_mixin(&self) -> Option<ClassOrExternMixin<'_>> {
        match self {
            Item::Item(_) => None,
            Item::Class(class) => Some(ClassOrExternMixin::Class(class)),
            Item::ExternMixin(mixin) => Some(ClassOrExternMixin::ExternMixin(mixin)),
        }
    }
    pub fn as_item(&self) -> Option<&syn::Item> {
        match self {
            Item::Item(item) => Some(item),
            Item::Class(_) | Item::ExternMixin(_) => None,
        }
    }
}

pub struct ClassWithAttrs {
    pub attrs: Vec<syn::Attribute>,
    pub class: Class,
}

impl Parse for ClassWithAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_inner)?;
        let class = input.parse()?;
        Ok(ClassWithAttrs { attrs, class })
    }
}

pub struct ExternMixin {
    pub attrs: Vec<syn::Attribute>,
    #[expect(dead_code)]
    pub vis: Visibility,
    #[expect(dead_code)]
    pub tk_extern: Token![extern],
    pub kw_mixin: kw::mixin,
    pub krate: syn::Ident,
    pub sep: Token![::],
    pub name: syn::Ident,
    #[expect(dead_code)]
    pub tk_semi: Token![;],
}

impl Parse for ExternMixin {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        Ok(Self {
            attrs,
            vis: input.parse()?,
            tk_extern: input.parse()?,
            kw_mixin: input.parse()?,
            krate: input.parse()?,
            sep: input.parse()?,
            // path: input.call(Punctuated::parse_separated_nonempty)?,
            name: input.parse()?,
            tk_semi: input.parse()?,
        })
    }
}

pub struct Class {
    pub attrs: Vec<syn::Attribute>,
    pub vis: Visibility,
    pub kw_abstract: Option<Token![abstract]>,
    pub kw_mixin: Option<kw::mixin>,
    pub kw_class: Option<kw::class>,
    pub extends: Option<Extends>,
    pub withs: Option<Withs>,
    pub ons: Option<Ons>,
    pub implements: Option<Implements>,
    pub name: syn::Ident,
    pub brace: token::Brace,
    pub item_struct: Option<ItemStruct>,
    pub class_items: Vec<ClassItem>,
}

fn parse_class_body(input: ParseStream<'_>) -> syn::Result<(Option<ItemStruct>, Vec<ClassItem>)> {
    let mut class_items = Vec::new();
    let attrs = input.call(syn::Attribute::parse_outer)?;
    let vis: Visibility = input.parse()?;
    let lookahead1 = input.lookahead1();
    let mut item_struct: Option<ItemStruct> = None;
    if lookahead1.peek(Token![struct]) {
        if !matches!(vis, Visibility::Inherited) {
            return Err(syn::Error::new_spanned(
                vis,
                ParseError::RedundantVisibilityBeforeStruct,
            ));
        }
        item_struct.insert(input.parse()?).attrs = attrs;
    } else if !input.is_empty() && ItemFn::peek(&lookahead1, input) {
        // eprintln!("parsing fn");
        let mut item_fn: ItemFn = input.parse()?;
        item_fn.vis = vis;
        item_fn.attrs = attrs;
        class_items.push(ClassItem::Fn(item_fn));
    } else if !input.is_empty() && lookahead1.peek(Token![const]) {
        let mut item_const: syn::ItemConst = input.parse()?;
        item_const.vis = vis.try_into()?;
        item_const.attrs = attrs;
        class_items.push(ClassItem::Const(item_const));
    }
    while !input.is_empty() {
        // eprintln!("parsing fn");
        class_items.push(input.parse()?);
    }
    // eprintln!(
    //     "parsing done: fn {:?}",
    //     class_items
    //         .iter()
    //         .filter_map(|item| item.as_item_fn())
    //         .map(|item| item.name.to_string())
    //         .collect::<Vec<_>>()
    // );
    Ok((item_struct, class_items))
}

impl Parse for Class {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if input.is_empty() {
            return Err(input.error(EofError));
        }
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let vis = input.parse()?;
        let kw_abstract: Option<Token![abstract]> = input.parse()?;
        let kw_mixin: Option<kw::mixin> = input.parse()?;
        let kw_class: Option<kw::class> = input.parse()?;
        if kw_mixin.is_none() && kw_class.is_none() {
            return Err(input.error(ParseError::ExpectClassOrMixin));
        }
        if kw_abstract.is_some() && kw_mixin.is_some() {
            return Err(input.error(ParseError::RedundantAbstractMixin));
        }
        let name = input.parse()?;
        let extends: Option<Extends> =
            input.peek(kw::extends).then(|| input.parse()).transpose()?;
        let withs: Option<Withs> = input.peek(kw::with).then(|| input.parse()).transpose()?;
        let ons: Option<Ons> = input.peek(kw::on).then(|| input.parse()).transpose()?;
        if let (Some(_class), Some(ons)) = (kw_class, &ons) {
            return Err(syn::Error::new(
                ons.kw_on.span,
                input.error(ParseError::InvalidClassOn),
            ));
        }
        if let (Some(_mixin), Some(extends)) = (kw_mixin, &extends) {
            return Err(syn::Error::new(
                extends.kw_extends.span,
                input.error(ParseError::InvalidMixinExtends),
            ));
        }
        if let (Some(_mixin), Some(withs)) = (kw_mixin, &withs) {
            return Err(syn::Error::new(
                withs.kw_with.span,
                input.error(ParseError::InvalidMixinWith),
            ));
        }
        let implements = input
            .peek(kw::implements)
            .then(|| input.parse())
            .transpose()?;
        let content;
        let brace = syn::braced!(content in input);
        let (item_struct, class_items) = parse_class_body(&content)?;
        Ok(Self {
            attrs,
            vis,
            kw_abstract,
            kw_mixin,
            kw_class,
            extends,
            withs,
            ons,
            implements,
            name,
            brace,
            item_struct,
            class_items,
        })
    }
}

pub struct Extends {
    pub kw_extends: kw::extends,
    pub ident: syn::Ident,
}

impl Parse for Extends {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            kw_extends: input.parse()?,
            ident: input.parse()?,
        })
    }
}

pub struct Withs {
    pub kw_with: kw::with,
    pub idents: Punctuated<syn::Ident, Token![,]>,
}

impl Parse for Withs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            kw_with: input.parse()?,
            idents: input.call(Punctuated::parse_separated_nonempty)?,
        })
    }
}

impl std::fmt::Debug for Withs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.idents.iter().map(|idents| idents.to_string()))
            .finish()
    }
}

impl Withs {
    pub fn contains(&self, interface: &syn::Ident) -> bool {
        self.prefixes(interface).next().is_some()
    }
    pub fn prefixes<'a>(
        &'a self,
        interface: &'a syn::Ident,
    ) -> impl Iterator<Item = impl Iterator<Item = &'a syn::Ident> + use<'a>> {
        self.idents
            .pairs()
            .enumerate()
            .filter_map(move |(i, pair)| {
                (pair.into_value() == interface).then(|| {
                    self.idents
                        .pairs()
                        .take(i + 1)
                        .map(|pair| pair.into_value())
                })
            })
    }
}

pub struct Ons {
    pub kw_on: kw::on,
    pub idents: Punctuated<syn::Ident, Token![,]>,
}

impl Parse for Ons {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            kw_on: input.parse()?,
            idents: input.call(Punctuated::parse_separated_nonempty)?,
        })
    }
}

pub struct Implements {
    #[expect(dead_code)]
    pub kw_implements: kw::implements,
    pub idents: Punctuated<syn::Ident, Token![,]>,
}

impl Parse for Implements {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            kw_implements: input.parse()?,
            idents: input.call(Punctuated::parse_separated_nonempty)?,
        })
    }
}

impl Implements {
    pub fn contains(&self, interface: &syn::Ident) -> bool {
        self.idents
            .pairs()
            .any(|pair| pair.into_value() == interface)
    }
}

pub struct ItemStruct {
    pub attrs: Vec<syn::Attribute>,
    pub kw_struct: Token![struct],
    pub brace: token::Brace,
    pub super_field: Option<SuperField>,
    pub fields: Punctuated<Field, Token![,]>,
}

impl ItemStruct {
    pub fn super_field(&self) -> Option<&SuperField> {
        self.super_field.as_ref()
    }
}

impl Parse for ItemStruct {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let kw_struct = input.parse()?;
        let content;
        let brace = syn::braced!(content in input);
        let mut field_attrs = content.call(syn::Attribute::parse_outer)?;
        let mut super_field: Option<SuperField> = None;
        if content.peek(Token![super]) {
            super_field.insert(content.parse()?).attrs = std::mem::take(&mut field_attrs);
        }
        let mut fields: Punctuated<Field, token::Comma> =
            content.call(Punctuated::parse_terminated)?;
        if !field_attrs.is_empty() {
            fields
                .first_mut()
                .ok_or_else(|| input.error(ParseError::ExpectField))?
                .attrs = field_attrs;
        }
        Ok(Self {
            attrs,
            kw_struct,
            brace,
            super_field,
            fields,
        })
    }
}

pub struct SuperField {
    pub attrs: Vec<syn::Attribute>,
    pub kw_super: Token![super],
    pub colon: Token![:],
    pub ident: syn::Ident,
    #[expect(dead_code)]
    pub comma: Option<Token![,]>,
}

impl Parse for SuperField {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            kw_super: input.parse()?,
            colon: input.parse()?,
            ident: input.parse()?,
            comma: input.parse()?,
        })
    }
}

pub struct Field {
    pub attrs: Vec<syn::Attribute>,
    pub vis: Visibility,
    pub kw_raw: Option<kw::raw>,
    pub kw_mutable: Option<kw::mutable>,
    pub kw_takecell: Option<kw::takecell>,
    pub kw_late: Option<kw::late>,
    pub kw_final: Option<Token![final]>,
    pub name: syn::Ident,
    pub colon: Token![:],
    pub ty: syn::Type,
    pub init: Option<FieldInit>,
}

impl Parse for Field {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            vis: input.parse()?,
            kw_raw: input.parse()?,
            kw_mutable: input.parse()?,
            kw_takecell: input.parse()?,
            kw_late: input.parse()?,
            kw_final: input.parse()?,
            name: input.parse()?,
            colon: input.parse()?,
            ty: input.parse()?,
            init: input.peek(Token![=]).then(|| input.parse()).transpose()?,
        })
    }
}

pub struct FieldInit {
    #[expect(dead_code)]
    pub eq: Token![=],
    pub expr: syn::Expr,
}

impl Parse for FieldInit {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let eq = input.parse()?;
        let expr = input.parse()?;
        Ok(Self { eq, expr })
    }
}

#[non_exhaustive]
pub enum ClassItem {
    Fn(ItemFn),
    Const(syn::ItemConst),
}

impl ClassItem {
    pub fn as_item_fn(&self) -> Option<&ItemFn> {
        match &self {
            ClassItem::Fn(item_fn) => Some(item_fn),
            ClassItem::Const(_) => None,
        }
    }

    pub fn as_item_const(&self) -> Option<&syn::ItemConst> {
        match &self {
            ClassItem::Const(item_const) => Some(item_const),
            ClassItem::Fn(_) => None,
        }
    }
}

impl Parse for ClassItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let vis = input.parse()?;
        let lookahead1 = input.lookahead1();
        if ItemFn::peek(&lookahead1, input) {
            let mut item_fn: ItemFn = input.parse()?;
            item_fn.attrs = attrs;
            item_fn.vis = vis;
            Ok(Self::Fn(item_fn))
        } else if lookahead1.peek(Token![const]) {
            let mut item_const: syn::ItemConst = input.parse()?;
            item_const.attrs = attrs;
            item_const.vis = vis.try_into()?;
            Ok(Self::Const(item_const))
        } else {
            Err(lookahead1.error())
        }
    }
}

pub struct ItemFn {
    pub attrs: Vec<syn::Attribute>,
    pub vis: Visibility,
    pub kw_async: Option<Token![async]>,
    pub kw_const: Option<Token![const]>,
    pub kw_unsafe: Option<Token![unsafe]>,
    pub abi: Option<syn::Abi>,
    pub kw_override: Option<Token![override]>,
    pub kw_final: Option<Token![final]>,
    pub kw_fn: Token![fn],
    pub overrid: Option<Override>,
    pub name: syn::Ident,
    pub params: FnParams,
    pub ret_body: FnRetAndBody,
}

impl ItemFn {
    pub fn peek(lookahead1: &Lookahead1<'_>, input: ParseStream<'_>) -> bool {
        lookahead1.peek(Token![async])
            || lookahead1.peek(Token![const]) && !input.peek2(syn::Ident)
            || lookahead1.peek(Token![unsafe])
            || lookahead1.peek(Token![extern])
            || lookahead1.peek(Token![override])
            || lookahead1.peek(Token![final])
            || lookahead1.peek(Token![fn])
    }
}

impl Parse for ItemFn {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let vis = input.parse()?;
        let kw_async = input.parse()?;
        let kw_const = input.parse()?;
        let kw_unsafe = input.parse()?;
        let abi = input.parse()?;
        let kw_override = input.parse()?;
        let kw_final = input.parse()?;
        let kw_fn = input.parse()?;
        let overrid = (input.peek(Token![<]) || input.peek2(Token![::]))
            .then(|| input.parse())
            .transpose()?;
        let name = input.parse()?;
        let params: FnParams = input.parse()?;
        let ret_body = if params.self_param.is_some() {
            // eprintln!("parsed method fn {name}");
            FnRetAndBody::Method(input.parse()?, input.parse()?)
        } else if input.peek(Token![->]) && input.peek3(Token![Self]) {
            // eprintln!("parsed ctor fn {name}");
            let body;
            let tk_rarrow = input.parse()?;
            let tk_self_type = input.parse()?;
            let brace = syn::braced!(body in input);
            let ctor_body: CtorFnBodyTokenStream = body.parse()?;
            let mut method_body = None;
            // after `let self = Self { .. } `;
            if ctor_body
                .tokens
                .last()
                .is_some_and(|token| matches!(token, CtorFnBodyTokenTree::LetSelfExpr(_)))
            {
                method_body = Some(body.parse()?);
            }
            FnRetAndBody::Ctor(tk_rarrow, tk_self_type, brace, ctor_body, method_body)
        } else {
            // eprintln!("parsed fn {name}");
            let ret = input.parse()?;
            if input.peek(Token![;]) {
                let mut err = input.error(ParseError::ExpectBlock);
                err.combine(syn::Error::new(
                    params.tk_paren.span.open(),
                    ParseError::MissingSelfReceiver,
                ));
                return Err(err);
            }
            FnRetAndBody::Function(ret, input.parse()?)
        };
        Ok(Self {
            attrs,
            vis,
            kw_async,
            kw_const,
            kw_unsafe,
            abi,
            kw_override,
            kw_final,
            kw_fn,
            overrid,
            name,
            params,
            ret_body,
        })
    }
}

pub enum Override {
    Superclass(syn::Ident, #[expect(dead_code)] Token![::]),
    Interface {
        #[expect(dead_code)]
        tk_lt: Token![<],
        alias: OverrideAlias,
        #[expect(dead_code)]
        tk_as: Token![as],
        interfaces: Punctuated<InterfaceMaybeSuperclass, Token![as]>,
        #[expect(dead_code)]
        tk_gt: Token![>],
        #[expect(dead_code)]
        tk_colon: Token![::],
    },
}

impl Parse for Override {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lookahead1 = input.lookahead1();
        if lookahead1.peek(Token![<]) {
            Ok(Self::Interface {
                tk_lt: input.parse()?,
                alias: input.parse()?,
                tk_as: input.parse()?,
                interfaces: input.call(Punctuated::parse_separated_nonempty)?,
                tk_gt: input.parse()?,
                tk_colon: input.parse()?,
            })
        } else if lookahead1.peek(syn::Ident) && input.peek2(Token![::]) {
            Ok(Self::Superclass(input.parse()?, input.parse()?))
        } else {
            Err(lookahead1.error())
        }
    }
}

pub enum InterfaceMaybeSuperclass {
    Interface(syn::Ident),
    InterfaceSuperclass {
        #[expect(dead_code)]
        tk_lt: Token![<],
        interface: syn::Ident,
        #[expect(dead_code)]
        tk_as: Token![as],
        superclass: syn::Ident,
        #[expect(dead_code)]
        tk_gt: Token![>],
    },
}

impl std::fmt::Debug for InterfaceMaybeSuperclass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Interface(interface) => write!(f, "{}", interface),
            Self::InterfaceSuperclass {
                interface,
                superclass,
                ..
            } => write!(f, "<{} as {}>", interface, superclass),
        }
    }
}

impl InterfaceMaybeSuperclass {
    pub fn interface_superclass(&self) -> &syn::Ident {
        match self {
            Self::Interface(interface) => interface,
            Self::InterfaceSuperclass { superclass, .. } => superclass,
        }
    }
}

impl Parse for InterfaceMaybeSuperclass {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lookahead1 = input.lookahead1();
        if lookahead1.peek(Token![<]) {
            Ok(Self::InterfaceSuperclass {
                tk_lt: input.parse()?,
                interface: input.parse()?,
                tk_as: input.parse()?,
                superclass: input.parse()?,
                tk_gt: input.parse()?,
            })
        } else if lookahead1.peek(syn::Ident) {
            Ok(Self::Interface(input.parse()?))
        } else {
            Err(lookahead1.error())
        }
    }
}

pub enum OverrideAlias {
    SelfClass(#[expect(dead_code)] Token![Self]),
    Super(kw::Super),
    Superclass(syn::Ident),
}

impl Parse for OverrideAlias {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lookahead1 = input.lookahead1();
        if lookahead1.peek(Token![Self]) {
            Ok(Self::SelfClass(input.parse()?))
        } else if lookahead1.peek(kw::Super) {
            Ok(Self::Super(input.parse()?))
        } else if lookahead1.peek(syn::Ident) {
            Ok(Self::Superclass(input.parse()?))
        } else {
            Err(lookahead1.error())
        }
    }
}

pub struct FnParams {
    pub tk_paren: token::Paren,
    pub self_param: Option<syn::Receiver>,
    #[expect(dead_code)]
    pub tk_comma: Option<Token![,]>,
    pub params: Punctuated<Param, Token![,]>,
}

impl Parse for FnParams {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let content;
        let tk_paren = syn::parenthesized!(content in input);
        let mut attrs = content.call(syn::Attribute::parse_outer)?;
        let mut self_param: Option<syn::Receiver> = None;
        let mut tk_comma = None;
        if content.peek(Token![&]) || content.peek(Token![self]) || content.peek2(Token![self]) {
            self_param.insert(content.parse()?).attrs = std::mem::take(&mut attrs);
            tk_comma = content.parse()?;
        }
        let mut params: Punctuated<Param, Token![,]> =
            content.call(Punctuated::parse_terminated)?;
        if !attrs.is_empty() {
            params
                .first_mut()
                .ok_or_else(|| content.error(ParseError::ExpectParam))?
                .attrs = attrs;
        }
        Ok(Self {
            tk_paren,
            self_param,
            tk_comma,
            params,
        })
    }
}

pub struct Param {
    pub attrs: Vec<syn::Attribute>,
    pub tk_mut: Option<Token![mut]>,
    pub name: syn::Ident,
    #[expect(dead_code)]
    pub tk_colon: Token![:],
    pub ty: Box<syn::Type>,
    pub default_value: Option<syn::Expr>,
}

impl Parse for Param {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            tk_mut: input.parse()?,
            name: input.parse()?,
            tk_colon: input.parse()?,
            ty: input.parse()?,
            default_value: input
                .parse::<Option<Token![=]>>()?
                .map(|_| input.parse::<syn::Expr>())
                .transpose()?,
        })
    }
}

pub enum FnRetAndBody {
    Method(syn::ReturnType, MethodBody),
    Function(syn::ReturnType, syn::Block),
    Ctor(
        #[expect(dead_code)] Token![->],
        Token![Self],
        token::Brace,
        CtorFnBodyTokenStream,
        Option<MethodFnBodyTokenStream>,
    ),
}

pub enum MethodBody {
    PureVirtual(Token![;]),
    Body(token::Brace, MethodFnBodyTokenStream),
}

impl MethodBody {
    pub fn body(&self) -> Option<&MethodFnBodyTokenStream> {
        match self {
            MethodBody::Body(_, body) => Some(body),
            MethodBody::PureVirtual(_) => None,
        }
    }
    pub fn span(&self) -> proc_macro2::Span {
        match self {
            MethodBody::Body(brace, _) => brace.span.join(),
            MethodBody::PureVirtual(tk_semi) => tk_semi.span,
        }
    }
}

impl Parse for MethodBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead1 = input.lookahead1();
        if lookahead1.peek(Token![;]) {
            Ok(Self::PureVirtual(input.parse()?))
        } else if lookahead1.peek(token::Brace) {
            let content;
            Ok(Self::Body(syn::braced!(content in input), content.parse()?))
        } else {
            Err(lookahead1.error())
        }
    }
}

pub struct FnBodyTokenStream<T> {
    pub(crate) tokens: Vec<T>,
}

impl<T: Parse> Parse for FnBodyTokenStream<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut tokens = Vec::new();
        while !input.is_empty() {
            tokens.push(input.parse()?);
        }
        Ok(Self { tokens })
    }
}

pub struct CtorFnBodyTokenStream(FnBodyTokenStream<CtorFnBodyTokenTree>);

impl std::ops::Deref for CtorFnBodyTokenStream {
    type Target = FnBodyTokenStream<CtorFnBodyTokenTree>;

    fn deref(&self) -> &FnBodyTokenStream<CtorFnBodyTokenTree> {
        &self.0
    }
}

impl Parse for CtorFnBodyTokenStream {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut tokens = Vec::new();
        while !input.is_empty() {
            let token: CtorFnBodyTokenTree = input.parse()?;
            if matches!(token, CtorFnBodyTokenTree::LetSelfExpr(_)) {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        Ok(Self(FnBodyTokenStream { tokens }))
    }
}

pub type MethodFnBodyTokenStream = FnBodyTokenStream<MethodFnBodyTokenTree>;
pub type MethodFnBodyTokenGroup = FnBodyTokenGroup<MethodFnBodyTokenTree>;
pub type CtorFnBodyTokenGroup = FnBodyTokenGroup<CtorFnBodyTokenTree>;

pub enum MethodFnBodyTokenTree {
    // Skip the inner items in a method body.
    Item(Box<syn::Item>),
    SelfValue(Token![self]),
    SuperValue(Token![super]),
    DotSuper(#[expect(dead_code)] Token![.], Token![super]),
    Group(MethodFnBodyTokenGroup),
    Ident(syn::Ident),
    Punct(proc_macro2::Punct),
    Literal(proc_macro2::Literal),
}

pub enum CtorFnBodyTokenTree {
    // Skip the inner items in a constructor body.
    Item(Box<syn::Item>),
    SelfStructExpr(Token![Self], token::Brace, SelfStructExpr),
    LetSelfExpr(LetSelfExpr),
    Group(CtorFnBodyTokenGroup),
    Ident(syn::Ident),
    Punct(proc_macro2::Punct),
    Literal(proc_macro2::Literal),
}

pub struct FnBodyTokenGroup<T> {
    pub(crate) delimiter: proc_macro2::Delimiter,
    pub(crate) span: proc_macro2::extra::DelimSpan,
    pub(crate) tokens: FnBodyTokenStream<T>,
}

impl Parse for MethodFnBodyTokenTree {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let fork = input.fork();
        if let Ok(item) = fork.parse::<Box<syn::Item>>() {
            if !matches!(&*item, syn::Item::Macro(_)) {
                input.advance_to(&fork);
                return Ok(Self::Item(item));
            }
        }
        if input.peek(Token![self]) {
            if !input.peek2(Token![::]) {
                return Ok(Self::SelfValue(input.parse()?));
            }
        }
        if input.peek(Token![super]) && !input.peek2(Token![::]) {
            return Ok(Self::SuperValue(input.parse()?));
        }
        if input.peek(Token![.]) && input.peek2(Token![super]) {
            return Ok(Self::DotSuper(input.parse()?, input.parse()?));
        }
        input.step(|cursor| {
            let (tt, next) = cursor.token_tree().ok_or_else(|| cursor.error(EofError))?;
            let ret = match tt {
                TokenTree::Group(group) => Self::Group(group.try_into()?),
                TokenTree::Ident(ident) => Self::Ident(ident),
                TokenTree::Punct(punct) => Self::Punct(punct),
                TokenTree::Literal(literal) => Self::Literal(literal),
            };
            Ok((ret, next))
        })
    }
}

impl Parse for CtorFnBodyTokenTree {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let fork = input.fork();
        if let Ok(item) = fork.parse::<Box<syn::Item>>() {
            if !matches!(&*item, syn::Item::Macro(_)) {
                input.advance_to(&fork);
                return Ok(Self::Item(item));
            }
        }
        if input.peek(Token![let]) && input.peek2(Token![self]) && input.peek3(Token![=]) {
            return Ok(Self::LetSelfExpr(input.parse()?));
        }
        if input.peek(Token![Self]) && input.peek2(token::Brace) {
            let content;
            return Ok(Self::SelfStructExpr(
                input.parse()?,
                syn::braced!(content in input),
                content.parse()?,
            ));
        }
        input.step(|cursor| {
            let (tt, next) = cursor.token_tree().ok_or_else(|| cursor.error(EofError))?;
            let ret = match tt {
                TokenTree::Group(group) => Self::Group(group.try_into()?),
                TokenTree::Ident(ident) => Self::Ident(ident),
                TokenTree::Punct(punct) => Self::Punct(punct),
                TokenTree::Literal(literal) => Self::Literal(literal),
            };
            Ok((ret, next))
        })
    }
}

impl<T: Parse> TryFrom<proc_macro2::Group> for FnBodyTokenGroup<T> {
    type Error = syn::Error;

    fn try_from(group: proc_macro2::Group) -> Result<Self, Self::Error> {
        Ok(Self {
            delimiter: group.delimiter(),
            span: group.delim_span(),
            tokens: syn::parse2(group.stream())?,
        })
    }
}

pub struct LetSelfExpr {
    pub(crate) tk_let: Token![let],
    pub(crate) tk_self: Token![self],
    pub(crate) tk_eq: Token![=],
    pub(super) tk_self_type: Token![Self],
    pub(crate) kind: LetSelfExprKind,
    pub(crate) tk_semi: Token![;],
}

pub enum LetSelfExprKind {
    Struct(token::Brace, SelfStructExpr),
    Method(Token![::], syn::Ident, token::Paren, TokenStream),
}

impl Parse for LetSelfExpr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            tk_let: input.parse()?,
            tk_self: input.parse()?,
            tk_eq: input.parse()?,
            tk_self_type: input.parse()?,
            kind: input.parse()?,
            tk_semi: input.parse()?,
        })
    }
}

impl Parse for LetSelfExprKind {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(token::Brace) {
            let content;
            Ok(Self::Struct(
                syn::braced!(content in input),
                content.parse()?,
            ))
        } else if lookahead.peek(Token![::]) {
            let content;
            Ok(Self::Method(
                input.parse()?,
                input.parse()?,
                syn::parenthesized!(content in input),
                content.parse()?,
            ))
        } else {
            Err(lookahead.error())
        }
    }
}

pub struct SelfStructExpr {
    pub super_field: Option<SuperFieldValue>,
    pub fields: Punctuated<FieldValue, Token![,]>,
    pub dot_dot: Option<Token![..]>,
}

impl Parse for SelfStructExpr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut super_field: Option<SuperFieldValue> = None;
        let mut attrs = input.call(syn::Attribute::parse_outer)?;
        let mut fields = Punctuated::new();
        let mut dot_dot = None;
        if input.peek(Token![super]) {
            super_field.insert(input.parse()?).attrs = std::mem::take(&mut attrs);
        }
        if !attrs.is_empty() {
            let mut value: FieldValue = input.parse()?;
            if !attrs.is_empty() {
                value.attrs = attrs;
            }
            fields.push(value);
        }
        while !input.is_empty() {
            if input.peek(Token![,]) {
                fields.push_punct(input.parse()?);
            }
            if input.peek(Token![..]) {
                dot_dot = Some(input.parse()?);
                break;
            }
            if input.is_empty() {
                break;
            }
            fields.push(input.parse()?);
        }
        Ok(Self {
            super_field,
            fields,
            dot_dot,
        })
    }
}

pub struct FieldValue<I = syn::Ident, E = Option<ColonExpr>> {
    pub attrs: Vec<syn::Attribute>,
    pub ident: I,
    pub expr: E,
    #[expect(dead_code)]
    pub comma: Option<Token![,]>,
}

pub type SuperFieldValue = FieldValue<Token![super], ColonCallExpr>;

impl Parse for FieldValue {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            ident: input.parse()?,
            expr: input.peek(Token![:]).then(|| input.parse()).transpose()?,
            comma: input.parse()?,
        })
    }
}

impl Parse for SuperFieldValue {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            ident: input.parse()?,
            expr: input.parse()?,
            comma: input.parse()?,
        })
    }
}

pub struct ColonExpr {
    pub colon: Token![:],
    pub expr: Box<syn::Expr>,
}

impl Parse for ColonExpr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let colon = input.parse()?;
        let expr = input.parse()?;
        Ok(Self { colon, expr })
    }
}

impl quote::ToTokens for ColonExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.colon.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}

pub struct ColonCallExpr {
    #[expect(dead_code)]
    pub colon: Token![:],
    pub super_ty: kw::Super,
    pub tk_path_sep: Token![::],
    pub method: syn::Ident,
    pub tk_paren: token::Paren,
    pub token_stream: TokenStream,
    pub chained_calls: Vec<ChainedCall>,
}

impl Parse for ColonCallExpr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let content;
        Ok(Self {
            colon: input.parse()?,
            super_ty: input.parse()?,
            tk_path_sep: input.parse()?,
            method: input.parse()?,
            tk_paren: syn::parenthesized!(content in input),
            token_stream: content.parse()?,
            chained_calls: input.call(ChainedCall::parse_chain)?,
        })
    }
}

pub struct ChainedCall {
    pub tk_dot: Token![.],
    pub method: syn::Ident,
    pub tk_paren: token::Paren,
    pub token_stream: TokenStream,
}

impl ChainedCall {
    pub fn parse_chain(input: ParseStream<'_>) -> syn::Result<Vec<Self>> {
        let mut chained_calls = Vec::new();
        while input.peek(Token![.]) {
            chained_calls.push(input.parse()?);
        }
        Ok(chained_calls)
    }
}

impl Parse for ChainedCall {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let content;
        Ok(Self {
            tk_dot: input.parse()?,
            method: input.parse()?,
            tk_paren: syn::parenthesized!(content in input),
            token_stream: content.parse()?,
        })
    }
}

impl quote::ToTokens for ChainedCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.tk_dot.to_tokens(tokens);
        self.method.to_tokens(tokens);
        self.tk_paren.surround(tokens, |tokens| {
            self.token_stream.to_tokens(tokens);
        });
    }
}

pub enum Visibility {
    Inherited,
    Pub(Token![pub]),
    PubAlone(Token![pub], token::Paren, Option<Token![in]>, VisPathAlone),
    PubPath(
        Token![pub],
        token::Paren,
        Token![in],
        VisPathBegin,
        Punctuated<VisPath, Token![::]>,
    ),
}

impl Visibility {
    pub fn is_pub(&self) -> bool {
        matches!(self, Self::Pub(_))
    }
}

impl TryFrom<Visibility> for syn::Visibility {
    type Error = syn::Error;
    fn try_from(visibility: Visibility) -> syn::Result<Self> {
        syn::parse2(quote::ToTokens::into_token_stream(visibility))
    }
}

pub enum VisPath {
    SuperPath(Token![super]),
    Ident(syn::Ident),
}

#[allow(clippy::enum_variant_names)]
pub enum VisPathAlone {
    SelfPath(Token![self]),
    SuperPath(Token![super]),
    CratePath(Token![crate]),
}

pub enum VisPathBegin {
    Global(Token![::]),
    Local,
    Special(VisPathAlone, Token![::]),
}

impl Parse for Visibility {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let Ok(tk_pub) = input.parse() else {
            return Ok(Self::Inherited);
        };
        if !input.peek(token::Paren) {
            return Ok(Self::Pub(tk_pub));
        }
        let content;
        let paren = syn::parenthesized!(content in input);
        let lookahead = content.lookahead1();
        if VisPathAlone::peek(&lookahead) {
            return Ok(Self::PubAlone(tk_pub, paren, None, content.parse()?));
        }
        if !lookahead.peek(Token![in]) {
            return Err(lookahead.error());
        }
        let tk_in = content.parse()?;
        let lookahead = content.lookahead1();
        let begin = if VisPathAlone::peek(&lookahead) {
            let path_alone = content.parse()?;
            if content.is_empty() {
                return Ok(Self::PubAlone(tk_pub, paren, Some(tk_in), path_alone));
            } else {
                VisPathBegin::Special(path_alone, content.parse()?)
            }
        } else if lookahead.peek(Token![::]) && content.peek3(syn::Ident) {
            VisPathBegin::Global(content.parse()?)
        } else if lookahead.peek(syn::Ident) {
            VisPathBegin::Local
        } else {
            return Err(lookahead.error());
        };
        let path = content.call(Punctuated::parse_terminated)?;
        Ok(Self::PubPath(tk_pub, paren, tk_in, begin, path))
    }
}

impl Parse for VisPath {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![super]) {
            Ok(VisPath::SuperPath(input.parse()?))
        } else if lookahead.peek(syn::Ident) {
            Ok(VisPath::Ident(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl quote::ToTokens for VisPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VisPath::SuperPath(super_path) => super_path.to_tokens(tokens),
            VisPath::Ident(ident) => ident.to_tokens(tokens),
        }
    }
}

impl quote::ToTokens for Visibility {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Visibility::Inherited => {}
            Visibility::Pub(tk_pub) => tk_pub.to_tokens(tokens),
            Visibility::PubAlone(tk_pub, paren, tk_in, path_alone) => {
                tk_pub.to_tokens(tokens);
                paren.surround(tokens, |tokens| {
                    tk_in.to_tokens(tokens);
                    path_alone.to_tokens(tokens);
                });
            }
            Visibility::PubPath(tk_pub, paren, tk_in, begin, path) => {
                tk_pub.to_tokens(tokens);
                paren.surround(tokens, |tokens| {
                    tk_in.to_tokens(tokens);
                    begin.to_tokens(tokens);
                    path.to_tokens(tokens);
                });
            }
        }
    }
}

impl VisPathAlone {
    pub fn peek(lookahead: &Lookahead1<'_>) -> bool {
        lookahead.peek(Token![self])
            || lookahead.peek(Token![super])
            || lookahead.peek(Token![crate])
    }
}

impl Parse for VisPathAlone {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![self]) {
            Ok(VisPathAlone::SelfPath(input.parse()?))
        } else if lookahead.peek(Token![crate]) {
            Ok(VisPathAlone::CratePath(input.parse()?))
        } else if lookahead.peek(Token![super]) {
            Ok(VisPathAlone::SuperPath(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl quote::ToTokens for VisPathAlone {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VisPathAlone::SelfPath(self_value) => self_value.to_tokens(tokens),
            VisPathAlone::SuperPath(super_path) => super_path.to_tokens(tokens),
            VisPathAlone::CratePath(crate_path) => crate_path.to_tokens(tokens),
        }
    }
}

impl quote::ToTokens for VisPathBegin {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VisPathBegin::Global(tk_colons) => tk_colons.to_tokens(tokens),
            VisPathBegin::Local => {}
            VisPathBegin::Special(begin, path) => {
                begin.to_tokens(tokens);
                path.to_tokens(tokens);
            }
        }
    }
}
