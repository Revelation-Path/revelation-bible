// SPDX-FileCopyrightText: 2024 Revelation Team
// SPDX-FileCopyrightText: 2025 Revelation Team
//
// SPDX-License-Identifier: MIT

use std::future::Future;

use masterror::AppResult;

use crate::domain::SearchResult;

/// Bible search port
pub trait BibleSearch: Send + Sync {
    fn search(
        &self,
        query: &str,
        limit: i64
    ) -> impl Future<Output = AppResult<Vec<SearchResult>>> + Send;

    fn symphony(
        &self,
        word: &str,
        limit: i64
    ) -> impl Future<Output = AppResult<Vec<SearchResult>>> + Send;

    fn word_count(&self, word: &str) -> impl Future<Output = AppResult<i64>> + Send;
}
