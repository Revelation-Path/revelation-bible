use serde::{Deserialize, Serialize};

/// Testament type (Old or New Testament)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(feature = "db", sqlx(type_name = "testament", rename_all = "snake_case"))]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub enum Testament {
    Old,
    New
}

/// Bible book
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct Book {
    /// Book ID (1-66)
    pub id: i16,
    /// English name
    pub name: String,
    /// Russian name
    pub name_ru: String,
    /// Short abbreviation (e.g., "Gen", "Ex")
    pub abbreviation: String,
    /// Old or New Testament
    pub testament: Testament,
    /// Number of chapters in this book
    pub chapters_count: i16
}
