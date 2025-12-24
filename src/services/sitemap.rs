//! Sitemap Generation Service
//!
//! Service for generating XML sitemaps.

use crate::models::sitemap::{
    Sitemap, SitemapIndex, SitemapEntry, SitemapUrl, SitemapType,
    SitemapConfig, ChangeFrequency, SitemapImage,
};
use chrono::{DateTime, Utc};

/// Service for generating and managing XML sitemaps
pub struct SitemapService {
    site_url: String,
    config: SitemapConfig,
}

impl SitemapService {
    pub fn new(site_url: String) -> Self {
        Self {
            site_url: site_url.trim_end_matches('/').to_string(),
            config: SitemapConfig::default(),
        }
    }

    pub fn with_config(mut self, config: SitemapConfig) -> Self {
        self.config = config;
        self
    }

    /// Generate sitemap index
    pub fn generate_index(&self, sitemaps: &[(&SitemapType, DateTime<Utc>)]) -> SitemapIndex {
        let mut index = SitemapIndex::new();

        for (sitemap_type, lastmod) in sitemaps {
            index.sitemaps.push(SitemapEntry {
                loc: format!("{}/{}", self.site_url, sitemap_type.filename()),
                lastmod: Some(*lastmod),
            });
        }

        index
    }

    /// Generate posts sitemap
    pub fn generate_posts_sitemap(&self, posts: Vec<PostData>) -> Sitemap {
        let mut sitemap = Sitemap::new(SitemapType::Posts);

        for post in posts {
            if self.is_excluded(&post.url) {
                continue;
            }

            let mut url = SitemapUrl::new(post.url)
                .with_lastmod(post.modified_at)
                .with_changefreq(ChangeFrequency::Weekly)
                .with_priority(0.8);

            if self.config.include_images {
                for image in post.images {
                    url.images.push(SitemapImage {
                        loc: image.url,
                        title: image.title,
                        caption: image.caption,
                        geo_location: None,
                        license: None,
                    });
                }
            }

            sitemap.urls.push(url);
        }

        sitemap
    }

    /// Generate pages sitemap
    pub fn generate_pages_sitemap(&self, pages: Vec<PageData>) -> Sitemap {
        let mut sitemap = Sitemap::new(SitemapType::Pages);

        for page in pages {
            if self.is_excluded(&page.url) {
                continue;
            }

            let priority = if page.is_front_page { 1.0 } else { 0.6 };
            let changefreq = if page.is_front_page {
                ChangeFrequency::Daily
            } else {
                ChangeFrequency::Monthly
            };

            let url = SitemapUrl::new(page.url)
                .with_lastmod(page.modified_at)
                .with_changefreq(changefreq)
                .with_priority(priority);

            sitemap.urls.push(url);
        }

        sitemap
    }

    /// Generate categories sitemap
    pub fn generate_categories_sitemap(&self, categories: Vec<CategoryData>) -> Sitemap {
        let mut sitemap = Sitemap::new(SitemapType::Categories);

        for category in categories {
            if self.is_excluded(&category.url) {
                continue;
            }

            let url = SitemapUrl::new(category.url)
                .with_lastmod(category.modified_at)
                .with_changefreq(ChangeFrequency::Weekly)
                .with_priority(0.5);

            sitemap.urls.push(url);
        }

        sitemap
    }

    /// Generate products sitemap
    pub fn generate_products_sitemap(&self, products: Vec<ProductData>) -> Sitemap {
        let mut sitemap = Sitemap::new(SitemapType::Products);

        for product in products {
            if self.is_excluded(&product.url) {
                continue;
            }

            let mut url = SitemapUrl::new(product.url)
                .with_lastmod(product.modified_at)
                .with_changefreq(ChangeFrequency::Daily)
                .with_priority(0.9);

            if self.config.include_images {
                for image in product.images {
                    url.images.push(SitemapImage {
                        loc: image.url,
                        title: Some(product.name.clone()),
                        caption: image.caption,
                        geo_location: None,
                        license: None,
                    });
                }
            }

            sitemap.urls.push(url);
        }

        sitemap
    }

    /// Check if URL is excluded
    fn is_excluded(&self, url: &str) -> bool {
        for pattern in &self.config.excluded_urls {
            if url.contains(pattern) {
                return true;
            }
        }
        false
    }

    /// Ping search engines about sitemap update
    pub async fn ping_search_engines(&self) -> Vec<PingResult> {
        let sitemap_url = format!("{}/sitemap_index.xml", self.site_url);
        let mut results = Vec::new();

        if !self.config.ping_search_engines {
            return results;
        }

        // Google
        results.push(PingResult {
            search_engine: "Google".to_string(),
            url: format!(
                "https://www.google.com/ping?sitemap={}",
                urlencoding::encode(&sitemap_url)
            ),
            success: true, // Would actually make HTTP request
            message: None,
        });

        // Bing
        results.push(PingResult {
            search_engine: "Bing".to_string(),
            url: format!(
                "https://www.bing.com/ping?sitemap={}",
                urlencoding::encode(&sitemap_url)
            ),
            success: true,
            message: None,
        });

        results
    }

    /// Get sitemap URL
    pub fn get_sitemap_url(&self, sitemap_type: &SitemapType) -> String {
        format!("{}/{}", self.site_url, sitemap_type.filename())
    }

    /// Get sitemap index URL
    pub fn get_index_url(&self) -> String {
        format!("{}/sitemap_index.xml", self.site_url)
    }

    /// Validate sitemap
    pub fn validate(&self, sitemap: &Sitemap) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check URL count
        if sitemap.urls.len() > 50000 {
            errors.push("Sitemap exceeds 50,000 URL limit".to_string());
        } else if sitemap.urls.len() > 45000 {
            warnings.push("Sitemap approaching 50,000 URL limit".to_string());
        }

        // Check for duplicate URLs
        let mut seen = std::collections::HashSet::new();
        for url in &sitemap.urls {
            if !seen.insert(&url.loc) {
                warnings.push(format!("Duplicate URL: {}", url.loc));
            }
        }

        // Estimate file size (rough)
        let xml = sitemap.to_xml();
        let size_mb = xml.len() as f64 / (1024.0 * 1024.0);
        if size_mb > 50.0 {
            errors.push(format!("Sitemap exceeds 50MB limit ({:.2}MB)", size_mb));
        } else if size_mb > 40.0 {
            warnings.push(format!("Sitemap approaching 50MB limit ({:.2}MB)", size_mb));
        }

        ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings,
            url_count: sitemap.urls.len(),
            size_bytes: xml.len(),
        }
    }
}

/// Post data for sitemap generation
pub struct PostData {
    pub url: String,
    pub modified_at: DateTime<Utc>,
    pub images: Vec<ImageData>,
}

/// Page data for sitemap generation
pub struct PageData {
    pub url: String,
    pub modified_at: DateTime<Utc>,
    pub is_front_page: bool,
}

/// Category data for sitemap generation
pub struct CategoryData {
    pub url: String,
    pub modified_at: DateTime<Utc>,
}

/// Product data for sitemap generation
pub struct ProductData {
    pub url: String,
    pub name: String,
    pub modified_at: DateTime<Utc>,
    pub images: Vec<ImageData>,
}

/// Image data
pub struct ImageData {
    pub url: String,
    pub title: Option<String>,
    pub caption: Option<String>,
}

/// Ping result
pub struct PingResult {
    pub search_engine: String,
    pub url: String,
    pub success: bool,
    pub message: Option<String>,
}

/// Validation result
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub url_count: usize,
    pub size_bytes: usize,
}

use urlencoding;
