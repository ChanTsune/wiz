use super::*;

trait StrExt {
    fn trim_line_head_whitespaces(&self) -> String;
}

impl StrExt for &str {
    fn trim_line_head_whitespaces(&self) -> String {
        self.split_terminator('\n')
            .map(|i| i.trim_start_matches(' '))
            .filter(|i| !i.is_empty())
            .map(|i| format!("{}\n", i))
            .collect()
    }
}

fn check_raw_str(s: &str, expected_hashes: u16, expected_err: Option<RawStrError>) {
    let s = &format!("r{}", s);
    let mut cursor = Cursor::new(s);
    cursor.bump();
    let (n_hashes, err) = cursor.raw_double_quoted_string(0);
    assert_eq!(n_hashes, expected_hashes);
    assert_eq!(err, expected_err);
}

#[test]
fn test_naked_raw_str() {
    check_raw_str(r#""abc""#, 0, None);
}

#[test]
fn test_raw_no_start() {
    check_raw_str(r##""abc"#"##, 0, None);
}

#[test]
fn test_too_many_terminators() {
    // this error is handled in the parser later
    check_raw_str(r###"#"abc"##"###, 1, None);
}

#[test]
fn test_unterminated() {
    check_raw_str(
        r#"#"abc"#,
        1,
        Some(RawStrError::NoTerminator {
            expected: 1,
            found: 0,
            possible_terminator_offset: None,
        }),
    );
    check_raw_str(
        r###"##"abc"#"###,
        2,
        Some(RawStrError::NoTerminator {
            expected: 2,
            found: 1,
            possible_terminator_offset: Some(7),
        }),
    );
    // We're looking for "# not just any #
    check_raw_str(
        r###"##"abc#"###,
        2,
        Some(RawStrError::NoTerminator {
            expected: 2,
            found: 0,
            possible_terminator_offset: None,
        }),
    )
}

#[test]
fn test_invalid_start() {
    check_raw_str(
        r##"#~"abc"#"##,
        1,
        Some(RawStrError::InvalidStarter { bad_char: '~' }),
    );
}

#[test]
fn test_unterminated_no_pound() {
    check_raw_str(
        r#"""#,
        0,
        Some(RawStrError::NoTerminator {
            expected: 0,
            found: 0,
            possible_terminator_offset: None,
        }),
    );
}

#[test]
fn test_valid_shebang() {
    let input = "#!/usr/bin/wizrun\nlet x = 5;";
    assert_eq!(strip_shebang(input), Some(17));
}

#[test]
fn test_invalid_shebang_valid_wiz_syntax() {
    let input = "#!    [bad_attribute]";
    assert_eq!(strip_shebang(input), None);
}

#[test]
fn test_shebang_second_line() {
    // Because shebangs are interpreted by the kernel, they must be on the first line
    let input = "\n#!/bin/bash";
    assert_eq!(strip_shebang(input), None);
}

#[test]
fn test_shebang_space() {
    let input = "#!    /bin/bash";
    assert_eq!(strip_shebang(input), Some(input.len()));
}

#[test]
fn test_shebang_empty_shebang() {
    let input = "#!    \n[attribute(foo)]";
    assert_eq!(strip_shebang(input), None);
}

#[test]
fn test_invalid_shebang_comment() {
    let input = "#!//bin/ami/a/comment\n[";
    assert_eq!(strip_shebang(input), None)
}

#[test]
fn test_invalid_shebang_another_comment() {
    let input = "#!/*bin/ami/a/comment*/\n[attribute";
    assert_eq!(strip_shebang(input), None)
}

#[test]
fn test_shebang_valid_wiz_after() {
    let input = "#!/*bin/ami/a/comment*/\npub fn main() {}";
    assert_eq!(strip_shebang(input), Some(23))
}

#[test]
fn test_shebang_followed_by_attrib() {
    let input = "#!/bin/wiz-scripts\n#![allow_unused(true)]";
    assert_eq!(strip_shebang(input), Some(18));
}

fn check_lexing(src: &str, expect: String) {
    let actual: String = tokenize(src)
        .map(|token| format!("{:?}\n", token))
        .collect();
    assert_eq!(actual, expect)
}

#[test]
fn smoke_test() {
    check_lexing(
        "/* my source file */ fn main() { println!(\"zebra\"); }\n",
        r#"
            Token { kind: BlockComment { doc_style: None, terminated: true }, len: 20 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Identifier, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Identifier, len: 4 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrace, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Identifier, len: 7 }
            Token { kind: Bang, len: 1 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 7 }, len: 7 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Semi, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: CloseBrace, len: 1 }
            Token { kind: Whitespace, len: 1 }
        "#
        .trim_line_head_whitespaces(),
    )
}

#[test]
fn comment_flavors() {
    check_lexing(
        r"
// line
//// line as well
/// outer doc line
//! inner doc line
/* block */
/**/
/*** also block */
/** outer doc block */
/*! inner doc block */
",
        r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: LineComment { doc_style: None }, len: 7 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: LineComment { doc_style: None }, len: 17 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: LineComment { doc_style: Some(Outer) }, len: 18 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: LineComment { doc_style: Some(Inner) }, len: 18 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: BlockComment { doc_style: None, terminated: true }, len: 11 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: BlockComment { doc_style: None, terminated: true }, len: 4 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: BlockComment { doc_style: None, terminated: true }, len: 18 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: BlockComment { doc_style: Some(Outer), terminated: true }, len: 22 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: BlockComment { doc_style: Some(Inner), terminated: true }, len: 22 }
            Token { kind: Whitespace, len: 1 }
        "#
        .trim_line_head_whitespaces(),
    )
}

#[test]
fn nested_block_comments() {
    check_lexing(
        "/* /* */ */'a'",
        r#"
            Token { kind: BlockComment { doc_style: None, terminated: true }, len: 11 }
            Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 3 }, len: 3 }
        "#
        .trim_line_head_whitespaces(),
    )
}

