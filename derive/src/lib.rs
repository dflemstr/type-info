#![feature(const_type_id)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;
#[cfg(test)]
extern crate type_info;

#[cfg(test)]
mod test;

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

fn impl_type_info(ast: syn::DeriveInput) -> quote::Tokens {
    let ident = &ast.ident;
    let type_info = build_type_info(&ast);

    quote! {
        impl ::type_info::TypeInfo for #ident {
            const TYPE: ::type_info::Type = #type_info;
        }

        impl ::type_info::DynamicTypeInfo for #ident {
            fn type_ref(&self) -> &'static ::type_info::Type {
                &<Self as ::type_info::TypeInfo>::TYPE
            }
        }
    }
}

fn build_type_info(ast: &syn::DeriveInput) -> quote::Tokens {
    let ident = &ast.ident;
    let ident_str = ident.as_ref();
    let data = build_data(&ast.data);

    quote! {
        ::type_info::Type {
            id: ::type_info::TypeId::of::<#ident>(),
            module: module_path!(),
            ident: #ident_str,
            data: #data,
        }
    }
}

fn build_data(data: &syn::Data) -> quote::Tokens {
    match *data {
        syn::Data::Struct(ref data_struct) => {
            let data_struct = build_data_struct(data_struct);

            quote! {
                ::type_info::Data::Struct(#data_struct)
            }
        }
        syn::Data::Enum(ref data_enum) => {
            let data_enum = build_data_enum(data_enum);

            quote! {
                ::type_info::Data::Enum(#data_enum)
            }
        }
        syn::Data::Union(ref data_union) => {
            let data_union = build_data_union(data_union);

            quote! {
                ::type_info::Data::Union(#data_union)
            }
        }
    }
}

fn build_data_struct(data_struct: &syn::DataStruct) -> quote::Tokens {
    let fields = build_fields(&data_struct.fields);

    quote! {
        ::type_info::DataStruct {
            fields: #fields,
        }
    }
}

fn build_data_enum(data_enum: &syn::DataEnum) -> quote::Tokens {
    let variants = data_enum.variants.iter().map(build_variant);

    quote! {
        ::type_info::DataEnum {
            variants: &[
                #(#variants,)*
            ],
        }
    }
}

fn build_data_union(data_union: &syn::DataUnion) -> quote::Tokens {
    let fields_named = build_fields_named(&data_union.fields);

    quote! {
        ::type_info::DataUnion {
            fields: #fields_named,
        }
    }
}

fn build_variant(variant: &syn::Variant) -> quote::Tokens {
    let ident = variant.ident.as_ref();
    let fields = build_fields(&variant.fields);

    quote! {
        ::type_info::Variant {
            ident: #ident,
            fields: #fields,
        }
    }
}

fn build_fields(fields: &syn::Fields) -> quote::Tokens {
    match *fields {
        syn::Fields::Named(ref fields_named) => {
            let fields_named = build_fields_named(fields_named);

            quote! {
                ::type_info::Fields::Named(#fields_named)
            }
        }
        syn::Fields::Unnamed(ref fields_unnamed) => {
            let fields_unnamed = build_fields_unnamed(fields_unnamed);

            quote! {
                ::type_info::Fields::Unnamed(#fields_unnamed)
            }
        }
        syn::Fields::Unit => {
            quote! {
                ::type_info::Fields::Unit
            }
        }
    }
}

fn build_fields_named(fields_named: &syn::FieldsNamed) -> quote::Tokens {
    let named = fields_named.named.iter().enumerate().map(|(i, f)| build_field(i, f));

    quote! {
        ::type_info::FieldsNamed {
            named: &[
                #(#named,)*
            ],
        }
    }
}

fn build_fields_unnamed(fields_unnamed: &syn::FieldsUnnamed) -> quote::Tokens {
    let unnamed = fields_unnamed.unnamed.iter().enumerate().map(|(i, f)| build_field(i, f));

    quote! {
        ::type_info::FieldsUnnamed {
            unnamed: &[
                #(#unnamed,)*
            ],
        }
    }
}

fn build_field(idx: usize, field: &syn::Field) -> quote::Tokens {
    match field.ident {
        Some(ref ident) => {
            let ident_str = ident.as_ref();
            let ty = &field.ty;
            quote! {
                ::type_info::Field {
                    id: ::type_info::FieldId::Named(#ident_str),
                    ident: Some(#ident_str),
                    ty: <#ty as ::type_info::TryTypeInfo>::TRY_TYPE,
                }
            }
        }
        None => {
            let ty = &field.ty;
            quote! {
                ::type_info::Field {
                    id: ::type_info::FieldId::Unnamed(#idx),
                    ident: None,
                    ty: <#ty as ::type_info::TryTypeInfo>::TRY_TYPE,
                }
            }
        }
    }
}
