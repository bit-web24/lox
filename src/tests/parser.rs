use crate::interpreter::Interpreter;
use crate::{object::Object, scanner};

#[test]
fn print() {
    let source = "print 20;".to_string();
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(tokens.len(), 3 + 1); // 'print', '20', ';' + EOF

    let mut parser = crate::parser::Parser::new(tokens);
    let statements = parser.parse::<Object>().unwrap();
    assert_eq!(statements.len(), 1);

    let mut interpreter = Interpreter::new();
    interpreter.interpret(statements).unwrap();

    // Capture stdout for assertion.
}