#[test]
fn characters() {
    check_lexing(
        "'a' ' ' '\\n'",
        r#"
            Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 3 }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 3 }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 4 }, len: 4 }
        "#
        .trim_line_head_whitespaces(),
    );
}

#[test]
fn lifetime() {
    check_lexing(
        "'abc",
        r#"
            Token { kind: Lifetime { starts_with_number: false }, len: 4 }
        "#
        .trim_line_head_whitespaces(),
    );
}

#[test]
fn raw_string() {
    check_lexing(
        "r###\"\"#a\\b\x00c\"\"###",
        r#"
            Token { kind: Literal { kind: RawStr { n_hashes: 3, err: None }, suffix_start: 17 }, len: 17 }
        "#.trim_line_head_whitespaces(),
    )
}

#[test]
fn raw_identifier() {
    check_lexing(
        "`let`",
        r#"
            Token { kind: RawIdentifier { err: None }, len: 5 }
        "#
        .trim_line_head_whitespaces(),
    )
}

#[test]
fn literal_suffixes() {
    check_lexing(
        r####"
'a'
b'a'
"a"
b"a"
1234
0b101
0xABC
1.0
1.0e10
2us
r###"raw"###suffix
br###"raw"###suffix
"####,
        r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 3 }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Byte { terminated: true }, suffix_start: 4 }, len: 4 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 3 }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: ByteStr { terminated: true }, suffix_start: 4 }, len: 4 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false }, suffix_start: 4 }, len: 4 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Int { base: Binary, empty_int: false }, suffix_start: 5 }, len: 5 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: false }, suffix_start: 5 }, len: 5 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Float { base: Decimal, empty_exponent: false }, suffix_start: 3 }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Float { base: Decimal, empty_exponent: false }, suffix_start: 6 }, len: 6 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false }, suffix_start: 1 }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: RawStr { n_hashes: 3, err: None }, suffix_start: 12 }, len: 18 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: RawByteStr { n_hashes: 3, err: None }, suffix_start: 13 }, len: 19 }
            Token { kind: Whitespace, len: 1 }
        "#.trim_line_head_whitespaces(),
    )
}
