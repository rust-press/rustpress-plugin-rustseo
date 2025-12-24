//! SEO Content Analysis Models
//!
//! Models for analyzing content for SEO optimization.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Complete SEO analysis for a content item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoAnalysis {
    pub id: Uuid,
    pub content_id: Uuid,
    pub overall_score: SeoScore,
    pub title_analysis: TitleAnalysis,
    pub meta_analysis: MetaAnalysis,
    pub content_analysis: ContentAnalysis,
    pub keyword_analysis: KeywordAnalysis,
    pub readability_analysis: ReadabilityAnalysis,
    pub link_analysis: LinkAnalysis,
    pub image_analysis: ImageAnalysis,
    pub technical_analysis: TechnicalAnalysis,
    pub suggestions: Vec<SeoSuggestion>,
    pub analyzed_at: DateTime<Utc>,
}

/// SEO score (0-100)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SeoScore {
    pub score: i32,
    pub grade: SeoGrade,
}

impl SeoScore {
    pub fn new(score: i32) -> Self {
        let score = score.clamp(0, 100);
        let grade = match score {
            90..=100 => SeoGrade::Excellent,
            70..=89 => SeoGrade::Good,
            50..=69 => SeoGrade::Fair,
            30..=49 => SeoGrade::Poor,
            _ => SeoGrade::Bad,
        };
        Self { score, grade }
    }
}

/// SEO grade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeoGrade {
    Excellent,
    Good,
    Fair,
    Poor,
    Bad,
}

impl SeoGrade {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Excellent => "Excellent",
            Self::Good => "Good",
            Self::Fair => "Fair",
            Self::Poor => "Poor",
            Self::Bad => "Bad",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Self::Excellent => "#00a32a",
            Self::Good => "#7ad03a",
            Self::Fair => "#ffb900",
            Self::Poor => "#dc3232",
            Self::Bad => "#8b0000",
        }
    }
}

/// Title tag analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleAnalysis {
    pub score: i32,
    pub title: String,
    pub length: usize,
    pub has_focus_keyword: bool,
    pub keyword_position: Option<usize>,
    pub issues: Vec<AnalysisIssue>,
}

impl TitleAnalysis {
    pub fn analyze(title: &str, focus_keyword: Option<&str>) -> Self {
        let length = title.len();
        let mut issues = Vec::new();
        let mut score = 100;

        // Check length
        if length < 30 {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Warning,
                "Title is too short",
                "The title should be at least 30 characters for better SEO.",
            ));
            score -= 15;
        } else if length > 60 {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Warning,
                "Title is too long",
                "The title exceeds 60 characters and may be truncated in search results.",
            ));
            score -= 10;
        }

        let (has_keyword, keyword_pos) = if let Some(kw) = focus_keyword {
            let lower_title = title.to_lowercase();
            let lower_kw = kw.to_lowercase();
            if let Some(pos) = lower_title.find(&lower_kw) {
                (true, Some(pos))
            } else {
                issues.push(AnalysisIssue::new(
                    IssueSeverity::Error,
                    "Focus keyword not in title",
                    "The focus keyword should appear in the title for better rankings.",
                ));
                score -= 25;
                (false, None)
            }
        } else {
            (false, None)
        };

        // Check if keyword is at the beginning
        if has_keyword {
            if let Some(pos) = keyword_pos {
                if pos > 20 {
                    issues.push(AnalysisIssue::new(
                        IssueSeverity::Info,
                        "Keyword not at start of title",
                        "Moving the keyword closer to the beginning may improve rankings.",
                    ));
                    score -= 5;
                }
            }
        }

        Self {
            score: score.max(0),
            title: title.to_string(),
            length,
            has_focus_keyword: has_keyword,
            keyword_position: keyword_pos,
            issues,
        }
    }
}

/// Meta description analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaAnalysis {
    pub score: i32,
    pub description: Option<String>,
    pub length: usize,
    pub has_focus_keyword: bool,
    pub issues: Vec<AnalysisIssue>,
}

impl MetaAnalysis {
    pub fn analyze(description: Option<&str>, focus_keyword: Option<&str>) -> Self {
        let mut issues = Vec::new();
        let mut score = 100;

        let (desc, length) = match description {
            Some(d) if !d.is_empty() => (Some(d.to_string()), d.len()),
            _ => {
                issues.push(AnalysisIssue::new(
                    IssueSeverity::Error,
                    "No meta description",
                    "Add a meta description to control how your page appears in search results.",
                ));
                return Self {
                    score: 0,
                    description: None,
                    length: 0,
                    has_focus_keyword: false,
                    issues,
                };
            }
        };

        // Check length
        if length < 120 {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Warning,
                "Meta description is too short",
                "The description should be at least 120 characters.",
            ));
            score -= 15;
        } else if length > 160 {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Warning,
                "Meta description is too long",
                "The description exceeds 160 characters and may be truncated.",
            ));
            score -= 10;
        }

        let has_keyword = if let (Some(d), Some(kw)) = (&desc, focus_keyword) {
            if d.to_lowercase().contains(&kw.to_lowercase()) {
                true
            } else {
                issues.push(AnalysisIssue::new(
                    IssueSeverity::Warning,
                    "Focus keyword not in meta description",
                    "Include your focus keyword in the meta description.",
                ));
                score -= 15;
                false
            }
        } else {
            false
        };

        Self {
            score: score.max(0),
            description: desc,
            length,
            has_focus_keyword: has_keyword,
            issues,
        }
    }
}

