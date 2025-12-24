//! SEO Analysis Service
//!
//! Service for analyzing content for SEO optimization.

use crate::models::analysis::*;
use chrono::Utc;
use uuid::Uuid;

/// Service for SEO content analysis
pub struct AnalysisService {
    settings: AnalysisSettings,
}

impl AnalysisService {
    pub fn new() -> Self {
        Self {
            settings: AnalysisSettings::default(),
        }
    }

    pub fn with_settings(mut self, settings: AnalysisSettings) -> Self {
        self.settings = settings;
        self
    }

    /// Perform complete SEO analysis
    pub fn analyze(&self, content_id: Uuid, data: AnalysisInput) -> SeoAnalysis {
        let title_analysis = TitleAnalysis::analyze(
            &data.title,
            data.focus_keyword.as_deref(),
        );

        let meta_analysis = MetaAnalysis::analyze(
            data.meta_description.as_deref(),
            data.focus_keyword.as_deref(),
        );

        let content_analysis = ContentAnalysis::analyze(
            &data.content,
            self.settings.min_word_count,
        );

        let keyword_analysis = self.analyze_keywords(&data);
        let readability_analysis = self.analyze_readability(&data.content);
        let link_analysis = self.analyze_links(&data);
        let image_analysis = self.analyze_images(&data);
        let technical_analysis = self.analyze_technical(&data);

        // Calculate overall score
        let scores = [
            title_analysis.score,
            meta_analysis.score,
            content_analysis.score,
            keyword_analysis.score,
            readability_analysis.score,
            link_analysis.score,
            image_analysis.score,
            technical_analysis.score,
        ];

        let overall_score = SeoScore::new(
            scores.iter().sum::<i32>() / scores.len() as i32
        );

        // Generate suggestions
        let suggestions = self.generate_suggestions(
            &title_analysis,
            &meta_analysis,
            &content_analysis,
            &keyword_analysis,
        );

        SeoAnalysis {
            id: Uuid::now_v7(),
            content_id,
            overall_score,
            title_analysis,
            meta_analysis,
            content_analysis,
            keyword_analysis,
            readability_analysis,
            link_analysis,
            image_analysis,
            technical_analysis,
            suggestions,
            analyzed_at: Utc::now(),
        }
    }

    /// Analyze keyword usage
    fn analyze_keywords(&self, data: &AnalysisInput) -> KeywordAnalysis {
        let mut issues = Vec::new();
        let mut score = 100;

        let (keyword, count, density, in_first, in_headings, in_url) =
            if let Some(kw) = &data.focus_keyword {
                let content_lower = data.content.to_lowercase();
                let kw_lower = kw.to_lowercase();

                let word_count = data.content.split_whitespace().count();
                let kw_count = content_lower.matches(&kw_lower).count();
                let kw_density = if word_count > 0 {
                    (kw_count as f32 / word_count as f32) * 100.0
                } else {
                    0.0
                };

                // Check if in first paragraph
                let first_para = data.content.split("\n\n").next().unwrap_or("");
                let in_first = first_para.to_lowercase().contains(&kw_lower);

                // Check if in headings
                let in_headings = data.headings.iter()
                    .any(|h| h.to_lowercase().contains(&kw_lower));

                // Check if in URL
                let in_url = data.url.to_lowercase().contains(&kw_lower);

                // Issues
                if kw_count == 0 {
                    issues.push(AnalysisIssue::new(
                        IssueSeverity::Error,
                        "Focus keyword not found",
                        "The focus keyword doesn't appear in your content.",
                    ));
                    score -= 30;
                } else if kw_density < self.settings.target_keyword_density * 0.5 {
                    issues.push(AnalysisIssue::new(
                        IssueSeverity::Warning,
                        "Keyword density too low",
                        "Consider using your focus keyword more often.",
                    ));
                    score -= 15;
                } else if kw_density > self.settings.max_keyword_density {
                    issues.push(AnalysisIssue::new(
                        IssueSeverity::Warning,
                        "Keyword density too high",
                        "You may be over-optimizing. Use the keyword more naturally.",
                    ));
                    score -= 10;
                }

                if !in_first {
                    issues.push(AnalysisIssue::new(
                        IssueSeverity::Warning,
                        "Keyword not in first paragraph",
                        "Include your focus keyword in the first paragraph.",
                    ));
                    score -= 10;
                }

                if !in_headings {
                    issues.push(AnalysisIssue::new(
                        IssueSeverity::Info,
                        "Keyword not in subheadings",
                        "Consider adding the keyword to at least one subheading.",
                    ));
                    score -= 5;
                }

                if !in_url {
                    issues.push(AnalysisIssue::new(
                        IssueSeverity::Info,
                        "Keyword not in URL",
                        "Including the keyword in the URL can help with SEO.",
                    ));
                    score -= 5;
                }

                (Some(kw.clone()), kw_count, kw_density, in_first, in_headings, in_url)
            } else {
                issues.push(AnalysisIssue::new(
                    IssueSeverity::Warning,
                    "No focus keyword set",
                    "Set a focus keyword to optimize your content.",
                ));
                score = 50;
                (None, 0, 0.0, false, false, false)
            };

        KeywordAnalysis {
            score: score.max(0),
            focus_keyword: keyword,
            keyword_count: count,
            keyword_density: density,
            in_first_paragraph: in_first,
            in_headings,
            in_url,
            issues,
        }
    }

