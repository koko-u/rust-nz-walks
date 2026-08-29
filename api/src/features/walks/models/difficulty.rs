#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::FromStr,
    derive_more::IsVariant,
    serde::Serialize,
    serde::Deserialize,
    sqlx::Type,
)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "difficulty")]
#[sqlx(rename_all = "snake_case")]
pub enum Difficulty {
    #[display("easy")]
    Easy,
    #[display("medium")]
    Medium,
    #[display("hard")]
    Hard,
    #[display("expert")]
    Expert,
}
