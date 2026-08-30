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
    derive_more::Into,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[display("{}", _0)]
#[serde(transparent)]
#[schema(value_type = uuid::Uuid)]
pub struct WalkId(uuid::Uuid);