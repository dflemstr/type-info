#![feature(const_type_id)]

extern crate type_info;
#[macro_use]
extern crate type_info_derive;

#[derive(TypeInfo)]
struct SimpleNamed {
    foo: String,
    bar: i32,
}

#[derive(TypeInfo)]
struct SimpleUnnamed(String, i32);

#[cfg(test)]
mod tests {
    use type_info;

    #[test]
    fn type_get_field_named() {
        use type_info::TypeInfo;

        let simple = super::SimpleNamed {
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
    fn type_get_mut_field_named() {
        use type_info::TypeInfo;

        let mut simple = super::SimpleNamed {
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
    }

    #[test]
    fn type_get_field_named_wrong_type() {
        use type_info::TypeInfo;

        let simple = super::SimpleNamed {
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
    fn type_get_field_named_wrong_name() {
        use type_info::TypeInfo;

        let simple = super::SimpleNamed {
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
    fn type_get_field_unnamed() {
        use type_info::TypeInfo;

        let simple = super::SimpleUnnamed("hello".to_owned(), 3);

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
    fn type_get_mut_field_unnamed() {
        use type_info::TypeInfo;

        let mut simple = super::SimpleUnnamed("hello".to_owned(), 3);

        assert_eq!(
            Some(&mut "hello".to_owned()),
            simple.field_mut::<String>(type_info::FieldId::Unnamed(0))
        );
        assert_eq!(
            Some(&mut 3),
            simple.field_mut::<i32>(type_info::FieldId::Unnamed(1))
        );
    }

    #[test]
    fn type_get_field_unnamed_wrong_type() {
        use type_info::TypeInfo;

        let simple = super::SimpleUnnamed("hello".to_owned(), 3);

        assert_eq!(None, simple.field::<i32>(type_info::FieldId::Unnamed(0)));
        assert_eq!(None, simple.field::<String>(type_info::FieldId::Unnamed(1)));
    }

    #[test]
    fn type_get_field_unnamed_wrong_name() {
        use type_info::TypeInfo;

        let simple = super::SimpleUnnamed("hello".to_owned(), 3);

        assert_eq!(None, simple.field::<String>(type_info::FieldId::Unnamed(2)));
        assert_eq!(None, simple.field::<i32>(type_info::FieldId::Unnamed(3)));
    }
}
