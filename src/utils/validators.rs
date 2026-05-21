pub fn validate_not_empty(value: &str, field_name: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{} no puede estar vacío", field_name));
    }
    Ok(())
}

pub fn validate_positive(value: i32, field_name: &str) -> Result<(), String> {
    if value <= 0 {
        return Err(format!("{} debe ser mayor a 0", field_name));
    }
    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), String> {
    if !email.contains('@') {
        return Err("Email inválido".to_string());
    }
    Ok(())
}
