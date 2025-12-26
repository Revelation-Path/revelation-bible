//! # revelation-bible
//!
//! Bible domain and loader for Revelation project.
//!
//! This crate provides Bible domain types and tools for loading Bible data.
//!
//! ## Features
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `db` | Database support via sqlx (PostgreSQL) |
//! | `api` | OpenAPI schema generation via utoipa |
//! | `user` | User-specific data: bookmarks, notes, highlights |
//! | `backend` | Backend error handling via masterror |
//! | `cli` | Command-line tools for Bible data loading |
//!
//! ## Core Types
//!
//! - [`Book`] - Bible book (Genesis, Exodus, etc.)
//! - [`Verse`] - Individual Bible verse
//! - [`ChapterInfo`] - Chapter metadata
//! - [`Pericope`] - Section headings
//! - [`DailyReading`] - Daily reading plan
//! - [`SearchResult`] - Search/symphony results
//!
//! ## User Data (feature = "user")
//!
//! - [`Bookmark`] - Saved verses with colors
//! - [`Note`] - Personal study notes
//! - [`Highlight`] - Text highlighting
//! - [`ReadingProgress`] - Reading progress tracking

pub mod domain;

#[cfg(feature = "cli")]
pub mod loader;

#[cfg(feature = "backend")]
pub mod ports;

#[cfg(feature = "user")]
pub mod user_data;

// Re-export domain types
pub use domain::*;

#[cfg(feature = "cli")]
pub use loader::{BibleLoader, LoadStats};

// Re-export user data types
#[cfg(feature = "user")]
pub use user_data::*;
