use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::Verse;

/// Daily reading for one-year Bible reading plan
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct DailyReading {
    /// Unique reading ID
    pub id: Uuid,
    /// Day of year (1-366)
    pub day_of_year: i16,
    /// Calendar date for this reading
    pub date: NaiveDate,
    /// Verses included in this daily reading
    pub verses: Vec<Verse>
}

/// User response to daily reading
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct VerseResponse {
    /// Response ID
    pub id: Uuid,
    /// User who wrote this response
    pub user_id: Uuid,
    /// Which daily reading this responds to
    pub daily_reading_id: Uuid,
    /// User's reflection content
    pub content: String,
    /// When response was created
    pub created_at: DateTime<Utc>
}

/// Request to create verse response
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct CreateVerseResponse {
    /// Which daily reading to respond to
    pub daily_reading_id: Uuid,

    /// User's reflection (1-10000 characters)
    #[validate(length(min = 1, max = 10000))]
    pub content: String
}
