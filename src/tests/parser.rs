use crate::{error, interpreter, parser, scanner};

#[test]
fn test_print_statement() {
    let source = "print 20;".to_string();
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(tokens.len(), 3 + 1); // 'print', '20', ';' + EOF

    let mut parser = parser::Parser::new(tokens);
    let statements = parser.parse().unwrap();
    assert_eq!(statements.len(), 1);

    let mut interpreter = interpreter::Interpreter::new();
    let result = interpreter.interpret(statements);

    assert!(result.is_ok());
}

#[test]
fn test_assignment_expression() {
    let source = r#"a = 20;"#.to_string();
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(tokens.len(), 5); // 'a', '=', '20', ';' + EOF

    let mut parser = parser::Parser::new(tokens);
    let statements = parser.parse().unwrap();
    assert_eq!(statements.len(), 1);

    let mut interpreter = interpreter::Interpreter::new();
    let result = interpreter.interpret(statements);

    assert!(result.is_err());
    assert_eq!(
        result
            .unwrap_err()
            .downcast::<error::LoxError>()
            .unwrap()
            .to_string(),
        "RuntimeError [line 1] : Undefined variable 'a'."
    );
}

#[test]
fn test_variable_declaration_and_assignment() {
    let source = r#"var a = 20;
    a = "bittu";"#
        .to_string();

    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(tokens.len(), 10); // 'var', 'a', '=', '20', ';', 'a', '=', 'bittu', ';' + EOF

    let mut parser = parser::Parser::new(tokens);
    let statements = parser.parse().unwrap();
    assert_eq!(statements.len(), 2);

    let mut interpreter = interpreter::Interpreter::new();
    let result = interpreter.interpret(statements);

    assert!(result.is_ok());
}
