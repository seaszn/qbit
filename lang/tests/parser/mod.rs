use qbit_lang::{
    ast::{expr::Expr, stmt::Stmt},
    parser::{ParseError, Parser},
};

mod expr;
mod stmt;

struct TestHelper;

impl TestHelper {
    pub fn expr(source: &str) -> Result<Expr, ParseError> {
        Parser::parse_expr(source)
    }

    pub fn stmt(source: &str) -> Result<Stmt, ParseError> {
        Parser::parse_stmt(source)
    }

    pub fn src(source: &str) -> Result<Vec<Stmt>, ParseError> {
        Parser::parse_src(source)
    }

    pub fn assert_expr(source: &str) -> Expr {
        Self::expr(source)
            .unwrap_or_else(|e| panic!("Failed to parse expression '{}': {}", source, e))
    }

    pub fn assert_expr_err(source: &str, expected: &str) {
        let error = Self::expr(source).unwrap_err();
        let error_str = format!("{}", error);

        assert!(
            error_str.contains(expected),
            "Expected error to contain '{}', got: {}",
            expected,
            error_str
        );
    }

    pub fn assert_stmt_err(source: &str, expected: &str) {
        let error = Self::stmt(source).unwrap_err();
        let error_str = format!("{}", error);

        assert!(
            error_str.contains(expected),
            "Expected error to contain '{}', got: {}",
            expected,
            error_str
        );
    }
}


mod assert_expr {
    use qbit_lang::ast::{
        expr::Expr,
        op::{BinaryOp, UnaryOp},
        value::Value,
    };

    pub fn literal_int(expr: &Expr, expected: i64) {
        match expr {
            Expr::Literal(Value::Int(actual)) => assert_eq!(*actual, expected),
            _ => panic!("Expected Int literal {}, got {:?}", expected, expr),
        }
    }

    pub fn literal_float(expr: &Expr, expected: f64) {
        match expr {
            Expr::Literal(Value::Float(actual)) => assert_eq!(*actual, expected),
            _ => panic!("Expected Float literal {}, got {:?}", expected, expr),
        }
    }

    pub fn literal_bool(expr: &Expr, expected: bool) {
        match expr {
            Expr::Literal(Value::Bool(actual)) => assert_eq!(*actual, expected),
            _ => panic!("Expected Bool literal {}, got {:?}", expected, expr),
        }
    }

    pub fn literal_string(expr: &Expr, expected: &str) {
        match expr {
            Expr::Literal(Value::Str(actual)) => assert_eq!(actual, expected),
            _ => panic!("Expected String literal '{}', got {:?}", expected, expr),
        }
    }

    pub fn variable(expr: &Expr, expected: &str) {
        match expr {
            Expr::Variable(actual) => assert_eq!(actual, expected),
            _ => panic!("Expected Variable '{}', got {:?}", expected, expr),
        }
    }

    pub fn binary_op(expr: &Expr, expected: BinaryOp) -> (&Expr, &Expr) {
        match expr {
            Expr::Binary { op, left, right } => {
                assert_eq!(
                    *op, expected,
                    "Expected binary op {:?}, got {:?}",
                    expected, op
                );
                (left, right)
            }
            _ => panic!("Expected Binary {:?}, got {:?}", expected, expr),
        }
    }

    pub fn unary_op(expr: &Expr, expected: UnaryOp) -> &Expr {
        match expr {
            Expr::Unary { op, operand } => {
                assert_eq!(
                    *op, expected,
                    "Expected unary op {:?}, got {:?}",
                    expected, op
                );
                operand
            }
            _ => panic!("Expected Unary {:?}, got {:?}", expected, expr),
        }
    }

    pub fn call<'a>(
        expr: &'a Expr,
        expected_callee: &'a str,
        expected_arg_count: usize,
    ) -> (&'a Expr, &'a Vec<Expr>) {
        match expr {
            Expr::Call { callee, args } => {
                variable(callee, expected_callee);
                assert_eq!(args.len(), expected_arg_count);
                (callee, args)
            }
            _ => panic!("Expected Call, got {:?}", expr),
        }
    }

    pub fn array(expr: &Expr, expected_len: usize) -> &Vec<Expr> {
        match expr {
            Expr::Array { elements } => {
                assert_eq!(elements.len(), expected_len);
                elements
            }
            _ => panic!("Expected Array, got {:?}", expr),
        }
    }

    pub fn member<'a>(expr: &'a Expr, expected_property: &'a str) -> &'a Expr {
        match expr {
            Expr::Member { object, property } => {
                assert_eq!(property, expected_property);
                object
            }
            _ => panic!("Expected Member access, got {:?}", expr),
        }
    }

    pub fn index(expr: &Expr) -> (&Expr, &Expr) {
        match expr {
            Expr::Index { object, index } => (object, index),
            _ => panic!("Expected Index, got {:?}", expr),
        }
    }

    pub fn group(expr: &Expr) -> &Expr {
        match expr {
            Expr::Group(inner) => inner,
            _ => panic!("Expected Group, got {:?}", expr),
        }
    }
}

