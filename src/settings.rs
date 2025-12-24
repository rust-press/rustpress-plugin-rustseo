//! RustSEO Settings
//!
//! Plugin settings configuration and defaults.

use serde::{Deserialize, Serialize};

/// Main settings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoSettings {
    pub site_url: String,
    pub site_name: String,
    pub separator: String,
    pub general: GeneralSettings,
    pub titles: TitleSettings,
    pub meta: MetaSettings,
    pub sitemap: SitemapSettings,
    pub schema: SchemaSettings,
    pub social: SocialSettings,
    pub redirects: RedirectSettings,
    pub robots: RobotsSettings,
    pub advanced: AdvancedSettings,
}

impl Default for SeoSettings {
    fn default() -> Self {
        Self {
            site_url: String::new(),
            site_name: String::new(),
            separator: " - ".to_string(),
            general: GeneralSettings::default(),
            titles: TitleSettings::default(),
            meta: MetaSettings::default(),
            sitemap: SitemapSettings::default(),
            schema: SchemaSettings::default(),
            social: SocialSettings::default(),
            redirects: RedirectSettings::default(),
            robots: RobotsSettings::default(),
            advanced: AdvancedSettings::default(),
        }
    }
}

/// General settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub enable_seo_analysis: bool,
    pub enable_readability_analysis: bool,
    pub show_seo_metabox: bool,
    pub default_robots_index: bool,
    pub default_robots_follow: bool,
    pub strip_category_base: bool,
    pub redirect_attachment_pages: bool,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            enable_seo_analysis: true,
            enable_readability_analysis: true,
            show_seo_metabox: true,
            default_robots_index: true,
            default_robots_follow: true,
            strip_category_base: false,
            redirect_attachment_pages: true,
        }
    }
}

/// Title settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleSettings {
    pub rewrite_titles: bool,
    pub force_rewrite: bool,
    pub home_title: String,
    pub post_title: String,
    pub page_title: String,
    pub category_title: String,
    pub tag_title: String,
    pub author_title: String,
    pub date_title: String,
    pub search_title: String,
    pub not_found_title: String,
}

impl Default for TitleSettings {
    fn default() -> Self {
        Self {
            rewrite_titles: true,
            force_rewrite: false,
            home_title: "%%sitename%% %%sep%% %%tagline%%".to_string(),
            post_title: "%%title%% %%sep%% %%sitename%%".to_string(),
            page_title: "%%title%% %%sep%% %%sitename%%".to_string(),
            category_title: "%%term_title%% Archives %%sep%% %%sitename%%".to_string(),
            tag_title: "%%term_title%% Archives %%sep%% %%sitename%%".to_string(),
            author_title: "%%name%% %%sep%% %%sitename%%".to_string(),
            date_title: "%%date%% %%sep%% %%sitename%%".to_string(),
            search_title: "Search Results for '%%searchphrase%%' %%sep%% %%sitename%%".to_string(),
            not_found_title: "Page Not Found %%sep%% %%sitename%%".to_string(),
        }
    }
}

/// Meta settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaSettings {
    pub generate_description: bool,
    pub description_length: i32,
    pub use_excerpt_as_description: bool,
    pub add_canonical: bool,
    pub add_next_prev: bool,
    pub noindex_subpages: bool,
    pub noindex_search: bool,
    pub noindex_archives: bool,
}

impl Default for MetaSettings {
    fn default() -> Self {
        Self {
            generate_description: true,
            description_length: 160,
            use_excerpt_as_description: true,
            add_canonical: true,
            add_next_prev: true,
            noindex_subpages: false,
            noindex_search: true,
            noindex_archives: false,
        }
    }
}

/// Sitemap settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapSettings {
    pub enabled: bool,
    pub include_posts: bool,
    pub include_pages: bool,
    pub include_categories: bool,
    pub include_tags: bool,
    pub include_authors: bool,
    pub include_images: bool,
    pub max_entries_per_sitemap: i32,
    pub ping_on_publish: bool,
    pub excluded_posts: Vec<String>,
    pub excluded_categories: Vec<String>,
}

impl Default for SitemapSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            include_posts: true,
            include_pages: true,
            include_categories: true,
            include_tags: false,
            include_authors: false,
            include_images: true,
            max_entries_per_sitemap: 1000,
            ping_on_publish: true,
            excluded_posts: vec![],
            excluded_categories: vec![],
        }
    }
}

/// Schema settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaSettings {
    pub enabled: bool,
    pub organization_type: String,
    pub organization_name: String,
    pub organization_logo: Option<String>,
    pub organization_url: Option<String>,
    pub social_profiles: Vec<String>,
    pub article_type: String,
    pub enable_breadcrumbs: bool,
    pub local_business: Option<LocalBusinessSettings>,
}

impl Default for SchemaSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            organization_type: "Organization".to_string(),
            organization_name: String::new(),
            organization_logo: None,
            organization_url: None,
            social_profiles: vec![],
            article_type: "Article".to_string(),
            enable_breadcrumbs: true,
            local_business: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalBusinessSettings {
    pub business_type: String,
    pub name: String,
    pub street_address: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub phone: Option<String>,
    pub price_range: Option<String>,
    pub opening_hours: Vec<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

/// Social media settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialSettings {
    pub enabled: bool,
    pub facebook: FacebookSettings,
    pub twitter: TwitterSettings,
    pub default_image: Option<String>,
}

impl Default for SocialSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            facebook: FacebookSettings::default(),
            twitter: TwitterSettings::default(),
            default_image: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacebookSettings {
    pub opengraph_enabled: bool,
    pub app_id: Option<String>,
    pub admin_id: Option<String>,
    pub default_image: Option<String>,
}