    /// Analyze readability
    fn analyze_readability(&self, content: &str) -> ReadabilityAnalysis {
        let mut issues = Vec::new();
        let mut score = 100;

        let words: Vec<&str> = content.split_whitespace().collect();
        let word_count = words.len();

        // Count sentences
        let sentence_count = content.matches(|c| c == '.' || c == '!' || c == '?').count().max(1);

        // Average sentence length
        let avg_sentence = word_count as f32 / sentence_count as f32;

        if avg_sentence > 25.0 {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Warning,
                "Sentences are too long",
                "Try to keep sentences under 20-25 words for better readability.",
            ));
            score -= 15;
        }

        // Average word length
        let total_chars: usize = words.iter().map(|w| w.len()).sum();
        let avg_word = total_chars as f32 / word_count.max(1) as f32;

        // Simple Flesch Reading Ease approximation
        let flesch = 206.835 - (1.015 * avg_sentence) - (84.6 * (avg_word / 5.0));

        if flesch < 30.0 {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Warning,
                "Content is very difficult to read",
                "Simplify your language and use shorter sentences.",
            ));
            score -= 20;
        } else if flesch < 50.0 {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Info,
                "Content is fairly difficult to read",
                "Consider simplifying some sentences.",
            ));
            score -= 10;
        }

        // Flesch-Kincaid Grade Level
        let grade = 0.39 * avg_sentence + 11.8 * (avg_word / 5.0) - 15.59;

        // Passive voice detection (simple heuristic)
        let passive_patterns = ["was ", "were ", "been ", "being ", "is being", "are being"];
        let passive_count: usize = passive_patterns.iter()
            .map(|p| content.to_lowercase().matches(p).count())
            .sum();
        let passive_pct = (passive_count as f32 / sentence_count as f32) * 100.0;

        if passive_pct > 20.0 {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Info,
                "High use of passive voice",
                "Try using more active voice for engaging content.",
            ));
            score -= 5;
        }

        // Transition words (simple check)
        let transitions = ["however", "therefore", "moreover", "furthermore", "additionally",
            "consequently", "meanwhile", "nevertheless", "also", "first", "second", "finally"];
        let transition_count: usize = transitions.iter()
            .map(|t| content.to_lowercase().matches(t).count())
            .sum();
        let transition_pct = (transition_count as f32 / sentence_count as f32) * 100.0;

        if transition_pct < 20.0 && sentence_count > 3 {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Info,
                "Few transition words",
                "Use more transition words to improve flow.",
            ));
            score -= 5;
        }

        ReadabilityAnalysis {
            score: score.max(0),
            flesch_reading_ease: flesch.max(0.0),
            flesch_kincaid_grade: grade.max(0.0),
            avg_sentence_length: avg_sentence,
            avg_word_length: avg_word,
            passive_voice_percentage: passive_pct,
            transition_word_percentage: transition_pct,
            issues,
        }
    }

    /// Analyze links
    fn analyze_links(&self, data: &AnalysisInput) -> LinkAnalysis {
        let mut issues = Vec::new();
        let mut score = 100;

        if data.internal_links == 0 {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Warning,
                "No internal links",
                "Add internal links to help visitors discover more content.",
            ));
            score -= 20;
        } else if data.internal_links < 3 {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Info,
                "Few internal links",
                "Consider adding more internal links.",
            ));
            score -= 10;
        }

        if data.external_links == 0 {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Info,
                "No outbound links",
                "Linking to authoritative sources can improve credibility.",
            ));
            score -= 5;
        }

        if !data.broken_links.is_empty() {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Error,
                "Broken links detected",
                &format!("Fix {} broken links.", data.broken_links.len()),
            ));
            score -= 25;
        }

        LinkAnalysis {
            score: score.max(0),
            internal_links: data.internal_links,
            external_links: data.external_links,
            broken_links: data.broken_links.clone(),
            nofollow_links: data.nofollow_links,
            issues,
        }
    }

    /// Analyze images
    fn analyze_images(&self, data: &AnalysisInput) -> ImageAnalysis {
        let mut issues = Vec::new();
        let mut score = 100;

        if data.images.is_empty() {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Info,
                "No images in content",
                "Adding images can improve engagement and SEO.",
            ));
            score -= 10;
        } else {
            let images_without_alt = data.images.iter()
                .filter(|img| img.alt.is_none() || img.alt.as_ref().map(|a| a.is_empty()).unwrap_or(true))
                .count();

            if images_without_alt > 0 {
                issues.push(AnalysisIssue::new(
                    IssueSeverity::Warning,
                    "Images missing alt text",
                    &format!("{} images are missing alt text.", images_without_alt),
                ));
                score -= 15;
            }

            let images_with_keyword = if let Some(kw) = &data.focus_keyword {
                data.images.iter()
                    .filter(|img| {
                        img.alt.as_ref()
                            .map(|a| a.to_lowercase().contains(&kw.to_lowercase()))
                            .unwrap_or(false)
                    })
                    .count()
            } else {
                0
            };

            if images_with_keyword == 0 && data.focus_keyword.is_some() {
                issues.push(AnalysisIssue::new(
                    IssueSeverity::Info,
                    "No images contain focus keyword",
                    "Add the focus keyword to at least one image alt text.",
                ));
                score -= 5;
            }
        }

        ImageAnalysis {
            score: score.max(0),
            total_images: data.images.len(),
            images_with_alt: data.images.iter()
                .filter(|img| img.alt.is_some() && !img.alt.as_ref().unwrap().is_empty())
                .count(),
            images_with_keyword: 0, // Already calculated above
            large_images: data.large_images.clone(),
            issues,
        }
    }

    /// Analyze technical SEO
    fn analyze_technical(&self, data: &AnalysisInput) -> TechnicalAnalysis {
        let mut issues = Vec::new();
        let mut score = 100;

        if !data.has_canonical {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Warning,
                "No canonical URL",
                "Set a canonical URL to prevent duplicate content issues.",
            ));
            score -= 15;
        }

        if !data.has_open_graph {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Info,
                "Missing OpenGraph tags",
                "Add OpenGraph tags for better social sharing.",
            ));
            score -= 5;
        }

        if !data.has_twitter_card {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Info,
                "Missing Twitter Card tags",
                "Add Twitter Card tags for better Twitter sharing.",
            ));
            score -= 5;
        }

        if !data.has_schema {
            issues.push(AnalysisIssue::new(
                IssueSeverity::Info,
                "No schema markup",
                "Add schema.org structured data for rich snippets.",
            ));
            score -= 10;
        }

        TechnicalAnalysis {
            score: score.max(0),
            has_canonical: data.has_canonical,
            has_robots_meta: data.has_robots_meta,
            has_open_graph: data.has_open_graph,
            has_twitter_card: data.has_twitter_card,
            has_schema: data.has_schema,
            page_load_time: data.page_load_time,
            mobile_friendly: data.mobile_friendly,
            issues,
        }
    }

    /// Generate improvement suggestions
    fn generate_suggestions(
        &self,
        title: &TitleAnalysis,
        meta: &MetaAnalysis,
        content: &ContentAnalysis,
        keyword: &KeywordAnalysis,
    ) -> Vec<SeoSuggestion> {
        let mut suggestions = Vec::new();

        // High priority
        if title.score < 50 {
            suggestions.push(SeoSuggestion {
                category: "Title".to_string(),
                priority: SuggestionPriority::High,
                title: "Improve your title".to_string(),
                description: "Your title needs significant improvement for SEO.".to_string(),
                action: Some("Add focus keyword and optimize length.".to_string()),
            });
        }

        if meta.score < 50 {
            suggestions.push(SeoSuggestion {
                category: "Meta Description".to_string(),
                priority: SuggestionPriority::High,
                title: "Add meta description".to_string(),
                description: "A good meta description improves click-through rates.".to_string(),
                action: Some("Write a compelling 150-160 character description.".to_string()),
            });
        }

        // Medium priority
        if content.word_count < self.settings.min_word_count {
            suggestions.push(SeoSuggestion {
                category: "Content".to_string(),
                priority: SuggestionPriority::Medium,
                title: "Add more content".to_string(),
                description: format!(
                    "Your content has {} words. Aim for at least {}.",
                    content.word_count, self.settings.min_word_count
                ),
                action: None,
            });
        }

        if keyword.focus_keyword.is_none() {
            suggestions.push(SeoSuggestion {
                category: "Keywords".to_string(),
                priority: SuggestionPriority::Medium,
                title: "Set a focus keyword".to_string(),
                description: "A focus keyword helps optimize your content.".to_string(),
                action: None,
            });
        }

        suggestions
    }
}

impl Default for AnalysisService {
    fn default() -> Self {
        Self::new()
    }
}

/// Input data for analysis
pub struct AnalysisInput {
    pub title: String,
    pub meta_description: Option<String>,
    pub content: String,
    pub url: String,
    pub focus_keyword: Option<String>,
    pub headings: Vec<String>,
    pub internal_links: usize,
    pub external_links: usize,
    pub nofollow_links: usize,
    pub broken_links: Vec<String>,
    pub images: Vec<ImageInput>,
    pub large_images: Vec<String>,
    pub has_canonical: bool,
    pub has_robots_meta: bool,
    pub has_open_graph: bool,
    pub has_twitter_card: bool,
    pub has_schema: bool,
    pub page_load_time: Option<f32>,
    pub mobile_friendly: bool,
}

pub struct ImageInput {
    pub src: String,
    pub alt: Option<String>,
}
