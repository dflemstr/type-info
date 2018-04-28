use proc_macro2;
use quote;
use syn;

use super::type_info_test;

#[macro_use]
mod macros;
mod utils;

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
                fn field<TypeInfoA>(&self, id: ::type_info::FieldId) -> ::std::option::Option<&TypeInfoA>
                where
                    TypeInfoA: ::std::any::Any,
                {
                    match id {
                        ::type_info::FieldId::Named("name") => {
                            ::std::any::Any::downcast_ref::<TypeInfoA>(&self.name)
                        }
                        ::type_info::FieldId::Named("age") => {
                            ::std::any::Any::downcast_ref::<TypeInfoA>(&self.age)
                        }
                        _ => ::std::option::Option::None,
                    }
                }
                fn field_mut<TypeInfoA>(&mut self, id: ::type_info::FieldId) -> ::std::option::Option<&mut TypeInfoA>
                where
                    TypeInfoA: ::std::any::Any,
                {
                    match id {
                        ::type_info::FieldId::Named("name") => {
                            ::std::any::Any::downcast_mut::<TypeInfoA>(&mut self.name)
                        }
                        ::type_info::FieldId::Named("age") => {
                            ::std::any::Any::downcast_mut::<TypeInfoA>(&mut self.age)
                        }
                        _ => ::std::option::Option::None,
                    }
                }
            }
            impl ::type_info::DynamicTypeInfo for Simple {
                fn type_ref(&self) -> &'static ::type_info::Type {
                    &<Self as ::type_info::TypeInfo>::TYPE
                }
                fn field_any(&self, id: ::type_info::FieldId) -> ::std::option::Option<&::std::any::Any> {
                    match id {
                        ::type_info::FieldId::Named("name") => Some(&self.name),
                        ::type_info::FieldId::Named("age") => Some(&self.age),
                        _ => ::std::option::Option::None,
                    }
                }
                fn field_any_mut(&mut self, id: ::type_info::FieldId) -> ::std::option::Option<&mut ::std::any::Any> {
                    match id {
                        ::type_info::FieldId::Named("name") => Some(&mut self.name),
                        ::type_info::FieldId::Named("age") => Some(&mut self.age),
                        _ => ::std::option::Option::None,
                    }
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
                fn field<TypeInfoA>(&self, id: ::type_info::FieldId) -> ::std::option::Option<&TypeInfoA>
                where
                    TypeInfoA: ::std::any::Any,
                {
                    match id {
                        ::type_info::FieldId::Unnamed(0usize) => {
                            ::std::any::Any::downcast_ref::<TypeInfoA>(&self.0)
                        }
                        ::type_info::FieldId::Unnamed(1usize) => {
                            ::std::any::Any::downcast_ref::<TypeInfoA>(&self.1)
                        }
                        _ => ::std::option::Option::None,
                    }
                }
                fn field_mut<TypeInfoA>(&mut self, id: ::type_info::FieldId) -> ::std::option::Option<&mut TypeInfoA>
                where
                    TypeInfoA: ::std::any::Any,
                {
                    match id {
                        ::type_info::FieldId::Unnamed(0usize) => {
                            ::std::any::Any::downcast_mut::<TypeInfoA>(&mut self.0)
                        }
                        ::type_info::FieldId::Unnamed(1usize) => {
                            ::std::any::Any::downcast_mut::<TypeInfoA>(&mut self.1)
                        }
                        _ => ::std::option::Option::None,
                    }
                }
            }
            impl ::type_info::DynamicTypeInfo for Simple {
                fn type_ref(&self) -> &'static ::type_info::Type {
                    &<Self as ::type_info::TypeInfo>::TYPE
                }
                fn field_any(&self, id: ::type_info::FieldId) -> ::std::option::Option<&::std::any::Any> {
                    match id {
                        ::type_info::FieldId::Unnamed(0usize) => Some(&self.0),
                        ::type_info::FieldId::Unnamed(1usize) => Some(&self.1),
                        _ => ::std::option::Option::None,
                    }
                }
                fn field_any_mut(&mut self, id: ::type_info::FieldId) -> ::std::option::Option<&mut ::std::any::Any> {
                    match id {
                        ::type_info::FieldId::Unnamed(0usize) => Some(&mut self.0),
                        ::type_info::FieldId::Unnamed(1usize) => Some(&mut self.1),
                        _ => ::std::option::Option::None,
                    }
                }
            }
        }
    }
}

