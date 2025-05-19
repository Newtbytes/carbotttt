use proptest::prelude::*;
use test_each_file::test_each_file;

use sillydrageon::driver;

test_each_file! { in "tests/valid/" => test_parse_valid }
fn test_parse_valid(program: &str) {
    let tokens = driver::tokenize(program).unwrap();
    driver::parser(tokens).unwrap();
}

test_each_file! { in "tests/invalid/" => test_parse_invalid }
fn test_parse_invalid(program: &str) {
    let tokens = driver::tokenize(program).unwrap();
    if let Ok(_) = driver::parser(tokens) {
        panic!()
    }
}

proptest! {
    #[test]
    fn doesnt_crash(s in any::<String>()) {
        if let Ok(tokens) = driver::tokenize(&s) {
            let _ = driver::parser(tokens);
        }
    }
}
