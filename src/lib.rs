//! RustSEO - Complete SEO Optimization Plugin for RustPress
//!
//! A comprehensive SEO solution providing:
//! - Meta tag management (title, description, keywords)
//! - XML sitemap generation
//! - Schema.org structured data markup
//! - OpenGraph and Twitter Cards
//! - Canonical URLs and redirects
//! - Robots.txt management
//! - SEO content analysis
//! - Breadcrumb navigation
//!
//! # Architecture
//!
//! The plugin follows a modular architecture:
//! - **Models**: Data structures for SEO metadata
//! - **Services**: Business logic for SEO operations
//! - **Handlers**: HTTP request handlers for API
//! - **Admin**: Admin interface components

pub mod models;
pub mod handlers;
pub mod services;
pub mod admin;
mod plugin;
mod settings;

use std::sync::Arc;
pub use plugin::RustSeoPlugin;
pub use settings::SeoSettings;

/// Create the RustSEO plugin instance
pub fn create_plugin() -> Arc<dyn rustpress_core::plugin::Plugin> {
    Arc::new(RustSeoPlugin::new())
}

/// Plugin version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Plugin ID
pub const PLUGIN_ID: &str = "rustseo";

/// API namespace
pub const API_NAMESPACE: &str = "seo/v1";

// Re-export commonly used types
pub use models::meta::{SeoMeta, MetaRobots};
pub use models::sitemap::{Sitemap, SitemapUrl, SitemapIndex};
pub use models::schema::{SchemaMarkup, SchemaType};
pub use models::social::{OpenGraphData, TwitterCardData};
pub use models::redirect::{Redirect, RedirectType};
pub use models::analysis::{SeoAnalysis, SeoScore, ContentAnalysis};
pub use services::meta::MetaService;
pub use services::sitemap::SitemapService;
pub use services::schema::SchemaService;
pub use services::analysis::AnalysisService;
