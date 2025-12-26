// SPDX-FileCopyrightText: 2024 Revelation Team
// SPDX-FileCopyrightText: 2025 Revelation Team
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use super::Verse;

/// Bible search result (symphony)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct SearchResult {
    /// The found verse
    pub verse:      Verse,
    /// Book name for display
    pub book_name:  String,
    /// Highlight positions (start, end) for matched text
    pub highlights: Vec<(usize, usize)>
}
