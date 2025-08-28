use super::{TestHelper, assert_expr};
use cases::{ARITHMETIC_OPS, BITWISE_OPS, COMPARISON_OPS, ERROR_CASES, PRECEDENCE_CASES};
use qbit_lang::ast::op::{BinaryOp, UnaryOp};

mod cases {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct BinaryOpCase {
        pub source: &'static str,
        pub op: BinaryOp,
        pub left: i64,
        pub right: i64,
    }

    #[derive(Debug, Clone)]
    pub struct PrecedenceCase {
        pub source: &'static str,
        pub expected: BinaryOp,
        // pub description: &'static str,
    }

    #[derive(Debug, Clone)]
    pub struct ErrorCase {
        pub source: &'static str,
        pub expected: &'static str,
        // pub description: &'static str,
    }

    pub const ARITHMETIC_OPS: &[BinaryOpCase] = &[
        BinaryOpCase {
            source: "5 + 3",
            op: BinaryOp::Add,
            left: 5,
            right: 3,
        },
        BinaryOpCase {
            source: "5 - 3",
            op: BinaryOp::Sub,
            left: 5,
            right: 3,
        },
        BinaryOpCase {
            source: "5 * 3",
            op: BinaryOp::Mul,
            left: 5,
            right: 3,
        },
        BinaryOpCase {
            source: "5 / 3",
            op: BinaryOp::Div,
            left: 5,
            right: 3,
        },
        BinaryOpCase {
            source: "5 % 3",
            op: BinaryOp::Mod,
            left: 5,
            right: 3,
        },
        BinaryOpCase {
            source: "5 ** 3",
            op: BinaryOp::Pow,
            left: 5,
            right: 3,
        },
        BinaryOpCase {
            source: "5 ^ 3",
            op: BinaryOp::Pow,
            left: 5,
            right: 3,
        },
    ];

    pub const COMPARISON_OPS: &[BinaryOpCase] = &[
        BinaryOpCase {
            source: "5 > 3",
            op: BinaryOp::Gt,
            left: 5,
            right: 3,
        },
        BinaryOpCase {
            source: "5 >= 3",
            op: BinaryOp::Ge,
            left: 5,
            right: 3,
        },
        BinaryOpCase {
            source: "5 < 3",
            op: BinaryOp::Lt,
            left: 5,
            right: 3,
        },
        BinaryOpCase {
            source: "5 <= 3",
            op: BinaryOp::Le,
            left: 5,
            right: 3,
        },
        BinaryOpCase {
            source: "5 == 3",
            op: BinaryOp::Eq,
            left: 5,
            right: 3,
        },
        BinaryOpCase {
            source: "5 != 3",
            op: BinaryOp::Neq,
            left: 5,
            right: 3,
        },
    ];

    pub const BITWISE_OPS: &[BinaryOpCase] = &[
        BinaryOpCase {
            source: "5 & 3",
            op: BinaryOp::BitAnd,
            left: 5,
            right: 3,
        },
        BinaryOpCase {
            source: "5 | 3",
            op: BinaryOp::BitOr,
            left: 5,
            right: 3,
        },
        BinaryOpCase {
            source: "5 << 3",
            op: BinaryOp::Shl,
            left: 5,
            right: 3,
        },
        BinaryOpCase {
            source: "5 >> 3",
            op: BinaryOp::Shr,
            left: 5,
            right: 3,
        },
    ];

    pub const PRECEDENCE_CASES: &[PrecedenceCase] = &[
        PrecedenceCase {
            source: "2 + 3 * 4",
            expected: BinaryOp::Add,
        },
        PrecedenceCase {
            source: "2 * 3 + 4",
            expected: BinaryOp::Add,
        },
        PrecedenceCase {
            source: "2 ** 3 ** 4",
            expected: BinaryOp::Pow,
        },
        PrecedenceCase {
            source: "true || false && true",
            expected: BinaryOp::Or,
        },
        PrecedenceCase {
            source: "1 + 2 < 3 + 4",
            expected: BinaryOp::Lt,
        },
        PrecedenceCase {
            source: "1 | 2 & 3",
            expected: BinaryOp::BitOr,
        },
    ];

    pub const ERROR_CASES: &[ErrorCase] = &[
        ErrorCase {
            source: "5 +",
            expected: "Unexpected end of file",
        },
        ErrorCase {
            source: "(5 + 3",
            expected: "Unexpected end of file",
        },
        ErrorCase {
            source: "[1, 2, 3",
            expected: "Unexpected end of file",
        },
        ErrorCase {
            source: "5 @",
            expected: "Invalid token",
        },
        ErrorCase {
            source: "func(1, 2,",
            expected: "Unexpected end of file",
        },
    ];
}

