use std::str;

use crate::features::regions::models as r_models;
use crate::features::walks::models;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QueryCommand {
    // pagination
    pub page: u32,
    pub per_page: u32,

    /// Filter by name containing the given string.
    pub name_contains: Option<String>,

    /// Filter by the difficulty
    pub difficulty: Option<models::Difficulty>,

    /// Filter by the region id
    pub region_id: Option<r_models::RegionId>,

    /// sort order
    pub sort_order: Vec<Sort>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Sort {
    pub field: SortField,
    pub direction: SortDirection,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, derive_more::Display)]
pub enum SortField {
    #[display("name")]
    Name,
    #[display("length")]
    Length,
    #[display("region_code")]
    RegionCode,
    #[display("difficulty")]
    Difficulty,
}

#[derive(Debug, Clone, Eq, PartialEq, derive_more::Display, derive_more::Error)]
#[display("Parse SortField Error: {}", message)]
pub struct SortFieldFromStrError {
    #[error(not(source))]
    pub message: String,
}

impl str::FromStr for SortField {
    type Err = SortFieldFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "name" => Ok(Self::Name),
            "length" => Ok(Self::Length),
            "region_code" => Ok(Self::RegionCode),
            "difficulty" => Ok(Self::Difficulty),
            _ => Err(SortFieldFromStrError {
                message: format!("Invalid sort field: {s}"),
            }),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, derive_more::Display, derive_more::FromStr)]
pub enum SortDirection {
    #[display("asc")]
    Asc,
    #[display("desc")]
    Desc,
}
