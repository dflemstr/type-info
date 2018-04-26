//! A procedural macro for deriving `TypeInfo` for any type.
//!
//! See the `type-info` crate for more information as to what this means.
#![feature(const_type_id)]
#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
#[cfg(test)]
extern crate type_info;

#[cfg(test)]
mod test;

struct MetaType<'a> {
    tokens: quote::Tokens,
    data: MetaData<'a>,
}

struct MetaData<'a> {
    tokens: quote::Tokens,
    fields: Option<MetaFields<'a>>,
}

struct MetaFields<'a> {
    tokens: quote::Tokens,
    fields: Vec<MetaField<'a>>,
}

struct MetaField<'a> {
    tokens: quote::Tokens,
    id: MetaFieldId<'a>,
}

enum MetaFieldId<'a> {
    Unnamed(syn::Index),
    Named(&'a syn::Ident),
}

/// Derive the `TypeInfo` and `DynamicTypeInfo` traits for a given type.
#[proc_macro_derive(TypeInfo, attributes(type_info))]
pub fn type_info(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let gen = impl_type_info(ast);
    gen.into()
}

#[cfg(test)]
pub fn type_info_test(input: proc_macro2::TokenStream) -> quote::Tokens {
    let ast = syn::parse2(input).unwrap();
    impl_type_info(ast)
}

