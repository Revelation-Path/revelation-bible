// SPDX-FileCopyrightText: 2024 Revelation Team
// SPDX-FileCopyrightText: 2025 Revelation Team
//
// SPDX-License-Identifier: MIT

//! # revelation-bible
//!
//! Bible domain types for Revelation project.
//!
//! This crate provides pure domain types for Bible data.
//!
//! ## Features
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `db` | SQLx type derives for PostgreSQL |
//! | `api` | OpenAPI schema generation via utoipa |
//! | `backend` | Repository traits via masterror |
//!
//! ## Core Types
//!
//! - [`Book`] - Bible book (Genesis, Exodus, etc.)
//! - [`Verse`] - Individual Bible verse
//! - [`ChapterInfo`] - Chapter metadata
//! - [`Pericope`] - Section headings
//! - [`DailyReading`] - Daily reading plan
//! - [`SearchResult`] - Search/symphony results

pub mod domain;

#[cfg(feature = "backend")]
pub mod ports;

pub use domain::*;