impl Default for FacebookSettings {
    fn default() -> Self {
        Self {
            opengraph_enabled: true,
            app_id: None,
            admin_id: None,
            default_image: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterSettings {
    pub cards_enabled: bool,
    pub card_type: String,
    pub site_username: Option<String>,
    pub default_image: Option<String>,
}

impl Default for TwitterSettings {
    fn default() -> Self {
        Self {
            cards_enabled: true,
            card_type: "summary_large_image".to_string(),
            site_username: None,
            default_image: None,
        }
    }
}

/// Redirect settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectSettings {
    pub enabled: bool,
    pub log_404s: bool,
    pub max_404_logs: i32,
    pub case_insensitive: bool,
    pub auto_redirect_slug_change: bool,
}

impl Default for RedirectSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            log_404s: true,
            max_404_logs: 1000,
            case_insensitive: true,
            auto_redirect_slug_change: true,
        }
    }
}

/// Robots.txt settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotsSettings {
    pub enabled: bool,
    pub custom_rules: String,
    pub block_ai_crawlers: bool,
    pub include_sitemap: bool,
}

impl Default for RobotsSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            custom_rules: String::new(),
            block_ai_crawlers: false,
            include_sitemap: true,
        }
    }
}

/// Advanced settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSettings {
    pub clean_permalinks: bool,
    pub remove_replytocom: bool,
    pub redirect_ugly_urls: bool,
    pub remove_feed_links: bool,
    pub remove_rsd_link: bool,
    pub remove_wlw_link: bool,
    pub remove_shortlink: bool,
    pub disable_author_archives: bool,
    pub disable_date_archives: bool,
    pub output_head_clean: bool,
    pub cache_enabled: bool,
    pub cache_ttl: i32,
}

impl Default for AdvancedSettings {
    fn default() -> Self {
        Self {
            clean_permalinks: true,
            remove_replytocom: true,
            redirect_ugly_urls: false,
            remove_feed_links: false,
            remove_rsd_link: true,
            remove_wlw_link: true,
            remove_shortlink: true,
            disable_author_archives: false,
            disable_date_archives: false,
            output_head_clean: false,
            cache_enabled: true,
            cache_ttl: 3600,
        }
    }
}

/// Settings validation
impl SeoSettings {
    pub fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Validate site URL
        if self.site_url.is_empty() {
            warnings.push("Site URL is not set".to_string());
        } else if !self.site_url.starts_with("http://") && !self.site_url.starts_with("https://") {
            errors.push("Site URL must start with http:// or https://".to_string());
        }

        // Validate site name
        if self.site_name.is_empty() {
            warnings.push("Site name is not set".to_string());
        }

        // Validate meta description length
        if self.meta.description_length < 50 {
            warnings.push("Meta description length is very short".to_string());
        } else if self.meta.description_length > 320 {
            warnings.push("Meta description length is very long".to_string());
        }

        // Validate sitemap settings
        if self.sitemap.max_entries_per_sitemap > 50000 {
            errors.push("Sitemap max entries cannot exceed 50000".to_string());
        }

        ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Title variable replacements
pub fn get_title_variables() -> Vec<TitleVariable> {
    vec![
        TitleVariable {
            variable: "%%sitename%%".to_string(),
            description: "Site name".to_string(),
        },
        TitleVariable {
            variable: "%%tagline%%".to_string(),
            description: "Site tagline".to_string(),
        },
        TitleVariable {
            variable: "%%title%%".to_string(),
            description: "Post/page title".to_string(),
        },
        TitleVariable {
            variable: "%%sep%%".to_string(),
            description: "Separator".to_string(),
        },
        TitleVariable {
            variable: "%%excerpt%%".to_string(),
            description: "Post excerpt".to_string(),
        },
        TitleVariable {
            variable: "%%term_title%%".to_string(),
            description: "Category/tag name".to_string(),
        },
        TitleVariable {
            variable: "%%name%%".to_string(),
            description: "Author name".to_string(),
        },
        TitleVariable {
            variable: "%%date%%".to_string(),
            description: "Date".to_string(),
        },
        TitleVariable {
            variable: "%%searchphrase%%".to_string(),
            description: "Search query".to_string(),
        },
        TitleVariable {
            variable: "%%page%%".to_string(),
            description: "Page number".to_string(),
        },
        TitleVariable {
            variable: "%%currentyear%%".to_string(),
            description: "Current year".to_string(),
        },
        TitleVariable {
            variable: "%%currentmonth%%".to_string(),
            description: "Current month".to_string(),
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleVariable {
    pub variable: String,
    pub description: String,
}

/// Replace title variables with actual values
pub fn replace_title_variables(
    template: &str,
    context: &TitleContext,
) -> String {
    let mut result = template.to_string();

    result = result.replace("%%sitename%%", &context.site_name);
    result = result.replace("%%tagline%%", &context.tagline);
    result = result.replace("%%title%%", &context.title);
    result = result.replace("%%sep%%", &context.separator);
    result = result.replace("%%excerpt%%", &context.excerpt);
    result = result.replace("%%term_title%%", &context.term_title);
    result = result.replace("%%name%%", &context.author_name);
    result = result.replace("%%date%%", &context.date);
    result = result.replace("%%searchphrase%%", &context.search_phrase);
    result = result.replace("%%page%%", &context.page.to_string());
    result = result.replace("%%currentyear%%", &context.current_year);
    result = result.replace("%%currentmonth%%", &context.current_month);

    result
}

#[derive(Debug, Clone, Default)]
pub struct TitleContext {
    pub site_name: String,
    pub tagline: String,
    pub title: String,
    pub separator: String,
    pub excerpt: String,
    pub term_title: String,
    pub author_name: String,
    pub date: String,
    pub search_phrase: String,
    pub page: i32,
    pub current_year: String,
    pub current_month: String,
}
