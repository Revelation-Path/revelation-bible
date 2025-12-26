use serde::{Deserialize, Serialize};

/// Pericope - Bible section heading
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct Pericope {
    /// Chapter where this pericope starts
    pub chapter: i16,
    /// Verse where this pericope starts
    pub verse: i16,
    /// Section heading text
    pub heading: String
}