#[test]
fn literal_expr() {
    // Integer literals
    let expr = TestHelper::assert_expr("42");
    assert_expr::literal_int(&expr, 42);

    let expr = TestHelper::assert_expr("-123");
    let operand = assert_expr::unary_op(&expr, UnaryOp::Neg);
    assert_expr::literal_int(operand, 123);

    // Float literals
    let expr = TestHelper::assert_expr("3.14159");
    assert_expr::literal_float(&expr, 3.14159);

    // Boolean literals
    let expr = TestHelper::assert_expr("true");
    assert_expr::literal_bool(&expr, true);

    let expr = TestHelper::assert_expr("false");
    assert_expr::literal_bool(&expr, false);

    // String literals
    let expr = TestHelper::assert_expr(r#""Hello, World!""#);
    assert_expr::literal_string(&expr, "Hello, World!");

    let expr = TestHelper::assert_expr(r#""with \"quotes\"""#);
    assert_expr::literal_string(&expr, r#"with "quotes""#);
}

#[test]
fn var_expr() {
    let expr = TestHelper::assert_expr("myVariable");
    assert_expr::variable(&expr, "myVariable");

    let expr = TestHelper::assert_expr("_underscore");
    assert_expr::variable(&expr, "_underscore");

    let expr = TestHelper::assert_expr("var123");
    assert_expr::variable(&expr, "var123");
}

#[test]
fn arithmetic_op_expr() {
    for case in ARITHMETIC_OPS {
        let expr = TestHelper::assert_expr(case.source);
        let (left, right) = assert_expr::binary_op(&expr, case.op);
        assert_expr::literal_int(left, case.left);
        assert_expr::literal_int(right, case.right);
    }
}

#[test]
fn comparison_op_expr() {
    for case in COMPARISON_OPS {
        let expr = TestHelper::assert_expr(case.source);
        let (left, right) = assert_expr::binary_op(&expr, case.op);
        assert_expr::literal_int(left, case.left);
        assert_expr::literal_int(right, case.right);
    }
}

#[test]
fn bitwise_op_expr() {
    for case in BITWISE_OPS {
        let expr = TestHelper::assert_expr(case.source);
        let (left, right) = assert_expr::binary_op(&expr, case.op);
        assert_expr::literal_int(left, case.left);
        assert_expr::literal_int(right, case.right);
    }
}

#[test]
fn logical_op_expr() {
    // AND
    let expr = TestHelper::assert_expr("true && false");
    let (left, right) = assert_expr::binary_op(&expr, BinaryOp::And);
    assert_expr::literal_bool(left, true);
    assert_expr::literal_bool(right, false);

    // OR
    let expr = TestHelper::assert_expr("true || false");
    let (left, right) = assert_expr::binary_op(&expr, BinaryOp::Or);
    assert_expr::literal_bool(left, true);
    assert_expr::literal_bool(right, false);
}

#[test]
fn unary_op_expr() {
    // Negation
    let expr = TestHelper::assert_expr("-42");
    let operand = assert_expr::unary_op(&expr, UnaryOp::Neg);
    assert_expr::literal_int(operand, 42);

    // Logical NOT
    let expr = TestHelper::assert_expr("!true");
    let operand = assert_expr::unary_op(&expr, UnaryOp::Not);
    assert_expr::literal_bool(operand, true);

    // Chained unary
    let expr = TestHelper::assert_expr("!!true");
    let operand1 = assert_expr::unary_op(&expr, UnaryOp::Not);
    let operand2 = assert_expr::unary_op(operand1, UnaryOp::Not);
    assert_expr::literal_bool(operand2, true);
}

#[test]
fn op_precedence_expr() {
    for case in PRECEDENCE_CASES {
        let expr = TestHelper::assert_expr(case.source);
        let (_, _) = assert_expr::binary_op(&expr, case.expected);
        // Could add more detailed precedence checking here
    }
}

#[test]
fn right_assoc_expr() {
    // Power operator should be right-associative: 2 ** 3 ** 2 = 2 ** (3 ** 2)
    let expr = TestHelper::assert_expr("2 ** 3 ** 2");
    let (left, right) = assert_expr::binary_op(&expr, BinaryOp::Pow);
    assert_expr::literal_int(left, 2);

    let (inner_left, inner_right) = assert_expr::binary_op(right, BinaryOp::Pow);
    assert_expr::literal_int(inner_left, 3);
    assert_expr::literal_int(inner_right, 2);
}

#[test]
fn left_assoc_expr() {
    // Subtraction should be left-associative: 10 - 5 - 2 = (10 - 5) - 2
    let expr = TestHelper::assert_expr("10 - 5 - 2");
    let (left, right) = assert_expr::binary_op(&expr, BinaryOp::Sub);
    assert_expr::literal_int(right, 2);

    let (inner_left, inner_right) = assert_expr::binary_op(left, BinaryOp::Sub);
    assert_expr::literal_int(inner_left, 10);
    assert_expr::literal_int(inner_right, 5);
}

#[test]
fn paren_expr() {
    // Override precedence with parentheses
    let expr = TestHelper::assert_expr("(2 + 3) * 4");
    let (left, right) = assert_expr::binary_op(&expr, BinaryOp::Mul);
    assert_expr::literal_int(right, 4);

    let inner = assert_expr::group(left);
    let (inner_left, inner_right) = assert_expr::binary_op(inner, BinaryOp::Add);
    assert_expr::literal_int(inner_left, 2);
    assert_expr::literal_int(inner_right, 3);

    // Nested parentheses
    let expr = TestHelper::assert_expr("((1 + 2) * 3)");
    let outer = assert_expr::group(&expr);
    let (left, right) = assert_expr::binary_op(outer, BinaryOp::Mul);
    assert_expr::literal_int(right, 3);

    let inner = assert_expr::group(left);
    let (inner_left, inner_right) = assert_expr::binary_op(inner, BinaryOp::Add);
    assert_expr::literal_int(inner_left, 1);
    assert_expr::literal_int(inner_right, 2);
}

#[test]
fn fn_call_expr() {
    // Simple function call
    let expr = TestHelper::assert_expr("foo()");
    let (_, args) = assert_expr::call(&expr, "foo", 0);
    assert_eq!(args.len(), 0);

    // Function call with arguments
    let expr = TestHelper::assert_expr("add(1, 2, 3)");
    let (_, args) = assert_expr::call(&expr, "add", 3);
    assert_expr::literal_int(&args[0], 1);
    assert_expr::literal_int(&args[1], 2);
    assert_expr::literal_int(&args[2], 3);

    // Nested function calls
    let expr = TestHelper::assert_expr("outer(inner(42))");
    let (_, outer_args) = assert_expr::call(&expr, "outer", 1);
    let (_, inner_args) = assert_expr::call(&outer_args[0], "inner", 1);
    assert_expr::literal_int(&inner_args[0], 42);
}

#[test]
fn arr_lit_expr() {
    // Empty array
    let expr = TestHelper::assert_expr("[]");
    let elements = assert_expr::array(&expr, 0);
    assert_eq!(elements.len(), 0);

    // Array with elements
    let expr = TestHelper::assert_expr("[1, 2, 3]");
    let elements = assert_expr::array(&expr, 3);
    assert_expr::literal_int(&elements[0], 1);
    assert_expr::literal_int(&elements[1], 2);
    assert_expr::literal_int(&elements[2], 3);

    // Nested arrays
    let expr = TestHelper::assert_expr("[[1, 2], [3, 4]]");
    let elements = assert_expr::array(&expr, 2);

    let first_nested = assert_expr::array(&elements[0], 2);
    assert_expr::literal_int(&first_nested[0], 1);
    assert_expr::literal_int(&first_nested[1], 2);

    let second_nested = assert_expr::array(&elements[1], 2);
    assert_expr::literal_int(&second_nested[0], 3);
    assert_expr::literal_int(&second_nested[1], 4);
}

#[test]
fn arr_index_expr() {
    // Simple indexing
    let expr = TestHelper::assert_expr("arr[0]");
    let (object, index) = assert_expr::index(&expr);
    assert_expr::variable(object, "arr");
    assert_expr::literal_int(index, 0);

    // Multi-dimensional indexing
    let expr = TestHelper::assert_expr("matrix[i][j]");
    let (outer_object, outer_index) = assert_expr::index(&expr);
    assert_expr::variable(outer_index, "j");

    let (inner_object, inner_index) = assert_expr::index(outer_object);
    assert_expr::variable(inner_object, "matrix");
    assert_expr::variable(inner_index, "i");

    // Complex expression as index
    let expr = TestHelper::assert_expr("arr[i + 1]");
    let (object, index) = assert_expr::index(&expr);
    assert_expr::variable(object, "arr");

    let (left, right) = assert_expr::binary_op(index, BinaryOp::Add);
    assert_expr::variable(left, "i");
    assert_expr::literal_int(right, 1);
}

#[test]
fn member_expr() {
    // Simple member access
    let expr = TestHelper::assert_expr("obj.property");
    let object = assert_expr::member(&expr, "property");
    assert_expr::variable(object, "obj");

    // Chained member access
    let expr = TestHelper::assert_expr("obj.prop1.prop2");
    let outer_object = assert_expr::member(&expr, "prop2");
    let inner_object = assert_expr::member(outer_object, "prop1");
    assert_expr::variable(inner_object, "obj");

    // Mixed member access and indexing
    let expr = TestHelper::assert_expr("obj.arr[0].prop");
    let final_object = assert_expr::member(&expr, "prop");
    let (index_object, index) = assert_expr::index(final_object);
    let member_object = assert_expr::member(index_object, "arr");
    assert_expr::variable(member_object, "obj");
    assert_expr::literal_int(index, 0);
}

#[test]
fn complex_expr() {
    // Test a complex expression with multiple operators and precedence
    let expr = TestHelper::assert_expr("(a + b) * func(x, y.prop[0]) >= 10 && !flag");

    // Should parse as: ((a + b) * func(x, y.prop[0]) >= 10) && (!flag)
    let (comparison, not_expr) = assert_expr::binary_op(&expr, BinaryOp::And);

    // Check right side: !flag
    let flag_operand = assert_expr::unary_op(not_expr, UnaryOp::Not);
    assert_expr::variable(flag_operand, "flag");

    // Check left side: (a + b) * func(x, y.prop[0]) >= 10
    let (multiplication, ten) = assert_expr::binary_op(comparison, BinaryOp::Ge);
    assert_expr::literal_int(ten, 10);

    // Check multiplication: (a + b) * func(x, y.prop[0])
    let (grouped_add, func_call) = assert_expr::binary_op(multiplication, BinaryOp::Mul);

    // Check grouped addition: (a + b)
    let addition = assert_expr::group(grouped_add);
    let (a_var, b_var) = assert_expr::binary_op(addition, BinaryOp::Add);
    assert_expr::variable(a_var, "a");
    assert_expr::variable(b_var, "b");

    // Check function call: func(x, y.prop[0])
    let (_, args) = assert_expr::call(func_call, "func", 2);
    assert_expr::variable(&args[0], "x");

    // Check second argument: y.prop[0]
    let (member_obj, index) = assert_expr::index(&args[1]);
    let y_obj = assert_expr::member(member_obj, "prop");
    assert_expr::variable(y_obj, "y");
    assert_expr::literal_int(index, 0);
}

#[test]
fn error_expr() {
    for case in ERROR_CASES {
        TestHelper::assert_expr_err(case.source, case.expected);
    }
}

#[test]
fn comment_expr() {
    // Line comments
    let expr = TestHelper::assert_expr("5 + 3 // this is a comment");
    let (left, right) = assert_expr::binary_op(&expr, BinaryOp::Add);
    assert_expr::literal_int(left, 5);
    assert_expr::literal_int(right, 3);

    // Block comments
    let expr = TestHelper::assert_expr("5 /* comment */ + /* another */ 3");
    let (left, right) = assert_expr::binary_op(&expr, BinaryOp::Add);
    assert_expr::literal_int(left, 5);
    assert_expr::literal_int(right, 3);

    // Multi-line block comments
    let expr = TestHelper::assert_expr("5 + /* multi\nline\ncomment */ 3");
    let (left, right) = assert_expr::binary_op(&expr, BinaryOp::Add);
    assert_expr::literal_int(left, 5);
    assert_expr::literal_int(right, 3);
}

#[test]
fn trailing_commas_expr() {
    // Function arguments with trailing comma
    let expr = TestHelper::assert_expr("func(1, 2, 3,)");
    let (_, args) = assert_expr::call(&expr, "func", 3);
    assert_expr::literal_int(&args[0], 1);
    assert_expr::literal_int(&args[1], 2);
    assert_expr::literal_int(&args[2], 3);

    // Array elements with trailing comma
    let expr = TestHelper::assert_expr("[1, 2, 3,]");
    let elements = assert_expr::array(&expr, 3);
    assert_expr::literal_int(&elements[0], 1);
    assert_expr::literal_int(&elements[1], 2);
    assert_expr::literal_int(&elements[2], 3);
}
