// SPDX-FileCopyrightText: 2024 Revelation Team
// SPDX-FileCopyrightText: 2025 Revelation Team
//
// SPDX-License-Identifier: MIT

use std::future::Future;

use masterror::AppResult;

use crate::domain::{Book, ChapterInfo, Pericope, Testament, Verse};

/// Bible repository port
pub trait BibleRepository: Send + Sync {
    fn get_books(&self) -> impl Future<Output = AppResult<Vec<Book>>> + Send;

    fn get_books_by_testament(
        &self,
        testament: Testament
    ) -> impl Future<Output = AppResult<Vec<Book>>> + Send;

    fn get_book(&self, id: i16) -> impl Future<Output = AppResult<Option<Book>>> + Send;

    fn get_chapter(
        &self,
        book_id: i16,
        chapter: i16
    ) -> impl Future<Output = AppResult<Vec<Verse>>> + Send;

    fn get_verse(
        &self,
        book_id: i16,
        chapter: i16,
        verse: i16
    ) -> impl Future<Output = AppResult<Option<Verse>>> + Send;

    fn get_verses_range(
        &self,
        book_id: i16,
        chapter: i16,
        start_verse: i16,
        end_verse: i16
    ) -> impl Future<Output = AppResult<Vec<Verse>>> + Send;

    fn get_pericopes(&self, book_id: i16)
    -> impl Future<Output = AppResult<Vec<Pericope>>> + Send;

    fn get_chapters_info(
        &self,
        book_id: i16
    ) -> impl Future<Output = AppResult<Vec<ChapterInfo>>> + Send;
}
