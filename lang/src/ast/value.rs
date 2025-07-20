#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Null,
}

impl Value {
    /// Get the type name as a string
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::Str(_) => "string",
            Value::Null => "null",
        }
    }

    /// Check if the value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::Str(s) => !s.is_empty(),
        }
    }

    /// Convert to boolean
    pub fn to_bool(&self) -> Value {
        Value::Bool(self.is_truthy())
    }

    /// Try to convert to integer
    pub fn to_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            Value::Float(f) => Some(*f as i64),
            Value::Bool(true) => Some(1),
            Value::Bool(false) => Some(0),
            Value::Str(s) => s.parse().ok(),
            Value::Null => None,
        }
    }

    /// Try to convert to float
    pub fn to_float(&self) -> Option<f64> {
        match self {
            Value::Int(i) => Some(*i as f64),
            Value::Float(f) => Some(*f),
            Value::Bool(true) => Some(1.0),
            Value::Bool(false) => Some(0.0),
            Value::Str(s) => s.parse().ok(),
            Value::Null => None,
        }
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Str(s) => s.clone(),
            Value::Null => "null".to_string(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Str(s) => write!(f, "{}", s),
            Value::Null => write!(f, "null"),
        }
    }
}

// Arithmetic operations
impl std::ops::Add for Value {
    type Output = Result<Value, String>;

    fn add(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + b as f64)),
            (Value::Str(a), Value::Str(b)) => Ok(Value::Str(a + &b)),
            (a, b) => Err(format!(
                "Cannot add {} and {}",
                a.type_name(),
                b.type_name()
            )),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Result<Value, String>;

    fn sub(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - b as f64)),
            (a, b) => Err(format!(
                "Cannot subtract {} from {}",
                b.type_name(),
                a.type_name()
            )),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Result<Value, String>;

    fn mul(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * b as f64)),
            (a, b) => Err(format!(
                "Cannot multiply {} and {}",
                a.type_name(),
                b.type_name()
            )),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Result<Value, String>;

    fn div(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => {
                if b == 0 {
                    Err("Division by zero".to_string())
                } else if a % b == 0 {
                    Ok(Value::Int(a / b))
                } else {
                    Ok(Value::Float(a as f64 / b as f64))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(a / b))
                }
            }
            (Value::Int(a), Value::Float(b)) => {
                if b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(a as f64 / b))
                }
            }
            (Value::Float(a), Value::Int(b)) => {
                if b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(a / b as f64))
                }
            }
            (a, b) => Err(format!(
                "Cannot divide {} by {}",
                a.type_name(),
                b.type_name()
            )),
        }
    }
}

// Comparison operations
impl std::cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            (Value::Int(a), Value::Float(b)) => (*a as f64).partial_cmp(b),
            (Value::Float(a), Value::Int(b)) => a.partial_cmp(&(*b as f64)),
            (Value::Str(a), Value::Str(b)) => a.partial_cmp(b),
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}
