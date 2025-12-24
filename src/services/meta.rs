//! Meta Tag Service
//!
//! Service for managing SEO meta tags.

use crate::models::meta::{SeoMeta, ContentType, MetaRobots};
use crate::models::social::{OpenGraphData, TwitterCardData, OpenGraphType, TwitterCardType};
use uuid::Uuid;

/// Service for managing SEO meta data
pub struct MetaService {
    site_name: String,
    site_url: String,
    separator: String,
    default_og_image: Option<String>,
    twitter_site: Option<String>,
}

impl MetaService {
    pub fn new(site_name: String, site_url: String) -> Self {
        Self {
            site_name,
            site_url,
            separator: " | ".to_string(),
            default_og_image: None,
            twitter_site: None,
        }
    }

    pub fn with_separator(mut self, sep: &str) -> Self {
        self.separator = sep.to_string();
        self
    }

    pub fn with_default_image(mut self, image: &str) -> Self {
        self.default_og_image = Some(image.to_string());
        self
    }

    pub fn with_twitter_site(mut self, handle: &str) -> Self {
        self.twitter_site = Some(handle.to_string());
        self
    }

    /// Generate complete head meta tags
    pub fn generate_head(
        &self,
        meta: &SeoMeta,
        title: &str,
        content_url: &str,
        image: Option<&str>,
        author: Option<&str>,
    ) -> String {
        let mut html = String::new();

        // Basic meta tags
        html.push_str(&meta.to_html(title, &self.site_name, &self.separator));

        // Canonical
        if meta.use_custom_canonical {
            if let Some(canonical) = &meta.canonical_url {
                html.push_str(&format!(
                    "<link rel=\"canonical\" href=\"{}\">\n",
                    canonical
                ));
            }
        } else {
            html.push_str(&format!(
                "<link rel=\"canonical\" href=\"{}\">\n",
                content_url
            ));
        }

        // OpenGraph
        let og = self.generate_opengraph(meta, title, content_url, image, author);
        html.push_str(&og.to_html());

        // Twitter Card
        let twitter = self.generate_twitter_card(meta, title, image);
        html.push_str(&twitter.to_html());

        html
    }

    /// Generate OpenGraph data
    pub fn generate_opengraph(
        &self,
        meta: &SeoMeta,
        title: &str,
        url: &str,
        image: Option<&str>,
        _author: Option<&str>,
    ) -> OpenGraphData {
        let og_type = match meta.content_type {
            ContentType::Post => OpenGraphType::Article,
            ContentType::Product => OpenGraphType::Product,
            _ => OpenGraphType::Website,
        };

        let mut og = OpenGraphData::new(
            og_type,
            meta.get_title(title, &self.site_name, &self.separator),
            url.to_string(),
        );

        og.description = meta.description.clone();
        og.site_name = Some(self.site_name.clone());
        og.image = image.map(|s| s.to_string())
            .or_else(|| self.default_og_image.clone());

        og
    }

    /// Generate Twitter Card data
    pub fn generate_twitter_card(
        &self,
        meta: &SeoMeta,
        title: &str,
        image: Option<&str>,
    ) -> TwitterCardData {
        let card_type = if image.is_some() || self.default_og_image.is_some() {
            TwitterCardType::SummaryLargeImage
        } else {
            TwitterCardType::Summary
        };

        let mut twitter = TwitterCardData::new(
            card_type,
            meta.get_title(title, &self.site_name, &self.separator),
        );

        twitter.description = meta.description.clone();
        twitter.image = image.map(|s| s.to_string())
            .or_else(|| self.default_og_image.clone());
        twitter.site = self.twitter_site.clone();

        twitter
    }

    /// Create default meta for a content type
    pub fn create_default_meta(&self, content_id: Uuid, content_type: ContentType) -> SeoMeta {
        SeoMeta::new(content_id, content_type)
    }

    /// Get default robots for content type
    pub fn get_default_robots(&self, content_type: ContentType) -> MetaRobots {
        match content_type {
            ContentType::Post | ContentType::Page | ContentType::Product => MetaRobots::new(),
            ContentType::Tag | ContentType::Author | ContentType::Archive => MetaRobots::noindex(),
            _ => MetaRobots::new(),
        }
    }

    /// Truncate description to optimal length
    pub fn truncate_description(description: &str, max_length: usize) -> String {
        if description.len() <= max_length {
            return description.to_string();
        }

        let mut truncated = description[..max_length].to_string();

        // Try to end at a word boundary
        if let Some(last_space) = truncated.rfind(' ') {
            truncated = truncated[..last_space].to_string();
        }

        // Remove trailing punctuation except period
        while truncated.ends_with(',') || truncated.ends_with(':') || truncated.ends_with(';') {
            truncated.pop();
        }

        if !truncated.ends_with('.') {
            truncated.push_str("...");
        }

        truncated
    }

    /// Generate excerpt from content for description
    pub fn generate_excerpt(content: &str, max_length: usize) -> String {
        // Remove HTML tags (simple approach)
        let text = content
            .replace(|c: char| c == '<', " <")
            .split('<')
            .filter_map(|s| {
                if let Some(pos) = s.find('>') {
                    Some(s[pos + 1..].to_string())
                } else {
                    Some(s.to_string())
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        // Clean up whitespace
        let clean: String = text
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");

        Self::truncate_description(&clean, max_length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_description() {
        let desc = "This is a very long description that needs to be truncated properly";
        let truncated = MetaService::truncate_description(desc, 30);
        assert!(truncated.len() <= 33); // 30 + "..."
    }

    #[test]
    fn test_generate_excerpt() {
        let html = "<p>This is a <strong>test</strong> paragraph.</p>";
        let excerpt = MetaService::generate_excerpt(html, 100);
        assert!(!excerpt.contains('<'));
        assert!(excerpt.contains("test"));
    }
}
