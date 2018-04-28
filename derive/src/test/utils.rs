use std::fs;
use std::io;
use std::process;

use quote;

pub fn assert_eq_tokens(expected: &quote::Tokens, actual: &quote::Tokens) {
    if actual != expected {
        let actual_string = actual.to_string();
        let expected_string = expected.to_string();

        let actual_pretty = pretty_print(&actual_string);
        let expected_pretty = pretty_print(&expected_string);
        let diff = diff(
            &unpretty_print(&expected_string),
            &unpretty_print(&actual_string),
        ).unwrap();
        panic!(
            "\
test_derive failed:
expected:
```
{}
```

got:
```
{}
```

diff:
```
{}
```
",
            expected_pretty, actual_pretty, diff,
        );
    }
}

use tempfile;

fn diff(expected: &str, actual: &str) -> io::Result<String> {
    use std::io::Write;

    let dir = tempfile::tempdir()?;
    let dir_path = dir.path();

    let expected_path = dir_path.join("expected");
    let actual_path = dir_path.join("actual");

    fs::File::create(&expected_path)?.write_all(expected.as_bytes())?;
    fs::File::create(&actual_path)?.write_all(actual.as_bytes())?;

    let output = process::Command::new("diff")
        .args(
            [
                "-u",
                &expected_path.to_string_lossy(),
                &actual_path.to_string_lossy(),
            ].iter(),
        )
        .output()?;

    Ok(String::from_utf8(output.stdout).unwrap())
}

fn pretty_print(s: &str) -> String {
    pretty_print_rustfmt(s).unwrap_or_else(|_| unpretty_print(s))
}

fn pretty_print_rustfmt(s: &str) -> io::Result<String> {
    use std::io::Write;

    let mut process = process::Command::new("rustfmt")
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()?;

    process.stdin.as_mut().unwrap().write_all(s.as_bytes())?;
    let output = process.wait_with_output()?;

    Ok(String::from_utf8(output.stdout).unwrap())
}

fn unpretty_print(mut s: &str) -> String {
    let mut res = String::new();

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
