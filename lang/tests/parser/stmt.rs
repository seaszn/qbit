use cases::LET_CASES;
use qbit_lang::ast::{expr::Expr, op::BinaryOp};

use super::{TestHelper, assert_expr, assert_stmt};

mod cases {
    #[derive(Debug, Clone)]
    pub struct VariableTestCase {
        pub source: &'static str,
        pub name: &'static str,
        pub expected: i64,
    }

    #[derive(Debug, Clone)]
    pub struct FunctionTestCase {
        pub source: &'static str,
        pub func_name: &'static str,
        pub param_names: &'static [&'static str],
    }

    #[derive(Debug, Clone)]
    pub struct ErrorTestCase {
        pub source: &'static str,
        pub expected: &'static str,
        // pub description: &'static str,
    }

    pub const LET_CASES: &[VariableTestCase] = &[
        VariableTestCase {
            source: "let x = 42;",
            name: "x",
            expected: 42,
        },
        VariableTestCase {
            source: "let myVar = 123;",
            name: "myVar",
            expected: 123,
        },
        VariableTestCase {
            source: "let _underscore = 456;",
            name: "_underscore",
            expected: 456,
        },
    ];

    pub const CONST_CASES: &[VariableTestCase] = &[
        VariableTestCase {
            source: "const PI = 314;",
            name: "PI",
            expected: 314,
        },
        VariableTestCase {
            source: "const MAX_SIZE = 1000;",
            name: "MAX_SIZE",
            expected: 1000,
        },
    ];

    pub const FUNCTION_CASES: &[FunctionTestCase] = &[
        FunctionTestCase {
            source: "fn test() { return 42; }",
            func_name: "test",
            param_names: &[],
        },
        FunctionTestCase {
            source: "fn add(a, b) { return a + b; }",
            func_name: "add",
            param_names: &["a", "b"],
        },
        FunctionTestCase {
            source: "fn complex(x, y, z) { let result = x + y * z; return result; }",
            func_name: "complex",
            param_names: &["x", "y", "z"],
        },
    ];

    pub const STATEMENT_ERROR_CASES: &[ErrorTestCase] = &[
        ErrorTestCase {
            source: "let;",
            expected: "Expected identifier",
        },
        ErrorTestCase {
            source: "let x = ;",
            expected: "Expected expression",
        },
        ErrorTestCase {
            source: "let x = 42",
            expected: "Unexpected end of file, expected Semicolon",
        },
        ErrorTestCase {
            source: "const;",
            expected: "Expected identifier",
        },
        ErrorTestCase {
            source: "fn;",
            expected: "Expected function name",
        },
        ErrorTestCase {
            source: "fn test;",
            expected: "Expected LeftParen",
        },
        ErrorTestCase {
            source: "fn test();",
            expected: "Expected LeftBrace",
        },
        ErrorTestCase {
            source: "fn test(a b) { }",
            expected: "Expected ',' or ')'",
        },
        ErrorTestCase {
            source: "if;",
            expected: "Expected expression",
        },
        ErrorTestCase {
            source: "if true;",
            expected: "Expected LeftBrace",
        },
        ErrorTestCase {
            source: "while;",
            expected: "Expected expression",
        },
        ErrorTestCase {
            source: "return",
            expected: "Unexpected end of file, expected Semicolon",
        },
        ErrorTestCase {
            source: "{ let x = 1;",
            expected: "expected RightBrace",
        },
        ErrorTestCase {
            source: "import;",
            expected: "Expected module name",
        },
        ErrorTestCase {
            source: "export;",
            expected: "Expected expression",
        },
    ];
}

