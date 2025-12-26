//! Text highlighting in Bible verses.

use chrono::{DateTime, Utc};
use entity_derive::Entity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Highlight color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(feature = "db", sqlx(type_name = "highlight_color", rename_all = "snake_case"))]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum HighlightColor {
    #[default]
    Yellow,
    Green,
    Blue,
    Pink,
    Orange,
    Purple
}

/// Text highlight within a Bible verse.
///
/// Allows users to highlight specific portions of verse text.
#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[entity(table = "bible_highlights", sql = "trait")]
pub struct Highlight {
    /// Unique highlight ID
    #[id]
    pub id: Uuid,

    /// User who created this highlight
    #[field(create, response)]
    pub user_id: Uuid,

    /// Verse containing the highlighted text
    #[field(create, response)]
    pub verse_id: i32,

    /// Start position in verse text (character index)
    #[field(create, response)]
    pub start_pos: i32,

    /// End position in verse text (character index)
    #[field(create, response)]
    pub end_pos: i32,

    /// Highlight color
    #[field(create, update, response)]
    pub color: HighlightColor,

    /// When highlight was created
    #[field(response)]
    #[auto]
    pub created_at: DateTime<Utc>
}
