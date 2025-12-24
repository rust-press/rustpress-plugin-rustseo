//! Redirect Service
//!
//! Service for managing URL redirects.

use crate::models::redirect::{Redirect, RedirectType, MatchType, NotFoundLog, RedirectSettings};
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;

/// Service for managing URL redirects
pub struct RedirectService {
    redirects: Vec<Redirect>,
    settings: RedirectSettings,
    not_found_log: HashMap<String, NotFoundLog>,
}

impl RedirectService {
    pub fn new() -> Self {
        Self {
            redirects: Vec::new(),
            settings: RedirectSettings::default(),
            not_found_log: HashMap::new(),
        }
    }

    pub fn with_settings(mut self, settings: RedirectSettings) -> Self {
        self.settings = settings;
        self
    }

    /// Add a redirect
    pub fn add_redirect(&mut self, redirect: Redirect) {
        self.redirects.push(redirect);
    }

    /// Create a simple 301 redirect
    pub fn add_301(&mut self, source: &str, target: &str) {
        let redirect = Redirect::new(
            source.to_string(),
            target.to_string(),
            RedirectType::Permanent,
        );
        self.redirects.push(redirect);
    }

    /// Create a 302 temporary redirect
    pub fn add_302(&mut self, source: &str, target: &str) {
        let redirect = Redirect::new(
            source.to_string(),
            target.to_string(),
            RedirectType::Temporary,
        );
        self.redirects.push(redirect);
    }

    /// Find redirect for a URL
    pub fn find_redirect(&self, url: &str) -> Option<&Redirect> {
        let url_to_check = if self.settings.case_insensitive {
            url.to_lowercase()
        } else {
            url.to_string()
        };

        for redirect in &self.redirects {
            if !redirect.is_active {
                continue;
            }

            let source = if self.settings.case_insensitive {
                redirect.source_url.to_lowercase()
            } else {
                redirect.source_url.clone()
            };

            let matches = match redirect.match_type {
                MatchType::Exact => url_to_check == source,
                MatchType::Prefix => url_to_check.starts_with(&source),
                MatchType::Contains => url_to_check.contains(&source),
                MatchType::Regex => {
                    if let Ok(re) = regex::Regex::new(&redirect.source_url) {
                        re.is_match(&url_to_check)
                    } else {
                        false
                    }
                }
            };

            if matches {
                return Some(redirect);
            }
        }

        None
    }

    /// Process a redirect and get target URL
    pub fn process_redirect(&mut self, url: &str) -> Option<RedirectResult> {
        if let Some(redirect) = self.find_redirect(url) {
            let target = redirect.get_target(url);
            let status_code = redirect.redirect_type.status_code();

            // Record hit (would need mutable access in real implementation)
            return Some(RedirectResult {
                target_url: target,
                status_code,
                redirect_id: redirect.id,
            });
        }

        None
    }

    /// Log a 404 error
    pub fn log_404(&mut self, url: &str, referrer: Option<&str>, user_agent: Option<&str>) {
        if !self.settings.log_404s {
            return;
        }

        if let Some(log) = self.not_found_log.get_mut(url) {
            log.record_hit();
        } else {
            let mut log = NotFoundLog::new(url.to_string());
            log.referrer = referrer.map(|s| s.to_string());
            log.user_agent = user_agent.map(|s| s.to_string());
            self.not_found_log.insert(url.to_string(), log);
        }
    }

    /// Get top 404 errors
    pub fn get_top_404s(&self, limit: usize) -> Vec<&NotFoundLog> {
        let mut logs: Vec<_> = self.not_found_log.values()
            .filter(|l| !l.is_ignored)
            .collect();

        logs.sort_by(|a, b| b.hit_count.cmp(&a.hit_count));
        logs.truncate(limit);
        logs
    }

    /// Create redirect from 404
    pub fn create_redirect_from_404(&mut self, url: &str, target: &str) -> Redirect {
        let redirect = Redirect::new(
            url.to_string(),
            target.to_string(),
            RedirectType::Permanent,
        );

        // Mark 404 as having redirect
        if let Some(log) = self.not_found_log.get_mut(url) {
            log.has_redirect = true;
        }

        self.redirects.push(redirect.clone());
        redirect
    }

    /// Ignore a 404
    pub fn ignore_404(&mut self, url: &str) {
        if let Some(log) = self.not_found_log.get_mut(url) {
            log.is_ignored = true;
        }
    }

