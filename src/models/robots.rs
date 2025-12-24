//! Robots.txt Models
//!
//! Models for generating and managing robots.txt file.

use serde::{Deserialize, Serialize};

/// Robots.txt configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotsTxt {
    pub rules: Vec<RobotsRule>,
    pub sitemaps: Vec<String>,
    pub crawl_delay: Option<u32>,
    pub custom_content: Option<String>,
}

/// Robot rule for a specific user agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotsRule {
    pub user_agent: String,
    pub allow: Vec<String>,
    pub disallow: Vec<String>,
    pub crawl_delay: Option<u32>,
}

impl RobotsTxt {
    pub fn new() -> Self {
        Self {
            rules: vec![],
            sitemaps: vec![],
            crawl_delay: None,
            custom_content: None,
        }
    }

    /// Create default robots.txt with common rules
    pub fn default_rules(site_url: &str) -> Self {
        let mut robots = Self::new();

        // Default rule for all bots
        robots.rules.push(RobotsRule {
            user_agent: "*".to_string(),
            allow: vec!["/".to_string()],
            disallow: vec![
                "/wp-admin/".to_string(),
                "/admin/".to_string(),
                "/api/".to_string(),
                "/login".to_string(),
                "/register".to_string(),
                "/*?*".to_string(),
                "/search".to_string(),
                "/checkout".to_string(),
                "/cart".to_string(),
                "/my-account".to_string(),
            ],
            crawl_delay: None,
        });

        // Add sitemap
        robots.sitemaps.push(format!("{}/sitemap_index.xml", site_url.trim_end_matches('/')));

        robots
    }

    /// Add a rule
    pub fn add_rule(&mut self, rule: RobotsRule) {
        self.rules.push(rule);
    }

    /// Add sitemap URL
    pub fn add_sitemap(&mut self, url: String) {
        if !self.sitemaps.contains(&url) {
            self.sitemaps.push(url);
        }
    }

    /// Generate robots.txt content
    pub fn to_string(&self) -> String {
        let mut content = String::new();

        // Add rules
        for rule in &self.rules {
            content.push_str(&format!("User-agent: {}\n", rule.user_agent));

            for allow in &rule.allow {
                content.push_str(&format!("Allow: {}\n", allow));
            }

            for disallow in &rule.disallow {
                content.push_str(&format!("Disallow: {}\n", disallow));
            }

            if let Some(delay) = rule.crawl_delay.or(self.crawl_delay) {
                content.push_str(&format!("Crawl-delay: {}\n", delay));
            }

            content.push('\n');
        }

        // Add sitemaps
        for sitemap in &self.sitemaps {
            content.push_str(&format!("Sitemap: {}\n", sitemap));
        }

        // Add custom content
        if let Some(custom) = &self.custom_content {
            content.push('\n');
            content.push_str(custom);
            content.push('\n');
        }

        content
    }

    /// Parse robots.txt content
    pub fn parse(content: &str) -> Self {
        let mut robots = Self::new();
        let mut current_rule: Option<RobotsRule> = None;

        for line in content.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse directive
            if let Some((directive, value)) = line.split_once(':') {
                let directive = directive.trim().to_lowercase();
                let value = value.trim().to_string();

                match directive.as_str() {
                    "user-agent" => {
                        // Save previous rule if exists
                        if let Some(rule) = current_rule.take() {
                            robots.rules.push(rule);
                        }
                        current_rule = Some(RobotsRule {
                            user_agent: value,
                            allow: vec![],
                            disallow: vec![],
                            crawl_delay: None,
                        });
                    }
                    "allow" => {
                        if let Some(ref mut rule) = current_rule {
                            rule.allow.push(value);
                        }
                    }
                    "disallow" => {
                        if let Some(ref mut rule) = current_rule {
                            rule.disallow.push(value);
                        }
                    }
                    "crawl-delay" => {
                        if let Ok(delay) = value.parse() {
                            if let Some(ref mut rule) = current_rule {
                                rule.crawl_delay = Some(delay);
                            } else {
                                robots.crawl_delay = Some(delay);
                            }
                        }
                    }
                    "sitemap" => {
                        robots.sitemaps.push(value);
                    }
                    _ => {}
                }
            }
        }

