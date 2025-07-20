use qbit_lang::ast::expr::Expr;
use qbit_lang::ast::{expr::*, stmt::Stmt, value::Value};
use qbit_lang::parser::ParseError;

use super::{parse_src, parse_stmt};

#[test]
fn let_stmt() {
    // Simple let statement
    let stmt = parse_stmt("let x = 42;").unwrap();
    match stmt {
        Stmt::Let { name, value } => {
            assert_eq!(name, "x");
            assert_eq!(value, Expr::Literal(Value::Int(42)));
        }
        _ => panic!("Expected Let statement"),
    }

    // Let with expression
    let stmt = parse_stmt("let result = a + b * c;").unwrap();
    match stmt {
        Stmt::Let { name, value } => {
            assert_eq!(name, "result");
            match value {
                Expr::Binary {
                    op: BinaryOp::Add, ..
                } => {}
                _ => panic!("Expected addition expression"),
            }
        }
        _ => panic!("Expected Let statement"),
    }

    // Let with function call
    let stmt = parse_stmt("let value = func(1, 2, 3);").unwrap();
    match stmt {
        Stmt::Let { name, value } => {
            assert_eq!(name, "value");
            match value {
                Expr::Call { callee, args } => {
                    assert_eq!(*callee, Expr::Variable("func".to_string()));
                    assert_eq!(args.len(), 3);
                }
                _ => panic!("Expected function call"),
            }
        }
        _ => panic!("Expected Let statement"),
    }
}