mod assert_stmt {
    use super::*;

    pub fn let_stmt<'a>(stmt: &'a Stmt, expected_name: &'a str) -> &'a Expr {
        match stmt {
            Stmt::Let { name, value } => {
                assert_eq!(name, expected_name);
                value
            }
            _ => panic!("Expected Let statement, got {:?}", stmt),
        }
    }

    pub fn const_stmt<'a>(stmt: &'a Stmt, expected_name: &str) -> &'a Expr {
        match stmt {
            Stmt::Const { name, value } => {
                assert_eq!(name, expected_name);
                value
            }
            _ => panic!("Expected Const statement, got {:?}", stmt),
        }
    }

    pub fn function_stmt<'a>(
        stmt: &'a Stmt,
        expected_name: &'a str,
        expected_param_count: usize,
    ) -> (&'a Vec<String>, &'a Stmt) {
        match stmt {
            Stmt::Function { name, params, body } => {
                assert_eq!(name, expected_name);
                assert_eq!(params.len(), expected_param_count);
                (params, body)
            }
            _ => panic!("Expected Function statement, got {:?}", stmt),
        }
    }

    pub fn if_stmt(stmt: &Stmt) -> (&Expr, &Stmt, &Option<Box<Stmt>>) {
        match stmt {
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => (condition, then_branch, else_branch),
            _ => panic!("Expected If statement, got {:?}", stmt),
        }
    }

    pub fn while_stmt(stmt: &Stmt) -> (&Expr, &Stmt) {
        match stmt {
            Stmt::While { condition, body } => (condition, body),
            _ => panic!("Expected While statement, got {:?}", stmt),
        }
    }

    pub fn for_stmt(stmt: &Stmt) -> (&Option<Box<Stmt>>, &Option<Expr>, &Option<Expr>, &Stmt) {
        match stmt {
            Stmt::For {
                init,
                condition,
                update,
                body,
            } => (init, condition, update, body),
            _ => panic!("Expected For statement, got {:?}", stmt),
        }
    }

    pub fn return_stmt(stmt: &Stmt) -> &Option<Expr> {
        match stmt {
            Stmt::Return { value } => value,
            _ => panic!("Expected Return statement, got {:?}", stmt),
        }
    }

    pub fn block_stmt(stmt: &Stmt, expected_len: usize) -> &Vec<Stmt> {
        match stmt {
            Stmt::Block { statements } => {
                assert_eq!(statements.len(), expected_len);
                statements
            }
            _ => panic!("Expected Block statement, got {:?}", stmt),
        }
    }

    pub fn expression_stmt(stmt: &Stmt) -> &Expr {
        match stmt {
            Stmt::Expression { expr } => expr,
            _ => panic!("Expected Expression statement, got {:?}", stmt),
        }
    }

    pub fn import_stmt(stmt: &Stmt, expected_module: &str) {
        match stmt {
            Stmt::Import { module } => {
                assert_eq!(module, expected_module);
            }
            _ => panic!("Expected Import statement, got {:?}", stmt),
        }
    }

    pub fn export_stmt(stmt: &Stmt) -> &Stmt {
        match stmt {
            Stmt::Export { statement } => statement,
            _ => panic!("Expected Export statement, got {:?}", stmt),
        }
    }

    pub fn break_stmt(stmt: &Stmt) {
        match stmt {
            Stmt::Break => {}
            _ => panic!("Expected Break statement, got {:?}", stmt),
        }
    }

    pub fn continue_stmt(stmt: &Stmt) {
        match stmt {
            Stmt::Continue => {}
            _ => panic!("Expected Continue statement, got {:?}", stmt),
        }
    }
}

mod assert_integ{
    
}