    /// Remove a redirect
    pub fn remove_redirect(&mut self, id: Uuid) -> bool {
        if let Some(pos) = self.redirects.iter().position(|r| r.id == id) {
            self.redirects.remove(pos);
            true
        } else {
            false
        }
    }

    /// Update a redirect
    pub fn update_redirect(&mut self, id: Uuid, source: Option<String>, target: Option<String>, redirect_type: Option<RedirectType>) -> bool {
        if let Some(redirect) = self.redirects.iter_mut().find(|r| r.id == id) {
            if let Some(s) = source {
                redirect.source_url = s;
            }
            if let Some(t) = target {
                redirect.target_url = t;
            }
            if let Some(rt) = redirect_type {
                redirect.redirect_type = rt;
            }
            redirect.updated_at = Utc::now();
            true
        } else {
            false
        }
    }

    /// Enable/disable a redirect
    pub fn set_redirect_active(&mut self, id: Uuid, active: bool) -> bool {
        if let Some(redirect) = self.redirects.iter_mut().find(|r| r.id == id) {
            redirect.is_active = active;
            redirect.updated_at = Utc::now();
            true
        } else {
            false
        }
    }

    /// Get all redirects
    pub fn get_redirects(&self) -> &[Redirect] {
        &self.redirects
    }

    /// Get redirect by ID
    pub fn get_redirect(&self, id: Uuid) -> Option<&Redirect> {
        self.redirects.iter().find(|r| r.id == id)
    }

    /// Import redirects from CSV format
    pub fn import_csv(&mut self, csv: &str) -> ImportResult {
        let mut imported = 0;
        let mut skipped = 0;
        let mut errors = Vec::new();

        for (line_num, line) in csv.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 2 {
                errors.push(format!("Line {}: Invalid format", line_num + 1));
                skipped += 1;
                continue;
            }

            let source = parts[0].trim().trim_matches('"');
            let target = parts[1].trim().trim_matches('"');

            let redirect_type = if parts.len() > 2 {
                match parts[2].trim() {
                    "301" | "permanent" => RedirectType::Permanent,
                    "302" | "temporary" => RedirectType::Temporary,
                    "307" => RedirectType::TemporaryPreserve,
                    "308" => RedirectType::PermanentPreserve,
                    "410" | "gone" => RedirectType::Gone,
                    _ => RedirectType::Permanent,
                }
            } else {
                RedirectType::Permanent
            };

            // Check for duplicate
            if self.redirects.iter().any(|r| r.source_url == source) {
                errors.push(format!("Line {}: Duplicate source URL", line_num + 1));
                skipped += 1;
                continue;
            }

            let redirect = Redirect::new(
                source.to_string(),
                target.to_string(),
                redirect_type,
            );

            self.redirects.push(redirect);
            imported += 1;
        }

        ImportResult { imported, skipped, errors }
    }

    /// Export redirects to CSV format
    pub fn export_csv(&self) -> String {
        let mut csv = String::from("source,target,type\n");

        for redirect in &self.redirects {
            csv.push_str(&format!(
                "\"{}\",\"{}\",{}\n",
                redirect.source_url,
                redirect.target_url,
                redirect.redirect_type.status_code()
            ));
        }

        csv
    }

    /// Test a URL against redirects
    pub fn test_url(&self, url: &str) -> TestResult {
        if let Some(redirect) = self.find_redirect(url) {
            TestResult {
                matches: true,
                redirect_id: Some(redirect.id),
                source: Some(redirect.source_url.clone()),
                target: Some(redirect.get_target(url)),
                status_code: Some(redirect.redirect_type.status_code()),
            }
        } else {
            TestResult {
                matches: false,
                redirect_id: None,
                source: None,
                target: None,
                status_code: None,
            }
        }
    }
}

impl Default for RedirectService {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of processing a redirect
pub struct RedirectResult {
    pub target_url: String,
    pub status_code: u16,
    pub redirect_id: Uuid,
}

/// Result of import operation
pub struct ImportResult {
    pub imported: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
}

/// Result of URL test
pub struct TestResult {
    pub matches: bool,
    pub redirect_id: Option<Uuid>,
    pub source: Option<String>,
    pub target: Option<String>,
    pub status_code: Option<u16>,
}