/// Content analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAnalysis {
    pub score: i32,
    pub word_count: usize,
    pub paragraph_count: usize,
    pub sentence_count: usize,
    pub heading_count: HeadingCount,
    pub has_h1: bool,
    pub issues: Vec<AnalysisIssue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HeadingCount {
    pub h1: usize,
    pub h2: usize,
    pub h3: usize,
    pub h4: usize,
    pub h5: usize,
    pub h6: usize,
}

impl ContentAnalysis {
    pub fn analyze(content: &str, min_word_count: usize) -> Self {
        let mut issues = Vec::new();
        let mut score = 100;

        // Count words
        let word_count = content.split_whitespace().count();
        if word_count < min_word_count {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Warning,
                "Content is too short",
                &format!("Add more content. Aim for at least {} words.", min_word_count),
            ));
            score -= 20;
        }

        // Count paragraphs (rough estimate)
        let paragraph_count = content.split("\n\n").filter(|p| !p.trim().is_empty()).count();

        // Count sentences (rough estimate)
        let sentence_count = content.matches(|c| c == '.' || c == '!' || c == '?').count();

        // Count headings
        let mut heading_count = HeadingCount::default();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                heading_count.h1 += 1;
            } else if trimmed.starts_with("## ") {
                heading_count.h2 += 1;
            } else if trimmed.starts_with("### ") {
                heading_count.h3 += 1;
            } else if trimmed.starts_with("#### ") {
                heading_count.h4 += 1;
            }
        }

        let has_h1 = heading_count.h1 > 0;
        if heading_count.h1 == 0 {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Error,
                "No H1 heading found",
                "Add an H1 heading that includes your focus keyword.",
            ));
            score -= 20;
        } else if heading_count.h1 > 1 {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Warning,
                "Multiple H1 headings",
                "Use only one H1 heading per page.",
            ));
            score -= 10;
        }

        if heading_count.h2 == 0 && word_count > 300 {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Info,
                "No subheadings used",
                "Break up your content with H2 subheadings for better readability.",
            ));
            score -= 5;
        }

        Self {
            score: score.max(0),
            word_count,
            paragraph_count,
            sentence_count,
            heading_count,
            has_h1,
            issues,
        }
    }
}

/// Keyword analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordAnalysis {
    pub score: i32,
    pub focus_keyword: Option<String>,
    pub keyword_count: usize,
    pub keyword_density: f32,
    pub in_first_paragraph: bool,
    pub in_headings: bool,
    pub in_url: bool,
    pub issues: Vec<AnalysisIssue>,
}

/// Readability analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadabilityAnalysis {
    pub score: i32,
    pub flesch_reading_ease: f32,
    pub flesch_kincaid_grade: f32,
    pub avg_sentence_length: f32,
    pub avg_word_length: f32,
    pub passive_voice_percentage: f32,
    pub transition_word_percentage: f32,
    pub issues: Vec<AnalysisIssue>,
}

/// Link analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkAnalysis {
    pub score: i32,
    pub internal_links: usize,
    pub external_links: usize,
    pub broken_links: Vec<String>,
    pub nofollow_links: usize,
    pub issues: Vec<AnalysisIssue>,
}

/// Image analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAnalysis {
    pub score: i32,
    pub total_images: usize,
    pub images_with_alt: usize,
    pub images_with_keyword: usize,
    pub large_images: Vec<String>,
    pub issues: Vec<AnalysisIssue>,
}

/// Technical SEO analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalAnalysis {
    pub score: i32,
    pub has_canonical: bool,
    pub has_robots_meta: bool,
    pub has_open_graph: bool,
    pub has_twitter_card: bool,
    pub has_schema: bool,
    pub page_load_time: Option<f32>,
    pub mobile_friendly: bool,
    pub issues: Vec<AnalysisIssue>,
}

/// Analysis issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisIssue {
    pub severity: IssueSeverity,
    pub title: String,
    pub description: String,
}

impl AnalysisIssue {
    pub fn new(severity: IssueSeverity, title: &str, description: &str) -> Self {
        Self {
            severity,
            title: title.to_string(),
            description: description.to_string(),
        }
    }
}

/// Issue severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueSeverity {
    Error,
    Warning,
    Info,
    Success,
}

impl IssueSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Success => "success",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Self::Error => "#dc3232",
            Self::Warning => "#ffb900",
            Self::Info => "#0073aa",
            Self::Success => "#00a32a",
        }
    }
}

/// SEO suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoSuggestion {
    pub category: String,
    pub priority: SuggestionPriority,
    pub title: String,
    pub description: String,
    pub action: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SuggestionPriority {
    High,
    Medium,
    Low,
}

/// Analysis settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSettings {
    pub enabled: bool,
    pub min_word_count: usize,
    pub target_keyword_density: f32,
    pub max_keyword_density: f32,
    pub check_readability: bool,
    pub check_links: bool,
    pub check_images: bool,
}

impl Default for AnalysisSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            min_word_count: 300,
            target_keyword_density: 2.0,
            max_keyword_density: 3.0,
            check_readability: true,
            check_links: true,
            check_images: true,
        }
    }
}
