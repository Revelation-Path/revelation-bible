use serde::{Deserialize, Serialize};

/// Chapter information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct ChapterInfo {
    /// Chapter number
    pub chapter:     i16,
    /// Number of verses in this chapter
    pub verse_count: i16
}
