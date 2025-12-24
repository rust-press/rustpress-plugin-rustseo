//! Redirects Admin
//!
//! Admin interface for managing URL redirects.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Redirects overview for admin dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectsOverview {
    pub total_redirects: i64,
    pub active_redirects: i64,
    pub total_hits: i64,
    pub recent_404s: i64,
    pub redirects: Vec<RedirectEntry>,
    pub pagination: Pagination,
}

/// Redirect entry for admin list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectEntry {
    pub id: Uuid,
    pub source_url: String,
    pub target_url: String,
    pub redirect_type: RedirectTypeDisplay,
    pub match_type: MatchTypeDisplay,
    pub is_active: bool,
    pub hit_count: i64,
    pub last_hit: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectTypeDisplay {
    pub code: u16,
    pub name: String,
    pub description: String,
}

impl RedirectTypeDisplay {
    pub fn permanent() -> Self {
        Self {
            code: 301,
            name: "301 Permanent".to_string(),
            description: "Permanently moved".to_string(),
        }
    }

    pub fn temporary() -> Self {
        Self {
            code: 302,
            name: "302 Temporary".to_string(),
            description: "Temporarily moved".to_string(),
        }
    }

    pub fn temporary_preserve() -> Self {
        Self {
            code: 307,
            name: "307 Temporary".to_string(),
            description: "Temporary, preserve method".to_string(),
        }
    }

    pub fn permanent_preserve() -> Self {
        Self {
            code: 308,
            name: "308 Permanent".to_string(),
            description: "Permanent, preserve method".to_string(),
        }
    }

    pub fn gone() -> Self {
        Self {
            code: 410,
            name: "410 Gone".to_string(),
            description: "Content deleted".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchTypeDisplay {
    pub value: String,
    pub name: String,
    pub description: String,
}

impl MatchTypeDisplay {
    pub fn exact() -> Self {
        Self {
            value: "exact".to_string(),
            name: "Exact".to_string(),
            description: "URL must match exactly".to_string(),
        }
    }

    pub fn prefix() -> Self {
        Self {
            value: "prefix".to_string(),
            name: "Prefix".to_string(),
            description: "URL starts with pattern".to_string(),
        }
    }

    pub fn contains() -> Self {
        Self {
            value: "contains".to_string(),
            name: "Contains".to_string(),
            description: "URL contains pattern".to_string(),
        }
    }

    pub fn regex() -> Self {
        Self {
            value: "regex".to_string(),
            name: "Regex".to_string(),
            description: "Regular expression match".to_string(),
        }
    }
}

/// Pagination info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: i32,
    pub per_page: i32,
    pub total_items: i64,
    pub total_pages: i32,
}

/// Form data for creating/editing redirects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectForm {
    pub source_url: String,
    pub target_url: String,
    pub redirect_type: String,
    pub match_type: String,
    pub is_active: bool,
    pub notes: Option<String>,
}

/// 404 log entry for admin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotFoundEntry {
    pub id: Uuid,
    pub url: String,
    pub hit_count: i64,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
    pub has_redirect: bool,
    pub is_ignored: bool,
}

/// 404 logs overview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotFoundOverview {
    pub total_404s: i64,
    pub unique_urls: i64,
    pub total_hits: i64,
    pub entries: Vec<NotFoundEntry>,
    pub pagination: Pagination,
}

