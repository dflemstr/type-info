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
extern crate tempfile;
#[cfg(test)]
extern crate type_info;

#[cfg(test)]
mod test;

struct MetaType<'a> {
    tokens: quote::Tokens,
    ident: syn::Ident,
    data: MetaData<'a>,
}

struct MetaData<'a> {
    tokens: quote::Tokens,
    fields: Option<MetaFields<'a>>,
    variants: Option<Vec<MetaVariant<'a>>>,
}

struct MetaFields<'a> {
    tokens: quote::Tokens,
    kind: MetaFieldsKind,
    fields: Vec<MetaField<'a>>,
}

enum MetaFieldsKind {
    Unit,
    Unnamed(usize),
    Named(usize),
}

struct MetaField<'a> {
    tokens: quote::Tokens,
    id: MetaFieldId<'a>,
}

struct MetaVariant<'a> {
    tokens: quote::Tokens,
    id: MetaVariantId,
    fields: MetaFields<'a>,
}

enum MetaFieldId<'a> {
    Unnamed(syn::Index),
    Named(&'a syn::Ident),
}

struct MetaVariantId(syn::Ident);

enum MetaBorrow {
    Ref,
    Mut,
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
    let tokens = &type_info.tokens;

    let field_fn = build_field_fn(&type_info);
    let field_mut_fn = build_field_mut_fn(&type_info);
    let variant_fn = build_variant_fn(&type_info);
    let field_any_fn = build_field_any_fn(&type_info);
    let field_any_mut_fn = build_field_any_mut_fn(&type_info);

    quote! {
        impl #impl_generics ::type_info::TypeInfo for #ident #ty_generics #where_clause {
            const TYPE: ::type_info::Type = #tokens;
            #field_fn
            #field_mut_fn
        }

        impl #impl_generics ::type_info::DynamicTypeInfo for #ident #ty_generics #where_clause {
            fn type_ref(&self) -> ::type_info::Type {
                <Self as ::type_info::TypeInfo>::TYPE
            }

            #variant_fn
            #field_any_fn
            #field_any_mut_fn
        }
    }
}

