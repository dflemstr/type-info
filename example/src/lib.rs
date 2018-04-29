#![feature(const_type_id)]

extern crate type_info;
#[macro_use]
extern crate type_info_derive;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, TypeInfo)]
struct NamedStruct {
    foo: String,
    bar: i32,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, TypeInfo)]
struct UnnamedStruct(String, i32);

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, TypeInfo)]
enum NamedFieldsEnum {
    Foo { foo: String, bar: i32 },
    Quz { bar: String, baz: u8 },
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, TypeInfo)]
enum UnnamedFieldsEnum {
    Foo(String, i32),
    Quz(String, u8),
}

#[cfg(test)]
mod tests {
    use type_info;

    #[test]
    fn struct_get_field_named() {
        use type_info::TypeInfo;

        let simple = super::NamedStruct {
            foo: "hello".to_owned(),
            bar: 3,
        };

        assert_eq!(
            Some(&"hello".to_owned()),
            simple.field::<String>(type_info::FieldId::Named("foo"))
        );
        assert_eq!(
            Some(&3),
            simple.field::<i32>(type_info::FieldId::Named("bar"))
        );
    }

    #[test]
    fn struct_get_mut_field_named() {
        use type_info::TypeInfo;

        let mut simple = super::NamedStruct {
            foo: "hello".to_owned(),
            bar: 3,
        };

        assert_eq!(
            Some(&mut "hello".to_owned()),
            simple.field_mut::<String>(type_info::FieldId::Named("foo"))
        );
        assert_eq!(
            Some(&mut 3),
            simple.field_mut::<i32>(type_info::FieldId::Named("bar"))
        );

        *simple
            .field_mut::<String>(type_info::FieldId::Named("foo"))
            .unwrap() = "world".to_owned();
        *simple
            .field_mut::<i32>(type_info::FieldId::Named("bar"))
            .unwrap() = 42;

        assert_eq!(
            simple,
            super::NamedStruct {
                foo: "world".to_owned(),
                bar: 42,
            }
        );
    }

    #[test]
    fn struct_get_field_named_wrong_type() {
        use type_info::TypeInfo;

        let simple = super::NamedStruct {
            foo: "hello".to_owned(),
            bar: 3,
        };

        assert_eq!(None, simple.field::<i32>(type_info::FieldId::Named("foo")));
        assert_eq!(
            None,
            simple.field::<String>(type_info::FieldId::Named("bar"))
        );
    }

    #[test]
    fn struct_get_field_named_wrong_name() {
        use type_info::TypeInfo;

        let simple = super::NamedStruct {
            foo: "hello".to_owned(),
            bar: 3,
        };

        assert_eq!(
            None,
            simple.field::<String>(type_info::FieldId::Named("foo3"))
        );
        assert_eq!(None, simple.field::<i32>(type_info::FieldId::Named("bar3")));
    }

    #[test]
    fn struct_get_field_unnamed() {
        use type_info::TypeInfo;

        let simple = super::UnnamedStruct("hello".to_owned(), 3);

        assert_eq!(
            Some(&"hello".to_owned()),
            simple.field::<String>(type_info::FieldId::Unnamed(0))
        );
        assert_eq!(
            Some(&3),
            simple.field::<i32>(type_info::FieldId::Unnamed(1))
        );
    }

    #[test]
    fn struct_get_mut_field_unnamed() {
        use type_info::TypeInfo;

        let mut simple = super::UnnamedStruct("hello".to_owned(), 3);

        assert_eq!(
            Some(&mut "hello".to_owned()),
            simple.field_mut::<String>(type_info::FieldId::Unnamed(0))
        );
        assert_eq!(
            Some(&mut 3),
            simple.field_mut::<i32>(type_info::FieldId::Unnamed(1))
        );

        *simple
            .field_mut::<String>(type_info::FieldId::Unnamed(0))
            .unwrap() = "world".to_owned();
        *simple
            .field_mut::<i32>(type_info::FieldId::Unnamed(1))
            .unwrap() = 42;

        assert_eq!(simple, super::UnnamedStruct("world".to_owned(), 42));
    }

    #[test]
    fn struct_get_field_unnamed_wrong_type() {
        use type_info::TypeInfo;

        let simple = super::UnnamedStruct("hello".to_owned(), 3);

        assert_eq!(None, simple.field::<i32>(type_info::FieldId::Unnamed(0)));
        assert_eq!(None, simple.field::<String>(type_info::FieldId::Unnamed(1)));
    }

    #[test]
    fn struct_get_field_unnamed_wrong_name() {
        use type_info::TypeInfo;

        let simple = super::UnnamedStruct("hello".to_owned(), 3);

        assert_eq!(None, simple.field::<String>(type_info::FieldId::Unnamed(2)));
        assert_eq!(None, simple.field::<i32>(type_info::FieldId::Unnamed(3)));
    }

