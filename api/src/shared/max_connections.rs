#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    derive_more::Display,
    derive_more::From,
    derive_more::Into,
    derive_more::FromStr,
    serde::Serialize,
    serde::Deserialize,
)]
#[display("{}", _0)]
#[serde(transparent)]
pub struct MaxConnections(u32);

impl Default for MaxConnections {
    fn default() -> Self {
        Self(10)
    }
}
