use std::future::Future;

use chrono::NaiveDate;
use masterror::AppResult;
use uuid::Uuid;

use crate::domain::DailyReading;

/// Reading plan response with user info
#[derive(Debug)]
pub struct VerseResponseWithUser {
    pub id:         Uuid,
    pub user_id:    Uuid,
    pub user_name:  Option<String>,
    pub content:    String,
    pub created_at: chrono::DateTime<chrono::Utc>
}

/// Reading plan port
pub trait ReadingPlan: Send + Sync {
    fn get_for_day(
        &self,
        day: i16
    ) -> impl Future<Output = AppResult<Option<DailyReading>>> + Send;

    fn get_for_date(
        &self,
        date: NaiveDate
    ) -> impl Future<Output = AppResult<Option<DailyReading>>> + Send;

    fn get_today(&self) -> impl Future<Output = AppResult<Option<DailyReading>>> + Send;

    fn get_responses(
        &self,
        daily_reading_id: Uuid
    ) -> impl Future<Output = AppResult<Vec<VerseResponseWithUser>>> + Send;

    fn add_response(
        &self,
        user_id: Uuid,
        daily_reading_id: Uuid,
        content: &str
    ) -> impl Future<Output = AppResult<Uuid>> + Send;
}