    #[test]
    fn enum_variant() {
        use type_info::DynamicTypeInfo;

        let simple = super::NamedFieldsEnum::Foo {
            foo: "hello".to_owned(),
            bar: 3,
        };

        assert_eq!(Some("Foo"), simple.variant());
    }

    #[test]
    fn enum_get_field_named() {
        use type_info::TypeInfo;

        let simple = super::NamedFieldsEnum::Foo {
            foo: "hello".to_owned(),
            bar: 3,
        };

        assert_eq!(
            Some(&"hello".to_owned()),
            simple.field::<String>(type_info::FieldId::Named("foo"))
        );
        assert_eq!(
            Some(&3),
            simple.field::<i32>(type_info::FieldId::Named("bar"))
        );
    }

    #[test]
    fn enum_get_mut_field_named() {
        use type_info::TypeInfo;

        let mut simple = super::NamedFieldsEnum::Foo {
            foo: "hello".to_owned(),
            bar: 3,
        };

        assert_eq!(
            Some(&mut "hello".to_owned()),
            simple.field_mut::<String>(type_info::FieldId::Named("foo"))
        );
        assert_eq!(
            Some(&mut 3),
            simple.field_mut::<i32>(type_info::FieldId::Named("bar"))
        );

        *simple
            .field_mut::<String>(type_info::FieldId::Named("foo"))
            .unwrap() = "world".to_owned();
        *simple
            .field_mut::<i32>(type_info::FieldId::Named("bar"))
            .unwrap() = 42;

        assert_eq!(
            simple,
            super::NamedFieldsEnum::Foo {
                foo: "world".to_owned(),
                bar: 42,
            }
        );
    }

    #[test]
    fn enum_get_field_named_wrong_type() {
        use type_info::TypeInfo;

        let simple = super::NamedFieldsEnum::Foo {
            foo: "hello".to_owned(),
            bar: 3,
        };

        assert_eq!(None, simple.field::<i32>(type_info::FieldId::Named("foo")));
        assert_eq!(
            None,
            simple.field::<String>(type_info::FieldId::Named("bar"))
        );
    }

    #[test]
    fn enum_get_field_named_wrong_name() {
        use type_info::TypeInfo;

        let simple = super::NamedFieldsEnum::Foo {
            foo: "hello".to_owned(),
            bar: 3,
        };

        assert_eq!(
            None,
            simple.field::<String>(type_info::FieldId::Named("foo3"))
        );
        assert_eq!(None, simple.field::<i32>(type_info::FieldId::Named("bar3")));
    }

    #[test]
    fn enum_get_field_unnamed() {
        use type_info::TypeInfo;

        let simple = super::UnnamedFieldsEnum::Foo("hello".to_owned(), 3);

        assert_eq!(
            Some(&"hello".to_owned()),
            simple.field::<String>(type_info::FieldId::Unnamed(0))
        );
        assert_eq!(
            Some(&3),
            simple.field::<i32>(type_info::FieldId::Unnamed(1))
        );
    }

    #[test]
    fn enum_get_mut_field_unnamed() {
        use type_info::TypeInfo;

        let mut simple = super::UnnamedFieldsEnum::Foo("hello".to_owned(), 3);

        assert_eq!(
            Some(&mut "hello".to_owned()),
            simple.field_mut::<String>(type_info::FieldId::Unnamed(0))
        );
        assert_eq!(
            Some(&mut 3),
            simple.field_mut::<i32>(type_info::FieldId::Unnamed(1))
        );

        *simple
            .field_mut::<String>(type_info::FieldId::Unnamed(0))
            .unwrap() = "world".to_owned();
        *simple
            .field_mut::<i32>(type_info::FieldId::Unnamed(1))
            .unwrap() = 42;

        assert_eq!(
            simple,
            super::UnnamedFieldsEnum::Foo("world".to_owned(), 42)
        );
    }

    #[test]
    fn enum_get_field_unnamed_wrong_type() {
        use type_info::TypeInfo;

        let simple = super::UnnamedFieldsEnum::Foo("hello".to_owned(), 3);

        assert_eq!(None, simple.field::<i32>(type_info::FieldId::Unnamed(0)));
        assert_eq!(None, simple.field::<String>(type_info::FieldId::Unnamed(1)));
    }

    #[test]
    fn enum_get_field_unnamed_wrong_name() {
        use type_info::TypeInfo;

        let simple = super::UnnamedFieldsEnum::Foo("hello".to_owned(), 3);

        assert_eq!(None, simple.field::<String>(type_info::FieldId::Unnamed(2)));
        assert_eq!(None, simple.field::<i32>(type_info::FieldId::Unnamed(3)));
    }
}
