//! XML Sitemap Models
//!
//! Models for generating XML sitemaps for search engines.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Sitemap index containing multiple sitemaps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapIndex {
    pub sitemaps: Vec<SitemapEntry>,
    pub generated_at: DateTime<Utc>,
}

/// Entry in sitemap index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapEntry {
    pub loc: String,
    pub lastmod: Option<DateTime<Utc>>,
}

/// Individual sitemap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sitemap {
    pub sitemap_type: SitemapType,
    pub urls: Vec<SitemapUrl>,
    pub generated_at: DateTime<Utc>,
}

/// Type of sitemap
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SitemapType {
    Posts,
    Pages,
    Products,
    Categories,
    Tags,
    Authors,
    Images,
    Videos,
    News,
    Custom,
}

impl SitemapType {
    pub fn filename(&self) -> &'static str {
        match self {
            Self::Posts => "post-sitemap.xml",
            Self::Pages => "page-sitemap.xml",
            Self::Products => "product-sitemap.xml",
            Self::Categories => "category-sitemap.xml",
            Self::Tags => "tag-sitemap.xml",
            Self::Authors => "author-sitemap.xml",
            Self::Images => "image-sitemap.xml",
            Self::Videos => "video-sitemap.xml",
            Self::News => "news-sitemap.xml",
            Self::Custom => "custom-sitemap.xml",
        }
    }
}

/// URL entry in sitemap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapUrl {
    /// URL location (required)
    pub loc: String,

    /// Last modification date
    pub lastmod: Option<DateTime<Utc>>,

    /// Change frequency
    pub changefreq: Option<ChangeFrequency>,

    /// Priority (0.0 to 1.0)
    pub priority: Option<f32>,

    /// Images for this URL
    pub images: Vec<SitemapImage>,

    /// Videos for this URL
    pub videos: Vec<SitemapVideo>,

    /// News article info
    pub news: Option<SitemapNews>,

    /// Alternate language versions
    pub alternates: Vec<SitemapAlternate>,
}

impl SitemapUrl {
    pub fn new(loc: String) -> Self {
        Self {
            loc,
            lastmod: None,
            changefreq: None,
            priority: None,
            images: vec![],
            videos: vec![],
            news: None,
            alternates: vec![],
        }
    }

    pub fn with_lastmod(mut self, lastmod: DateTime<Utc>) -> Self {
        self.lastmod = Some(lastmod);
        self
    }

    pub fn with_changefreq(mut self, freq: ChangeFrequency) -> Self {
        self.changefreq = Some(freq);
        self
    }

    pub fn with_priority(mut self, priority: f32) -> Self {
        self.priority = Some(priority.clamp(0.0, 1.0));
        self
    }
}

/// Change frequency values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
            Self::Always => "always",
            Self::Hourly => "hourly",
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::Monthly => "monthly",
            Self::Yearly => "yearly",
            Self::Never => "never",
        }
    }
}

/// Image in sitemap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapImage {
    pub loc: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub geo_location: Option<String>,
    pub license: Option<String>,
}

/// Video in sitemap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapVideo {
    pub content_loc: Option<String>,
    pub player_loc: Option<String>,
    pub thumbnail_loc: String,
    pub title: String,
    pub description: String,
    pub duration: Option<i32>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub rating: Option<f32>,
    pub view_count: Option<i64>,
    pub publication_date: Option<DateTime<Utc>>,
    pub family_friendly: bool,
    pub tags: Vec<String>,
    pub category: Option<String>,
    pub requires_subscription: bool,
    pub live: bool,
}

/// News article info for news sitemap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapNews {
    pub publication_name: String,
    pub publication_language: String,
    pub publication_date: DateTime<Utc>,
    pub title: String,
    pub keywords: Vec<String>,
    pub stock_tickers: Vec<String>,
}

/// Alternate language version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapAlternate {
    pub hreflang: String,
    pub href: String,
}

impl Sitemap {
    pub fn new(sitemap_type: SitemapType) -> Self {
        Self {
            sitemap_type,
            urls: vec![],
            generated_at: Utc::now(),
        }
    }

