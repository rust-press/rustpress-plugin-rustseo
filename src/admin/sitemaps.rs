//! Sitemap Admin
//!
//! Admin interface for managing XML sitemaps.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Sitemap admin overview data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapOverview {
    pub enabled: bool,
    pub sitemaps: Vec<SitemapInfo>,
    pub total_urls: i64,
    pub last_generated: Option<DateTime<Utc>>,
    pub sitemap_index_url: String,
    pub search_engines: SearchEngineStatus,
}

/// Individual sitemap info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapInfo {
    pub sitemap_type: SitemapType,
    pub name: String,
    pub url: String,
    pub url_count: i64,
    pub last_modified: Option<DateTime<Utc>>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SitemapType {
    Posts,
    Pages,
    Categories,
    Tags,
    Authors,
    Products,
    Images,
    News,
    Video,
    Custom,
}

impl SitemapType {
    pub fn display_name(&self) -> &'static str {
        match self {
            SitemapType::Posts => "Posts",
            SitemapType::Pages => "Pages",
            SitemapType::Categories => "Categories",
            SitemapType::Tags => "Tags",
            SitemapType::Authors => "Authors",
            SitemapType::Products => "Products",
            SitemapType::Images => "Images",
            SitemapType::News => "News",
            SitemapType::Video => "Video",
            SitemapType::Custom => "Custom",
        }
    }
}

/// Search engine ping status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEngineStatus {
    pub google: PingStatus,
    pub bing: PingStatus,
    pub yandex: PingStatus,
}