/// Redirect import/export format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectImportExport {
    pub format: ImportFormat,
    pub redirects: Vec<RedirectImportEntry>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImportFormat {
    Csv,
    Json,
    Htaccess,
    NginxConf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectImportEntry {
    pub source: String,
    pub target: String,
    pub redirect_type: Option<String>,
    pub match_type: Option<String>,
}

/// Import result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub success: bool,
    pub imported: i32,
    pub skipped: i32,
    pub errors: Vec<ImportError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportError {
    pub line: i32,
    pub source: String,
    pub message: String,
}

/// Redirect test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectTestResult {
    pub url: String,
    pub matched: bool,
    pub redirect: Option<MatchedRedirect>,
    pub redirect_chain: Vec<RedirectChainEntry>,
    pub final_url: Option<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedRedirect {
    pub id: Uuid,
    pub source: String,
    pub target: String,
    pub status_code: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectChainEntry {
    pub url: String,
    pub status_code: u16,
    pub step: i32,
}

/// Redirect settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectSettings {
    pub enabled: bool,
    pub log_404s: bool,
    pub max_404_log_entries: i32,
    pub case_insensitive: bool,
    pub trailing_slash_handling: TrailingSlashHandling,
    pub query_string_handling: QueryStringHandling,
    pub auto_redirect_post_slug_change: bool,
    pub monitor_redirects: bool,
    pub max_redirect_chain: i32,
}

impl Default for RedirectSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            log_404s: true,
            max_404_log_entries: 1000,
            case_insensitive: true,
            trailing_slash_handling: TrailingSlashHandling::Ignore,
            query_string_handling: QueryStringHandling::Ignore,
            auto_redirect_post_slug_change: true,
            monitor_redirects: true,
            max_redirect_chain: 5,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrailingSlashHandling {
    Ignore,
    Add,
    Remove,
    Exact,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryStringHandling {
    Ignore,
    Pass,
    Exact,
}

/// Bulk action for redirects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkAction {
    pub action: BulkActionType,
    pub ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BulkActionType {
    Activate,
    Deactivate,
    Delete,
    Export,
}

/// Bulk action result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkActionResult {
    pub success: bool,
    pub affected: i32,
    pub errors: Vec<String>,
}

/// Redirect statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectStats {
    pub total_redirects: i64,
    pub active_redirects: i64,
    pub total_hits_today: i64,
    pub total_hits_week: i64,
    pub total_hits_month: i64,
    pub top_redirects: Vec<TopRedirect>,
    pub redirect_types: Vec<TypeCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopRedirect {
    pub id: Uuid,
    pub source: String,
    pub target: String,
    pub hits: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeCount {
    pub redirect_type: String,
    pub count: i64,
}

/// Get redirect type options for form
pub fn get_redirect_type_options() -> Vec<RedirectTypeDisplay> {
    vec![
        RedirectTypeDisplay::permanent(),
        RedirectTypeDisplay::temporary(),
        RedirectTypeDisplay::temporary_preserve(),
        RedirectTypeDisplay::permanent_preserve(),
        RedirectTypeDisplay::gone(),
    ]
}

/// Get match type options for form
pub fn get_match_type_options() -> Vec<MatchTypeDisplay> {
    vec![
        MatchTypeDisplay::exact(),
        MatchTypeDisplay::prefix(),
        MatchTypeDisplay::contains(),
        MatchTypeDisplay::regex(),
    ]
}

/// Redirect admin tabs
pub fn get_redirect_tabs() -> Vec<RedirectTab> {
    vec![
        RedirectTab {
            id: "redirects".to_string(),
            title: "Redirects".to_string(),
            icon: "arrow-right".to_string(),
            active: true,
        },
        RedirectTab {
            id: "404s".to_string(),
            title: "404 Monitor".to_string(),
            icon: "alert-triangle".to_string(),
            active: false,
        },
        RedirectTab {
            id: "import-export".to_string(),
            title: "Import/Export".to_string(),
            icon: "download".to_string(),
            active: false,
        },
        RedirectTab {
            id: "settings".to_string(),
            title: "Settings".to_string(),
            icon: "settings".to_string(),
            active: false,
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectTab {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub active: bool,
}

/// Common redirect patterns for quick add
pub fn get_common_patterns() -> Vec<CommonPattern> {
    vec![
        CommonPattern {
            name: "WWW to non-WWW".to_string(),
            source: "^https?://www\\.example\\.com/(.*)$".to_string(),
            target: "https://example.com/$1".to_string(),
            match_type: "regex".to_string(),
            description: "Redirect www subdomain to root domain".to_string(),
        },
        CommonPattern {
            name: "Non-WWW to WWW".to_string(),
            source: "^https?://example\\.com/(.*)$".to_string(),
            target: "https://www.example.com/$1".to_string(),
            match_type: "regex".to_string(),
            description: "Redirect root domain to www subdomain".to_string(),
        },
        CommonPattern {
            name: "HTTP to HTTPS".to_string(),
            source: "^http://(.*)$".to_string(),
            target: "https://$1".to_string(),
            match_type: "regex".to_string(),
            description: "Redirect all HTTP to HTTPS".to_string(),
        },
        CommonPattern {
            name: "Remove trailing slash".to_string(),
            source: "^(.+)/$".to_string(),
            target: "$1".to_string(),
            match_type: "regex".to_string(),
            description: "Remove trailing slashes from URLs".to_string(),
        },
        CommonPattern {
            name: "Category to new URL".to_string(),
            source: "/category/old-category/".to_string(),
            target: "/category/new-category/".to_string(),
            match_type: "exact".to_string(),
            description: "Redirect old category to new category".to_string(),
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonPattern {
    pub name: String,
    pub source: String,
    pub target: String,
    pub match_type: String,
    pub description: String,
}