#[test]
fn const_stmt() {
    // Simple const statement
    let stmt = parse_stmt("const PI = 3.14159;").unwrap();
    match stmt {
        Stmt::Const { name, value } => {
            assert_eq!(name, "PI");
            assert_eq!(value, Expr::Literal(Value::Float(3.14159)));
        }
        _ => panic!("Expected Const statement"),
    }

    // Const with string
    let stmt = parse_stmt(r#"const MESSAGE = "Hello, World!";"#).unwrap();
    match stmt {
        Stmt::Const { name, value } => {
            assert_eq!(name, "MESSAGE");
            assert_eq!(
                value,
                Expr::Literal(Value::Str("Hello, World!".to_string()))
            );
        }
        _ => panic!("Expected Const statement"),
    }

    // Const with computed value
    let stmt = parse_stmt("const MAX_VALUE = 100 * 2;").unwrap();
    match stmt {
        Stmt::Const { name, value } => {
            assert_eq!(name, "MAX_VALUE");
            match value {
                Expr::Binary {
                    op: BinaryOp::Mul, ..
                } => {}
                _ => panic!("Expected multiplication expression"),
            }
        }
        _ => panic!("Expected Const statement"),
    }
}

#[test]
fn function_stmt() -> Result<(), ParseError> {
    // Simple function without parameters
    let stmt = parse_stmt("fn greet() { return \"Hello!\"; }").unwrap();
    match stmt {
        Stmt::Function { name, params, body } => {
            assert_eq!(name, "greet");
            assert_eq!(params.len(), 0);

            match &*body {
                Stmt::Block { statements } => {
                    assert_eq!(statements.len(), 1);
                    match &statements[0] {
                        Stmt::Return { value: Some(_) } => {}
                        _ => panic!("Expected return statement"),
                    }
                }
                _ => panic!("Expected block statement"),
            }
        }
        _ => panic!("Expected Function statement"),
    }

    // Function with parameters
    let stmt = parse_stmt("fn add(a, b, c) { return a + b + c; }").unwrap();
    match stmt {
        Stmt::Function { name, params, body } => {
            assert_eq!(name, "add");
            assert_eq!(
                params,
                vec!["a".to_string(), "b".to_string(), "c".to_string()]
            );

            match &*body {
                Stmt::Block { statements } => {
                    assert_eq!(statements.len(), 1);
                }
                _ => panic!("Expected block statement"),
            }
        }
        _ => panic!("Expected Function statement"),
    }

    // Function with complex body
    let stmt = parse_stmt(
        r#"
        fn factorial(n) {
            if n <= 1 {
                return 1;
            } else {
                return n * factorial(n - 1);
            }
        }
    "#,
    )?;

    match stmt {
        Stmt::Function { name, params, body } => {
            assert_eq!(name, "factorial");
            assert_eq!(params, vec!["n".to_string()]);

            match &*body {
                Stmt::Block { statements } => {
                    assert_eq!(statements.len(), 1);

                    match &statements[0] {
                        Stmt::If { .. } => Ok(()),
                        _ => panic!("Expected if statement"),
                    }
                }
                _ => panic!("Expected block statement"),
            }
        }
        _ => panic!("Expected Function statement"),
    }
}

#[test]
fn if_stmt() -> Result<(), ParseError> {
    // Simple if statement
    let stmt = parse_stmt("if x > 5 { return true; }").unwrap();

    match stmt {
        Stmt::If {
            condition,
            then_branch,
            else_branch,
        } => {
            match condition {
                Expr::Binary {
                    op: BinaryOp::Gt, ..
                } => {}
                _ => panic!("Expected comparison condition"),
            }
            match &*then_branch {
                Stmt::Block { statements } => {
                    assert_eq!(statements.len(), 1);
                }
                _ => panic!("Expected block statement"),
            }
            assert!(else_branch.is_none());
        }
        _ => panic!("Expected If statement"),
    }

    // If-else statement
    let stmt = parse_stmt(
        r#"
        if score >= 90 {
            grade = "A";
        } else {
            grade = "B";
        }
    "#,
    )?;

    match stmt {
        Stmt::If {
            condition,
            else_branch,
            ..
        } => {
            match condition {
                Expr::Binary {
                    op: BinaryOp::Ge, ..
                } => {}
                _ => panic!("Expected comparison condition"),
            }
            assert!(else_branch.is_some());
            match else_branch.as_ref().unwrap().as_ref() {
                Stmt::Block { statements } => {
                    assert_eq!(statements.len(), 1);
                }
                _ => panic!("Expected block statement in else"),
            }
        }
        _ => panic!("Expected If statement"),
    }

    // If-else if-else chain
    let stmt = parse_stmt(
        r#"
        if x > 10 {
            return "high";
        } else if x > 5 {
            return "medium";
        } else {
            return "low";
        }
    "#,
    )?;

    match stmt {
        Stmt::If { else_branch, .. } => {
            // Check that else_branch contains another if statement
            assert!(else_branch.is_some());
            match else_branch.as_ref().unwrap().as_ref() {
                Stmt::If { .. } => Ok(()),
                _ => panic!("Expected if statement in else branch"),
            }
        }
        _ => panic!("Expected If statement"),
    }
}

#[test]
fn while_stmt() -> Result<(), ParseError> {
    // Simple while loop
    let stmt = parse_stmt("while i < 10 { i += 1; }").unwrap();
    match stmt {
        Stmt::While { condition, body } => {
            match condition {
                Expr::Binary {
                    op: BinaryOp::Lt,
                    left,
                    right,
                } => {
                    assert_eq!(*left, Expr::Variable("i".to_string()));
                    assert_eq!(*right, Expr::Literal(Value::Int(10)));
                }
                _ => panic!("Expected comparison condition"),
            }
            match &*body {
                Stmt::Block { statements } => {
                    assert_eq!(statements.len(), 1);
                    match &statements[0] {
                        Stmt::Expression { expr } => match expr {
                            Expr::CompoundAssignment { .. } => {}
                            _ => panic!("Expected compound assignment"),
                        },
                        _ => panic!("Expected expression statement"),
                    }
                }
                _ => panic!("Expected block statement"),
            }
        }
        _ => panic!("Expected While statement"),
    }

    // While loop with complex condition
    let stmt = parse_stmt("while running && health > 0 { update(); }").unwrap();
    match stmt {
        Stmt::While { condition, .. } => match condition {
            Expr::Binary {
                op: BinaryOp::And, ..
            } => {}
            _ => panic!("Expected AND condition"),
        },
        _ => panic!("Expected While statement"),
    }

    // Nested while loops
    let stmt = parse_stmt(
        r#"
        while i < rows {
            while j < cols {
                process(i, j);
                j += 1;
            }
            i += 1;
        }
    "#,
    )?;

    match stmt {
        Stmt::While { body, .. } => match &*body {
            Stmt::Block { statements } => {
                assert_eq!(statements.len(), 2);
                match &statements[0] {
                    Stmt::While { .. } => Ok(()),
                    _ => panic!("Expected nested while loop"),
                }
            }
            _ => panic!("Expected block statement"),
        },
        _ => panic!("Expected While statement"),
    }
}

#[test]
fn for_stmt() {
    // Simple for loop
    let stmt = parse_stmt("for (let i = 0; i < 10; i += 1) { sum += i; }").unwrap();
    match stmt {
        Stmt::For {
            init,
            condition,
            update,
            body,
        } => {
            // Check init
            assert!(init.is_some());
            match init.as_ref().unwrap().as_ref() {
                Stmt::Let { name, .. } => {
                    assert_eq!(name, "i");
                }
                _ => panic!("Expected let statement in init"),
            }

            // Check condition
            assert!(condition.is_some());
            match condition.as_ref().unwrap() {
                Expr::Binary {
                    op: BinaryOp::Lt, ..
                } => {}
                _ => panic!("Expected comparison condition"),
            }

            // Check update
            assert!(update.is_some());
            match update.as_ref().unwrap() {
                Expr::CompoundAssignment { .. } => {}
                _ => panic!("Expected compound assignment in update"),
            }

            // Check body
            match &*body {
                Stmt::Block { statements } => {
                    assert_eq!(statements.len(), 1);
                }
                _ => panic!("Expected block statement"),
            }
        }
        _ => panic!("Expected For statement"),
    }

    // For loop with missing init
    let stmt = parse_stmt("for (; i < 10; i += 1) { process(); }").unwrap();
    match stmt {
        Stmt::For {
            init,
            condition,
            update,
            ..
        } => {
            assert!(init.is_none());
            assert!(condition.is_some());
            assert!(update.is_some());
        }
        _ => panic!("Expected For statement"),
    }

    // For loop with missing condition
    let stmt = parse_stmt("for (let i = 0; ; i += 1) { process(); }").unwrap();
    match stmt {
        Stmt::For {
            init,
            condition,
            update,
            ..
        } => {
            assert!(init.is_some());
            assert!(condition.is_none());
            assert!(update.is_some());
        }
        _ => panic!("Expected For statement"),
    }

    // For loop with missing update
    let stmt = parse_stmt("for (let i = 0; i < 10; ) { process(); }").unwrap();
    match stmt {
        Stmt::For {
            init,
            condition,
            update,
            ..
        } => {
            assert!(init.is_some());
            assert!(condition.is_some());
            assert!(update.is_none());
        }
        _ => panic!("Expected For statement"),
    }

    // Infinite for loop
    let stmt = parse_stmt("for (;;) { work(); }").unwrap();
    match stmt {
        Stmt::For {
            init,
            condition,
            update,
            ..
        } => {
            assert!(init.is_none());
            assert!(condition.is_none());
            assert!(update.is_none());
        }
        _ => panic!("Expected For statement"),
    }
}

#[test]
fn break_continue_stmt() {
    // Break statement
    let stmt = parse_stmt("break;").unwrap();
    match stmt {
        Stmt::Break => {}
        _ => panic!("Expected Break statement"),
    }

    // Continue statement
    let stmt = parse_stmt("continue;").unwrap();
    match stmt {
        Stmt::Continue => {}
        _ => panic!("Expected Continue statement"),
    }

    // Break and continue in loop context
    let stmt = parse_stmt(
        r#"
        while true {
            if condition1 {
                break;
            }
            if condition2 {
                continue;
            }
            work();
        }
    "#,
    )
    .unwrap();

    match stmt {
        Stmt::While { body, .. } => {
            match &*body {
                Stmt::Block { statements } => {
                    assert_eq!(statements.len(), 3);

                    // First if should contain break
                    match &statements[0] {
                        Stmt::If { then_branch, .. } => match &**then_branch {
                            Stmt::Block { statements } => match &statements[0] {
                                Stmt::Break => {}
                                _ => panic!("Expected break statement"),
                            },
                            _ => panic!("Expected block"),
                        },
                        _ => panic!("Expected if statement"),
                    }

                    // Second if should contain continue
                    match &statements[1] {
                        Stmt::If { then_branch, .. } => match &**then_branch {
                            Stmt::Block { statements } => match &statements[0] {
                                Stmt::Continue => {}
                                _ => panic!("Expected continue statement"),
                            },
                            _ => panic!("Expected block"),
                        },
                        _ => panic!("Expected if statement"),
                    }
                }
                _ => panic!("Expected block statement"),
            }
        }
        _ => panic!("Expected While statement"),
    }
}

#[test]
fn return_stmt() {
    // Return without value
    let stmt = parse_stmt("return;").unwrap();
    match stmt {
        Stmt::Return { value } => {
            assert!(value.is_none());
        }
        _ => panic!("Expected Return statement"),
    }

    // Return with value
    let stmt = parse_stmt("return 42;").unwrap();
    match stmt {
        Stmt::Return { value } => {
            assert!(value.is_some());
            assert_eq!(*value.as_ref().unwrap(), Expr::Literal(Value::Int(42)));
        }
        _ => panic!("Expected Return statement"),
    }

    // Return with complex expression
    let stmt = parse_stmt("return a + b * c;").unwrap();
    match stmt {
        Stmt::Return { value } => {
            assert!(value.is_some());
            match value.as_ref().unwrap() {
                Expr::Binary {
                    op: BinaryOp::Add, ..
                } => {}
                _ => panic!("Expected addition expression"),
            }
        }
        _ => panic!("Expected Return statement"),
    }

    // Return with function call
    let stmt = parse_stmt("return factorial(n - 1);").unwrap();
    match stmt {
        Stmt::Return { value } => {
            assert!(value.is_some());
            match value.as_ref().unwrap() {
                Expr::Call { callee, args } => {
                    assert_eq!(**callee, Expr::Variable("factorial".to_string()));
                    assert_eq!(args.len(), 1);
                }
                _ => panic!("Expected function call"),
            }
        }
        _ => panic!("Expected Return statement"),
    }
}

#[test]
fn block_stmt() {
    // Empty block
    let stmt = parse_stmt("{}").unwrap();
    match stmt {
        Stmt::Block { statements } => {
            assert_eq!(statements.len(), 0);
        }
        _ => panic!("Expected Block statement"),
    }

    // Block with statements
    let stmt = parse_stmt(
        r#"
        {
            let x = 5;
            let y = 10;
            return x + y;
        }
    "#,
    )
    .unwrap();
    match stmt {
        Stmt::Block { statements } => {
            assert_eq!(statements.len(), 3);

            match &statements[0] {
                Stmt::Let { .. } => {}
                _ => panic!("Expected let statement"),
            }

            match &statements[1] {
                Stmt::Let { .. } => {}
                _ => panic!("Expected let statement"),
            }

            match &statements[2] {
                Stmt::Return { .. } => {}
                _ => panic!("Expected return statement"),
            }
        }
        _ => panic!("Expected Block statement"),
    }

    // Nested blocks
    let stmt = parse_stmt(
        r#"
        {
            let outer = 1;
            {
                let inner = 2;
                {
                    let deepest = 3;
                }
            }
        }
    "#,
    )
    .unwrap();
    match stmt {
        Stmt::Block { statements } => {
            assert_eq!(statements.len(), 2);

            match &statements[1] {
                Stmt::Block { statements } => {
                    assert_eq!(statements.len(), 2);

                    match &statements[1] {
                        Stmt::Block { statements } => {
                            assert_eq!(statements.len(), 1);
                        }
                        _ => panic!("Expected nested block"),
                    }
                }
                _ => panic!("Expected nested block"),
            }
        }
        _ => panic!("Expected Block statement"),
    }
}

#[test]
fn import_stmt() {
    // Import statement with string
    let stmt = parse_stmt(r#"import "math";"#).unwrap();
    match stmt {
        Stmt::Import { module } => {
            assert_eq!(module, "math");
        }
        _ => panic!("Expected Import statement"),
    }

    // Import statement with identifier
    let stmt = parse_stmt("import utils;").unwrap();
    match stmt {
        Stmt::Import { module } => {
            assert_eq!(module, "utils");
        }
        _ => panic!("Expected Import statement"),
    }
}

#[test]
fn export_stmt() {
    // Export function
    let stmt = parse_stmt("export fn publicFunction() { return 42; }").unwrap();
    match stmt {
        Stmt::Export { statement } => match &*statement {
            Stmt::Function { name, .. } => {
                assert_eq!(name, "publicFunction");
            }
            _ => panic!("Expected function in export"),
        },
        _ => panic!("Expected Export statement"),
    }

    // Export variable
    let stmt = parse_stmt("export const API_KEY = \"secret\";").unwrap();
    match stmt {
        Stmt::Export { statement } => match &*statement {
            Stmt::Const { name, .. } => {
                assert_eq!(name, "API_KEY");
            }
            _ => panic!("Expected const in export"),
        },
        _ => panic!("Expected Export statement"),
    }

    // Export let
    let stmt = parse_stmt("export let counter = 0;").unwrap();
    match stmt {
        Stmt::Export { statement } => match &*statement {
            Stmt::Let { name, .. } => {
                assert_eq!(name, "counter");
            }
            _ => panic!("Expected let in export"),
        },
        _ => panic!("Expected Export statement"),
    }
}

#[test]
fn expression_stmt() {
    // Function call as statement
    let stmt = parse_stmt("doSomething();").unwrap();
    match stmt {
        Stmt::Expression { expr } => match expr {
            Expr::Call { callee, args } => {
                assert_eq!(*callee, Expr::Variable("doSomething".to_string()));
                assert_eq!(args.len(), 0);
            }
            _ => panic!("Expected function call"),
        },
        _ => panic!("Expected Expression statement"),
    }

    // Assignment as statement
    let stmt = parse_stmt("x = y + z;").unwrap();
    match stmt {
        Stmt::Expression { expr } => match expr {
            Expr::Assignment { .. } => {}
            _ => panic!("Expected assignment"),
        },
        _ => panic!("Expected Expression statement"),
    }

    // Increment as statement
    let stmt = parse_stmt("counter++;").unwrap();
    match stmt {
        Stmt::Expression { expr } => match expr {
            Expr::PostIncrement { .. } => {}
            _ => panic!("Expected post-increment"),
        },
        _ => panic!("Expected Expression statement"),
    }

    // Complex expression as statement
    let stmt = parse_stmt("array[index] = func(a, b) + c;").unwrap();
    match stmt {
        Stmt::Expression { expr } => match expr {
            Expr::Assignment { target, value } => {
                match &*target {
                    Expr::Index { .. } => {}
                    _ => panic!("Expected array indexing"),
                }
                match &*value {
                    Expr::Binary {
                        op: BinaryOp::Add, ..
                    } => {}
                    _ => panic!("Expected addition"),
                }
            }
            _ => panic!("Expected assignment"),
        },
        _ => panic!("Expected Expression statement"),
    }
}

#[test]
fn complex_program() {
    // Test a complex program with multiple statement types
    let program = parse_src(
        r#"
        import "std";
        
        const VERSION = "1.0.0";
        let globalCounter = 0;
        
        fn fibonacci(n) {
            if n <= 1 {
                return n;
            } else {
                return fibonacci(n - 1) + fibonacci(n - 2);
            }
        }
        
        fn processArray(arr) {
            let sum = 0;
            for (let i = 0; i < arr.length; i += 1) {
                sum += arr[i];
                globalCounter++;
                
                if sum > 100 {
                    break;
                }
                
                if arr[i] < 0 {
                    continue;
                }
            }
            return sum;
        }
        
        export fn main() {
            let numbers = [1, 2, 3, 4, 5];
            let result = processArray(numbers);
            
            while result > 0 {
                result -= fibonacci(3);
                if result <= 0 {
                    break;
                }
            }
            
            return result;
        }
    "#,
    )
    .unwrap();

    assert_eq!(program.len(), 6);

    // Verify structure
    assert!(matches!(program[0], Stmt::Import { .. }));
    assert!(matches!(program[1], Stmt::Const { .. }));
    assert!(matches!(program[2], Stmt::Let { .. }));
    assert!(matches!(program[3], Stmt::Function { .. }));
    assert!(matches!(program[4], Stmt::Function { .. }));
    assert!(matches!(program[5], Stmt::Export { .. }));

    // Verify specific details
    match &program[3] {
        Stmt::Function { name, params, .. } => {
            assert_eq!(name, "fibonacci");
            assert_eq!(params.len(), 1);
        }
        _ => panic!("Expected fibonacci function"),
    }

    match &program[4] {
        Stmt::Function { name, params, .. } => {
            assert_eq!(name, "processArray");
            assert_eq!(params.len(), 1);
        }
        _ => panic!("Expected processArray function"),
    }
}


#[test]
fn stmt_errors() {
    // Missing semicolon
    assert!(parse_stmt("let x = 5").is_err());
    assert!(parse_stmt("return 42").is_err());
    assert!(parse_stmt("break").is_err());
    assert!(parse_stmt("continue").is_err());
    
    // Invalid function syntax
    assert!(parse_stmt("fn { return 5; }").is_err());
    assert!(parse_stmt("fn test( { return 5; }").is_err());
    assert!(parse_stmt("fn test) { return 5; }").is_err());
    
    // Invalid if syntax
    assert!(parse_stmt("if { return 5; }").is_err());
    assert!(parse_stmt("if x > 5 return 5;").is_err());
    
    // Invalid while syntax
    assert!(parse_stmt("while { work(); }").is_err());
    assert!(parse_stmt("while x > 5 work();").is_err());
    
    // Invalid for syntax
    assert!(parse_stmt("for { work(); }").is_err());
    assert!(parse_stmt("for (let i = 0 i < 10; i++) { work(); }").is_err());
    assert!(parse_stmt("for (let i = 0; i < 10 i++) { work(); }").is_err());
    
    // Unclosed blocks
    assert!(parse_stmt("{ let x = 5;").is_err());
    assert!(parse_stmt("fn test() { return 5;").is_err());
    
    // Invalid variable names
    assert!(parse_stmt("let 123 = 5;").is_err());
    assert!(parse_stmt("const if = 10;").is_err());
    
    // Missing equals in declarations
    assert!(parse_stmt("let x 5;").is_err());
    assert!(parse_stmt("const PI 3.14;").is_err());
    
    // Invalid import/export
    assert!(parse_stmt("import;").is_err());
    assert!(parse_stmt("export;").is_err());
    assert!(parse_stmt("import 123;").is_err());
}