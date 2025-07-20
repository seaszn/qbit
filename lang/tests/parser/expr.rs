use qbit_lang::ast::expr::Expr;
use qbit_lang::ast::{expr::*, stmt::Stmt, value::Value};

use super::{parse_expr, parse_stmt};

#[test]
fn literal_expr() {
    // Integer literals
    let expr = parse_expr("42").unwrap();
    assert_eq!(expr, Expr::Literal(Value::Int(42)));

    // Float literals
    let expr = parse_expr("3.14159").unwrap();
    assert_eq!(expr, Expr::Literal(Value::Float(3.14159)));

    // Boolean literals
    let expr = parse_expr("true").unwrap();
    assert_eq!(expr, Expr::Literal(Value::Bool(true)));

    let expr = parse_expr("false").unwrap();
    assert_eq!(expr, Expr::Literal(Value::Bool(false)));

    // String literals
    let expr = parse_expr(r#""Hello, World!""#).unwrap();
    assert_eq!(expr, Expr::Literal(Value::Str("Hello, World!".to_string())));
}

#[test]
fn variable_expr() {
    let expr = parse_expr("myVariable").unwrap();
    assert_eq!(expr, Expr::Variable("myVariable".to_string()));

    let expr = parse_expr("_underscore").unwrap();
    assert_eq!(expr, Expr::Variable("_underscore".to_string()));

    let expr = parse_expr("var123").unwrap();
    assert_eq!(expr, Expr::Variable("var123".to_string()));
}

#[test]
fn arithmetic_expr() {
    // Test basic precedence: 2 + 3 * 4 should be 2 + (3 * 4)
    let expr = parse_expr("2 + 3 * 4").unwrap();
    match expr {
        Expr::Binary {
            op: BinaryOp::Add,
            left,
            right,
        } => {
            assert_eq!(*left, Expr::Literal(Value::Int(2)));

            match &*right {
                Expr::Binary {
                    op: BinaryOp::Mul,
                    left,
                    right,
                } => {
                    assert_eq!(**left, Expr::Literal(Value::Int(3)));
                    assert_eq!(**right, Expr::Literal(Value::Int(4)));
                }
                _ => panic!("Expected multiplication on right side"),
            }
        }
        _ => panic!("Expected addition at top level"),
    }

    // Test division precedence: 10 / 2 + 3 should be (10 / 2) + 3
    let expr = parse_expr("10 / 2 + 3").unwrap();

    match expr {
        Expr::Binary {
            op: BinaryOp::Add,
            left,
            right,
        } => {
            match &*left {
                Expr::Binary {
                    op: BinaryOp::Div,
                    left,
                    right,
                } => {
                    assert_eq!(**left, Expr::Literal(Value::Int(10)));
                    assert_eq!(**right, Expr::Literal(Value::Int(2)));
                }
                _ => panic!("Expected division on left side"),
            }
            assert_eq!(*right, Expr::Literal(Value::Int(3)));
        }
        _ => panic!("Expected addition at top level"),
    }
}

#[test]
fn power_expr() {
    // Test right associativity: 2 ** 3 ** 2 should be 2 ** (3 ** 2)
    let expr = parse_expr("2 ** 3 ** 2").unwrap();
    match expr {
        Expr::Binary {
            op: BinaryOp::Pow,
            left,
            right,
        } => {
            assert_eq!(*left, Expr::Literal(Value::Int(2)));

            match &*right {
                Expr::Binary {
                    op: BinaryOp::Pow,
                    left,
                    right,
                } => {
                    assert_eq!(**left, Expr::Literal(Value::Int(3)));
                    assert_eq!(**right, Expr::Literal(Value::Int(2)));
                }
                _ => panic!("Expected power on right side"),
            }
        }
        _ => panic!("Expected power at top level"),
    }

    // Test with caret operator too
    let expr = parse_expr("2 ^ 3 ^ 2").unwrap();
    match expr {
        Expr::Binary {
            op: BinaryOp::Pow,
            left,
            right,
        } => {
            assert_eq!(*left, Expr::Literal(Value::Int(2)));
            match &*right {
                Expr::Binary {
                    op: BinaryOp::Pow,
                    left,
                    right,
                } => {
                    assert_eq!(**left, Expr::Literal(Value::Int(3)));
                    assert_eq!(**right, Expr::Literal(Value::Int(2)));
                }
                _ => panic!("Expected power on right side"),
            }
        }
        _ => panic!("Expected power at top level"),
    }
}

#[test]
fn comparison_expr() {
    let ops = vec![
        ("5 > 3", BinaryOp::Gt),
        ("5 >= 3", BinaryOp::Ge),
        ("5 < 3", BinaryOp::Lt),
        ("5 <= 3", BinaryOp::Le),
        ("5 == 3", BinaryOp::Eq),
        ("5 != 3", BinaryOp::Neq),
    ];

    for (source, expected_op) in ops {
        let expr = parse_expr(source).unwrap();
        match expr {
            Expr::Binary { op, left, right } => {
                assert_eq!(op, expected_op);
                assert_eq!(*left, Expr::Literal(Value::Int(5)));
                assert_eq!(*right, Expr::Literal(Value::Int(3)));
            }
            _ => panic!("Expected binary expression for {}", source),
        }
    }
}

#[test]
fn logical_expr() {
    // Test AND
    let expr = parse_expr("true && false").unwrap();
    match expr {
        Expr::Binary {
            op: BinaryOp::And,
            left,
            right,
        } => {
            assert_eq!(*left, Expr::Literal(Value::Bool(true)));
            assert_eq!(*right, Expr::Literal(Value::Bool(false)));
        }
        _ => panic!("Expected AND expression"),
    }

    // Test OR
    let expr = parse_expr("true || false").unwrap();
    match expr {
        Expr::Binary {
            op: BinaryOp::Or,
            left,
            right,
        } => {
            assert_eq!(*left, Expr::Literal(Value::Bool(true)));
            assert_eq!(*right, Expr::Literal(Value::Bool(false)));
        }
        _ => panic!("Expected OR expression"),
    }

    // Test precedence: true || false && true should be true || (false && true)
    let expr = parse_expr("true || false && true").unwrap();
    match expr {
        Expr::Binary {
            op: BinaryOp::Or,
            left,
            right,
        } => {
            assert_eq!(*left, Expr::Literal(Value::Bool(true)));

            match &*right {
                Expr::Binary {
                    op: BinaryOp::And,
                    left,
                    right,
                } => {
                    assert_eq!(**left, Expr::Literal(Value::Bool(false)));
                    assert_eq!(**right, Expr::Literal(Value::Bool(true)));
                }
                _ => panic!("Expected AND on right side"),
            }
        }
        _ => panic!("Expected OR at top level"),
    }
}

#[test]
fn bitwise_expr() {
    let ops = vec![
        ("5 & 3", BinaryOp::BitAnd),
        ("5 | 3", BinaryOp::BitOr),
        ("5 << 3", BinaryOp::Shl),
        ("5 >> 3", BinaryOp::Shr),
    ];

    for (source, expected_op) in ops {
        let expr = parse_expr(source).unwrap();
        match expr {
            Expr::Binary { op, left, right } => {
                assert_eq!(op, expected_op);
                assert_eq!(*left, Expr::Literal(Value::Int(5)));
                assert_eq!(*right, Expr::Literal(Value::Int(3)));
            }
            _ => panic!("Expected binary expression for {}", source),
        }
    }
}

#[test]
fn unary_expr() {
    // Test negation
    let expr = parse_expr("-42").unwrap();
    match expr {
        Expr::Unary {
            op: UnaryOp::Neg,
            operand,
        } => {
            assert_eq!(*operand, Expr::Literal(Value::Int(42)));
        }
        _ => panic!("Expected negation"),
    }

    // Test logical NOT
    let expr = parse_expr("!true").unwrap();
    match expr {
        Expr::Unary {
            op: UnaryOp::Not,
            operand,
        } => {
            assert_eq!(*operand, Expr::Literal(Value::Bool(true)));
        }
        _ => panic!("Expected logical NOT"),
    }

    // Test chained unary
    let expr = parse_expr("!!true").unwrap();
    match expr {
        Expr::Unary {
            op: UnaryOp::Not,
            operand,
        } => match &*operand {
            Expr::Unary {
                op: UnaryOp::Not,
                operand,
            } => {
                assert_eq!(**operand, Expr::Literal(Value::Bool(true)));
            }
            _ => panic!("Expected nested NOT"),
        },
        _ => panic!("Expected NOT at top level"),
    }
}

#[test]
fn pre_increment_expr() {
    // Pre-increment
    let expr = parse_expr("++x").unwrap();
    match expr {
        Expr::PreIncrement { operand } => {
            assert_eq!(*operand, Expr::Variable("x".to_string()));
        }
        _ => panic!("Expected pre-increment"),
    }

    // Pre-decrement
    let expr = parse_expr("--y").unwrap();
    match expr {
        Expr::PreDecrement { operand } => {
            assert_eq!(*operand, Expr::Variable("y".to_string()));
        }
        _ => panic!("Expected pre-decrement"),
    }
}

#[test]
fn post_increment_expr() {
    // Post-increment (needs to be in a statement context)
    let stmt = parse_stmt("x++;").unwrap();
    match stmt {
        Stmt::Expression { expr } => match expr {
            Expr::PostIncrement { operand } => {
                assert_eq!(*operand, Expr::Variable("x".to_string()));
            }
            _ => panic!("Expected post-increment"),
        },
        _ => panic!("Expected expression statement"),
    }

    // Post-decrement
    let stmt = parse_stmt("y--;").unwrap();
    match stmt {
        Stmt::Expression { expr } => match expr {
            Expr::PostDecrement { operand } => {
                assert_eq!(*operand, Expr::Variable("y".to_string()));
            }
            _ => panic!("Expected post-decrement"),
        },
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn fn_call_expr() {
    // Simple function call
    let expr = parse_expr("foo()").unwrap();
    match expr {
        Expr::Call { callee, args } => {
            assert_eq!(*callee, Expr::Variable("foo".to_string()));
            assert_eq!(args.len(), 0);
        }
        _ => panic!("Expected function call"),
    }

    // Function call with arguments
    let expr = parse_expr("add(1, 2, 3)").unwrap();
    match expr {
        Expr::Call { callee, args } => {
            assert_eq!(*callee, Expr::Variable("add".to_string()));
            assert_eq!(args.len(), 3);
            assert_eq!(args[0], Expr::Literal(Value::Int(1)));
            assert_eq!(args[1], Expr::Literal(Value::Int(2)));
            assert_eq!(args[2], Expr::Literal(Value::Int(3)));
        }
        _ => panic!("Expected function call"),
    }

    // Chained function calls
    // let expr = parse_expr("obj.method()().getValue()").unwrap();
    // match expr {
    //     Expr::Call { callee, args } => {
    //         assert_eq!(args.len(), 0);
    //         match &*callee {
    //             Expr::Member { object, property } => {
    //                 assert_eq!(property, "getValue");
    //                 match &**object {
    //                     Expr::Call { callee, args } => {
    //                         assert_eq!(args.len(), 0);
    //                         match &**callee {
    //                             Expr::Call { callee, args } => {
    //                                 assert_eq!(args.len(), 0);
    //                                 match &**callee {
    //                                     Expr::Member { object, property } => {
    //                                         assert_eq!(**object, Expr::Variable("obj".to_string()));
    //                                         assert_eq!(property, "method");
    //                                     }
    //                                     _ => panic!("Expected member access"),
    //                                 }
    //                             }
    //                             _ => panic!("Expected function call"),
    //                         }
    //                     }
    //                     _ => panic!("Expected function call"),
    //                 }
    //             }
    //             _ => panic!("Expected member access"),
    //         }
    //     }
    //     _ => panic!("Expected function call"),
    // }
}

#[test]
fn arr_index_expr() {
    // Simple array indexing
    let expr = parse_expr("arr[0]").unwrap();
    match expr {
        Expr::Index { object, index } => {
            assert_eq!(*object, Expr::Variable("arr".to_string()));
            assert_eq!(*index, Expr::Literal(Value::Int(0)));
        }
        _ => panic!("Expected array indexing"),
    }

    // Multi-dimensional indexing
    let expr = parse_expr("matrix[i][j]").unwrap();
    match expr {
        Expr::Index { object, index } => {
            assert_eq!(*index, Expr::Variable("j".to_string()));
            match &*object {
                Expr::Index { object, index } => {
                    assert_eq!(**object, Expr::Variable("matrix".to_string()));
                    assert_eq!(**index, Expr::Variable("i".to_string()));
                }
                _ => panic!("Expected nested indexing"),
            }
        }
        _ => panic!("Expected array indexing"),
    }

    // Complex expression as index
    let expr = parse_expr("arr[i + 1]").unwrap();
    match expr {
        Expr::Index { object, index } => {
            assert_eq!(*object, Expr::Variable("arr".to_string()));
            match &*index {
                Expr::Binary {
                    op: BinaryOp::Add,
                    left,
                    right,
                } => {
                    assert_eq!(**left, Expr::Variable("i".to_string()));
                    assert_eq!(**right, Expr::Literal(Value::Int(1)));
                }
                _ => panic!("Expected addition in index"),
            }
        }
        _ => panic!("Expected array indexing"),
    }
}

#[test]
fn arr_literal_expr() {
    // Empty array
    let expr = parse_expr("[]").unwrap();
    match expr {
        Expr::Array { elements } => {
            assert_eq!(elements.len(), 0);
        }
        _ => panic!("Expected array literal"),
    }

    // Array with elements
    let expr = parse_expr("[1, 2, 3]").unwrap();
    match expr {
        Expr::Array { elements } => {
            assert_eq!(elements.len(), 3);
            assert_eq!(elements[0], Expr::Literal(Value::Int(1)));
            assert_eq!(elements[1], Expr::Literal(Value::Int(2)));
            assert_eq!(elements[2], Expr::Literal(Value::Int(3)));
        }
        _ => panic!("Expected array literal"),
    }

    // Nested arrays
    let expr = parse_expr("[[1, 2], [3, 4]]").unwrap();
    match expr {
        Expr::Array { elements } => {
            assert_eq!(elements.len(), 2);
            match &elements[0] {
                Expr::Array { elements } => {
                    assert_eq!(elements.len(), 2);
                    assert_eq!(elements[0], Expr::Literal(Value::Int(1)));
                    assert_eq!(elements[1], Expr::Literal(Value::Int(2)));
                }
                _ => panic!("Expected nested array"),
            }
        }
        _ => panic!("Expected array literal"),
    }
}

#[test]
fn assignment_expr() {
    // Simple assignment
    let stmt = parse_stmt("x = 5;").unwrap();
    match stmt {
        Stmt::Expression { expr } => match expr {
            Expr::Assignment { target, value } => {
                assert_eq!(*target, Expr::Variable("x".to_string()));
                assert_eq!(*value, Expr::Literal(Value::Int(5)));
            }
            _ => panic!("Expected assignment"),
        },
        _ => panic!("Expected expression statement"),
    }

    // Compound assignments
    let compounds = vec![
        ("x += 5", BinaryOp::Add),
        ("x -= 5", BinaryOp::Sub),
        ("x *= 5", BinaryOp::Mul),
        ("x /= 5", BinaryOp::Div),
        ("x %= 5", BinaryOp::Mod),
        ("x ^= 5", BinaryOp::Pow),
        ("x &= 5", BinaryOp::BitAnd),
        ("x |= 5", BinaryOp::BitOr),
        ("x <<= 5", BinaryOp::Shl),
        ("x >>= 5", BinaryOp::Shr),
    ];

    for (source, expected_op) in compounds {
        let stmt = parse_stmt(&format!("{};", source)).unwrap();
        match stmt {
            Stmt::Expression { expr } => match expr {
                Expr::CompoundAssignment { target, op, value } => {
                    assert_eq!(*target, Expr::Variable("x".to_string()));
                    assert_eq!(op, expected_op);
                    assert_eq!(*value, Expr::Literal(Value::Int(5)));
                }
                _ => panic!("Expected compound assignment for {}", source),
            },
            _ => panic!("Expected expression statement for {}", source),
        }
    }
}

#[test]
fn grouping_expr() {
    // Test that parentheses override precedence
    let expr = parse_expr("(2 + 3) * 4").unwrap();
    match expr {
        Expr::Binary {
            op: BinaryOp::Mul,
            left,
            right,
        } => {
            assert_eq!(*right, Expr::Literal(Value::Int(4)));
            match &*left {
                Expr::Group(inner) => match &**inner {
                    Expr::Binary {
                        op: BinaryOp::Add,
                        left,
                        right,
                    } => {
                        assert_eq!(**left, Expr::Literal(Value::Int(2)));
                        assert_eq!(**right, Expr::Literal(Value::Int(3)));
                    }
                    _ => panic!("Expected addition in group"),
                },
                _ => panic!("Expected grouped expression"),
            }
        }
        _ => panic!("Expected multiplication"),
    }

    // Nested parentheses
    let expr = parse_expr("((1 + 2) * 3)").unwrap();
    match expr {
        Expr::Group(outer) => match &*outer {
            Expr::Binary {
                op: BinaryOp::Mul,
                left,
                right,
            } => {
                assert_eq!(**right, Expr::Literal(Value::Int(3)));
                match &**left {
                    Expr::Group(inner) => match &**inner {
                        Expr::Binary {
                            op: BinaryOp::Add,
                            left,
                            right,
                        } => {
                            assert_eq!(**left, Expr::Literal(Value::Int(1)));
                            assert_eq!(**right, Expr::Literal(Value::Int(2)));
                        }
                        _ => panic!("Expected addition in inner group"),
                    },
                    _ => panic!("Expected grouped expression"),
                }
            }
            _ => panic!("Expected multiplication"),
        },
        _ => panic!("Expected grouped expression"),
    }
}

#[test]
fn member_expr() {
    // Simple member access
    let expr = parse_expr("obj.property").unwrap();
    match expr {
        Expr::Member { object, property } => {
            assert_eq!(*object, Expr::Variable("obj".to_string()));
            assert_eq!(property, "property");
        }
        _ => panic!("Expected member access"),
    }

    // Chained member access
    let expr = parse_expr("obj.prop1.prop2").unwrap();
    match expr {
        Expr::Member { object, property } => {
            assert_eq!(property, "prop2");
            match &*object {
                Expr::Member { object, property } => {
                    assert_eq!(**object, Expr::Variable("obj".to_string()));
                    assert_eq!(property, "prop1");
                }
                _ => panic!("Expected chained member access"),
            }
        }
        _ => panic!("Expected member access"),
    }
}

#[test]
fn complex_expr() {
    // Test a very complex expression
    let expr = parse_expr("(a + b) * func(x, y.prop[0]) >= 10 && !flag").unwrap();

    // This should parse as: ((a + b) * func(x, y.prop[0]) >= 10) && (!flag)
    match expr {
        Expr::Binary {
            op: BinaryOp::And,
            left,
            right,
        } => {
            // Check the left side: (a + b) * func(x, y.prop[0]) >= 10
            match &*left {
                Expr::Binary {
                    op: BinaryOp::Ge,
                    left,
                    right,
                } => {
                    assert_eq!(**right, Expr::Literal(Value::Int(10)));
                    // Left side should be (a + b) * func(x, y.prop[0])
                    match &**left {
                        Expr::Binary {
                            op: BinaryOp::Mul,
                            left,
                            right,
                        } => {
                            // Check (a + b)
                            match &**left {
                                Expr::Group(inner) => match &**inner {
                                    Expr::Binary {
                                        op: BinaryOp::Add,
                                        left,
                                        right,
                                    } => {
                                        assert_eq!(**left, Expr::Variable("a".to_string()));
                                        assert_eq!(**right, Expr::Variable("b".to_string()));
                                    }
                                    _ => panic!("Expected addition in group"),
                                },
                                _ => panic!("Expected grouped expression"),
                            }

                            // Check func(x, y.prop[0])
                            match &**right {
                                Expr::Call { callee, args } => {
                                    assert_eq!(**callee, Expr::Variable("func".to_string()));
                                    assert_eq!(args.len(), 2);
                                    assert_eq!(args[0], Expr::Variable("x".to_string()));

                                    // Check y.prop[0]
                                    match &args[1] {
                                        Expr::Index { object, index } => {
                                            match &**object {
                                                Expr::Member { object, property } => {
                                                    assert_eq!(
                                                        **object,
                                                        Expr::Variable("y".to_string())
                                                    );
                                                    assert_eq!(property, "prop");
                                                }
                                                _ => panic!("Expected member access"),
                                            }
                                            assert_eq!(**index, Expr::Literal(Value::Int(0)));
                                        }
                                        _ => panic!("Expected array indexing"),
                                    }
                                }
                                _ => panic!("Expected function call"),
                            }
                        }
                        _ => panic!("Expected multiplication"),
                    }
                }
                _ => panic!("Expected greater-than-or-equal comparison"),
            }

            // Check the right side: !flag
            match &*right {
                Expr::Unary {
                    op: UnaryOp::Not,
                    operand,
                } => {
                    assert_eq!(**operand, Expr::Variable("flag".to_string()));
                }
                _ => panic!("Expected logical NOT"),
            }
        }
        _ => panic!("Expected AND at top level"),
    }
}

#[test]
fn expr_err() {
    // Missing operand
    assert!(parse_expr("+").is_err());
    assert!(parse_expr("5 +").is_err());

    // Mismatched parentheses
    assert!(parse_expr("(5 + 3").is_err());
    assert!(parse_expr("5 + 3)").is_err());

    // Mismatched brackets
    assert!(parse_expr("[1, 2, 3").is_err());
    assert!(parse_expr("arr[0").is_err());

    // Invalid tokens
    assert!(parse_expr("5 @ 3").is_err());
    assert!(parse_expr("$invalid").is_err());
}
