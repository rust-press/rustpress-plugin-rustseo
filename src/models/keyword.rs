//! Keyword Tracking Models
//!
//! Models for tracking and managing focus keywords and rankings.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Focus keyword for a content item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusKeyword {
    pub id: Uuid,
    pub content_id: Uuid,
    pub keyword: String,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
}

impl FocusKeyword {
    pub fn new(content_id: Uuid, keyword: String, is_primary: bool) -> Self {
        Self {
            id: Uuid::now_v7(),
            content_id,
            keyword,
            is_primary,
            created_at: Utc::now(),
        }
    }
}

/// Keyword ranking data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordRanking {
    pub id: Uuid,
    pub keyword_id: Uuid,
    pub keyword: String,
    pub search_engine: SearchEngine,
    pub position: Option<i32>,
    pub previous_position: Option<i32>,
    pub url: String,
    pub search_volume: Option<i64>,
    pub cpc: Option<f32>,
    pub competition: Option<f32>,
    pub checked_at: DateTime<Utc>,
}

/// Supported search engines
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchEngine {
    Google,
    Bing,
    Yahoo,
    DuckDuckGo,
    Yandex,
    Baidu,
}

impl SearchEngine {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Google => "Google",
            Self::Bing => "Bing",
            Self::Yahoo => "Yahoo",
            Self::DuckDuckGo => "DuckDuckGo",
            Self::Yandex => "Yandex",
            Self::Baidu => "Baidu",
        }
    }
}

/// Keyword suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordSuggestion {
    pub keyword: String,
    pub search_volume: Option<i64>,
    pub difficulty: Option<f32>,
    pub cpc: Option<f32>,
    pub trend: Option<KeywordTrend>,
    pub related_keywords: Vec<String>,
}

/// Keyword trend direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KeywordTrend {
    Rising,
    Stable,
    Declining,
}

/// Keyword research result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordResearch {
    pub seed_keyword: String,
    pub suggestions: Vec<KeywordSuggestion>,
    pub questions: Vec<String>,
    pub long_tail: Vec<String>,
    pub related_topics: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

/// Keyword density analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordDensity {
    pub keyword: String,
    pub count: usize,
    pub density: f32,
    pub is_optimal: bool,
    pub recommendation: Option<String>,
}

impl KeywordDensity {
    pub fn calculate(keyword: &str, content: &str, target_density: f32) -> Self {
        let word_count = content.split_whitespace().count();
        let keyword_lower = keyword.to_lowercase();
        let content_lower = content.to_lowercase();

        let count = content_lower.matches(&keyword_lower).count();
        let density = if word_count > 0 {
            (count as f32 / word_count as f32) * 100.0
        } else {
            0.0
        };

        let is_optimal = density >= target_density * 0.5 && density <= target_density * 1.5;

        let recommendation = if density < target_density * 0.5 {
            Some(format!(
                "Consider adding the keyword {} more times",
                ((target_density * word_count as f32 / 100.0) as usize).saturating_sub(count)
            ))
        } else if density > target_density * 1.5 {
            Some("The keyword density is too high. Consider removing some occurrences.".to_string())
        } else {
            None
        };

        Self {
            keyword: keyword.to_string(),
            count,
            density,
            is_optimal,
            recommendation,
        }
    }
}

/// Keyword tracking settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordSettings {
    pub tracking_enabled: bool,
    pub check_frequency: CheckFrequency,
    pub search_engines: Vec<SearchEngine>,
    pub country: String,
    pub language: String,
    pub target_density: f32,
    pub max_keywords_per_post: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckFrequency {
    Daily,
    Weekly,
    Monthly,
}

impl Default for KeywordSettings {
    fn default() -> Self {
        Self {
            tracking_enabled: false,
            check_frequency: CheckFrequency::Weekly,
            search_engines: vec![SearchEngine::Google],
            country: "US".to_string(),
            language: "en".to_string(),
            target_density: 2.0,
            max_keywords_per_post: 5,
        }
    }
}

/// Internal linking suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkingSuggestion {
    pub source_id: Uuid,
    pub source_title: String,
    pub target_id: Uuid,
    pub target_title: String,
    pub target_url: String,
    pub anchor_text: String,
    pub relevance_score: f32,
}

/// Orphaned content (no internal links pointing to it)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrphanedContent {
    pub content_id: Uuid,
    pub title: String,
    pub url: String,
    pub content_type: String,
    pub published_at: DateTime<Utc>,
    pub suggested_links: Vec<LinkingSuggestion>,
}
