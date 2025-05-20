use proptest::prelude::*;
use test_each_file::test_each_file;

use sillydrageon::driver;

const FAIL_VALID: &str = "parsing a valid program should never fail";
const FAIL_INVALID: &str = "parsing an invalid program should never succeed";

test_each_file! { in "tests/valid/" => test_parse_valid }
fn test_parse_valid(program: &str) {
    let tokens = driver::tokenize(program).expect(FAIL_VALID);
    driver::parser(tokens).expect(FAIL_VALID);
}

test_each_file! { in "tests/invalid/" => test_parse_invalid }
fn test_parse_invalid(program: &str) {
    let tokens = driver::tokenize(program).expect(FAIL_INVALID);
    if driver::parser(tokens).is_ok() {
        panic!("{}", FAIL_INVALID)
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
