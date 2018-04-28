macro_rules! test_derive {
    ($name:path { $($i:tt)* } expands to { $($o:tt)* }) => {
        {
            #[allow(dead_code)]
            fn ensure_compiles() {
                $($i)*
                $($o)*
            }

            test_derive!($name { $($i)* } expands to { $($o)* } no_build);
        }
    };
    ($name:path { $($i:tt)* } expands to { $($o:tt)* } no_build) => {
        {
            let i = stringify!($($i)*);
            let o = stringify!($($o)*);

            let parsed = syn::parse_str::<proc_macro2::TokenStream>(i).unwrap();
            let actual = $name(parsed);
            let expected = quote::ToTokens::into_tokens(&syn::parse_str::<syn::File>(o).unwrap());

            ::test::utils::assert_eq_tokens(&expected, &actual);
        }
    };
}
