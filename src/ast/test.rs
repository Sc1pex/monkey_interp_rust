use super::*;
use crate::lexer::Lexer;

#[test]
fn let_stmt() {
    let input = r#"
let x = 10;
let y = 9;
let baz = 323325;
    "#
    .into();

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let Program { statements } = parser.parse().unwrap();

    let expected_idents = vec!["x", "y", "baz"];

    assert_eq!(expected_idents.len(), statements.len());
    for (exp, stmt) in expected_idents.into_iter().zip(statements.into_iter()) {
        check_let_stmt(exp, stmt)
    }
}

fn check_let_stmt(exp: &str, stmt: Statement) {
    match stmt {
        Statement::Let(LetStmt { ident, .. }) => assert_eq!(exp, ident),
        _ => panic!("Expected let statement, found {:?}", stmt),
    }
}

#[test]
fn return_stmt() {
    let input = r#"
return 5;
return 10;
return 993322; "#
        .into();

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let Program { statements } = parser.parse().unwrap();

    assert_eq!(3, statements.len());
    for stmt in statements {
        assert!(matches!(stmt, Statement::Return(..)));
    }
}

#[test]
fn ident_expr() {
    let input = "foobar;".into();

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let Program { statements } = parser.parse().unwrap();

    assert_eq!(1, statements.len());
    let expr = match statements[0] {
        Statement::Expression(ref e) => e,
        _ => panic!("expected ExpressionStatement, got {:?}", statements[0]),
    };

    match &expr.expr {
        Expression::Ident(i) => assert_eq!(i, "foobar"),
        e => panic!("expected Ident expression, got {:?}", e),
    }
}

#[test]
fn number_expr() {
    let input = "69420;".into();

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let Program { statements } = parser.parse().unwrap();

    assert_eq!(1, statements.len());
    let expr = match statements[0] {
        Statement::Expression(ref e) => e,
        _ => panic!("expected ExpressionStatement, got {:?}", statements[0]),
    };

    match &expr.expr {
        Expression::Number(x) => assert_eq!(*x, 69420),
        e => panic!("expected Ident expression, got {:?}", e),
    }
}

#[test]
fn ast_to_string() {
    let ast = Program {
        statements: vec![
            Statement::Let(LetStmt {
                ident: "myVar".into(),
                expr: Expression::Ident("anotherVar".into()),
            }),
            Statement::Return(ReturnStmt {
                expr: Expression::Ident("y".into()),
            }),
        ],
    };

    let expected = r#"let myVar = anotherVar;
return y;
"#;
    assert_eq!(ast.to_string(), expected);
}
