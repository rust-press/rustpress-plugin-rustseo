//! RustSEO HTTP Handlers
//!
//! REST API handlers for the SEO plugin.

pub mod dashboard;
pub mod settings;
pub mod meta;
pub mod sitemaps;
pub mod redirects;
pub mod analysis;
pub mod schema;
pub mod robots;

use serde::{Deserialize, Serialize};

/// Standard API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub errors: Option<Vec<ApiError>>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            errors: None,
        }
    }

    pub fn success_with_message(data: T, message: &str) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message.to_string()),
            errors: None,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message.to_string()),
            errors: None,
        }
    }

    pub fn validation_error(errors: Vec<ApiError>) -> Self {
        Self {
            success: false,
            data: None,
            message: Some("Validation failed".to_string()),
            errors: Some(errors),
        }
    }
}

/// API error detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub field: Option<String>,
    pub code: String,
    pub message: String,
}

impl ApiError {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            field: None,
            code: code.to_string(),
            message: message.to_string(),
        }
    }

    pub fn field_error(field: &str, code: &str, message: &str) -> Self {
        Self {
            field: Some(field.to_string()),
            code: code.to_string(),
            message: message.to_string(),
        }
    }
}

/// Pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_per_page")]
    pub per_page: i32,
}

fn default_page() -> i32 { 1 }
fn default_per_page() -> i32 { 20 }

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub page: i32,
    pub per_page: i32,
    pub total_items: i64,
    pub total_pages: i32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, page: i32, per_page: i32, total_items: i64) -> Self {
        let total_pages = ((total_items as f64) / (per_page as f64)).ceil() as i32;
        Self {
            items,
            page,
            per_page,
            total_items,
            total_pages,
        }
    }
}

/// Sort parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortParams {
    #[serde(default = "default_sort_field")]
    pub sort_by: String,
    #[serde(default = "default_sort_order")]
    pub sort_order: String,
}

fn default_sort_field() -> String { "created_at".to_string() }
fn default_sort_order() -> String { "desc".to_string() }

impl Default for SortParams {
    fn default() -> Self {
        Self {
            sort_by: "created_at".to_string(),
            sort_order: "desc".to_string(),
        }
    }
}

/// Filter parameters for content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentFilterParams {
    pub content_type: Option<String>,
    pub status: Option<String>,
    pub author_id: Option<String>,
    pub search: Option<String>,
}

/// Common request context
#[derive(Debug, Clone)]
pub struct RequestContext {
    pub user_id: Option<String>,
    pub is_admin: bool,
    pub site_url: String,
    pub locale: String,
}

impl Default for RequestContext {
    fn default() -> Self {
        Self {
            user_id: None,
            is_admin: false,
            site_url: String::new(),
            locale: "en".to_string(),
        }
    }
}
