//! SEO Analysis Admin
//!
//! Admin interface for SEO content analysis and reports.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// SEO Analysis overview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisOverview {
    pub overall_score: f32,
    pub overall_grade: String,
    pub total_content: i64,
    pub analyzed_content: i64,
    pub score_distribution: ScoreDistribution,
    pub issue_summary: IssueSummary,
    pub recent_analyses: Vec<RecentAnalysis>,
    pub top_issues: Vec<TopIssue>,
}

/// Score distribution across content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreDistribution {
    pub excellent: i64,    // 90-100
    pub good: i64,         // 70-89
    pub needs_work: i64,   // 50-69
    pub poor: i64,         // 0-49
}

/// Summary of issues across all content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueSummary {
    pub critical: i64,
    pub warnings: i64,
    pub suggestions: i64,
    pub passed: i64,
}

/// Recent analysis entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentAnalysis {
    pub id: Uuid,
    pub content_type: String,
    pub title: String,
    pub url: String,
    pub score: i32,
    pub grade: String,
    pub issues_count: i32,
    pub analyzed_at: DateTime<Utc>,
}

/// Top issue across content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopIssue {
    pub issue_type: String,
    pub severity: IssueSeverity,
    pub affected_count: i64,
    pub description: String,
    pub how_to_fix: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueSeverity {
    Critical,
    Warning,
    Suggestion,
    Info,
}

impl IssueSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            IssueSeverity::Critical => "critical",
            IssueSeverity::Warning => "warning",
            IssueSeverity::Suggestion => "suggestion",
            IssueSeverity::Info => "info",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            IssueSeverity::Critical => "#dc3545",
            IssueSeverity::Warning => "#ffc107",
            IssueSeverity::Suggestion => "#17a2b8",
            IssueSeverity::Info => "#6c757d",
        }
    }
}

/// Detailed content analysis result for admin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAnalysisResult {
    pub id: Uuid,
    pub content_id: String,
    pub content_type: String,
    pub title: String,
    pub url: String,
    pub overall_score: i32,
    pub overall_grade: String,
    pub focus_keyword: Option<String>,
    pub meta_analysis: MetaAnalysisResult,
    pub content_analysis: ContentAnalysisDetail,
    pub keyword_analysis: Option<KeywordAnalysisResult>,
    pub readability_analysis: ReadabilityResult,
    pub link_analysis: LinkAnalysisResult,
    pub image_analysis: ImageAnalysisResult,
    pub schema_analysis: SchemaAnalysisResult,
    pub social_analysis: SocialAnalysisResult,
    pub issues: Vec<AnalysisIssue>,
    pub suggestions: Vec<AnalysisSuggestion>,
    pub analyzed_at: DateTime<Utc>,
}

