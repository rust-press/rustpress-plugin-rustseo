//! SEO Meta Data Models
//!
//! Models for managing SEO meta tags for posts, pages, and other content.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// SEO metadata for a content item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoMeta {
    pub id: Uuid,
    pub content_id: Uuid,
    pub content_type: ContentType,

    // Title settings
    pub title: Option<String>,
    pub title_template: Option<String>,
    pub use_custom_title: bool,

    // Description
    pub description: Option<String>,
    pub use_custom_description: bool,

    // Keywords (legacy but still used)
    pub keywords: Vec<String>,
    pub focus_keyword: Option<String>,

    // Robots directives
    pub robots: MetaRobots,

    // Canonical URL
    pub canonical_url: Option<String>,
    pub use_custom_canonical: bool,

    // Advanced
    pub no_snippet: bool,
    pub no_archive: bool,
    pub no_image_index: bool,
    pub max_snippet: Option<i32>,
    pub max_image_preview: Option<ImagePreviewSize>,
    pub max_video_preview: Option<i32>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Content type for SEO meta
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Post,
    Page,
    Product,
    Category,
    Tag,
    Author,
    Archive,
    Custom,
}

/// Robots meta directives
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetaRobots {
    pub index: bool,
    pub follow: bool,
    pub no_archive: bool,
    pub no_snippet: bool,
    pub no_image_index: bool,
    pub no_translate: bool,
}

impl MetaRobots {
    pub fn new() -> Self {
        Self {
            index: true,
            follow: true,
            no_archive: false,
            no_snippet: false,
            no_image_index: false,
            no_translate: false,
        }
    }

    pub fn noindex() -> Self {
        Self {
            index: false,
            follow: true,
            ..Default::default()
        }
    }

    pub fn nofollow() -> Self {
        Self {
            index: true,
            follow: false,
            ..Default::default()
        }
    }

    pub fn noindex_nofollow() -> Self {
        Self {
            index: false,
            follow: false,
            ..Default::default()
        }
    }

    /// Generate robots meta content string
    pub fn to_content_string(&self) -> String {
        let mut directives = Vec::new();

        if self.index {
            directives.push("index");
        } else {
            directives.push("noindex");
        }

        if self.follow {
            directives.push("follow");
        } else {
            directives.push("nofollow");
        }

        if self.no_archive {
            directives.push("noarchive");
        }

        if self.no_snippet {
            directives.push("nosnippet");
        }

        if self.no_image_index {
            directives.push("noimageindex");
        }

        if self.no_translate {
            directives.push("notranslate");
        }

        directives.join(", ")
    }
}

/// Image preview size for Google
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImagePreviewSize {
    None,
    Standard,
    Large,
}

impl SeoMeta {
    pub fn new(content_id: Uuid, content_type: ContentType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            content_id,
            content_type,
            title: None,
            title_template: None,
            use_custom_title: false,
            description: None,
            use_custom_description: false,
            keywords: vec![],
            focus_keyword: None,
            robots: MetaRobots::new(),
            canonical_url: None,
            use_custom_canonical: false,
            no_snippet: false,
            no_archive: false,
            no_image_index: false,
            max_snippet: None,
            max_image_preview: None,
            max_video_preview: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Generate the final title based on template
    pub fn get_title(&self, post_title: &str, site_name: &str, separator: &str) -> String {
        if self.use_custom_title && self.title.is_some() {
            return self.title.clone().unwrap();
        }

        let template = self.title_template.as_deref()
            .unwrap_or("post_title | site_name");

        template
            .replace("post_title", post_title)
            .replace("site_name", site_name)
            .replace(" | ", separator)
            .replace(" - ", separator)
    }

    /// Generate meta tags HTML
    pub fn to_html(&self, post_title: &str, site_name: &str, separator: &str) -> String {
        let mut html = String::new();

        // Title
        let title = self.get_title(post_title, site_name, separator);
        html.push_str(&format!("<title>{}</title>\n", html_escape(&title)));

        // Description
        if let Some(desc) = &self.description {
            html.push_str(&format!(
                "<meta name=\"description\" content=\"{}\">\n",
                html_escape(desc)
            ));
        }

        // Keywords
        if !self.keywords.is_empty() {
            html.push_str(&format!(
                "<meta name=\"keywords\" content=\"{}\">\n",
                html_escape(&self.keywords.join(", "))
            ));
        }

        // Robots
        html.push_str(&format!(
            "<meta name=\"robots\" content=\"{}\">\n",
            self.robots.to_content_string()
        ));

        // Canonical
        if let Some(canonical) = &self.canonical_url {
            html.push_str(&format!(
                "<link rel=\"canonical\" href=\"{}\">\n",
                html_escape(canonical)
            ));
        }

        html
    }
}

/// Simple HTML escape
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Meta tag configuration for content types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentTypeMeta {
    pub content_type: ContentType,
    pub title_template: String,
    pub description_template: Option<String>,
    pub robots: MetaRobots,
    pub show_in_sitemap: bool,
    pub schema_type: Option<String>,
}

impl Default for ContentTypeMeta {
    fn default() -> Self {
        Self {
            content_type: ContentType::Post,
            title_template: "post_title | site_name".to_string(),
            description_template: None,
            robots: MetaRobots::new(),
            show_in_sitemap: true,
            schema_type: Some("Article".to_string()),
        }
    }
}

/// Homepage meta configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomepageMeta {
    pub title: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub og_image: Option<String>,
}

impl Default for HomepageMeta {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            keywords: vec![],
            og_image: None,
        }
    }
}
