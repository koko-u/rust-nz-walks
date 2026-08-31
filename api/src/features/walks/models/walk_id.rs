#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Default,
    derive_more::Display,
    derive_more::From,
    derive_more::FromStr,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[display("{}", _0)]
#[serde(transparent)]
#[schema(value_type = uuid::Uuid)]
pub struct WalkId(uuid::Uuid);

impl WalkId {
    pub fn into_inner(self) -> uuid::Uuid {
        self.0
    }
}