/// Meta tags analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaAnalysisResult {
    pub score: i32,
    pub title: TitleAnalysisResult,
    pub description: DescriptionAnalysisResult,
    pub canonical: CanonicalAnalysisResult,
    pub robots: RobotsAnalysisResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleAnalysisResult {
    pub exists: bool,
    pub content: Option<String>,
    pub length: i32,
    pub optimal_length: bool,
    pub has_keyword: bool,
    pub keyword_position: Option<String>,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescriptionAnalysisResult {
    pub exists: bool,
    pub content: Option<String>,
    pub length: i32,
    pub optimal_length: bool,
    pub has_keyword: bool,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalAnalysisResult {
    pub exists: bool,
    pub url: Option<String>,
    pub is_self_referencing: bool,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotsAnalysisResult {
    pub is_indexable: bool,
    pub directives: Vec<String>,
    pub issues: Vec<String>,
}

/// Content analysis detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAnalysisDetail {
    pub score: i32,
    pub word_count: i32,
    pub paragraph_count: i32,
    pub sentence_count: i32,
    pub heading_structure: HeadingStructure,
    pub content_quality: ContentQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingStructure {
    pub h1_count: i32,
    pub h2_count: i32,
    pub h3_count: i32,
    pub h4_count: i32,
    pub h5_count: i32,
    pub h6_count: i32,
    pub has_single_h1: bool,
    pub proper_hierarchy: bool,
    pub headings: Vec<HeadingEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingEntry {
    pub level: i32,
    pub text: String,
    pub has_keyword: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentQuality {
    pub has_enough_content: bool,
    pub min_recommended: i32,
    pub uses_subheadings: bool,
    pub has_lists: bool,
    pub has_media: bool,
}

/// Keyword analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordAnalysisResult {
    pub score: i32,
    pub focus_keyword: String,
    pub density: f32,
    pub optimal_density: bool,
    pub occurrences: i32,
    pub in_title: bool,
    pub in_meta_description: bool,
    pub in_first_paragraph: bool,
    pub in_headings: bool,
    pub in_url: bool,
    pub in_image_alt: bool,
    pub related_keywords: Vec<RelatedKeyword>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedKeyword {
    pub keyword: String,
    pub occurrences: i32,
    pub density: f32,
}

/// Readability analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadabilityResult {
    pub score: i32,
    pub grade_level: String,
    pub flesch_reading_ease: f32,
    pub flesch_kincaid_grade: f32,
    pub avg_sentence_length: f32,
    pub avg_word_length: f32,
    pub passive_voice_percentage: f32,
    pub transition_word_percentage: f32,
    pub issues: Vec<ReadabilityIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadabilityIssue {
    pub issue_type: String,
    pub description: String,
    pub sentence: Option<String>,
    pub suggestion: String,
}

/// Link analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkAnalysisResult {
    pub score: i32,
    pub internal_links: i32,
    pub external_links: i32,
    pub broken_links: i32,
    pub nofollow_links: i32,
    pub has_internal_links: bool,
    pub has_external_links: bool,
    pub links: Vec<LinkEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkEntry {
    pub url: String,
    pub text: String,
    pub is_internal: bool,
    pub is_nofollow: bool,
    pub is_broken: bool,
    pub status_code: Option<u16>,
}

/// Image analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAnalysisResult {
    pub score: i32,
    pub total_images: i32,
    pub images_with_alt: i32,
    pub images_without_alt: i32,
    pub images_with_keyword: i32,
    pub large_images: i32,
    pub images: Vec<ImageEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageEntry {
    pub src: String,
    pub alt: Option<String>,
    pub has_alt: bool,
    pub has_keyword: bool,
    pub file_size: Option<i64>,
    pub dimensions: Option<ImageDimensions>,
    pub is_optimized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageDimensions {
    pub width: i32,
    pub height: i32,
}

/// Schema analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaAnalysisResult {
    pub score: i32,
    pub has_schema: bool,
    pub schema_types: Vec<String>,
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Social analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialAnalysisResult {
    pub score: i32,
    pub opengraph: OpenGraphResult,
    pub twitter_cards: TwitterCardsResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenGraphResult {
    pub has_tags: bool,
    pub has_title: bool,
    pub has_description: bool,
    pub has_image: bool,
    pub image_dimensions_ok: bool,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterCardsResult {
    pub has_tags: bool,
    pub card_type: Option<String>,
    pub has_title: bool,
    pub has_description: bool,
    pub has_image: bool,
    pub issues: Vec<String>,
}

/// Analysis issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisIssue {
    pub id: String,
    pub category: String,
    pub severity: IssueSeverity,
    pub title: String,
    pub description: String,
    pub how_to_fix: String,
    pub learn_more_url: Option<String>,
}

/// Analysis suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSuggestion {
    pub id: String,
    pub category: String,
    pub title: String,
    pub description: String,
    pub impact: String,
    pub effort: String,
}

/// Bulk analysis request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkAnalysisRequest {
    pub content_type: Option<String>,
    pub content_ids: Option<Vec<String>>,
    pub analyze_all: bool,
    pub reanalyze_existing: bool,
}

/// Bulk analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkAnalysisResult {
    pub success: bool,
    pub analyzed: i32,
    pub failed: i32,
    pub skipped: i32,
    pub errors: Vec<String>,
    pub duration_ms: i64,
}

/// Analysis settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSettings {
    pub auto_analyze_on_save: bool,
    pub min_content_length: i32,
    pub target_keyword_density_min: f32,
    pub target_keyword_density_max: f32,
    pub check_broken_links: bool,
    pub check_image_sizes: bool,
    pub max_image_size_kb: i32,
    pub readability_target_grade: i32,
}

impl Default for AnalysisSettings {
    fn default() -> Self {
        Self {
            auto_analyze_on_save: true,
            min_content_length: 300,
            target_keyword_density_min: 0.5,
            target_keyword_density_max: 2.5,
            check_broken_links: true,
            check_image_sizes: true,
            max_image_size_kb: 200,
            readability_target_grade: 8,
        }
    }
}

/// Analysis filter options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisFilters {
    pub content_type: Option<String>,
    pub score_min: Option<i32>,
    pub score_max: Option<i32>,
    pub has_issues: Option<bool>,
    pub issue_type: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
}

/// Content list for bulk editor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentListItem {
    pub id: String,
    pub content_type: String,
    pub title: String,
    pub url: String,
    pub seo_score: Option<i32>,
    pub focus_keyword: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub is_indexable: bool,
    pub issues_count: i32,
    pub last_analyzed: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
}

/// Bulk editor update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkEditorUpdate {
    pub content_id: String,
    pub focus_keyword: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub is_indexable: Option<bool>,
}

