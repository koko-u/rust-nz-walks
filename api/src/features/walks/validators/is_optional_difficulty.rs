use super::is_difficulty::is_difficulty;

pub fn is_optional_difficulty(value: &Option<String>, _: &()) -> garde::Result {
    match value {
        Some(value) => is_difficulty(value, &()),
        None => Ok(()),
    }
}
