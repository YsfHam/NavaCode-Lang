use super::{RuntimeError, RuntimeValue};

pub fn add(left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
    if let (RuntimeValue::Number(l), RuntimeValue::Number(r)) = (left, right) {
        Ok(RuntimeValue::Number(l + r))
    }
    else {
        Err(RuntimeError::InvalidOperation)
    }
}

pub fn sub(left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
    if let (RuntimeValue::Number(l), RuntimeValue::Number(r)) = (left, right) {
        Ok(RuntimeValue::Number(l - r))
    }
    else {
        Err(RuntimeError::InvalidOperation)
    }
}

pub fn mul(left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
    if let (RuntimeValue::Number(l), RuntimeValue::Number(r)) = (left, right) {
        Ok(RuntimeValue::Number(l * r))
    }
    else {
        Err(RuntimeError::InvalidOperation)
    }
}

pub fn div(left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
    if let (RuntimeValue::Number(l), RuntimeValue::Number(r)) = (left, right) {
        if r == 0 {
            Err(RuntimeError::DivisionByZero)
        }
        else {
            Ok(RuntimeValue::Number(l / r))
        }
    }
    else {
        Err(RuntimeError::InvalidOperation)
    }
}

pub fn eq(left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
    match (left, right) {
        (RuntimeValue::Number(l), RuntimeValue::Number(r)) => Ok(RuntimeValue::Bool(l == r)),
        (RuntimeValue::Bool(l), RuntimeValue::Bool(r)) => Ok(RuntimeValue::Bool(l == r)),

        _ => Err(RuntimeError::InvalidOperation)
    }
}

pub fn gt(left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
    if let (RuntimeValue::Number(l), RuntimeValue::Number(r)) = (left, right) {
        Ok(RuntimeValue::Bool(l > r))
    }
    else {
        Err(RuntimeError::InvalidOperation)
    }
}

pub fn gt_eq(left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
    if let (RuntimeValue::Number(l), RuntimeValue::Number(r)) = (left, right) {
        Ok(RuntimeValue::Bool(l >= r))
    }
    else {
        Err(RuntimeError::InvalidOperation)
    }
}

pub fn lt(left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
    if let (RuntimeValue::Number(l), RuntimeValue::Number(r)) = (left, right) {
        Ok(RuntimeValue::Bool(l < r))
    }
    else {
        Err(RuntimeError::InvalidOperation)
    }
}

pub fn lt_eq(left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
    if let (RuntimeValue::Number(l), RuntimeValue::Number(r)) = (left, right) {
        Ok(RuntimeValue::Bool(l <= r))
    }
    else {
        Err(RuntimeError::InvalidOperation)
    }
}

pub fn not_eq(left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
    if let (RuntimeValue::Number(l), RuntimeValue::Number(r)) = (left, right) {
        Ok(RuntimeValue::Bool(l != r))
    }
    else {
        Err(RuntimeError::InvalidOperation)
    }
}

pub fn and(left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
    if let (RuntimeValue::Bool(l), RuntimeValue::Bool(r)) = (left, right) {
        Ok(RuntimeValue::Bool(l && r))
    }
    else {
        Err(RuntimeError::InvalidOperation)
    }
}

pub fn or(left: RuntimeValue, right: RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
    if let (RuntimeValue::Bool(l), RuntimeValue::Bool(r)) = (left, right) {
        Ok(RuntimeValue::Bool(l || r))
    }
    else {
        Err(RuntimeError::InvalidOperation)
    }
}

pub fn negate(value: RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
    if let RuntimeValue::Number(value) = value {
        Ok(RuntimeValue::Number(- value))
    }
    else {
        Err(RuntimeError::InvalidOperation)
    }
}

pub fn not(value: RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
    if let RuntimeValue::Bool(value) = value {
        Ok(RuntimeValue::Bool(!value))
    }
    else {
        Err(RuntimeError::InvalidOperation)
    }
}