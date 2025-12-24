//! URL Redirect Models
//!
//! Models for managing URL redirects (301, 302, etc.)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// URL redirect rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Redirect {
    pub id: Uuid,
    pub source_url: String,
    pub target_url: String,
    pub redirect_type: RedirectType,
    pub match_type: MatchType,
    pub is_regex: bool,
    pub is_active: bool,
    pub hit_count: i64,
    pub last_accessed: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Redirect type (HTTP status code)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RedirectType {
    /// 301 Moved Permanently
    Permanent,
    /// 302 Found (Temporary)
    Temporary,
    /// 307 Temporary Redirect (preserves method)
    TemporaryPreserve,
    /// 308 Permanent Redirect (preserves method)
    PermanentPreserve,
    /// 410 Gone (content deleted)
    Gone,
    /// 451 Unavailable for Legal Reasons
    LegalRestriction,
}

impl RedirectType {
    pub fn status_code(&self) -> u16 {
        match self {
            Self::Permanent => 301,
            Self::Temporary => 302,
            Self::TemporaryPreserve => 307,
            Self::PermanentPreserve => 308,
            Self::Gone => 410,
            Self::LegalRestriction => 451,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Permanent => "301 Moved Permanently",
            Self::Temporary => "302 Found (Temporary)",
            Self::TemporaryPreserve => "307 Temporary Redirect",
            Self::PermanentPreserve => "308 Permanent Redirect",
            Self::Gone => "410 Gone",
            Self::LegalRestriction => "451 Unavailable for Legal Reasons",
        }
    }
}

/// Match type for source URL
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchType {
    /// Exact match
    Exact,
    /// Prefix match (starts with)
    Prefix,
    /// Contains
    Contains,
    /// Regex pattern
    Regex,
}

impl Redirect {
    pub fn new(source_url: String, target_url: String, redirect_type: RedirectType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            source_url,
            target_url,
            redirect_type,
            match_type: MatchType::Exact,
            is_regex: false,
            is_active: true,
            hit_count: 0,
            last_accessed: None,
            notes: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if a URL matches this redirect rule
    pub fn matches(&self, url: &str) -> bool {
        if !self.is_active {
            return false;
        }

        match self.match_type {
            MatchType::Exact => url == self.source_url,
            MatchType::Prefix => url.starts_with(&self.source_url),
            MatchType::Contains => url.contains(&self.source_url),
            MatchType::Regex => {
                if let Ok(re) = regex::Regex::new(&self.source_url) {
                    re.is_match(url)
                } else {
                    false
                }
            }
        }
    }

    /// Get the target URL, applying regex replacements if needed
    pub fn get_target(&self, url: &str) -> String {
        if self.is_regex && self.match_type == MatchType::Regex {
            if let Ok(re) = regex::Regex::new(&self.source_url) {
                return re.replace(url, &self.target_url).to_string();
            }
        }
        self.target_url.clone()
    }

    /// Increment hit counter
    pub fn record_hit(&mut self) {
        self.hit_count += 1;
        self.last_accessed = Some(Utc::now());
    }
}

/// 404 error log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotFoundLog {
    pub id: Uuid,
    pub url: String,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub hit_count: i64,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub has_redirect: bool,
    pub is_ignored: bool,
}

impl NotFoundLog {
    pub fn new(url: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            url,
            referrer: None,
            user_agent: None,
            ip_address: None,
            hit_count: 1,
            first_seen: now,
            last_seen: now,
            has_redirect: false,
            is_ignored: false,
        }
    }

    pub fn record_hit(&mut self) {
        self.hit_count += 1;
        self.last_seen = Utc::now();
    }
}

/// Redirect group for organizing redirects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectGroup {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub redirect_count: i32,
    pub created_at: DateTime<Utc>,
}

impl RedirectGroup {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            name,
            description: None,
            is_active: true,
            redirect_count: 0,
            created_at: Utc::now(),
        }
    }
}

/// Redirect import/export format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectImport {
    pub source: String,
    pub target: String,
    pub redirect_type: Option<String>,
    pub match_type: Option<String>,
}

/// Redirect statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectStats {
    pub total_redirects: i64,
    pub active_redirects: i64,
    pub total_hits: i64,
    pub hits_today: i64,
    pub hits_this_week: i64,
    pub hits_this_month: i64,
    pub top_redirects: Vec<RedirectHitSummary>,
    pub recent_404s: Vec<NotFoundSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectHitSummary {
    pub source_url: String,
    pub target_url: String,
    pub hit_count: i64,
    pub last_accessed: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotFoundSummary {
    pub url: String,
    pub hit_count: i64,
    pub last_seen: DateTime<Utc>,
}

/// Redirect settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectSettings {
    pub enabled: bool,
    pub log_404s: bool,
    pub auto_redirect_404_to_homepage: bool,
    pub redirect_attachment_pages: bool,
    pub redirect_category_base: bool,
    pub redirect_tag_base: bool,
    pub pass_query_string: bool,
    pub monitor_changes: bool,
    pub case_insensitive: bool,
}

impl Default for RedirectSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            log_404s: true,
            auto_redirect_404_to_homepage: false,
            redirect_attachment_pages: true,
            redirect_category_base: false,
            redirect_tag_base: false,
            pass_query_string: true,
            monitor_changes: true,
            case_insensitive: true,
        }
    }
}