        // Save last rule
        if let Some(rule) = current_rule {
            robots.rules.push(rule);
        }

        robots
    }
}

impl Default for RobotsTxt {
    fn default() -> Self {
        Self::new()
    }
}

impl RobotsRule {
    pub fn new(user_agent: &str) -> Self {
        Self {
            user_agent: user_agent.to_string(),
            allow: vec![],
            disallow: vec![],
            crawl_delay: None,
        }
    }

    pub fn allow(mut self, path: &str) -> Self {
        self.allow.push(path.to_string());
        self
    }

    pub fn disallow(mut self, path: &str) -> Self {
        self.disallow.push(path.to_string());
        self
    }

    pub fn with_crawl_delay(mut self, delay: u32) -> Self {
        self.crawl_delay = Some(delay);
        self
    }
}

/// Robots meta tag settings per content type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotsContentSettings {
    pub posts: RobotsDirectives,
    pub pages: RobotsDirectives,
    pub categories: RobotsDirectives,
    pub tags: RobotsDirectives,
    pub authors: RobotsDirectives,
    pub archives: RobotsDirectives,
    pub search: RobotsDirectives,
    pub products: RobotsDirectives,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotsDirectives {
    pub index: bool,
    pub follow: bool,
    pub show_in_sitemap: bool,
}

impl Default for RobotsDirectives {
    fn default() -> Self {
        Self {
            index: true,
            follow: true,
            show_in_sitemap: true,
        }
    }
}

impl Default for RobotsContentSettings {
    fn default() -> Self {
        Self {
            posts: RobotsDirectives::default(),
            pages: RobotsDirectives::default(),
            categories: RobotsDirectives::default(),
            tags: RobotsDirectives {
                index: false,
                follow: true,
                show_in_sitemap: false,
            },
            authors: RobotsDirectives {
                index: false,
                follow: true,
                show_in_sitemap: false,
            },
            archives: RobotsDirectives {
                index: false,
                follow: true,
                show_in_sitemap: false,
            },
            search: RobotsDirectives {
                index: false,
                follow: false,
                show_in_sitemap: false,
            },
            products: RobotsDirectives::default(),
        }
    }
}

/// Robots.txt settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotsTxtSettings {
    pub enabled: bool,
    pub use_virtual: bool,
    pub include_sitemap: bool,
    pub block_ai_crawlers: bool,
    pub custom_rules: String,
}

impl Default for RobotsTxtSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            use_virtual: true,
            include_sitemap: true,
            block_ai_crawlers: false,
            custom_rules: String::new(),
        }
    }
}

/// Common bot user agents
pub fn common_bots() -> Vec<(&'static str, &'static str)> {
    vec![
        ("*", "All robots"),
        ("Googlebot", "Google"),
        ("Googlebot-Image", "Google Images"),
        ("Googlebot-News", "Google News"),
        ("Googlebot-Video", "Google Video"),
        ("Bingbot", "Bing"),
        ("Slurp", "Yahoo"),
        ("DuckDuckBot", "DuckDuckGo"),
        ("Baiduspider", "Baidu"),
        ("YandexBot", "Yandex"),
        ("facebookexternalhit", "Facebook"),
        ("Twitterbot", "Twitter"),
        ("LinkedInBot", "LinkedIn"),
        ("GPTBot", "OpenAI GPT"),
        ("ChatGPT-User", "ChatGPT"),
        ("Claude-Web", "Anthropic Claude"),
        ("CCBot", "Common Crawl"),
        ("Amazonbot", "Amazon"),
    ]
}

/// AI crawler user agents that can be blocked
pub fn ai_crawlers() -> Vec<&'static str> {
    vec![
        "GPTBot",
        "ChatGPT-User",
        "Claude-Web",
        "CCBot",
        "anthropic-ai",
        "Google-Extended",
        "Amazonbot",
        "Bytespider",
        "FacebookBot",
    ]
}
