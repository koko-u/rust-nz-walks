use crate::features::walks::models;

pub fn is_difficulty(value: &str, _: &()) -> garde::Result {
    match serde_json::from_value::<models::Difficulty>(serde_json::Value::String(value.to_string())) {
        Ok(_) => Ok(()),
        Err(_) => Err(garde::error::Error::new(format!("Invalid difficulty: {value}"))),
    }
}