    /// Generate XML string for this sitemap
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\"");

        // Add namespaces if needed
        if self.urls.iter().any(|u| !u.images.is_empty()) {
            xml.push_str(" xmlns:image=\"http://www.google.com/schemas/sitemap-image/1.1\"");
        }
        if self.urls.iter().any(|u| !u.videos.is_empty()) {
            xml.push_str(" xmlns:video=\"http://www.google.com/schemas/sitemap-video/1.1\"");
        }
        if self.urls.iter().any(|u| u.news.is_some()) {
            xml.push_str(" xmlns:news=\"http://www.google.com/schemas/sitemap-news/0.9\"");
        }
        if self.urls.iter().any(|u| !u.alternates.is_empty()) {
            xml.push_str(" xmlns:xhtml=\"http://www.w3.org/1999/xhtml\"");
        }
        xml.push_str(">\n");

        for url in &self.urls {
            xml.push_str("  <url>\n");
            xml.push_str(&format!("    <loc>{}</loc>\n", xml_escape(&url.loc)));

            if let Some(lastmod) = &url.lastmod {
                xml.push_str(&format!(
                    "    <lastmod>{}</lastmod>\n",
                    lastmod.format("%Y-%m-%dT%H:%M:%S%:z")
                ));
            }

            if let Some(freq) = &url.changefreq {
                xml.push_str(&format!("    <changefreq>{}</changefreq>\n", freq.as_str()));
            }

            if let Some(priority) = url.priority {
                xml.push_str(&format!("    <priority>{:.1}</priority>\n", priority));
            }

            // Images
            for image in &url.images {
                xml.push_str("    <image:image>\n");
                xml.push_str(&format!("      <image:loc>{}</image:loc>\n", xml_escape(&image.loc)));
                if let Some(title) = &image.title {
                    xml.push_str(&format!("      <image:title>{}</image:title>\n", xml_escape(title)));
                }
                if let Some(caption) = &image.caption {
                    xml.push_str(&format!("      <image:caption>{}</image:caption>\n", xml_escape(caption)));
                }
                xml.push_str("    </image:image>\n");
            }

            // Alternates (hreflang)
            for alt in &url.alternates {
                xml.push_str(&format!(
                    "    <xhtml:link rel=\"alternate\" hreflang=\"{}\" href=\"{}\"/>\n",
                    xml_escape(&alt.hreflang),
                    xml_escape(&alt.href)
                ));
            }

            xml.push_str("  </url>\n");
        }

        xml.push_str("</urlset>\n");
        xml
    }
}

impl SitemapIndex {
    pub fn new() -> Self {
        Self {
            sitemaps: vec![],
            generated_at: Utc::now(),
        }
    }

    /// Generate XML string for sitemap index
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<sitemapindex xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");

        for sitemap in &self.sitemaps {
            xml.push_str("  <sitemap>\n");
            xml.push_str(&format!("    <loc>{}</loc>\n", xml_escape(&sitemap.loc)));
            if let Some(lastmod) = &sitemap.lastmod {
                xml.push_str(&format!(
                    "    <lastmod>{}</lastmod>\n",
                    lastmod.format("%Y-%m-%dT%H:%M:%S%:z")
                ));
            }
            xml.push_str("  </sitemap>\n");
        }

        xml.push_str("</sitemapindex>\n");
        xml
    }
}

impl Default for SitemapIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// XML escape utility
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Sitemap configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapConfig {
    pub enabled: bool,
    pub max_urls_per_sitemap: usize,
    pub include_images: bool,
    pub include_lastmod: bool,
    pub ping_search_engines: bool,
    pub excluded_urls: Vec<String>,
    pub additional_urls: Vec<SitemapUrl>,
    pub content_types: Vec<SitemapType>,
}

impl Default for SitemapConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_urls_per_sitemap: 1000,
            include_images: true,
            include_lastmod: true,
            ping_search_engines: true,
            excluded_urls: vec![],
            additional_urls: vec![],
            content_types: vec![
                SitemapType::Posts,
                SitemapType::Pages,
                SitemapType::Categories,
            ],
        }
    }
}
