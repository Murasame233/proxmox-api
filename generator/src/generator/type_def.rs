use std::collections::BTreeSet;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Ident};

use crate::raw::KnownFormat;

use super::{
    field_def::FieldDef, proxmox_api, struct_def::AdditionalProperties, EnumDef, NumItemsDef,
    StructDef,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PrimitiveTypeDef {
    String,
    Number,
    Integer,
    Boolean,
}

impl ToTokens for PrimitiveTypeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let this_tokens = match self {
            PrimitiveTypeDef::String => quote!(String),
            PrimitiveTypeDef::Number => quote!(f64),
            PrimitiveTypeDef::Integer => quote!(u64),
            PrimitiveTypeDef::Boolean => quote!(bool),
        };

        tokens.extend(this_tokens)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeDef {
    // TODO: get rid of TypeDef::Unit. It means nothing,
    // and is mostly here to make life slightly easier
    Unit,
    Primitive(PrimitiveTypeDef),
    KnownType {
        format: KnownFormat,
        fallback: PrimitiveTypeDef,
    },
    Array(Box<TypeDef>),
    Struct(StructDef),
    Enum(EnumDef),
    NumberedItems(Box<NumItemsDef>),
}

impl TypeDef {
    pub const DEFAULT_DERIVES: [&'static str; 4] = [
        "Clone",
        "Debug",
        "::serde::Serialize",
        "::serde::Deserialize",
    ];

    pub fn is_unit(&self) -> bool {
        matches!(self, TypeDef::Unit)
    }

    pub fn is_array(&self) -> bool {
        matches!(self, TypeDef::Array(..))
    }

    pub fn primitive(&self) -> Option<PrimitiveTypeDef> {
        if let Self::Primitive(p) = self {
            Some(*p)
        } else {
            None
        }
    }

    pub fn new_enum<I: AsRef<str>, T: IntoIterator<Item = I>>(
        name: String,
        extra_derives: T,
        values: BTreeSet<String>,
        default: Option<String>,
        doc: impl Iterator<Item = String>,
    ) -> Self {
        Self::Enum(EnumDef::new(
            name,
            extra_derives,
            values,
            default,
            doc.collect(),
        ))
    }

    pub fn new_struct(
        name: String,
        fields: Vec<FieldDef>,
        additional_props: AdditionalProperties,
    ) -> Self {
        Self::Struct(StructDef::new(name, fields, additional_props))
    }

    pub fn as_field_ty(&self, optional: bool) -> TokenStream {
        let ty = match self {
            TypeDef::Unit => quote!(()),
            TypeDef::NumberedItems(_) => panic!(),
            TypeDef::Struct(strt) => {
                let ident = Ident::new(strt.name(), quote!().span());
                quote!(#ident)
            }
            TypeDef::KnownType { format, fallback } => Self::known_type(format, fallback),
            TypeDef::Primitive(name) => name.to_token_stream(),
            TypeDef::Array(inner) => {
                let inner = inner.as_field_ty(false);
                quote!(Vec<#inner>)
            }
            TypeDef::Enum(en) => {
                let ident = Ident::new(en.name(), quote!().span());
                quote!(#ident)
            }
        };

        if optional {
            quote!(Option<#ty>)
        } else {
            ty
        }
    }

    fn known_type(known: &KnownFormat, fallback: &PrimitiveTypeDef) -> TokenStream {
        match known {
            KnownFormat::PveVmId => proxmox_api(quote!(types::VmId)),
            KnownFormat::Ipv4 => quote!(::std::net::Ipv4Addr),
            KnownFormat::Ipv6 => quote!(::std::net::Ipv6Addr),
            KnownFormat::Ip => quote!(::std::net::IpAddr),
            KnownFormat::MacAddr(allow_multicast) => {
                proxmox_api(quote!(types::MacAddr<#allow_multicast>))
            }
            _ => fallback.to_token_stream(),
        }
    }
}

impl ToTokens for TypeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeDef::Primitive(_) | TypeDef::Unit | TypeDef::KnownType { .. } => {}
            TypeDef::Array(inner) => inner.to_tokens(tokens),
            TypeDef::Enum(def) => def.to_tokens(tokens),
            TypeDef::Struct(strt) => strt.to_tokens(tokens),
            TypeDef::NumberedItems(strt) => strt.to_tokens(tokens),
        }
    }
}
