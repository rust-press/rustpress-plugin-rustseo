//! Robots.txt Handlers
//!
//! API handlers for robots.txt management.

use serde::{Deserialize, Serialize};
use crate::models::robots::RobotsTxtSettings;

/// Get robots.txt content
pub async fn get_robots_txt() -> Result<String, String> {
    Ok(String::new())
}

/// Update robots.txt content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRobotsTxtRequest {
    pub content: String,
}

pub async fn update_robots_txt(_request: UpdateRobotsTxtRequest) -> Result<String, String> {
    Ok(String::new())
}

/// Get robots.txt settings
pub async fn get_robots_settings() -> Result<RobotsTxtSettings, String> {
    Ok(RobotsTxtSettings::default())
}

/// Update robots.txt settings
pub async fn update_robots_settings(_settings: RobotsTxtSettings) -> Result<RobotsTxtSettings, String> {
    Ok(RobotsTxtSettings::default())
}

/// Validate robots.txt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateRobotsRequest {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotsValidationResult {
    pub valid: bool,
    pub errors: Vec<RobotsError>,
    pub warnings: Vec<RobotsWarning>,
    pub rules_count: i32,
    pub sitemaps_found: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotsError {
    pub line: i32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotsWarning {
    pub line: i32,
    pub message: String,
}

pub async fn validate_robots(request: ValidateRobotsRequest) -> Result<RobotsValidationResult, String> {
    let lines: Vec<&str> = request.content.lines().collect();
    let rules_count = lines.iter()
        .filter(|l| l.to_lowercase().starts_with("user-agent:"))
        .count() as i32;

    let sitemaps_found: Vec<String> = lines.iter()
        .filter(|l| l.to_lowercase().starts_with("sitemap:"))
        .map(|l| l.split(':').skip(1).collect::<Vec<_>>().join(":").trim().to_string())
        .collect();

    Ok(RobotsValidationResult {
        valid: true,
        errors: vec![],
        warnings: vec![],
        rules_count,
        sitemaps_found,
    })
}

/// Test URL against robots.txt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRobotsRequest {
    pub url: String,
    pub user_agent: String,
    pub robots_content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotsTestResult {
    pub allowed: bool,
    pub matched_rule: Option<String>,
    pub user_agent_matched: String,
}

pub async fn test_robots_url(_request: TestRobotsRequest) -> Result<RobotsTestResult, String> {
    Ok(RobotsTestResult {
        allowed: true,
        matched_rule: None,
        user_agent_matched: "*".to_string(),
    })
}

/// Generate robots.txt from settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRobotsRequest {
    pub settings: RobotsTxtSettings,
    pub site_url: String,
}

pub async fn generate_robots(request: GenerateRobotsRequest) -> Result<String, String> {
    let mut content = String::new();

    // Default rule for all bots
    content.push_str("User-agent: *\n");
    content.push_str("Allow: /\n");
    content.push_str("Disallow: /admin/\n");
    content.push_str("Disallow: /wp-admin/\n");
    content.push_str("Disallow: /login\n");
    content.push_str("Disallow: /register\n");
    content.push_str("\n");

    // Block AI crawlers if enabled
    if request.settings.block_ai_crawlers {
        let ai_crawlers = vec![
            "GPTBot", "ChatGPT-User", "CCBot", "Google-Extended",
            "anthropic-ai", "Claude-Web", "Bytespider", "Omgilibot",
        ];

        for crawler in ai_crawlers {
            content.push_str(&format!("User-agent: {}\n", crawler));
            content.push_str("Disallow: /\n\n");
        }
    }

    // Add sitemap reference
    if request.settings.include_sitemap {
        content.push_str(&format!("Sitemap: {}/sitemap_index.xml\n", request.site_url));
    }

    // Add custom rules
    if !request.settings.custom_rules.is_empty() {
        content.push('\n');
        content.push_str(&request.settings.custom_rules);
    }

    Ok(content)
}

/// Reset robots.txt to default
pub async fn reset_robots_txt() -> Result<String, String> {
    Ok(r#"User-agent: *
Allow: /
Disallow: /admin/
Disallow: /wp-admin/
Disallow: /login
Disallow: /register

Sitemap: /sitemap_index.xml
"#.to_string())
}

/// Get common user agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAgentInfo {
    pub name: String,
    pub user_agent: String,
    pub description: String,
    pub category: String,
}

pub fn get_common_user_agents() -> Vec<UserAgentInfo> {
    vec![
        UserAgentInfo {
            name: "All Bots".to_string(),
            user_agent: "*".to_string(),
            description: "Default rule for all crawlers".to_string(),
            category: "general".to_string(),
        },
        UserAgentInfo {
            name: "Googlebot".to_string(),
            user_agent: "Googlebot".to_string(),
            description: "Google's main web crawler".to_string(),
            category: "search".to_string(),
        },
        UserAgentInfo {
            name: "Googlebot Images".to_string(),
            user_agent: "Googlebot-Image".to_string(),
            description: "Google's image crawler".to_string(),
            category: "search".to_string(),
        },
        UserAgentInfo {
            name: "Bingbot".to_string(),
            user_agent: "Bingbot".to_string(),
            description: "Microsoft Bing's crawler".to_string(),
            category: "search".to_string(),
        },
        UserAgentInfo {
            name: "Yandex".to_string(),
            user_agent: "Yandex".to_string(),
            description: "Yandex search crawler".to_string(),
            category: "search".to_string(),
        },
        UserAgentInfo {
            name: "GPTBot".to_string(),
            user_agent: "GPTBot".to_string(),
            description: "OpenAI's GPT crawler".to_string(),
            category: "ai".to_string(),
        },
        UserAgentInfo {
            name: "Claude".to_string(),
            user_agent: "Claude-Web".to_string(),
            description: "Anthropic's Claude crawler".to_string(),
            category: "ai".to_string(),
        },
        UserAgentInfo {
            name: "CCBot".to_string(),
            user_agent: "CCBot".to_string(),
            description: "Common Crawl bot".to_string(),
            category: "ai".to_string(),
        },
        UserAgentInfo {
            name: "Google Extended".to_string(),
            user_agent: "Google-Extended".to_string(),
            description: "Google's AI training crawler".to_string(),
            category: "ai".to_string(),
        },
    ]
}

/// Get AI crawlers list
pub fn get_ai_crawlers() -> Vec<UserAgentInfo> {
    get_common_user_agents()
        .into_iter()
        .filter(|ua| ua.category == "ai")
        .collect()
}

/// Preview robots.txt changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewRobotsRequest {
    pub current: String,
    pub proposed: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotsPreviewResult {
    pub added_rules: Vec<String>,
    pub removed_rules: Vec<String>,
    pub modified_rules: Vec<String>,
}

pub async fn preview_robots_changes(request: PreviewRobotsRequest) -> Result<RobotsPreviewResult, String> {
    let current_lines: Vec<&str> = request.current.lines().collect();
    let proposed_lines: Vec<&str> = request.proposed.lines().collect();

    let added: Vec<String> = proposed_lines.iter()
        .filter(|l| !current_lines.contains(l) && !l.trim().is_empty())
        .map(|s| s.to_string())
        .collect();

    let removed: Vec<String> = current_lines.iter()
        .filter(|l| !proposed_lines.contains(l) && !l.trim().is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok(RobotsPreviewResult {
        added_rules: added,
        removed_rules: removed,
        modified_rules: vec![],
    })
}