/// Analysis tabs for admin
pub fn get_analysis_tabs() -> Vec<AnalysisTab> {
    vec![
        AnalysisTab {
            id: "overview".to_string(),
            title: "Overview".to_string(),
            icon: "bar-chart".to_string(),
        },
        AnalysisTab {
            id: "content".to_string(),
            title: "Content Analysis".to_string(),
            icon: "file-text".to_string(),
        },
        AnalysisTab {
            id: "bulk-editor".to_string(),
            title: "Bulk Editor".to_string(),
            icon: "edit".to_string(),
        },
        AnalysisTab {
            id: "issues".to_string(),
            title: "Issues".to_string(),
            icon: "alert-circle".to_string(),
        },
        AnalysisTab {
            id: "settings".to_string(),
            title: "Settings".to_string(),
            icon: "settings".to_string(),
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisTab {
    pub id: String,
    pub title: String,
    pub icon: String,
}

/// SEO score color helper
pub fn get_score_color(score: i32) -> &'static str {
    match score {
        90..=100 => "#28a745",  // Green
        70..=89 => "#7cb342",   // Light green
        50..=69 => "#ffc107",   // Yellow
        30..=49 => "#ff9800",   // Orange
        _ => "#dc3545",         // Red
    }
}

/// SEO grade from score
pub fn get_grade_from_score(score: i32) -> &'static str {
    match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    }
}

/// Common SEO issues
pub fn get_common_issues() -> Vec<CommonIssue> {
    vec![
        CommonIssue {
            id: "missing_meta_description".to_string(),
            title: "Missing Meta Description".to_string(),
            severity: IssueSeverity::Critical,
            category: "meta".to_string(),
            description: "Meta descriptions help search engines understand page content and improve click-through rates.".to_string(),
            how_to_fix: "Add a compelling meta description between 120-160 characters that includes your focus keyword.".to_string(),
        },
        CommonIssue {
            id: "missing_focus_keyword".to_string(),
            title: "No Focus Keyword Set".to_string(),
            severity: IssueSeverity::Warning,
            category: "keyword".to_string(),
            description: "Setting a focus keyword helps optimize your content for search engines.".to_string(),
            how_to_fix: "Choose a focus keyword that your target audience would search for.".to_string(),
        },
        CommonIssue {
            id: "title_too_long".to_string(),
            title: "Title Too Long".to_string(),
            severity: IssueSeverity::Warning,
            category: "meta".to_string(),
            description: "Titles longer than 60 characters may be truncated in search results.".to_string(),
            how_to_fix: "Shorten your title to under 60 characters while keeping it descriptive.".to_string(),
        },
        CommonIssue {
            id: "missing_alt_text".to_string(),
            title: "Images Missing Alt Text".to_string(),
            severity: IssueSeverity::Warning,
            category: "images".to_string(),
            description: "Alt text helps search engines understand images and improves accessibility.".to_string(),
            how_to_fix: "Add descriptive alt text to all images, including the focus keyword where appropriate.".to_string(),
        },
        CommonIssue {
            id: "low_word_count".to_string(),
            title: "Content Too Short".to_string(),
            severity: IssueSeverity::Suggestion,
            category: "content".to_string(),
            description: "Longer, comprehensive content tends to rank better in search results.".to_string(),
            how_to_fix: "Expand your content to at least 300 words, covering the topic thoroughly.".to_string(),
        },
        CommonIssue {
            id: "no_internal_links".to_string(),
            title: "No Internal Links".to_string(),
            severity: IssueSeverity::Suggestion,
            category: "links".to_string(),
            description: "Internal links help search engines discover content and distribute page authority.".to_string(),
            how_to_fix: "Add 2-3 relevant internal links to other pages on your site.".to_string(),
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonIssue {
    pub id: String,
    pub title: String,
    pub severity: IssueSeverity,
    pub category: String,
    pub description: String,
    pub how_to_fix: String,
}
