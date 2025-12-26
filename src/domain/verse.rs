use serde::{Deserialize, Serialize};

/// Bible verse
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct Verse {
    /// Unique verse ID
    pub id: i32,
    /// Book ID (1-66)
    pub book_id: i16,
    /// Chapter number
    pub chapter: i16,
    /// Verse number within chapter
    pub verse: i16,
    /// Verse text content
    pub text: String
}
