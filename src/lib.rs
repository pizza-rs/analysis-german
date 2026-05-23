#![cfg_attr(not(feature = "std"), no_std)]
//! German language analysis for Pizza search engine.
//!
//! Provides a full-featured German analyzer with character normalization
//! (ß→ss, ä→a, ae→a, etc.), light stemming, and stop words.
//!
//! # Components
//!
//! - [`GermanNormalizationFilter`] — Normalizes umlauts, ß, and digraphs
//! - [`GermanLightStemFilter`] — Light suffix-stripping stemmer
//! - [`GermanStopFilter`] — German stop words filter
extern crate alloc;
mod normalization;
mod stem;
mod stop;

pub mod register;

pub use normalization::GermanNormalizationFilter;
pub use register::register_all;
pub use stem::{GermanLightStemFilter, GermanMinimalStemFilter};
pub use stop::GermanStopFilter;
