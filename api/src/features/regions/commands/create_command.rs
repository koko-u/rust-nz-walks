#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateCommand {
    pub code: String,
    pub name: String,
    pub image_url: Option<String>,
}