fn impl_type_info(mut ast: syn::DeriveInput) -> quote::Tokens {
    let ident = &ast.ident;

    add_static(&mut ast.generics);

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let type_info = build_type_info(&ast);
    let tokens = type_info.tokens;

    let fields = type_info
        .data
        .fields
        .as_ref()
        .map(|f| f.fields.as_slice())
        .unwrap_or_else(|| &[])
        .iter()
        .map(|f| match f.id {
            MetaFieldId::Unnamed(ref i) => {
                let i_usize = i.index as usize;
                quote! {
                    ::type_info::FieldId::Unnamed(#i_usize) => {
                        ::std::any::Any::downcast_ref::<TypeInfoA>(&self.#i)
                    }
                }
            }
            MetaFieldId::Named(i) => {
                let i_str = i.as_ref();
                quote! {
                    ::type_info::FieldId::Named(#i_str) => {
                        ::std::any::Any::downcast_ref::<TypeInfoA>(&self.#i)
                    }
                }
            }
        });

    let fields_dyn = type_info
        .data
        .fields
        .as_ref()
        .map(|f| f.fields.as_slice())
        .unwrap_or_else(|| &[])
        .iter()
        .map(|f| match f.id {
            MetaFieldId::Unnamed(ref i) => {
                let i_usize = i.index as usize;
                quote! {
                    ::type_info::FieldId::Unnamed(#i_usize) => Some(&self.#i),
                }
            }
            MetaFieldId::Named(i) => {
                let i_str = i.as_ref();
                quote! {
                    ::type_info::FieldId::Named(#i_str) => Some(&self.#i),
                }
            }
        });

    let fields_mut = type_info
        .data
        .fields
        .as_ref()
        .map(|f| f.fields.as_slice())
        .unwrap_or_else(|| &[])
        .iter()
        .map(|f| match f.id {
            MetaFieldId::Unnamed(ref i) => {
                let i_usize = i.index as usize;
                quote! {
                    ::type_info::FieldId::Unnamed(#i_usize) => {
                        ::std::any::Any::downcast_mut::<TypeInfoA>(&mut self.#i)
                    }
                }
            }
            MetaFieldId::Named(i) => {
                let i_str = i.as_ref();
                quote! {
                    ::type_info::FieldId::Named(#i_str) => {
                        ::std::any::Any::downcast_mut::<TypeInfoA>(&mut self.#i)
                    }
                }
            }
        });

    let fields_dyn_mut = type_info
        .data
        .fields
        .as_ref()
        .map(|f| f.fields.as_slice())
        .unwrap_or_else(|| &[])
        .iter()
        .map(|f| match f.id {
            MetaFieldId::Unnamed(ref i) => {
                let i_usize = i.index as usize;
                quote! {
                    ::type_info::FieldId::Unnamed(#i_usize) => Some(&mut self.#i),
                }
            }
            MetaFieldId::Named(i) => {
                let i_str = i.as_ref();
                quote! {
                    ::type_info::FieldId::Named(#i_str) => Some(&mut self.#i),
                }
            }
        });

    quote! {
        impl #impl_generics ::type_info::TypeInfo for #ident #ty_generics #where_clause {
            const TYPE: ::type_info::Type = #tokens;

            fn field<TypeInfoA>(&self, id: ::type_info::FieldId) -> ::std::option::Option<&TypeInfoA>
            where
                TypeInfoA: ::std::any::Any,
            {
                match id {
                    #(#fields)*
                    _ => ::std::option::Option::None,
                }
            }

            fn field_mut<TypeInfoA>(&mut self, id: ::type_info::FieldId) -> ::std::option::Option<&mut TypeInfoA>
            where
                TypeInfoA: ::std::any::Any,
            {
                match id {
                    #(#fields_mut)*
                    _ => ::std::option::Option::None,
                }
            }
        }

        impl #impl_generics ::type_info::DynamicTypeInfo for #ident #ty_generics #where_clause {
            fn type_ref(&self) -> &'static ::type_info::Type {
                &<Self as ::type_info::TypeInfo>::TYPE
            }

            fn field_dyn(&self, id: ::type_info::FieldId) -> ::std::option::Option<&::std::any::Any> {
                match id {
                    #(#fields_dyn)*
                    _ => ::std::option::Option::None,
                }
            }

            fn field_dyn_mut(&mut self, id: ::type_info::FieldId) -> ::std::option::Option<&mut ::std::any::Any> {
                match id {
                    #(#fields_dyn_mut)*
                    _ => ::std::option::Option::None,
                }
            }
        }
    }
}

fn add_static(generics: &mut syn::Generics) {
    for type_param in generics.type_params_mut() {
        if !type_param.bounds.empty_or_trailing() {
            type_param
                .bounds
                .push_punct(syn::token::Add::new(proc_macro2::Span::call_site()));
        }
        type_param
            .bounds
            .push_value(syn::TypeParamBound::Trait(parse_quote!(::std::any::Any)));
    }
}

fn build_type_info(ast: &syn::DeriveInput) -> MetaType {
    let ident = &ast.ident;
    let (_, generics, _) = &ast.generics.split_for_impl();
    let ident_str = ident.as_ref();
    let data = build_data(&ast.data);
    let tokens = {
        let data = &data.tokens;
        quote! {
            ::type_info::Type {
                id: ::type_info::TypeId::of::<#ident #generics>(),
                module: module_path!(),
                ident: #ident_str,
                data: #data,
            }
        }
    };

    MetaType { tokens, data }
}

fn build_data(data: &syn::Data) -> MetaData {
    match *data {
        syn::Data::Struct(ref data_struct) => build_data_struct(data_struct).map_tokens(|tokens| {
            quote! {
                ::type_info::Data::Struct(#tokens)
            }
        }),
        syn::Data::Enum(ref data_enum) => build_data_enum(data_enum).map_tokens(|tokens| {
            quote! {
                ::type_info::Data::Enum(#tokens)
            }
        }),
        syn::Data::Union(ref data_union) => build_data_union(data_union).map_tokens(|tokens| {
            quote! {
                ::type_info::Data::Union(#tokens)
            }
        }),
    }
}

fn build_data_struct(data_struct: &syn::DataStruct) -> MetaData {
    let data_struct_fields = build_fields(&data_struct.fields);
    let tokens = {
        let fields = &data_struct_fields.tokens;
        quote! {
            ::type_info::DataStruct {
                fields: #fields,
            }
        }
    };

    MetaData {
        tokens,
        fields: Some(data_struct_fields),
    }
}

fn build_data_enum(data_enum: &syn::DataEnum) -> MetaData {
    let tokens = data_enum
        .variants
        .iter()
        .map(build_variant)
        .map(|v| v.tokens);
    let tokens = quote! {
        ::type_info::DataEnum {
            variants: &[
                #(#tokens,)*
            ],
        }
    };

    MetaData {
        tokens,
        fields: None,
    }
}

fn build_data_union(data_union: &syn::DataUnion) -> MetaData {
    let fields_named = build_fields_named(&data_union.fields);
    let tokens = {
        let fields = &fields_named.tokens;
        quote! {
            ::type_info::DataUnion {
                fields: #fields,
            }
        }
    };

    MetaData {
        tokens,
        fields: Some(fields_named),
    }
}

fn build_variant(variant: &syn::Variant) -> MetaFields {
    let ident = variant.ident.as_ref();
    build_fields(&variant.fields).map_tokens(|tokens| {
        quote! {
            ::type_info::Variant {
                ident: #ident,
                fields: #tokens,
            }
        }
    })
}

fn build_fields(fields: &syn::Fields) -> MetaFields {
    match *fields {
        syn::Fields::Named(ref fields_named) => {
            build_fields_named(fields_named).map_tokens(|tokens| {
                quote! {
                    ::type_info::Fields::Named(#tokens)
                }
            })
        }
        syn::Fields::Unnamed(ref fields_unnamed) => build_fields_unnamed(fields_unnamed)
            .map_tokens(|tokens| {
                quote! {
                    ::type_info::Fields::Unnamed(#tokens)
                }
            }),
        syn::Fields::Unit => MetaFields {
            tokens: quote! {
                ::type_info::Fields::Unit
            },
            fields: vec![],
        },
    }
}

fn build_fields_named(fields_named: &syn::FieldsNamed) -> MetaFields {
    let fields = fields_named
        .named
        .iter()
        .enumerate()
        .map(|(i, f)| build_field(i, f))
        .collect::<Vec<_>>();
    let tokens = {
        let named = fields.iter().map(|f| &f.tokens);
        quote! {
            ::type_info::FieldsNamed {
                named: &[
                    #(#named,)*
                ],
            }
        }
    };

    MetaFields { tokens, fields }
}

fn build_fields_unnamed(fields_unnamed: &syn::FieldsUnnamed) -> MetaFields {
    let fields = fields_unnamed
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, f)| build_field(i, f))
        .collect::<Vec<_>>();
    let tokens = {
        let unnamed = fields.iter().map(|f| &f.tokens);
        quote! {
            ::type_info::FieldsUnnamed {
                unnamed: &[
                    #(#unnamed,)*
                ],
            }
        }
    };

    MetaFields { tokens, fields }
}

fn build_field(idx: usize, field: &syn::Field) -> MetaField {
    match field.ident {
        Some(ref ident) => {
            let ident_str = ident.as_ref();
            let ty = &field.ty;
            let tokens = quote! {
                ::type_info::Field {
                    id: ::type_info::FieldId::Named(#ident_str),
                    ident: Some(#ident_str),
                    ty: <#ty as ::type_info::TryTypeInfo>::TRY_TYPE,
                }
            };
            let id = MetaFieldId::Named(ident);

            MetaField { tokens, id }
        }
        None => {
            let ty = &field.ty;
            let tokens = quote! {
                ::type_info::Field {
                    id: ::type_info::FieldId::Unnamed( #idx),
                    ident: None,
                    ty: <#ty as::type_info::TryTypeInfo >::TRY_TYPE,
                }
            };
            let id = MetaFieldId::Unnamed(syn::Index {
                index: idx as u32,
                span: proc_macro2::Span::call_site(),
            });

            MetaField { tokens, id }
        }
    }
}

impl<'a> MetaData<'a> {
    fn map_tokens<F>(self, mapper: F) -> Self
    where
        F: FnOnce(quote::Tokens) -> quote::Tokens,
    {
        MetaData {
            tokens: mapper(self.tokens),
            fields: self.fields,
        }
    }
}

impl<'a> MetaFields<'a> {
    fn map_tokens<F>(self, mapper: F) -> Self
    where
        F: FnOnce(quote::Tokens) -> quote::Tokens,
    {
        MetaFields {
            tokens: mapper(self.tokens),
            fields: self.fields,
        }
    }
}