#[test]
fn test_struct_named_fields_generics() {
    test_derive! {
        type_info_test {
            struct Simple<A> {
                name: A,
                age: u32,
            }
        }
        expands to {
            impl<A: ::std::any::Any> ::type_info::TypeInfo for Simple<A> {
                const TYPE: ::type_info::Type = ::type_info::Type {
                    id: ::type_info::TypeId::of::<Simple<A>>(),
                    module: module_path!(),
                    ident: "Simple",
                    data: ::type_info::Data::Struct(::type_info::DataStruct {
                        fields: ::type_info::Fields::Named(::type_info::FieldsNamed {
                            named: &[
                                ::type_info::Field {
                                    id: ::type_info::FieldId::Named("name"),
                                    ident: Some("name"),
                                    ty: <A as ::type_info::TryTypeInfo>::TRY_TYPE,
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
                fn field<TypeInfoA>(&self, id: ::type_info::FieldId) -> ::std::option::Option<&TypeInfoA>
                where
                    TypeInfoA: ::std::any::Any,
                {
                    match id {
                        ::type_info::FieldId::Named("name") => {
                            ::std::any::Any::downcast_ref::<TypeInfoA>(&self.name)
                        }
                        ::type_info::FieldId::Named("age") => {
                            ::std::any::Any::downcast_ref::<TypeInfoA>(&self.age)
                        }
                        _ => ::std::option::Option::None,
                    }
                }
                fn field_mut<TypeInfoA>(&mut self, id: ::type_info::FieldId) -> ::std::option::Option<&mut TypeInfoA>
                where
                    TypeInfoA: ::std::any::Any,
                {
                    match id {
                        ::type_info::FieldId::Named("name") => {
                            ::std::any::Any::downcast_mut::<TypeInfoA>(&mut self.name)
                        }
                        ::type_info::FieldId::Named("age") => {
                            ::std::any::Any::downcast_mut::<TypeInfoA>(&mut self.age)
                        }
                        _ => ::std::option::Option::None,
                    }
                }
            }
            impl<A: ::std::any::Any> ::type_info::DynamicTypeInfo for Simple<A> {
                fn type_ref(&self) -> &'static ::type_info::Type {
                    &<Self as ::type_info::TypeInfo>::TYPE
                }
                fn field_any(&self, id: ::type_info::FieldId) -> ::std::option::Option<&::std::any::Any> {
                    match id {
                        ::type_info::FieldId::Named("name") => Some(&self.name),
                        ::type_info::FieldId::Named("age") => Some(&self.age),
                        _ => ::std::option::Option::None,
                    }
                }
                fn field_any_mut(&mut self, id: ::type_info::FieldId) -> ::std::option::Option<&mut ::std::any::Any> {
                    match id {
                        ::type_info::FieldId::Named("name") => Some(&mut self.name),
                        ::type_info::FieldId::Named("age") => Some(&mut self.age),
                        _ => ::std::option::Option::None,
                    }
                }
            }
        }
    }
}

#[test]
fn test_struct_unnamed_fields_generics() {
    test_derive! {
        type_info_test {
            struct Simple<A>(A, u32);
        }
        expands to {
            impl<A: ::std::any::Any> ::type_info::TypeInfo for Simple<A> {
                const TYPE: ::type_info::Type = ::type_info::Type {
                    id: ::type_info::TypeId::of::<Simple<A>>(),
                    module: module_path!(),
                    ident: "Simple",
                    data: ::type_info::Data::Struct(::type_info::DataStruct {
                        fields: ::type_info::Fields::Unnamed(::type_info::FieldsUnnamed {
                            unnamed: &[
                                ::type_info::Field {
                                    id: ::type_info::FieldId::Unnamed(0usize),
                                    ident: None,
                                    ty: <A as ::type_info::TryTypeInfo>::TRY_TYPE,
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
                fn field<TypeInfoA>(&self, id: ::type_info::FieldId) -> ::std::option::Option<&TypeInfoA>
                where
                    TypeInfoA: ::std::any::Any,
                {
                    match id {
                        ::type_info::FieldId::Unnamed(0usize) => {
                            ::std::any::Any::downcast_ref::<TypeInfoA>(&self.0)
                        }
                        ::type_info::FieldId::Unnamed(1usize) => {
                            ::std::any::Any::downcast_ref::<TypeInfoA>(&self.1)
                        }
                        _ => ::std::option::Option::None,
                    }
                }
                fn field_mut<TypeInfoA>(&mut self, id: ::type_info::FieldId) -> ::std::option::Option<&mut TypeInfoA>
                where
                    TypeInfoA: ::std::any::Any,
                {
                    match id {
                        ::type_info::FieldId::Unnamed(0usize) => {
                            ::std::any::Any::downcast_mut::<TypeInfoA>(&mut self.0)
                        }
                        ::type_info::FieldId::Unnamed(1usize) => {
                            ::std::any::Any::downcast_mut::<TypeInfoA>(&mut self.1)
                        }
                        _ => ::std::option::Option::None,
                    }
                }
            }
            impl<A: ::std::any::Any> ::type_info::DynamicTypeInfo for Simple<A> {
                fn type_ref(&self) -> &'static ::type_info::Type {
                    &<Self as ::type_info::TypeInfo>::TYPE
                }
                fn field_any(&self, id: ::type_info::FieldId) -> ::std::option::Option<&::std::any::Any> {
                    match id {
                        ::type_info::FieldId::Unnamed(0usize) => Some(&self.0),
                        ::type_info::FieldId::Unnamed(1usize) => Some(&self.1),
                        _ => ::std::option::Option::None,
                    }
                }
                fn field_any_mut(&mut self, id: ::type_info::FieldId) -> ::std::option::Option<&mut ::std::any::Any> {
                    match id {
                        ::type_info::FieldId::Unnamed(0usize) => Some(&mut self.0),
                        ::type_info::FieldId::Unnamed(1usize) => Some(&mut self.1),
                        _ => ::std::option::Option::None,
                    }
                }
            }
        }
    }
}

#[test]
fn test_enum_unit() {
    test_derive! {
        type_info_test {
            enum Simple {}
        }
        expands to {
            impl ::type_info::TypeInfo for Simple {
                const TYPE: ::type_info::Type = ::type_info::Type {
                    id: ::type_info::TypeId::of::<Simple>(),
                    module: module_path!(),
                    ident: "Simple",
                    data: ::type_info::Data::Enum(::type_info::DataEnum { variants: &[], }),
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
fn test_enum_c_like() {
    test_derive! {
        type_info_test {
            enum Simple {
                First,
                Second,
            }
        }
        expands to {
            impl ::type_info::TypeInfo for Simple {
                const TYPE: ::type_info::Type = ::type_info::Type {
                    id: ::type_info::TypeId::of::<Simple>(),
                    module: module_path!(),
                    ident: "Simple",
                    data: ::type_info::Data::Enum(::type_info::DataEnum {
                        variants: &[
                            ::type_info::Variant {
                                ident: "First",
                                fields: ::type_info::Fields::Unit,
                            },
                            ::type_info::Variant {
                                ident: "Second",
                                fields: ::type_info::Fields::Unit,
                            },
                        ],
                    }),
                };
            }
            impl ::type_info::DynamicTypeInfo for Simple {
                fn type_ref(&self) -> &'static ::type_info::Type {
                    &<Self as ::type_info::TypeInfo>::TYPE
                }
                fn variant(&self) -> ::std::option::Option<&str> {
                    match *self {
                        Simple::First => Some("First"),
                        Simple::Second => Some("Second"),
                    }
                }
            }
        }
    }
}

#[test]
fn test_enum_unnamed_fields() {
    test_derive! {
        type_info_test {
            enum Simple {
                First(usize, i32),
                Second(String),
            }
        }
        expands to {
            impl ::type_info::TypeInfo for Simple {
                const TYPE: ::type_info::Type = ::type_info::Type {
                    id: ::type_info::TypeId::of::<Simple>(),
                    module: module_path!(),
                    ident: "Simple",
                    data: ::type_info::Data::Enum(::type_info::DataEnum {
                        variants: &[
                            ::type_info::Variant {
                                ident: "First",
                                fields: ::type_info::Fields::Unnamed(::type_info::FieldsUnnamed {
                                    unnamed: &[
                                        ::type_info::Field {
                                            id: ::type_info::FieldId::Unnamed(0usize),
                                            ident: None,
                                            ty: <usize as ::type_info::TryTypeInfo>::TRY_TYPE,
                                        },
                                        ::type_info::Field {
                                            id: ::type_info::FieldId::Unnamed(1usize),
                                            ident: None,
                                            ty: <i32 as ::type_info::TryTypeInfo>::TRY_TYPE,
                                        },
                                    ],
                                }),
                            },
                            ::type_info::Variant {
                                ident: "Second",
                                fields: ::type_info::Fields::Unnamed(::type_info::FieldsUnnamed {
                                    unnamed: &[
                                        ::type_info::Field {
                                            id: ::type_info::FieldId::Unnamed(0usize),
                                            ident: None,
                                            ty: <String as ::type_info::TryTypeInfo>::TRY_TYPE,
                                        },
                                    ],
                                }),
                            },
                        ],
                    }),
                };
            }
            impl ::type_info::DynamicTypeInfo for Simple {
                fn type_ref(&self) -> &'static ::type_info::Type {
                    &<Self as ::type_info::TypeInfo>::TYPE
                }
                fn variant(&self) -> ::std::option::Option<&str> {
                    match *self {
                        Simple::First(..) => Some("First"),
                        Simple::Second(..) => Some("Second"),
                    }
                }
            }
        }
    }
}

#[test]
fn test_enum_named_fields() {
    test_derive! {
        type_info_test {
            enum Simple {
                First { a: usize, b: i32 },
                Second { a: String },
            }
        }
        expands to {
            impl ::type_info::TypeInfo for Simple {
                const TYPE: ::type_info::Type = ::type_info::Type {
                    id: ::type_info::TypeId::of::<Simple>(),
                    module: module_path!(),
                    ident: "Simple",
                    data: ::type_info::Data::Enum(::type_info::DataEnum {
                        variants: &[
                            ::type_info::Variant {
                                ident: "First",
                                fields: ::type_info::Fields::Named(::type_info::FieldsNamed {
                                    named: &[
                                        ::type_info::Field {
                                            id: ::type_info::FieldId::Named("a"),
                                            ident: Some("a"),
                                            ty: <usize as ::type_info::TryTypeInfo>::TRY_TYPE,
                                        },
                                        ::type_info::Field {
                                            id: ::type_info::FieldId::Named("b"),
                                            ident: Some("b"),
                                            ty: <i32 as ::type_info::TryTypeInfo>::TRY_TYPE,
                                        },
                                    ],
                                }),
                            },
                            ::type_info::Variant {
                                ident: "Second",
                                fields: ::type_info::Fields::Named(::type_info::FieldsNamed {
                                    named: &[
                                        ::type_info::Field {
                                            id: ::type_info::FieldId::Named("a"),
                                            ident: Some("a"),
                                            ty: <String as ::type_info::TryTypeInfo>::TRY_TYPE,
                                        },
                                    ],
                                }),
                            },
                        ],
                    }),
                };
            }
            impl ::type_info::DynamicTypeInfo for Simple {
                fn type_ref(&self) -> &'static ::type_info::Type {
                    &<Self as ::type_info::TypeInfo>::TYPE
                }
                fn variant(&self) -> ::std::option::Option<&str> {
                    match *self {
                        Simple::First { .. } => Some("First"),
                        Simple::Second { .. } => Some("Second"),
                    }
                }
            }
        }
    }
}
