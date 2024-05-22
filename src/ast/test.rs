use super::*;
use crate::lexer::Lexer;

#[test]
fn let_stmt() {
    let inputs = vec![
        (
            "let x = 10;",
            Statement::Let(LetStmt {
                ident: "x".into(),
                expr: Expression::Number(10),
            }),
        ),
        (
            "let y = true;",
            Statement::Let(LetStmt {
                ident: "y".into(),
                expr: Expression::Bool(true),
            }),
        ),
        (
            "let baz = y;",
            Statement::Let(LetStmt {
                ident: "baz".into(),
                expr: Expression::Ident("y".into()),
            }),
        ),
        (
            "let baz = \"foobar\";",
            Statement::Let(LetStmt {
                ident: "baz".into(),
                expr: Expression::String("foobar".into()),
            }),
        ),
    ];

    for (inp, expect) in inputs {
        let lexer = Lexer::new(inp.into());
        let mut parser = Parser::new(lexer);

        let Program { statements } = parser.parse().unwrap();

        assert_eq!(1, statements.len());
        assert_eq!(statements[0], expect);
    }
}

#[test]
fn return_stmt() {
    let inputs = vec![
        (
            "return 5;",
            Statement::Return(ReturnStmt {
                expr: Expression::Number(5),
            }),
        ),
        (
            "return false;",
            Statement::Return(ReturnStmt {
                expr: Expression::Bool(false),
            }),
        ),
        (
            "return foobar;",
            Statement::Return(ReturnStmt {
                expr: Expression::Ident("foobar".into()),
            }),
        ),
    ];

    for (inp, expect) in inputs {
        let lexer = Lexer::new(inp.into());
        let mut parser = Parser::new(lexer);

        let Program { statements } = parser.parse().unwrap();

        assert_eq!(1, statements.len());
        assert_eq!(statements[0], expect);
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

    match &expr {
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

    match &expr {
        Expression::Number(x) => assert_eq!(*x, 69420),
        e => panic!("expected Number expression, got {:?}", e),
    }
}

#[test]
fn string_expr() {
    let input = "\"hello there\";".into();

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let Program { statements } = parser.parse().unwrap();

    assert_eq!(1, statements.len());
    let expr = match statements[0] {
        Statement::Expression(ref e) => e,
        _ => panic!("expected ExpressionStatement, got {:?}", statements[0]),
    };

    match &expr {
        Expression::String(s) => assert_eq!(s, "hello there"),
        e => panic!("expected String expression, got {:?}", e),
    }
}

#[test]
fn prefix_expr() {
    let inputs = vec![
        (
            "!5",
            PrefixExpr {
                operator: TokenType::Bang,
                right: Box::new(Expression::Number(5)),
            },
        ),
        (
            "-abc",
            PrefixExpr {
                operator: TokenType::Minus,
                right: Box::new(Expression::Ident("abc".into())),
            },
        ),
    ];

    for (inp, expect) in inputs {
        let lexer = Lexer::new(inp.into());
        let mut parser = Parser::new(lexer);

        let Program { statements } = parser.parse().unwrap();

        assert_eq!(1, statements.len());
        let expr = match statements[0] {
            Statement::Expression(ref e) => e,
            _ => panic!("expected ExpressionStatement, got {:?}", statements[0]),
        };

        match &expr {
            Expression::Prefix(p) => assert_eq!(p, &expect),
            e => panic!("expected Prefix expression, got {:?}", e),
        }
    }
}

#[test]
fn infix_expr() {
    let inputs = vec![
        (
            "5 + 5",
            InfixExpr {
                left: Box::new(Expression::Number(5)),
                operator: TokenType::Plus,
                right: Box::new(Expression::Number(5)),
            },
        ),
        (
            "5 - 5",
            InfixExpr {
                left: Box::new(Expression::Number(5)),
                operator: TokenType::Minus,
                right: Box::new(Expression::Number(5)),
            },
        ),
        (
            "5 * 5",
            InfixExpr {
                left: Box::new(Expression::Number(5)),
                operator: TokenType::Star,
                right: Box::new(Expression::Number(5)),
            },
        ),
        (
            "5 / 5",
            InfixExpr {
                left: Box::new(Expression::Number(5)),
                operator: TokenType::Slash,
                right: Box::new(Expression::Number(5)),
            },
        ),
        (
            "5 > 5",
            InfixExpr {
                left: Box::new(Expression::Number(5)),
                operator: TokenType::Gt,
                right: Box::new(Expression::Number(5)),
            },
        ),
        (
            "5 < 5",
            InfixExpr {
                left: Box::new(Expression::Number(5)),
                operator: TokenType::Lt,
                right: Box::new(Expression::Number(5)),
            },
        ),
        (
            "5 == 5",
            InfixExpr {
                left: Box::new(Expression::Number(5)),
                operator: TokenType::Eq,
                right: Box::new(Expression::Number(5)),
            },
        ),
        (
            "5 != 5",
            InfixExpr {
                left: Box::new(Expression::Number(5)),
                operator: TokenType::NotEq,
                right: Box::new(Expression::Number(5)),
            },
        ),
    ];

    for (inp, expect) in inputs {
        let lexer = Lexer::new(inp.into());
        let mut parser = Parser::new(lexer);

        let Program { statements } = parser.parse().unwrap();

        assert_eq!(1, statements.len());
        let expr = match statements[0] {
            Statement::Expression(ref e) => e,
            _ => panic!("expected ExpressionStatement, got {:?}", statements[0]),
        };

        match &expr {
            Expression::Infix(p) => assert_eq!(p, &expect),
            e => panic!("expected Infix expression, got {:?}", e),
        }
    }
}

#[test]
fn bool_expr() {
    let inputs = [
        ("true;", Expression::Bool(true)),
        ("false;", Expression::Bool(false)),
    ];

    for (inp, expect) in inputs {
        let lexer = Lexer::new(inp.into());
        let mut parser = Parser::new(lexer);

        let Program { statements } = parser.parse().unwrap();

        assert_eq!(1, statements.len());
        let expr = match statements[0] {
            Statement::Expression(ref e) => e,
            _ => panic!("expected ExpressionStatement, got {:?}", statements[0]),
        };
        assert_eq!(expr, &expect);
    }
}

#[test]
fn if_else_expr() {
    let inputs = [
        (
            "if (x < y) { x }",
            IfExpr {
                condition: Box::new(Expression::Infix(InfixExpr {
                    left: Box::new(Expression::Ident("x".into())),
                    operator: TokenType::Lt,
                    right: Box::new(Expression::Ident("y".into())),
                })),
                if_branch: vec![Statement::Expression(Expression::Ident("x".into()))],
                else_branch: None,
            },
        ),
        (
            "if (x < y) { x } else { y }",
            IfExpr {
                condition: Box::new(Expression::Infix(InfixExpr {
                    left: Box::new(Expression::Ident("x".into())),
                    operator: TokenType::Lt,
                    right: Box::new(Expression::Ident("y".into())),
                })),
                if_branch: vec![Statement::Expression(Expression::Ident("x".into()))],
                else_branch: Some(vec![Statement::Expression(Expression::Ident("y".into()))]),
            },
        ),
    ];

    for (inp, expect) in inputs {
        let lexer = Lexer::new(inp.into());
        let mut parser = Parser::new(lexer);

        let Program { statements } = parser.parse().unwrap();

        assert_eq!(1, statements.len());
        let expr = match statements[0] {
            Statement::Expression(ref e) => e,
            _ => panic!("expected ExpressionStatement, got {:?}", statements[0]),
        };

        match &expr {
            Expression::If(i) => assert_eq!(i, &expect),
            e => panic!("expected If expression, got {:?}", e),
        }
    }
}

#[test]
fn func_expr() {
    let input = "fn(x, y) { x * y; }";
    let expected = FuncExpr {
        params: vec!["x".into(), "y".into()],
        body: vec![Statement::Expression(Expression::Infix(InfixExpr {
            left: Box::new(Expression::Ident("x".into())),
            operator: TokenType::Star,
            right: Box::new(Expression::Ident("y".into())),
        }))],
    };

    let lexer = Lexer::new(input.into());
    let mut parser = Parser::new(lexer);

    let Program { statements } = parser.parse().unwrap();

    assert_eq!(1, statements.len());
    let expr = match statements[0] {
        Statement::Expression(ref e) => e,
        _ => panic!("expected ExpressionStatement, got {:?}", statements[0]),
    };
    match &expr {
        Expression::Func(i) => assert_eq!(i, &expected),
        e => panic!("expected Func expression, got {:?}", e),
    }
}

#[test]
fn func_params() {
    let inputs = [
        (
            "fn(x, y, sum) {}",
            vec!["x".to_string(), "y".into(), "sum".into()],
        ),
        ("fn(x) {}", vec!["x".into()]),
        ("fn() {}", vec![]),
    ];

    for (inp, expect) in inputs {
        let lexer = Lexer::new(inp.into());
        let mut parser = Parser::new(lexer);

        let Program { statements } = parser.parse().unwrap();

        assert_eq!(1, statements.len());
        let expr = match statements[0] {
            Statement::Expression(ref e) => e,
            _ => panic!("expected ExpressionStatement, got {:?}", statements[0]),
        };
        match &expr {
            Expression::Func(i) => assert_eq!(i.params, expect),
            e => panic!("expected Func expression, got {:?}", e),
        }
    }
}

#[test]
fn call_expr() {
    let input = "add(1, 2+3, x*y)";
    let expected = CallExpr {
        func: Box::new(Expression::Ident("add".into())),
        arguments: vec![
            Expression::Number(1),
            Expression::Infix(InfixExpr {
                left: Box::new(Expression::Number(2)),
                operator: TokenType::Plus,
                right: Box::new(Expression::Number(3)),
            }),
            Expression::Infix(InfixExpr {
                left: Box::new(Expression::Ident("x".into())),
                operator: TokenType::Star,
                right: Box::new(Expression::Ident("y".into())),
            }),
        ],
    };

    let lexer = Lexer::new(input.into());
    let mut parser = Parser::new(lexer);

    let Program { statements } = parser.parse().unwrap();

    assert_eq!(1, statements.len());
    let expr = match statements[0] {
        Statement::Expression(ref e) => e,
        _ => panic!("expected ExpressionStatement, got {:?}", statements[0]),
    };
    match &expr {
        Expression::Call(i) => assert_eq!(i, &expected),
        e => panic!("expected Call expression, got {:?}", e),
    }
}

#[test]
fn call_expr_arguments() {
    let inputs = [
        (
            "add(x, y, sum)",
            vec![
                Expression::Ident("x".into()),
                Expression::Ident("y".into()),
                Expression::Ident("sum".into()),
            ],
        ),
        ("add(x)", vec![Expression::Ident("x".into())]),
        ("add()", vec![]),
    ];

    for (inp, expect) in inputs {
        let lexer = Lexer::new(inp.into());
        let mut parser = Parser::new(lexer);

        let Program { statements } = parser.parse().unwrap();

        assert_eq!(1, statements.len());
        let expr = match statements[0] {
            Statement::Expression(ref e) => e,
            _ => panic!("expected ExpressionStatement, got {:?}", statements[0]),
        };
        match &expr {
            Expression::Call(i) => assert_eq!(i.arguments, expect),
            e => panic!("expected Func expression, got {:?}", e),
        }
    }
}

#[test]
fn array_expr() {
    let inputs = [
        ("[]", Expression::Array(ArrayExpr { elements: vec![] })),
        (
            "[1, 2 * 2, 3 + 3]",
            Expression::Array(ArrayExpr {
                elements: vec![
                    Expression::Number(1),
                    Expression::Infix(InfixExpr {
                        left: Box::new(Expression::Number(2)),
                        operator: TokenType::Star,
                        right: Box::new(Expression::Number(2)),
                    }),
                    Expression::Infix(InfixExpr {
                        left: Box::new(Expression::Number(3)),
                        operator: TokenType::Plus,
                        right: Box::new(Expression::Number(3)),
                    }),
                ],
            }),
        ),
    ];

    for (inp, expect) in inputs {
        let lexer = Lexer::new(inp.into());
        let mut parser = Parser::new(lexer);

        let Program { statements } = parser.parse().unwrap();

        assert_eq!(1, statements.len());
        let expr = match statements[0] {
            Statement::Expression(ref e) => e,
            _ => panic!("expected ExpressionStatement, got {:?}", statements[0]),
        };
        assert_eq!(expr, &expect);
    }
}

#[test]
fn index_expr() {
    let input = "arr[1 + 3]";
    let expect = Expression::Index(IndexExpr {
        left: Box::new(Expression::Ident("arr".into())),
        index: Box::new(Expression::Infix(InfixExpr {
            left: Box::new(Expression::Number(1)),
            operator: TokenType::Plus,
            right: Box::new(Expression::Number(3)),
        })),
    });

    let lexer = Lexer::new(input.into());
    let mut parser = Parser::new(lexer);

    let Program { statements } = parser.parse().unwrap();

    assert_eq!(1, statements.len());
    let expr = match statements[0] {
        Statement::Expression(ref e) => e,
        _ => panic!("expected ExpressionStatement, got {:?}", statements[0]),
    };
    assert_eq!(expr, &expect);
}

#[test]
fn hash_expr() {
    let inputs = [
        ("{}", Expression::Hash(HashExpr { pairs: vec![] })),
        (
            r#"{"one": 1, "two": 5 - 3, "three": 3}"#,
            Expression::Hash(HashExpr {
                pairs: vec![
                    (Expression::String("one".into()), Expression::Number(1)),
                    (
                        Expression::String("two".into()),
                        Expression::Infix(InfixExpr {
                            left: Box::new(Expression::Number(5)),
                            operator: TokenType::Minus,
                            right: Box::new(Expression::Number(3)),
                        }),
                    ),
                    (Expression::String("three".into()), Expression::Number(3)),
                ],
            }),
        ),
    ];

    for (inp, expect) in inputs {
        let lexer = Lexer::new(inp.into());
        let mut parser = Parser::new(lexer);

        let Program { statements } = parser.parse().unwrap();

        assert_eq!(1, statements.len());
        let expr = match statements[0] {
            Statement::Expression(ref e) => e,
            _ => panic!("expected ExpressionStatement, got {:?}", statements[0]),
        };
        assert_eq!(expr, &expect);
    }
}

#[test]
fn operator_precedence() {
    let inputs = [
        ("-a * b", "((-a) * b)\n"),
        ("!-a", "(!(-a))\n"),
        ("a + b + c", "((a + b) + c)\n"),
        ("a + b - c", "((a + b) - c)\n"),
        ("a * b * c", "((a * b) * c)\n"),
        ("a * b / c", "((a * b) / c)\n"),
        ("a + b / c", "(a + (b / c))\n"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)\n"),
        ("3 + 4; -5 * 5", "(3 + 4)\n((-5) * 5)\n"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))\n"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))\n"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))\n",
        ),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)\n"),
        ("(5 + 5) * 2", "((5 + 5) * 2)\n"),
        ("2 / (5 + 5)", "(2 / (5 + 5))\n"),
        ("-(5 + 5)", "(-(5 + 5))\n"),
        ("!(true == true)", "(!(true == true))\n"),
        ("a + add(b * c) + d", "((a + add((b * c))) + d)\n"),
        (
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))\n",
        ),
        (
            "add(a + b + c * d / f + g)",
            "add((((a + b) + ((c * d) / f)) + g))\n",
        ),
        (
            "a * [1, 2, 3, 4][b * c] * d",
            "((a * ([1, 2, 3, 4][(b * c)])) * d)\n",
        ),
        (
            "add(a * b[2], b[1], 2 * [1, 2][1])",
            "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))\n",
        ),
    ];

    for (inp, exp) in inputs {
        let lexer = Lexer::new(inp.into());
        let mut parser = Parser::new(lexer);

        let program = parser.parse().unwrap();
        assert_eq!(program.to_string(), exp);
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
