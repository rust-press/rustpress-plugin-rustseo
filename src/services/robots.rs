//! Robots.txt Service
//!
//! Service for managing robots.txt file.

use crate::models::robots::{RobotsTxt, RobotsRule, RobotsTxtSettings, ai_crawlers};

/// Service for managing robots.txt
pub struct RobotsService {
    site_url: String,
    settings: RobotsTxtSettings,
}

impl RobotsService {
    pub fn new(site_url: String) -> Self {
        Self {
            site_url: site_url.trim_end_matches('/').to_string(),
            settings: RobotsTxtSettings::default(),
        }
    }

    pub fn with_settings(mut self, settings: RobotsTxtSettings) -> Self {
        self.settings = settings;
        self
    }

    /// Generate robots.txt content
    pub fn generate(&self) -> String {
        if !self.settings.enabled {
            return String::new();
        }

        let mut robots = RobotsTxt::default_rules(&self.site_url);

        // Block AI crawlers if enabled
        if self.settings.block_ai_crawlers {
            for crawler in ai_crawlers() {
                robots.rules.push(RobotsRule {
                    user_agent: crawler.to_string(),
                    allow: vec![],
                    disallow: vec!["/".to_string()],
                    crawl_delay: None,
                });
            }
        }

        // Add sitemap reference
        if self.settings.include_sitemap {
            robots.add_sitemap(format!("{}/sitemap_index.xml", self.site_url));
        }

        // Add custom rules
        if !self.settings.custom_rules.is_empty() {
            robots.custom_content = Some(self.settings.custom_rules.clone());
        }

        robots.to_string()
    }

    /// Generate robots.txt from custom configuration
    pub fn generate_custom(&self, config: RobotsTxt) -> String {
        config.to_string()
    }

    /// Parse existing robots.txt
    pub fn parse(&self, content: &str) -> RobotsTxt {
        RobotsTxt::parse(content)
    }

    /// Validate robots.txt
    pub fn validate(&self, content: &str) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        let robots = RobotsTxt::parse(content);

        // Check for empty rules
        if robots.rules.is_empty() {
            warnings.push("No user-agent rules defined".to_string());
        }

        // Check for conflicting rules
        for rule in &robots.rules {
            if rule.user_agent.is_empty() {
                errors.push("Empty user-agent found".to_string());
            }

            // Check for both allow and disallow of same path
            for allow in &rule.allow {
                if rule.disallow.contains(allow) {
                    warnings.push(format!(
                        "Conflicting rules for path '{}' in {}",
                        allow, rule.user_agent
                    ));
                }
            }
        }

        // Check sitemap URLs
        for sitemap in &robots.sitemaps {
            if !sitemap.starts_with("http://") && !sitemap.starts_with("https://") {
                errors.push(format!("Invalid sitemap URL: {}", sitemap));
            }
        }

        ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    /// Check if a path is allowed for a user agent
    pub fn is_allowed(&self, content: &str, path: &str, user_agent: &str) -> bool {
        let robots = RobotsTxt::parse(content);

        // Find matching rule
        let rule = robots.rules.iter()
            .find(|r| r.user_agent == user_agent || r.user_agent == "*")
            .or_else(|| robots.rules.iter().find(|r| r.user_agent == "*"));

        if let Some(rule) = rule {
            // Check allow rules first (more specific)
            for allow in &rule.allow {
                if path.starts_with(allow) {
                    return true;
                }
            }

            // Check disallow rules
            for disallow in &rule.disallow {
                if disallow.is_empty() {
                    continue; // Empty disallow means allow all
                }
                if path.starts_with(disallow) {
                    return false;
                }
            }
        }

        // Default: allowed
        true
    }

    /// Get sitemap URL from robots.txt
    pub fn get_sitemap_url(&self, content: &str) -> Option<String> {
        let robots = RobotsTxt::parse(content);
        robots.sitemaps.first().cloned()
    }

    /// Generate meta robots tag
    pub fn generate_meta_tag(&self, index: bool, follow: bool) -> String {
        let directives = match (index, follow) {
            (true, true) => "index, follow",
            (true, false) => "index, nofollow",
            (false, true) => "noindex, follow",
            (false, false) => "noindex, nofollow",
        };

        format!("<meta name=\"robots\" content=\"{}\">", directives)
    }
}

/// Validation result
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_robots() {
        let service = RobotsService::new("https://example.com".to_string());
        let content = service.generate();

        assert!(content.contains("User-agent: *"));
        assert!(content.contains("Sitemap:"));
    }

    #[test]
    fn test_is_allowed() {
        let service = RobotsService::new("https://example.com".to_string());
        let content = "User-agent: *\nDisallow: /admin/\nAllow: /";

        assert!(service.is_allowed(&content, "/page", "Googlebot"));
        assert!(!service.is_allowed(&content, "/admin/settings", "Googlebot"));
    }
}