impl Default for SearchEngineStatus {
    fn default() -> Self {
        Self {
            google: PingStatus::default(),
            bing: PingStatus::default(),
            yandex: PingStatus::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingStatus {
    pub enabled: bool,
    pub last_ping: Option<DateTime<Utc>>,
    pub last_status: Option<String>,
    pub success: bool,
}

impl Default for PingStatus {
    fn default() -> Self {
        Self {
            enabled: true,
            last_ping: None,
            last_status: None,
            success: false,
        }
    }
}

/// Sitemap settings for admin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapSettings {
    pub enabled: bool,
    pub max_urls_per_sitemap: i32,
    pub include_images: bool,
    pub include_news: bool,
    pub include_video: bool,
    pub content_types: SitemapContentTypes,
    pub taxonomies: SitemapTaxonomies,
    pub excluded_items: ExcludedItems,
    pub search_engine_ping: SearchEnginePing,
}

impl Default for SitemapSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            max_urls_per_sitemap: 1000,
            include_images: true,
            include_news: false,
            include_video: false,
            content_types: SitemapContentTypes::default(),
            taxonomies: SitemapTaxonomies::default(),
            excluded_items: ExcludedItems::default(),
            search_engine_ping: SearchEnginePing::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapContentTypes {
    pub posts: ContentTypeSitemap,
    pub pages: ContentTypeSitemap,
    pub products: ContentTypeSitemap,
    pub custom_types: Vec<ContentTypeSitemap>,
}

impl Default for SitemapContentTypes {
    fn default() -> Self {
        Self {
            posts: ContentTypeSitemap {
                name: "Posts".to_string(),
                enabled: true,
                change_frequency: ChangeFrequency::Weekly,
                priority: 0.8,
            },
            pages: ContentTypeSitemap {
                name: "Pages".to_string(),
                enabled: true,
                change_frequency: ChangeFrequency::Monthly,
                priority: 0.6,
            },
            products: ContentTypeSitemap {
                name: "Products".to_string(),
                enabled: true,
                change_frequency: ChangeFrequency::Daily,
                priority: 0.9,
            },
            custom_types: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentTypeSitemap {
    pub name: String,
    pub enabled: bool,
    pub change_frequency: ChangeFrequency,
    pub priority: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeFrequency {
    Always,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Never,
}

impl ChangeFrequency {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChangeFrequency::Always => "always",
            ChangeFrequency::Hourly => "hourly",
            ChangeFrequency::Daily => "daily",
            ChangeFrequency::Weekly => "weekly",
            ChangeFrequency::Monthly => "monthly",
            ChangeFrequency::Yearly => "yearly",
            ChangeFrequency::Never => "never",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapTaxonomies {
    pub categories: TaxonomySitemap,
    pub tags: TaxonomySitemap,
    pub custom_taxonomies: Vec<TaxonomySitemap>,
}

impl Default for SitemapTaxonomies {
    fn default() -> Self {
        Self {
            categories: TaxonomySitemap {
                name: "Categories".to_string(),
                enabled: true,
                change_frequency: ChangeFrequency::Weekly,
                priority: 0.5,
            },
            tags: TaxonomySitemap {
                name: "Tags".to_string(),
                enabled: false,
                change_frequency: ChangeFrequency::Monthly,
                priority: 0.3,
            },
            custom_taxonomies: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomySitemap {
    pub name: String,
    pub enabled: bool,
    pub change_frequency: ChangeFrequency,
    pub priority: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExcludedItems {
    pub posts: Vec<String>,
    pub pages: Vec<String>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEnginePing {
    pub google: bool,
    pub bing: bool,
    pub yandex: bool,
    pub auto_ping_on_publish: bool,
}

impl Default for SearchEnginePing {
    fn default() -> Self {
        Self {
            google: true,
            bing: true,
            yandex: false,
            auto_ping_on_publish: true,
        }
    }
}

/// Sitemap URL entry for admin display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapUrlEntry {
    pub url: String,
    pub title: String,
    pub content_type: String,
    pub last_modified: Option<DateTime<Utc>>,
    pub priority: f32,
    pub images: i32,
    pub is_excluded: bool,
}

/// Sitemap generation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResult {
    pub success: bool,
    pub sitemaps_generated: i32,
    pub total_urls: i64,
    pub generation_time_ms: i64,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Action to regenerate sitemap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegenerateSitemapRequest {
    pub sitemap_type: Option<SitemapType>,
    pub ping_search_engines: bool,
}

/// Sitemap preview data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapPreview {
    pub sitemap_type: SitemapType,
    pub urls: Vec<SitemapUrlEntry>,
    pub total_count: i64,
    pub page: i32,
    pub per_page: i32,
}

/// News sitemap settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsSitemapSettings {
    pub enabled: bool,
    pub publication_name: String,
    pub publication_language: String,
    pub genres: Vec<String>,
    pub categories: Vec<String>,
    pub max_age_days: i32,
}

impl Default for NewsSitemapSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            publication_name: String::new(),
            publication_language: "en".to_string(),
            genres: vec![],
            categories: vec![],
            max_age_days: 2,
        }
    }
}

/// Video sitemap settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSitemapSettings {
    pub enabled: bool,
    pub default_thumbnail: Option<String>,
    pub include_embedded: bool,
    pub platforms: Vec<String>,
}

impl Default for VideoSitemapSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            default_thumbnail: None,
            include_embedded: true,
            platforms: vec!["web".to_string(), "mobile".to_string()],
        }
    }
}

/// Image sitemap settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSitemapSettings {
    pub enabled: bool,
    pub include_featured_images: bool,
    pub include_content_images: bool,
    pub include_gallery_images: bool,
}

impl Default for ImageSitemapSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            include_featured_images: true,
            include_content_images: true,
            include_gallery_images: true,
        }
    }
}

/// Helper to generate sitemap admin menu items
pub fn get_sitemap_tabs() -> Vec<SitemapTab> {
    vec![
        SitemapTab {
            id: "overview".to_string(),
            title: "Overview".to_string(),
            active: true,
        },
        SitemapTab {
            id: "settings".to_string(),
            title: "Settings".to_string(),
            active: false,
        },
        SitemapTab {
            id: "content-types".to_string(),
            title: "Content Types".to_string(),
            active: false,
        },
        SitemapTab {
            id: "taxonomies".to_string(),
            title: "Taxonomies".to_string(),
            active: false,
        },
        SitemapTab {
            id: "exclusions".to_string(),
            title: "Exclusions".to_string(),
            active: false,
        },
        SitemapTab {
            id: "special".to_string(),
            title: "News/Video/Images".to_string(),
            active: false,
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapTab {
    pub id: String,
    pub title: String,
    pub active: bool,
}
