//! SEO Dashboard
//!
//! Main dashboard showing SEO overview and statistics.

use serde::{Deserialize, Serialize};

/// Dashboard data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub overview: SeoOverview,
    pub recent_posts: Vec<PostSeoStatus>,
    pub issues: Vec<SeoIssue>,
    pub sitemap_status: SitemapStatus,
    pub search_console: Option<SearchConsoleData>,
}

/// SEO overview statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoOverview {
    pub total_posts: i64,
    pub posts_with_focus_keyword: i64,
    pub posts_with_meta_description: i64,
    pub posts_with_good_score: i64,
    pub posts_needing_improvement: i64,
    pub average_seo_score: f32,
    pub indexed_pages: Option<i64>,
}

/// Post SEO status for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSeoStatus {
    pub id: String,
    pub title: String,
    pub url: String,
    pub seo_score: i32,
    pub focus_keyword: Option<String>,
    pub has_meta_description: bool,
    pub issues_count: i32,
}

/// SEO issue summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoIssue {
    pub issue_type: IssueType,
    pub severity: String,
    pub count: i32,
    pub description: String,
    pub action_url: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueType {
    MissingMetaDescription,
    MissingFocusKeyword,
    LowContentLength,
    MissingAltText,
    BrokenLinks,
    MissingCanonical,
    DuplicateContent,
    SlowPageSpeed,
}

/// Sitemap status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitemapStatus {
    pub enabled: bool,
    pub last_generated: Option<String>,
    pub total_urls: i64,
    pub sitemap_url: String,
    pub indexed_urls: Option<i64>,
}

/// Search Console data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConsoleData {
    pub connected: bool,
    pub total_clicks: i64,
    pub total_impressions: i64,
    pub average_ctr: f32,
    pub average_position: f32,
    pub top_queries: Vec<SearchQuery>,
    pub top_pages: Vec<TopPage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub clicks: i64,
    pub impressions: i64,
    pub ctr: f32,
    pub position: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopPage {
    pub url: String,
    pub clicks: i64,
    pub impressions: i64,
}

impl DashboardData {
    pub fn empty() -> Self {
        Self {
            overview: SeoOverview {
                total_posts: 0,
                posts_with_focus_keyword: 0,
                posts_with_meta_description: 0,
                posts_with_good_score: 0,
                posts_needing_improvement: 0,
                average_seo_score: 0.0,
                indexed_pages: None,
            },
            recent_posts: vec![],
            issues: vec![],
            sitemap_status: SitemapStatus {
                enabled: true,
                last_generated: None,
                total_urls: 0,
                sitemap_url: String::new(),
                indexed_urls: None,
            },
            search_console: None,
        }
    }
}

/// Dashboard widget configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    pub id: String,
    pub title: String,
    pub position: i32,
    pub enabled: bool,
}

/// Get default dashboard widgets
pub fn get_default_widgets() -> Vec<DashboardWidget> {
    vec![
        DashboardWidget {
            id: "seo-overview".to_string(),
            title: "SEO Overview".to_string(),
            position: 1,
            enabled: true,
        },
        DashboardWidget {
            id: "seo-issues".to_string(),
            title: "Issues to Fix".to_string(),
            position: 2,
            enabled: true,
        },
        DashboardWidget {
            id: "recent-posts".to_string(),
            title: "Recent Posts SEO".to_string(),
            position: 3,
            enabled: true,
        },
        DashboardWidget {
            id: "sitemap-status".to_string(),
            title: "Sitemap Status".to_string(),
            position: 4,
            enabled: true,
        },
        DashboardWidget {
            id: "search-console".to_string(),
            title: "Search Console".to_string(),
            position: 5,
            enabled: true,
        },
    ]
}
