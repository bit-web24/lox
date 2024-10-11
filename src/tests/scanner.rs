use crate::scanner::Scanner;

#[test]
pub fn test_var() {
    let source = r#"
        var a = 123;
        var b = 456;
        var c = a + b;
        "#
    .to_string();

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(tokens.len(), 18);
}

#[test]
pub fn test_string() {
    use crate::scanner::Scanner;

    let source = r#""this is a string""#;
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    assert_eq!(tokens.len(), 2);
}