fn build_field_fn(type_info: &MetaType) -> quote::Tokens {
    build_field_fn_body(
        type_info,
        |a| quote!({::std::any::Any::downcast_ref::<TypeInfoA>(#a)}),
        MetaBorrow::Ref,
    ).map(|body| {
        quote! {
            fn field<TypeInfoA>(&self, id: ::type_info::FieldId) -> ::std::option::Option<&TypeInfoA>
            where
                TypeInfoA: ::std::any::Any,
            {
                #body
            }
        }
    })
        .unwrap_or(quote!())
}

fn build_field_mut_fn(type_info: &MetaType) -> quote::Tokens {
    build_field_fn_body(
        type_info,
        |a| quote!({::std::any::Any::downcast_mut::<TypeInfoA>(#a)}),
        MetaBorrow::Mut,
    ).map(|body| {
        quote! {
            fn field_mut<TypeInfoA>(&mut self, id: ::type_info::FieldId) -> ::std::option::Option<&mut TypeInfoA>
            where
                TypeInfoA: ::std::any::Any,
            {
                #body
            }
        }
    })
        .unwrap_or(quote!())
}

fn build_field_any_fn(type_info: &MetaType) -> quote::Tokens {
    build_field_fn_body(
        type_info,
        |a| quote!(::std::option::Option::Some(#a),),
        MetaBorrow::Ref,
    ).map(|body| {
        quote! {
            fn field_any(&self, id: ::type_info::FieldId) -> ::std::option::Option<&::std::any::Any> {
                #body
            }
        }
    })
        .unwrap_or(quote!())
}

fn build_field_any_mut_fn(type_info: &MetaType) -> quote::Tokens {
    build_field_fn_body(
        type_info,
        |a| quote!(::std::option::Option::Some(#a),),
        MetaBorrow::Mut,
    ).map(|body| {
        quote! {
            fn field_any_mut(&mut self, id: ::type_info::FieldId) -> ::std::option::Option<&mut ::std::any::Any> {
                #body
            }
        }
    })
        .unwrap_or(quote!())
}

fn build_field_fn_body<A>(
    type_info: &MetaType,
    accessor_builder: A,
    meta_borrow: MetaBorrow,
) -> Option<quote::Tokens>
where
    A: FnMut(quote::Tokens) -> quote::Tokens,
{
    if let Some(ref variants) = type_info.data.variants {
        if variants.iter().all(|v| v.fields.fields.is_empty()) {
            None
        } else {
            Some(build_field_fn_body_from_variants(
                &type_info.ident,
                variants,
                accessor_builder,
                meta_borrow,
            ))
        }
    } else if let Some(MetaFields { ref fields, .. }) = type_info.data.fields {
        if fields.is_empty() {
            None
        } else {
            Some(build_field_fn_body_from_fields(
                fields,
                accessor_builder,
                meta_borrow,
            ))
        }
    } else {
        None
    }
}

fn build_field_fn_body_from_fields<A>(
    meta_fields: &[MetaField],
    mut accessor_builder: A,
    meta_borrow: MetaBorrow,
) -> quote::Tokens
where
    A: FnMut(quote::Tokens) -> quote::Tokens,
{
    let fields = meta_fields.iter().map(|f| match f.id {
        MetaFieldId::Unnamed(ref i) => {
            let i_usize = i.index as usize;
            let accessor = accessor_builder(match meta_borrow {
                MetaBorrow::Ref => quote!(&self.#i),
                MetaBorrow::Mut => quote!(&mut self.#i),
            });

            quote! {
                ::type_info::FieldId::Unnamed(#i_usize) => #accessor
            }
        }
        MetaFieldId::Named(i) => {
            let i_str = i.as_ref();
            let accessor = accessor_builder(match meta_borrow {
                MetaBorrow::Ref => quote!(&self.#i),
                MetaBorrow::Mut => quote!(&mut self.#i),
            });

            quote! {
                ::type_info::FieldId::Named(#i_str) => #accessor
            }
        }
    });

    quote! {
        match id {
            #(#fields)*
            _ => ::std::option::Option::None,
        }
    }
}

fn build_field_fn_body_from_variants<A>(
    type_ident: &syn::Ident,
    meta_variants: &[MetaVariant],
    mut accessor_builder: A,
    meta_borrow: MetaBorrow,
) -> quote::Tokens
where
    A: FnMut(quote::Tokens) -> quote::Tokens,
{
    let variants = meta_variants.iter().map(|v| {
        let ident = v.id.0;
        let meta_fields = &v.fields.fields;

        let syn_idents = (0..meta_fields.len())
            .map(|idx| syn::Ident::from(format!("_{}", idx).as_str()))
            .collect::<Vec<_>>();
        let pat_syn_idents = syn_idents.iter().map(|ident| match meta_borrow {
            MetaBorrow::Ref => quote!(ref #ident),
            MetaBorrow::Mut => quote!(ref mut #ident),
        });

        match v.fields.kind {
            MetaFieldsKind::Unit => quote! { #type_ident::#ident => ::std::option::Option::None, },
            MetaFieldsKind::Unnamed(_) => {
                let body = build_field_fn_variant_field_match(
                    meta_fields,
                    &syn_idents,
                    &mut accessor_builder,
                );

                quote! { #type_ident::#ident(#(#pat_syn_idents,)*) => #body }
            }
            MetaFieldsKind::Named(_) => {
                let pat_idents = meta_fields.iter().map(|f| match f.id {
                    MetaFieldId::Named(ident) => ident,
                    _ => unreachable!(),
                });
                let body = build_field_fn_variant_field_match(
                    meta_fields,
                    &syn_idents,
                    &mut accessor_builder,
                );

                quote! { #type_ident::#ident { #(#pat_idents: #pat_syn_idents,)* } => #body }
            }
        }
    });

    quote! {
        match *self {
            #(#variants)*
        }
    }
}

fn build_field_fn_variant_field_match<A>(
    meta_fields: &[MetaField],
    syn_idents: &[syn::Ident],
    mut accessor_builder: A,
) -> quote::Tokens
where
    A: FnMut(quote::Tokens) -> quote::Tokens,
{
    if meta_fields.is_empty() {
        quote!(::std::option::Option::None,)
    } else {
        let fields = meta_fields
            .iter()
            .zip(syn_idents)
            .map(|(f, syn_ident)| match f.id {
                MetaFieldId::Unnamed(ref i) => {
                    let i_usize = i.index as usize;
                    let accessor = accessor_builder(quote!(#syn_ident));

                    quote! {
                        ::type_info::FieldId::Unnamed(#i_usize) => #accessor
                    }
                }
                MetaFieldId::Named(i) => {
                    let i_str = i.as_ref();
                    let accessor = accessor_builder(quote!(#syn_ident));

                    quote! {
                        ::type_info::FieldId::Named(#i_str) => #accessor
                    }
                }
            });

        quote! {
            match id {
                #(#fields)*
                _ => ::std::option::Option::None,
            },
        }
    }
}

fn build_variant_fn(type_info: &MetaType) -> quote::Tokens {
    match type_info.data.variants {
        Some(ref meta_variants) if !meta_variants.is_empty() => {
            let variants = meta_variants.iter().map(|v| {
                let type_ident = type_info.ident;
                let ident = v.id.0;
                let ident_str = ident.as_ref();
                match v.fields.kind {
                    MetaFieldsKind::Unit => {
                        quote! { #type_ident::#ident => ::std::option::Option::Some(#ident_str), }
                    }
                    MetaFieldsKind::Unnamed(_) => {
                        quote! { #type_ident::#ident( .. ) => ::std::option::Option::Some(#ident_str), }
                    }
                    MetaFieldsKind::Named(_) => {
                        quote! { #type_ident::#ident { .. } => ::std::option::Option::Some(#ident_str), }
                    }
                }
            });

            quote! {
                fn variant(&self) -> ::std::option::Option<&str> {
                    match *self {
                        #(#variants)*
                    }
                }
            }
        }
        _ => quote!(),
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
    let ident = ast.ident.clone();

    MetaType {
        tokens,
        ident,
        data,
    }
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
        variants: None,
    }
}

fn build_data_enum(data_enum: &syn::DataEnum) -> MetaData {
    let variants = data_enum
        .variants
        .iter()
        .map(build_variant)
        .collect::<Vec<_>>();

    let tokens = {
        let variant_tokens = variants.iter().map(|v| &v.tokens);
        quote! {
            ::type_info::DataEnum {
                variants: &[
                    #(#variant_tokens,)*
                ],
            }
        }
    };

    MetaData {
        tokens,
        fields: None,
        variants: Some(variants),
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
        variants: None,
    }
}

fn build_variant(variant: &syn::Variant) -> MetaVariant {
    let ident = variant.ident.as_ref();
    let fields = build_fields(&variant.fields);
    let tokens = {
        let field_tokens = &fields.tokens;
        quote! {
            ::type_info::Variant {
                ident: #ident,
                fields: #field_tokens,
            }
        }
    };

    MetaVariant {
        tokens,
        id: MetaVariantId(variant.ident),
        fields,
    }
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
            kind: MetaFieldsKind::Unit,
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
    let kind = MetaFieldsKind::Named(fields.len());

    MetaFields {
        tokens,
        fields,
        kind,
    }
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
    let kind = MetaFieldsKind::Unnamed(fields.len());

    MetaFields {
        tokens,
        fields,
        kind,
    }
}

fn build_field(idx: usize, field: &syn::Field) -> MetaField {
    match field.ident {
        Some(ref ident) => {
            let ident_str = ident.as_ref();
            let ty = &field.ty;
            let tokens = quote! {
                ::type_info::Field {
                    id: ::type_info::FieldId::Named(#ident_str),
                    ident: ::std::option::Option::Some(#ident_str),
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
                    ident: ::std::option::Option::None,
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
            variants: self.variants,
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
            kind: self.kind,
            fields: self.fields,
        }
    }
}
