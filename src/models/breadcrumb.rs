//! Breadcrumb Navigation Models
//!
//! Models for generating breadcrumb navigation with schema markup.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Breadcrumb trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breadcrumb {
    pub items: Vec<BreadcrumbItem>,
    pub separator: String,
    pub show_home: bool,
    pub home_text: String,
}

/// Single breadcrumb item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreadcrumbItem {
    pub name: String,
    pub url: Option<String>,
    pub position: i32,
}

impl Breadcrumb {
    pub fn new() -> Self {
        Self {
            items: vec![],
            separator: " > ".to_string(),
            show_home: true,
            home_text: "Home".to_string(),
        }
    }

    pub fn add(&mut self, name: String, url: Option<String>) -> &mut Self {
        let position = (self.items.len() + 1) as i32;
        self.items.push(BreadcrumbItem { name, url, position });
        self
    }

    pub fn with_home(mut self, home_url: &str) -> Self {
        if self.show_home {
            let mut items = vec![BreadcrumbItem {
                name: self.home_text.clone(),
                url: Some(home_url.to_string()),
                position: 1,
            }];

            // Adjust positions
            for (i, mut item) in self.items.into_iter().enumerate() {
                item.position = (i + 2) as i32;
                items.push(item);
            }

            self.items = items;
        }
        self
    }

    /// Generate HTML for breadcrumbs
    pub fn to_html(&self) -> String {
        let mut html = String::new();
        html.push_str("<nav class=\"breadcrumb\" aria-label=\"Breadcrumb\">\n");
        html.push_str("  <ol class=\"breadcrumb-list\">\n");

        for (i, item) in self.items.iter().enumerate() {
            let is_last = i == self.items.len() - 1;

            html.push_str("    <li class=\"breadcrumb-item\">\n");

            if let Some(url) = &item.url {
                if !is_last {
                    html.push_str(&format!(
                        "      <a href=\"{}\">{}</a>\n",
                        html_escape(url),
                        html_escape(&item.name)
                    ));
                } else {
                    html.push_str(&format!(
                        "      <span aria-current=\"page\">{}</span>\n",
                        html_escape(&item.name)
                    ));
                }
            } else {
                html.push_str(&format!(
                    "      <span>{}</span>\n",
                    html_escape(&item.name)
                ));
            }

            if !is_last {
                html.push_str(&format!(
                    "      <span class=\"breadcrumb-separator\">{}</span>\n",
                    html_escape(&self.separator)
                ));
            }

            html.push_str("    </li>\n");
        }

        html.push_str("  </ol>\n");
        html.push_str("</nav>\n");
        html
    }

    /// Generate JSON-LD schema markup
    pub fn to_json_ld(&self) -> Value {
        let items: Vec<Value> = self.items.iter().map(|item| {
            let mut obj = json!({
                "@type": "ListItem",
                "position": item.position,
                "name": item.name
            });

            if let Some(url) = &item.url {
                obj["item"] = json!(url);
            }

            obj
        }).collect();

        json!({
            "@context": "https://schema.org",
            "@type": "BreadcrumbList",
            "itemListElement": items
        })
    }

    /// Generate complete HTML with schema markup
    pub fn to_html_with_schema(&self) -> String {
        let mut html = self.to_html();
        html.push_str(&format!(
            "<script type=\"application/ld+json\">\n{}\n</script>\n",
            serde_json::to_string_pretty(&self.to_json_ld()).unwrap_or_default()
        ));
        html
    }
}

impl Default for Breadcrumb {
    fn default() -> Self {
        Self::new()
    }
}

/// Breadcrumb settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreadcrumbSettings {
    pub enabled: bool,
    pub show_on_homepage: bool,
    pub show_home: bool,
    pub home_text: String,
    pub separator: String,
    pub show_prefix: bool,
    pub prefix_text: String,
    pub show_last_as_link: bool,
    pub show_post_type: bool,
    pub show_category: bool,
    pub show_parent_pages: bool,
    pub schema_enabled: bool,
}

impl Default for BreadcrumbSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            show_on_homepage: false,
            show_home: true,
            home_text: "Home".to_string(),
            separator: " > ".to_string(),
            show_prefix: false,
            prefix_text: "You are here: ".to_string(),
            show_last_as_link: false,
            show_post_type: false,
            show_category: true,
            show_parent_pages: true,
            schema_enabled: true,
        }
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
