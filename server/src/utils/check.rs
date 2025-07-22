use crate::types::common::AppError;

// 验证辅助函数
pub fn validate_not_empty(field: &str, value: &str) -> Result<(), AppError> {
    if value.trim().is_empty() {
        return Err(AppError::validation(field, "cannot be empty"));
    }
    Ok(())
}

pub fn validate_min_length(field: &str, value: &str, min: usize) -> Result<(), AppError> {
    if value.len() < min {
        return Err(AppError::validation(
            field,
            format!("must be at least {} characters", min),
        ));
    }
    Ok(())
}

pub fn validate_positive(field: &str, value: i32) -> Result<(), AppError> {
    if value <= 0 {
        return Err(AppError::validation(field, "must be positive"));
    }
    Ok(())
}