#[test]
fn let_stmt() {
    for case in LET_CASES {
        let stmt = TestHelper::stmt(case.source)
            .unwrap_or_else(|e| panic!("Failed to parse let statement '{}': {}", case.source, e));

        let value = assert_stmt::let_stmt(&stmt, case.name);
        assert_expr::literal_int(value, case.expected);
    }

    // Test let with complex expressions
    let stmt = TestHelper::stmt("let result = 2 + 3 * 4;").unwrap();
    let value = assert_stmt::let_stmt(&stmt, "result");
    let (left, right) = assert_expr::binary_op(value, BinaryOp::Add);
    assert_expr::literal_int(left, 2);

    let (mul_left, mul_right) = assert_expr::binary_op(right, BinaryOp::Mul);
    assert_expr::literal_int(mul_left, 3);
    assert_expr::literal_int(mul_right, 4);

    // Test let with string literal
    let stmt = TestHelper::stmt(r#"let message = "Hello, World!";"#).unwrap();
    let value = assert_stmt::let_stmt(&stmt, "message");
    assert_expr::literal_string(value, "Hello, World!");

    // Test let with boolean
    let stmt = TestHelper::stmt("let flag = true;").unwrap();
    let value = assert_stmt::let_stmt(&stmt, "flag");
    assert_expr::literal_bool(value, true);

    // Test let with variable reference
    let stmt = TestHelper::stmt("let copy = original;").unwrap();
    let value = assert_stmt::let_stmt(&stmt, "copy");
    assert_expr::variable(value, "original");
}

#[test]
fn const_stmt() {
    for case in cases::CONST_CASES {
        let stmt = TestHelper::stmt(case.source)
            .unwrap_or_else(|e| panic!("Failed to parse const statement '{}': {}", case.source, e));

        let value = assert_stmt::const_stmt(&stmt, case.name);
        assert_expr::literal_int(value, case.expected);
    }

    // Test const with expression
    let stmt = TestHelper::stmt("const DOUBLED = 21 * 2;").unwrap();
    let value = assert_stmt::const_stmt(&stmt, "DOUBLED");
    let (left, right) = assert_expr::binary_op(value, BinaryOp::Mul);
    assert_expr::literal_int(left, 21);
    assert_expr::literal_int(right, 2);
}

#[test]
fn fn_stmt() {
    for case in cases::FUNCTION_CASES {
        let stmt = TestHelper::stmt(case.source)
            .unwrap_or_else(|e| panic!("Failed to parse function '{}': {}", case.source, e));

        let (params, _body) =
            assert_stmt::function_stmt(&stmt, case.func_name, case.param_names.len());

        for (i, expected_param) in case.param_names.iter().enumerate() {
            assert_eq!(&params[i], expected_param);
        }
    }

    // Test function with complex body
    let stmt = TestHelper::stmt(
        r#"
            fn calculate(x, y) {
                let sum = x + y;
                let product = x * y;
                return sum + product;
            }
        "#,
    )
    .unwrap();

    let (params, body) = assert_stmt::function_stmt(&stmt, "calculate", 2);
    assert_eq!(params, &["x", "y"]);

    let statements = assert_stmt::block_stmt(body, 3);

    // Check first statement: let sum = x + y;
    let sum_value = assert_stmt::let_stmt(&statements[0], "sum");
    let (sum_left, sum_right) = assert_expr::binary_op(sum_value, BinaryOp::Add);

    assert_expr::variable(sum_left, "x");
    assert_expr::variable(sum_right, "y");

    // Check second statement: let product = x * y;
    let product_value = assert_stmt::let_stmt(&statements[1], "product");
    let (prod_left, prod_right) = assert_expr::binary_op(product_value, BinaryOp::Mul);

    assert_expr::variable(prod_left, "x");
    assert_expr::variable(prod_right, "y");

    // Check third statement: return sum + product;
    let return_value = assert_stmt::return_stmt(&statements[2]);

    assert!(return_value.is_some());

    let (ret_left, ret_right) =
        assert_expr::binary_op(return_value.as_ref().unwrap(), BinaryOp::Add);

    assert_expr::variable(ret_left, "sum");
    assert_expr::variable(ret_right, "product");
}

#[test]
fn if_stmt() {
    // Simple if statement
    let stmt = TestHelper::stmt("if x > 0 { return x; }").unwrap();
    let (condition, then_branch, else_branch) = assert_stmt::if_stmt(&stmt);

    let (cond_left, cond_right) = assert_expr::binary_op(condition, BinaryOp::Gt);
    assert_expr::variable(cond_left, "x");
    assert_expr::literal_int(cond_right, 0);

    let then_statements = assert_stmt::block_stmt(then_branch, 1);
    let return_value = assert_stmt::return_stmt(&then_statements[0]);
    assert!(return_value.is_some());
    assert_expr::variable(return_value.as_ref().unwrap(), "x");

    assert!(else_branch.is_none());

    // If-else statement
    let stmt = TestHelper::stmt(
        r#"
            if score >= 90 {
                grade = "A";
            } else {
                grade = "B";
            }
        "#,
    )
    .unwrap();

    let (condition, .., else_branch) = assert_stmt::if_stmt(&stmt);

    let (cond_left, cond_right) = assert_expr::binary_op(condition, BinaryOp::Ge);
    assert_expr::variable(cond_left, "score");
    assert_expr::literal_int(cond_right, 90);

    assert!(else_branch.is_some());
    let else_statements = assert_stmt::block_stmt(else_branch.as_ref().unwrap(), 1);
    let else_expr = assert_stmt::expression_stmt(&else_statements[0]);

    // Should be assignment: grade = "B"
    match else_expr {
        Expr::Assignment { target, value } => {
            assert_expr::variable(target, "grade");
            assert_expr::literal_string(value, "B");
        }
        _ => panic!("Expected assignment in else branch"),
    }

    // If-else if-else chain
    let stmt = TestHelper::stmt(
        r#"
            if x < 0 {
                return -1;
            } else if x == 0 {
                return 0;
            } else {
                return 1;
            }
        "#,
    )
    .unwrap();

    let (.., else_branch) = assert_stmt::if_stmt(&stmt);
    assert!(else_branch.is_some());

    // The else branch should be another if statement
    let else_if = else_branch.as_ref().unwrap();
    let (else_if_condition, _else_if_then, _final_else) = assert_stmt::if_stmt(else_if);
    let (cond_left, cond_right) = assert_expr::binary_op(else_if_condition, BinaryOp::Eq);
    assert_expr::variable(cond_left, "x");
    assert_expr::literal_int(cond_right, 0);
}

#[test]
fn while_stmt() {
    // Simple while loop
    let stmt = TestHelper::stmt(
        r#"
            while i < 10 {
                i = i + 1;
            }
        "#,
    )
    .unwrap();

    let (condition, body) = assert_stmt::while_stmt(&stmt);

    let (cond_left, cond_right) = assert_expr::binary_op(condition, BinaryOp::Lt);
    assert_expr::variable(cond_left, "i");
    assert_expr::literal_int(cond_right, 10);

    let body_statements = assert_stmt::block_stmt(body, 1);
    let assignment = assert_stmt::expression_stmt(&body_statements[0]);

    match assignment {
        Expr::Assignment { target, value } => {
            assert_expr::variable(target, "i");
            let (val_left, val_right) = assert_expr::binary_op(value, BinaryOp::Add);
            assert_expr::variable(val_left, "i");
            assert_expr::literal_int(val_right, 1);
        }
        _ => panic!("Expected assignment in while body"),
    }

    // While with complex condition
    let stmt = TestHelper::stmt(
        r#"
            while running && count < max_count {
                process();
                count++;
            }
        "#,
    )
    .unwrap();

    let (condition, body) = assert_stmt::while_stmt(&stmt);
    let (left_cond, right_cond) = assert_expr::binary_op(condition, BinaryOp::And);
    assert_expr::variable(left_cond, "running");

    let (count_var, max_var) = assert_expr::binary_op(right_cond, BinaryOp::Lt);
    assert_expr::variable(count_var, "count");
    assert_expr::variable(max_var, "max_count");

    let body_statements = assert_stmt::block_stmt(body, 2);

    // First statement should be function call
    let call_expr = assert_stmt::expression_stmt(&body_statements[0]);
    let (_, args) = assert_expr::call(call_expr, "process", 0);
    assert_eq!(args.len(), 0);

    // Second statement should be post-increment
    let inc_expr = assert_stmt::expression_stmt(&body_statements[1]);
    match inc_expr {
        Expr::PostIncrement { operand } => {
            assert_expr::variable(operand, "count");
        }
        _ => panic!("Expected post-increment"),
    }
}

#[test]
fn for_stmt() {
    // C-style for loop
    let stmt = TestHelper::stmt(
        r#"
            for (let i = 0; i < 10; i++) {
                print(i);
            }
        "#,
    )
    .unwrap();

    let (init, condition, update, body) = assert_stmt::for_stmt(&stmt);

    // Check init: let i = 0
    assert!(init.is_some());
    let init_value = assert_stmt::let_stmt(init.as_ref().unwrap(), "i");
    assert_expr::literal_int(init_value, 0);

    // Check condition: i < 10
    assert!(condition.is_some());
    let (cond_left, cond_right) = assert_expr::binary_op(condition.as_ref().unwrap(), BinaryOp::Lt);
    assert_expr::variable(cond_left, "i");
    assert_expr::literal_int(cond_right, 10);

    // Check update: i++
    assert!(update.is_some());
    match update.as_ref().unwrap() {
        Expr::PostIncrement { operand } => {
            assert_expr::variable(&operand, "i");
        }
        _ => panic!("Expected post-increment in for update"),
    }

    // Check body
    let body_statements = assert_stmt::block_stmt(body, 1);
    let call_expr = assert_stmt::expression_stmt(&body_statements[0]);
    let (_, args) = assert_expr::call(call_expr, "print", 1);
    assert_expr::variable(&args[0], "i");

    // For loop with missing parts
    let stmt = TestHelper::stmt("for (; i < 10; ) { break; }").unwrap();
    let (init, condition, update, _body) = assert_stmt::for_stmt(&stmt);

    assert!(init.is_none());
    assert!(condition.is_some());
    assert!(update.is_none());

    // Infinite for loop
    let stmt = TestHelper::stmt("for (;;) { break; }").unwrap();
    let (init, condition, update, _body) = assert_stmt::for_stmt(&stmt);

    assert!(init.is_none());
    assert!(condition.is_none());
    assert!(update.is_none());
}

#[test]
fn return_stmt() {
    // Return with value
    let stmt = TestHelper::stmt("return 42;").unwrap();
    let return_value = assert_stmt::return_stmt(&stmt);
    assert!(return_value.is_some());
    assert_expr::literal_int(return_value.as_ref().unwrap(), 42);

    // Return with expression
    let stmt = TestHelper::stmt("return x + y;").unwrap();
    let return_value = assert_stmt::return_stmt(&stmt);
    assert!(return_value.is_some());
    let (left, right) = assert_expr::binary_op(return_value.as_ref().unwrap(), BinaryOp::Add);
    assert_expr::variable(left, "x");
    assert_expr::variable(right, "y");

    // Return with function call
    let stmt = TestHelper::stmt("return calculateSum(a, b, c);").unwrap();
    let return_value = assert_stmt::return_stmt(&stmt);
    assert!(return_value.is_some());
    let (_, args) = assert_expr::call(return_value.as_ref().unwrap(), "calculateSum", 3);
    assert_expr::variable(&args[0], "a");
    assert_expr::variable(&args[1], "b");
    assert_expr::variable(&args[2], "c");

    // Return without value
    let stmt = TestHelper::stmt("return;").unwrap();
    let return_value = assert_stmt::return_stmt(&stmt);
    assert!(return_value.is_none());
}

#[test]
fn block_stmt() {
    // Empty block
    let stmt = TestHelper::stmt("{}").unwrap();
    let statements = assert_stmt::block_stmt(&stmt, 0);
    assert_eq!(statements.len(), 0);

    // Block with multiple statements
    let stmt = TestHelper::stmt(
        r#"
            {
                let x = 5;
                let y = 10;
                let sum = x + y;
                return sum;
            }
        "#,
    )
    .unwrap();

    let statements = assert_stmt::block_stmt(&stmt, 4);

    let x_value = assert_stmt::let_stmt(&statements[0], "x");
    assert_expr::literal_int(x_value, 5);

    let y_value = assert_stmt::let_stmt(&statements[1], "y");
    assert_expr::literal_int(y_value, 10);

    let sum_value = assert_stmt::let_stmt(&statements[2], "sum");
    let (sum_left, sum_right) = assert_expr::binary_op(sum_value, BinaryOp::Add);
    assert_expr::variable(sum_left, "x");
    assert_expr::variable(sum_right, "y");

    let return_value = assert_stmt::return_stmt(&statements[3]);
    assert!(return_value.is_some());
    assert_expr::variable(return_value.as_ref().unwrap(), "sum");

    // Nested blocks
    let stmt = TestHelper::stmt(
        r#"
            {
                let outer = 1;
                {
                    let inner = 2;
                    let total = outer + inner;
                }
                let final = outer;
            }
        "#,
    )
    .unwrap();

    let outer_statements = assert_stmt::block_stmt(&stmt, 3);
    assert_stmt::let_stmt(&outer_statements[0], "outer");

    let inner_statements = assert_stmt::block_stmt(&outer_statements[1], 2);
    assert_stmt::let_stmt(&inner_statements[0], "inner");
    assert_stmt::let_stmt(&inner_statements[1], "total");

    assert_stmt::let_stmt(&outer_statements[2], "final");
}

#[test]
fn expression_stmt() {
    // Function call as statement
    let stmt = TestHelper::stmt("doSomething();").unwrap();
    let expr = assert_stmt::expression_stmt(&stmt);
    let (_, args) = assert_expr::call(expr, "doSomething", 0);
    assert_eq!(args.len(), 0);

    // Assignment as statement
    let stmt = TestHelper::stmt("x = 42;").unwrap();
    let expr = assert_stmt::expression_stmt(&stmt);
    match expr {
        Expr::Assignment { target, value } => {
            assert_expr::variable(target, "x");
            assert_expr::literal_int(value, 42);
        }
        _ => panic!("Expected assignment"),
    }

    // Compound assignment as statement
    let stmt = TestHelper::stmt("counter += 1;").unwrap();
    let expr = assert_stmt::expression_stmt(&stmt);
    match expr {
        Expr::CompoundAssignment { target, op, value } => {
            assert_expr::variable(target, "counter");
            assert_eq!(*op, BinaryOp::Add);
            assert_expr::literal_int(value, 1);
        }
        _ => panic!("Expected compound assignment"),
    }

    // Post-increment as statement
    let stmt = TestHelper::stmt("i++;").unwrap();
    let expr = assert_stmt::expression_stmt(&stmt);
    match expr {
        Expr::PostIncrement { operand } => {
            assert_expr::variable(operand, "i");
        }
        _ => panic!("Expected post-increment"),
    }
}

#[test]
fn import_stmt() {
    // Import with string literal
    let stmt = TestHelper::stmt(r#"import "math";"#).unwrap();
    assert_stmt::import_stmt(&stmt, "math");

    // Import with identifier
    let stmt = TestHelper::stmt("import utils;").unwrap();
    assert_stmt::import_stmt(&stmt, "utils");

    // Import with path-like string
    let stmt = TestHelper::stmt(r#"import "lib/collections";"#).unwrap();
    assert_stmt::import_stmt(&stmt, "lib/collections");
}

#[test]
fn export_stmt() {
    // Export function
    let stmt = TestHelper::stmt("export fn add(a, b) { return a + b; }").unwrap();
    let exported = assert_stmt::export_stmt(&stmt);
    let (_, _) = assert_stmt::function_stmt(exported, "add", 2);

    // Export variable
    let stmt = TestHelper::stmt("export let API_KEY = \"secret\";").unwrap();
    let exported = assert_stmt::export_stmt(&stmt);
    let value = assert_stmt::let_stmt(exported, "API_KEY");
    assert_expr::literal_string(value, "secret");

    // Export const
    let stmt = TestHelper::stmt("export const VERSION = 1;").unwrap();
    let exported = assert_stmt::export_stmt(&stmt);
    let value = assert_stmt::const_stmt(exported, "VERSION");
    assert_expr::literal_int(value, 1);
}

#[test]
fn break_continue_stmt() {
    // Break statement
    let stmt = TestHelper::stmt("break;").unwrap();
    assert_stmt::break_stmt(&stmt);

    // Continue statement
    let stmt = TestHelper::stmt("continue;").unwrap();
    assert_stmt::continue_stmt(&stmt);
}

#[test]
fn errors_stmt() {
    for case in cases::STATEMENT_ERROR_CASES {
        TestHelper::assert_stmt_err(case.source, case.expected);
    }
}

#[test]
fn comment_stmt() {
    // Comments in function
    let stmt = TestHelper::stmt(
        r#"
            fn test(/* param comment */ x) { // function comment
                // inside comment
                let y = x + 1; /* inline comment */
                return y; // return comment
            }
        "#,
    )
    .unwrap();

    let (params, body) = assert_stmt::function_stmt(&stmt, "test", 1);
    assert_eq!(params[0], "x");

    let statements = assert_stmt::block_stmt(body, 2);
    assert_stmt::let_stmt(&statements[0], "y");
    assert_stmt::return_stmt(&statements[1]);

    // Comments in blocks
    let stmt = TestHelper::stmt(
        r#"
            {
                // First comment
                let a = 1;
                /* Block comment
                   spanning multiple
                   lines */
                let b = 2;
                // Final comment
            }
        "#,
    )
    .unwrap();

    let statements = assert_stmt::block_stmt(&stmt, 2);
    assert_stmt::let_stmt(&statements[0], "a");
    assert_stmt::let_stmt(&statements[1], "b");
}

#[test]
fn trail_commas_stmt() {
    // Function parameters with trailing comma
    let stmt = TestHelper::stmt("fn test(a, b, c,) { return a + b + c; }").unwrap();
    let (params, _) = assert_stmt::function_stmt(&stmt, "test", 3);
    assert_eq!(params, &["a", "b", "c"]);

    // This should still work even if trailing commas are disabled
    // (since it's a parser config test, we'd need to test with custom config)
}

#[test]
fn nesting_stmt() {
    // Complex nested program
    let program = TestHelper::src(
        r#"
            fn fibonacci(n) {
                if n <= 1 {
                    return n;
                } else {
                    return fibonacci(n - 1) + fibonacci(n - 2);
                }
            }
            
            fn main() {
                let count = 10;
                for (let i = 0; i < count; i++) {
                    let result = fibonacci(i);
                    if result > 50 {
                        break;
                    }
                    print(result);
                }
                
                while true {
                    let input = readInput();
                    if input == "quit" {
                        break;
                    }
                    process(input);
                }
            }
            
            export fn utility() {
                const MAX_RETRIES = 3;
                let attempts = 0;
                
                while attempts < MAX_RETRIES {
                    if tryOperation() {
                        return true;
                    }
                    attempts++;
                }
                
                return false;
            }
        "#,
    )
    .unwrap();

    assert_eq!(program.statements().len(), 3);

    // Test fibonacci function
    let (fib_params, fib_body) = assert_stmt::function_stmt(&program.statements()[0], "fibonacci", 1);
    assert_eq!(fib_params[0], "n");

    let fib_statements = assert_stmt::block_stmt(fib_body, 1);
    let (fib_condition, fib_then, fib_else) = assert_stmt::if_stmt(&fib_statements[0]);

    // Check condition: n <= 1
    let (cond_left, cond_right) = assert_expr::binary_op(fib_condition, BinaryOp::Le);
    assert_expr::variable(cond_left, "n");
    assert_expr::literal_int(cond_right, 1);

    // Check then branch: return n
    let then_statements = assert_stmt::block_stmt(fib_then, 1);
    let then_return = assert_stmt::return_stmt(&then_statements[0]);
    assert!(then_return.is_some());
    assert_expr::variable(then_return.as_ref().unwrap(), "n");

    // Check else branch: return fibonacci(n-1) + fibonacci(n-2)
    assert!(fib_else.is_some());
    let else_statements = assert_stmt::block_stmt(fib_else.as_ref().unwrap(), 1);
    let else_return = assert_stmt::return_stmt(&else_statements[0]);
    assert!(else_return.is_some());

    let (add_left, add_right) =
        assert_expr::binary_op(else_return.as_ref().unwrap(), BinaryOp::Add);

    // Left side: fibonacci(n - 1)
    let (_, left_args) = assert_expr::call(add_left, "fibonacci", 1);
    let (sub_left, sub_right) = assert_expr::binary_op(&left_args[0], BinaryOp::Sub);
    assert_expr::variable(sub_left, "n");
    assert_expr::literal_int(sub_right, 1);

    // Right side: fibonacci(n - 2)
    let (_, right_args) = assert_expr::call(add_right, "fibonacci", 1);
    let (sub_left2, sub_right2) = assert_expr::binary_op(&right_args[0], BinaryOp::Sub);
    assert_expr::variable(sub_left2, "n");
    assert_expr::literal_int(sub_right2, 2);

    // Test main function
    let (main_params, main_body) = assert_stmt::function_stmt(&program.statements()[1], "main", 0);
    assert_eq!(main_params.len(), 0);

    let main_statements = assert_stmt::block_stmt(main_body, 3);

    // First statement: let count = 10;
    let count_value = assert_stmt::let_stmt(&main_statements[0], "count");
    assert_expr::literal_int(count_value, 10);

    // Second statement: for loop
    let (for_init, for_condition, for_update, for_body) =
        assert_stmt::for_stmt(&main_statements[1]);

    assert!(for_init.is_some());
    let init_value = assert_stmt::let_stmt(for_init.as_ref().unwrap(), "i");
    assert_expr::literal_int(init_value, 0);

    assert!(for_condition.is_some());
    let (for_cond_left, for_cond_right) =
        assert_expr::binary_op(for_condition.as_ref().unwrap(), BinaryOp::Lt);
    assert_expr::variable(for_cond_left, "i");
    assert_expr::variable(for_cond_right, "count");

    assert!(for_update.is_some());
    match for_update.as_ref().unwrap() {
        Expr::PostIncrement { operand } => assert_expr::variable(&operand, "i"),
        _ => panic!("Expected post-increment"),
    }

    let for_body_statements = assert_stmt::block_stmt(for_body, 3);

    // let result = fibonacci(i);
    let result_value = assert_stmt::let_stmt(&for_body_statements[0], "result");
    let (_, fib_args) = assert_expr::call(result_value, "fibonacci", 1);
    assert_expr::variable(&fib_args[0], "i");

    // if result > 50 { break; }
    let (if_condition, if_then, if_else) = assert_stmt::if_stmt(&for_body_statements[1]);
    let (if_left, if_right) = assert_expr::binary_op(if_condition, BinaryOp::Gt);
    assert_expr::variable(if_left, "result");
    assert_expr::literal_int(if_right, 50);

    let if_statements = assert_stmt::block_stmt(if_then, 1);
    assert_stmt::break_stmt(&if_statements[0]);
    assert!(if_else.is_none());

    // print(result);
    let print_expr = assert_stmt::expression_stmt(&for_body_statements[2]);
    let (_, print_args) = assert_expr::call(print_expr, "print", 1);
    assert_expr::variable(&print_args[0], "result");

    // Third statement: while loop
    let (while_condition, while_body) = assert_stmt::while_stmt(&main_statements[2]);
    assert_expr::literal_bool(while_condition, true);

    let while_statements = assert_stmt::block_stmt(while_body, 3);

    // let input = readInput();
    let input_value = assert_stmt::let_stmt(&while_statements[0], "input");
    let (_, input_args) = assert_expr::call(input_value, "readInput", 0);
    assert_eq!(input_args.len(), 0);

    // if input == "quit" { break; }
    let (quit_condition, quit_then, quit_else) = assert_stmt::if_stmt(&while_statements[1]);
    let (quit_left, quit_right) = assert_expr::binary_op(quit_condition, BinaryOp::Eq);
    assert_expr::variable(quit_left, "input");
    assert_expr::literal_string(quit_right, "quit");

    let quit_statements = assert_stmt::block_stmt(quit_then, 1);
    assert_stmt::break_stmt(&quit_statements[0]);
    assert!(quit_else.is_none());

    // process(input);
    let process_expr = assert_stmt::expression_stmt(&while_statements[2]);
    let (_, process_args) = assert_expr::call(process_expr, "process", 1);
    assert_expr::variable(&process_args[0], "input");

    // Test export utility function
    let exported = assert_stmt::export_stmt(&program.statements()[2]);
    let (utility_params, utility_body) = assert_stmt::function_stmt(exported, "utility", 0);
    assert_eq!(utility_params.len(), 0);

    let utility_statements = assert_stmt::block_stmt(utility_body, 4);

    // const MAX_RETRIES = 3;
    let max_retries_value = assert_stmt::const_stmt(&utility_statements[0], "MAX_RETRIES");
    assert_expr::literal_int(max_retries_value, 3);

    // let attempts = 0;
    let attempts_value = assert_stmt::let_stmt(&utility_statements[1], "attempts");
    assert_expr::literal_int(attempts_value, 0);

    // while attempts < MAX_RETRIES { ... }
    let (util_while_condition, util_while_body) = assert_stmt::while_stmt(&utility_statements[2]);
    let (util_cond_left, util_cond_right) =
        assert_expr::binary_op(util_while_condition, BinaryOp::Lt);
    assert_expr::variable(util_cond_left, "attempts");
    assert_expr::variable(util_cond_right, "MAX_RETRIES");

    let util_while_statements = assert_stmt::block_stmt(util_while_body, 2);

    // if tryOperation() { return true; }
    let (try_condition, try_then, try_else) = assert_stmt::if_stmt(&util_while_statements[0]);
    let (_, try_args) = assert_expr::call(try_condition, "tryOperation", 0);
    assert_eq!(try_args.len(), 0);

    let try_statements = assert_stmt::block_stmt(try_then, 1);
    let try_return = assert_stmt::return_stmt(&try_statements[0]);
    assert!(try_return.is_some());
    assert_expr::literal_bool(try_return.as_ref().unwrap(), true);
    assert!(try_else.is_none());

    // attempts++;
    let attempts_inc = assert_stmt::expression_stmt(&util_while_statements[1]);
    match attempts_inc {
        Expr::PostIncrement { operand } => assert_expr::variable(operand, "attempts"),
        _ => panic!("Expected post-increment"),
    }

    // return false;
    let final_return = assert_stmt::return_stmt(&utility_statements[3]);
    assert!(final_return.is_some());
    assert_expr::literal_bool(final_return.as_ref().unwrap(), false);
}

#[test]
fn edge_cases() {
    // Function with no parameters but with spaces
    let stmt = TestHelper::stmt("fn test( ) { return 1; }").unwrap();
    let (params, _) = assert_stmt::function_stmt(&stmt, "test", 0);
    assert_eq!(params.len(), 0);

    // Empty return in various contexts
    let stmt = TestHelper::stmt(
        r#"
            fn test() {
                if true {
                    return;
                }
                return;
            }
        "#,
    )
    .unwrap();

    let (_, body) = assert_stmt::function_stmt(&stmt, "test", 0);
    let statements = assert_stmt::block_stmt(body, 2);

    let (_, if_then, _) = assert_stmt::if_stmt(&statements[0]);
    let if_statements = assert_stmt::block_stmt(if_then, 1);
    let if_return = assert_stmt::return_stmt(&if_statements[0]);
    assert!(if_return.is_none());

    let final_return = assert_stmt::return_stmt(&statements[1]);
    assert!(final_return.is_none());

    // Deeply nested blocks
    let stmt = TestHelper::stmt(
        r#"
            {
                {
                    {
                        {
                            let deep = 42;
                        }
                    }
                }
            }
        "#,
    )
    .unwrap();

    let level1 = assert_stmt::block_stmt(&stmt, 1);
    let level2 = assert_stmt::block_stmt(&level1[0], 1);
    let level3 = assert_stmt::block_stmt(&level2[0], 1);
    let level4 = assert_stmt::block_stmt(&level3[0], 1);
    let deep_value = assert_stmt::let_stmt(&level4[0], "deep");
    assert_expr::literal_int(deep_value, 42);

    // For loop with complex expressions
    let stmt = TestHelper::stmt(
        r#"
            for (let i = start + offset; i < getMax() - 1; i += step * 2) {
                process(arr[i]);
            }
        "#,
    )
    .unwrap();

    let (init, condition, update, body) = assert_stmt::for_stmt(&stmt);

    // Check complex init: let i = start + offset
    assert!(init.is_some());
    let init_value = assert_stmt::let_stmt(init.as_ref().unwrap(), "i");
    let (init_left, init_right) = assert_expr::binary_op(init_value, BinaryOp::Add);
    assert_expr::variable(init_left, "start");
    assert_expr::variable(init_right, "offset");

    // Check complex condition: i < getMax() - 1
    assert!(condition.is_some());
    let (cond_left, cond_right) = assert_expr::binary_op(condition.as_ref().unwrap(), BinaryOp::Lt);
    assert_expr::variable(cond_left, "i");

    let (max_left, max_right) = assert_expr::binary_op(cond_right, BinaryOp::Sub);
    let (_, max_args) = assert_expr::call(max_left, "getMax", 0);
    assert_eq!(max_args.len(), 0);
    assert_expr::literal_int(max_right, 1);

    // Check complex update: i += step * 2
    assert!(update.is_some());
    match update.as_ref().unwrap() {
        Expr::CompoundAssignment { target, op, value } => {
            assert_expr::variable(target, "i");
            assert_eq!(*op, BinaryOp::Add);
            let (step_left, step_right) = assert_expr::binary_op(value, BinaryOp::Mul);
            assert_expr::variable(step_left, "step");
            assert_expr::literal_int(step_right, 2);
        }
        _ => panic!("Expected compound assignment in for update"),
    }

    // Check body: process(arr[i])
    let body_statements = assert_stmt::block_stmt(body, 1);
    let process_expr = assert_stmt::expression_stmt(&body_statements[0]);
    let (_, process_args) = assert_expr::call(process_expr, "process", 1);

    let (arr_obj, arr_index) = assert_expr::index(&process_args[0]);
    assert_expr::variable(arr_obj, "arr");
    assert_expr::variable(arr_index, "i");
}
