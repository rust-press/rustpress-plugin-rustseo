//! RustSEO Admin Module
//!
//! Admin interface components for SEO management.

pub mod dashboard;
pub mod settings;
pub mod sitemaps;
pub mod redirects;
pub mod analysis;

use serde::{Deserialize, Serialize};

/// Admin menu configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMenu {
    pub items: Vec<AdminMenuItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMenuItem {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub url: String,
    pub capability: String,
    pub children: Vec<AdminSubmenuItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminSubmenuItem {
    pub id: String,
    pub title: String,
    pub url: String,
}

impl AdminMenu {
    pub fn new() -> Self {
        Self {
            items: vec![
                AdminMenuItem {
                    id: "rustseo".to_string(),
                    title: "RustSEO".to_string(),
                    icon: "search".to_string(),
                    url: "/admin/plugins/rustseo".to_string(),
                    capability: "manage_rustseo".to_string(),
                    children: vec![
                        AdminSubmenuItem {
                            id: "seo-dashboard".to_string(),
                            title: "Dashboard".to_string(),
                            url: "/admin/plugins/rustseo".to_string(),
                        },
                        AdminSubmenuItem {
                            id: "seo-settings".to_string(),
                            title: "General Settings".to_string(),
                            url: "/admin/plugins/rustseo/settings".to_string(),
                        },
                        AdminSubmenuItem {
                            id: "seo-appearance".to_string(),
                            title: "Search Appearance".to_string(),
                            url: "/admin/plugins/rustseo/appearance".to_string(),
                        },
                        AdminSubmenuItem {
                            id: "seo-social".to_string(),
                            title: "Social Media".to_string(),
                            url: "/admin/plugins/rustseo/social".to_string(),
                        },
                        AdminSubmenuItem {
                            id: "seo-sitemaps".to_string(),
                            title: "XML Sitemaps".to_string(),
                            url: "/admin/plugins/rustseo/sitemaps".to_string(),
                        },
                        AdminSubmenuItem {
                            id: "seo-schema".to_string(),
                            title: "Schema Markup".to_string(),
                            url: "/admin/plugins/rustseo/schema".to_string(),
                        },
                        AdminSubmenuItem {
                            id: "seo-redirects".to_string(),
                            title: "Redirects".to_string(),
                            url: "/admin/plugins/rustseo/redirects".to_string(),
                        },
                        AdminSubmenuItem {
                            id: "seo-robots".to_string(),
                            title: "Robots.txt".to_string(),
                            url: "/admin/plugins/rustseo/robots".to_string(),
                        },
                        AdminSubmenuItem {
                            id: "seo-analysis".to_string(),
                            title: "SEO Analysis".to_string(),
                            url: "/admin/plugins/rustseo/analysis".to_string(),
                        },
                        AdminSubmenuItem {
                            id: "seo-tools".to_string(),
                            title: "Tools".to_string(),
                            url: "/admin/plugins/rustseo/tools".to_string(),
                        },
                    ],
                },
            ],
        }
    }
}

impl Default for AdminMenu {
    fn default() -> Self {
        Self::new()
    }
}

/// Get admin routes
pub fn get_admin_routes() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        // (method, path, handler)
        ("GET", "/admin/plugins/rustseo", "dashboard::index"),
        ("GET", "/admin/plugins/rustseo/settings", "settings::general"),
        ("POST", "/admin/plugins/rustseo/settings", "settings::update"),
        ("GET", "/admin/plugins/rustseo/appearance", "settings::appearance"),
        ("GET", "/admin/plugins/rustseo/social", "settings::social"),
        ("GET", "/admin/plugins/rustseo/sitemaps", "sitemaps::index"),
        ("POST", "/admin/plugins/rustseo/sitemaps/regenerate", "sitemaps::regenerate"),
        ("GET", "/admin/plugins/rustseo/schema", "settings::schema"),
        ("GET", "/admin/plugins/rustseo/redirects", "redirects::list"),
        ("POST", "/admin/plugins/rustseo/redirects", "redirects::create"),
        ("PUT", "/admin/plugins/rustseo/redirects/:id", "redirects::update"),
        ("DELETE", "/admin/plugins/rustseo/redirects/:id", "redirects::delete"),
        ("GET", "/admin/plugins/rustseo/robots", "settings::robots"),
        ("POST", "/admin/plugins/rustseo/robots", "settings::update_robots"),
        ("GET", "/admin/plugins/rustseo/analysis", "analysis::overview"),
        ("GET", "/admin/plugins/rustseo/tools", "settings::tools"),
    ]
}
