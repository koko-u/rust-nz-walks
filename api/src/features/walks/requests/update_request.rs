use garde::Validate;

use crate::features::walks::commands;
use crate::features::walks::models;
use crate::features::walks::validators;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, garde::Validate, utoipa::ToSchema)]
pub struct UpdateRequest {
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
    #[garde(custom(validators::is_optional_difficulty))]
    pub difficulty: Option<String>,
}

impl UpdateRequest {
    pub fn validate_into(self, id: models::WalkId) -> Result<commands::UpdateCommand, garde::Report> {
        self.validate()?;
        let difficulty = self.difficulty.map(|d| {
            serde_json::from_value::<models::Difficulty>(serde_json::Value::String(d))
                .expect("difficulty value passed the validation, but cannot convert into models::Difficulty")
        });

        Ok(commands::UpdateCommand {
            id,
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

