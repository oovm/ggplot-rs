mod field_kind;

pub use self::field_kind::MergeFieldKind;

use super::*;
use syn::TypePath;
use proc_macro2::{Ident, Span};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DataStruct, Field, Fields, Path, PathSegment};
use syn::{GenericArgument, PathArguments, Type};

pub fn merge_expand(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let fields = match input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => fields.named,
        _ => panic!("enum does not merge-able"),
    };
    let mut parsed = vec![];
    for i in fields {
        match MergeField::parse_field(&i) {
            Some(s) => { parsed.push(s)}
            None => {panic!("field does not merge-able")}
        }
    }
    let getters = parsed.iter().map(|field|field.as_getter());
    let setters = parsed.iter().map(|field|field.as_setter());
    let builder = parsed.iter().map(|field|field.as_builder());
    quote! {
        impl #name {
            #(#getters)*
            #(#setters)*
            #(#builder)*
        }
    }
}


pub struct MergeField {
    kind: MergeFieldKind,
    field_name: Ident,
}

impl MergeField {
    pub fn mg(self: Vec<Self>) {

    }
}


impl MergeField {
    pub fn parse_field(f: &Field) -> Option<Self> {
        let name = f.ident.as_ref()?.to_owned();
        let kind = MergeFieldKind::parse(&f.ty)?;
        Some(Self { kind, field_name: name })
    }
}


impl MergeField {
    fn getter_name(&self) -> Ident {
        let name = format!("get_{}", self.field_name);
        Ident::new(&name, Span::call_site())
    }
    fn getter_type(&self) -> &Type {
        self.kind.as_getter_type()
    }
    pub fn as_getter(&self) -> TokenStream {
        let field_name = &self.field_name;
        let field_ty = &self.getter_type();
        let getter = self.getter_name();

        quote! {
            #[inline]
            #[automatically_derived]
            pub fn #getter(&self) -> &#field_ty {
                &self.#field_name.unwrap_or_default()
            }
        }
    }
}

impl MergeField {
    fn setter_name(&self) -> Ident {
        let name = format!("set_{}", self.field_name);
        Ident::new(&name, Span::call_site())
    }
    fn setter_type(&self) -> &Type {
        self.kind.as_setter_type()
    }
    pub fn as_setter(&self) -> TokenStream {
        let field_name = &self.field_name;
        let field_ty = &self.setter_type();
        let setter = self.setter_name();

        quote! {
            #[inline]
            #[automatically_derived]
            pub fn #setter(&self, #field_name: #field_ty) {
                &self.#field_name = Some(#field_name)
            }
        }
    }
}


impl MergeField {
    fn builder_name(&self) -> Ident {
        let name = format!("with_{}", self.field_name);
        Ident::new(&name, Span::call_site())
    }
    fn builder_type(&self) -> &Type {
        self.kind.as_builder_type()
    }
    pub fn as_builder(&self) -> TokenStream {
        let field_name = &self.field_name;
        let field_ty = &self.builder_type();
        let builder = self.builder_name();

        quote! {
            #[inline]
            #[automatically_derived]
            pub fn #builder(self, #field_name: #field_ty) -> Self {
                Self { #field_name: Some(#field_name), ..self }
            }
        }
    }
}
