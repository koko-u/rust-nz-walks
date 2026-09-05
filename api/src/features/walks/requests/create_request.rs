use garde::Validate;

use crate::features::walks::commands;
use crate::features::walks::models;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, garde::Validate, utoipa::ToSchema)]
pub struct CreateRequest {
    #[garde(required, length(min = 1, max = 255))]
    pub name: Option<String>,
    #[garde(length(max = 4000))]
    pub description: Option<String>,
    #[garde(range(min = 0.0))]
    pub length: Option<f64>,
    #[garde(url)]
    pub image_url: Option<String>,
    #[garde(required, alphanumeric, length(min = 1, max = 100))]
    pub region_code: Option<String>,
    #[garde(custom(is_optional_difficulty))]
    pub difficulty: Option<String>,
}

impl CreateRequest {
    pub fn validate_into(self) -> Result<commands::CreateCommand, garde::Report> {
        self.validate()?;
        let difficulty = self.difficulty.map(|d| {
            serde_json::from_value::<models::Difficulty>(serde_json::Value::String(d))
                .expect("difficulty value passed the validation, but cannot convert into models::Difficulty")
        });

        Ok(commands::CreateCommand {
            name: self
                .name
                .expect("name is required, but skip the validation. unexpectedly"),
            description: self.description,
            length: self.length,
            image_url: self.image_url,
            region_code: self
                .region_code
                .expect("region_code is required, but skip the validation. unexpectedly"),
            difficulty,
        })
    }
}

fn is_difficulty(value: &str, _: &()) -> garde::Result {
    match serde_json::from_value::<models::Difficulty>(serde_json::Value::String(value.to_string())) {
        Ok(_) => Ok(()),
        Err(_) => Err(garde::error::Error::new(format!("Invalid difficulty: {value}"))),
    }
}
fn is_optional_difficulty(value: &Option<String>, _: &()) -> garde::Result {
    match value {
        Some(value) => is_difficulty(value, &()),
        None => Ok(()),
    }
}
