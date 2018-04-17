use std::fmt;

use proc_macro2;
use quote;
use syn;

use super::type_info_test;

#[macro_use]
mod macros;

#[test]
fn test_struct_unit() {
    test_derive! {
        type_info_test {
            struct Simple;
        }
        expands to {
            impl ::type_info::TypeInfo for Simple {
                const TYPE: ::type_info::Type = ::type_info::Type {
                    id: ::type_info::TypeId::of::<Simple>(),
                    module: module_path!(),
                    ident: "Simple",
                    data: ::type_info::Data::Struct(::type_info::DataStruct {
                        fields: ::type_info::Fields::Unit,
                    }),
                };
            }

            impl ::type_info::DynamicTypeInfo for Simple {
                fn type_ref(&self) -> &'static ::type_info::Type {
                    &<Self as ::type_info::TypeInfo>::TYPE
                }
            }
        }
    }
}

#[test]
fn test_struct_named_fields() {
    test_derive! {
        type_info_test {
            struct Simple {
                name: String,
                age: u32,
            }
        }
        expands to {
            impl ::type_info::TypeInfo for Simple {
                const TYPE: ::type_info::Type = ::type_info::Type {
                    id: ::type_info::TypeId::of::<Simple>(),
                    module: module_path!(),
                    ident: "Simple",
                    data: ::type_info::Data::Struct(::type_info::DataStruct {
                        fields: ::type_info::Fields::Named(::type_info::FieldsNamed {
                            named: &[
                                ::type_info::Field {
                                    id: ::type_info::FieldId::Named("name"),
                                    ident: Some("name"),
                                    ty: <String as ::type_info::TryTypeInfo>::TRY_TYPE,
                                },
                                ::type_info::Field {
                                    id: ::type_info::FieldId::Named("age"),
                                    ident: Some("age"),
                                    ty: <u32 as ::type_info::TryTypeInfo>::TRY_TYPE,
                                },
                            ],
                        }),
                    }),
                };
            }

            impl ::type_info::DynamicTypeInfo for Simple {
                fn type_ref(&self) -> &'static ::type_info::Type {
                    &<Self as ::type_info::TypeInfo>::TYPE
                }
            }
        }
    }
}

#[test]
fn test_struct_unnamed_fields() {
    test_derive! {
        type_info_test {
            struct Simple(String, u32);
        }
        expands to {
            impl ::type_info::TypeInfo for Simple {
                const TYPE: ::type_info::Type = ::type_info::Type {
                    id: ::type_info::TypeId::of::<Simple>(),
                    module: module_path!(),
                    ident: "Simple",
                    data: ::type_info::Data::Struct(::type_info::DataStruct {
                        fields: ::type_info::Fields::Unnamed(::type_info::FieldsUnnamed {
                            unnamed: &[
                                ::type_info::Field {
                                    id: ::type_info::FieldId::Unnamed(0usize),
                                    ident: None,
                                    ty: <String as ::type_info::TryTypeInfo>::TRY_TYPE,
                                },
                                ::type_info::Field {
                                    id: ::type_info::FieldId::Unnamed(1usize),
                                    ident: None,
                                    ty: <u32 as ::type_info::TryTypeInfo>::TRY_TYPE,
                                },
                            ],
                        }),
                    }),
                };
            }

            impl ::type_info::DynamicTypeInfo for Simple {
                fn type_ref(&self) -> &'static ::type_info::Type {
                    &<Self as ::type_info::TypeInfo>::TYPE
                }
            }
        }
    }
}
