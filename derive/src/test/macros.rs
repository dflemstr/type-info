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

            fn unpretty_print<T: fmt::Display>(ts: T) -> String {
                let mut res = String::new();

                let raw_s = ts.to_string();
                let mut s = &raw_s[..];
                let mut indent = 0;
                while let Some(i) = s.find(&['(', '{', '[', ')', '}', ']', ';'][..]) {
                    match &s[i..i + 1] {
                        "(" | "{" | "[" => indent += 1,
                        ")" | "}" | "]" => indent -= 1,
                        _ => {}
                    }
                    res.push_str(&s[..i + 1]);
                    res.push('\n');
                    for _ in 0..indent {
                        res.push_str("    ");
                    }
                    s = s[i + 1..].trim_left_matches(' ');
                }
                res.push_str(s);
                res
            }

            if actual != expected {
                let actual_pretty = unpretty_print(actual);
                let expected_pretty = unpretty_print(expected);
                panic!("\
test_derive failed:
expected:
```
{}
```

got:
```
{}
```\n",
                    expected_pretty,
                    actual_pretty,
                );
            }
        }
    };
}
