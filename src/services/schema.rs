//! Schema Markup Service
//!
//! Service for generating JSON-LD structured data.

use crate::models::schema::*;
use crate::models::breadcrumb::Breadcrumb;
use chrono::{DateTime, Utc};
use serde_json::Value;

/// Service for generating schema.org structured data
pub struct SchemaService {
    site_name: String,
    site_url: String,
    organization: Option<OrganizationSchema>,
}

impl SchemaService {
    pub fn new(site_name: String, site_url: String) -> Self {
        Self {
            site_name,
            site_url: site_url.trim_end_matches('/').to_string(),
            organization: None,
        }
    }

    pub fn with_organization(mut self, org: OrganizationSchema) -> Self {
        self.organization = Some(org);
        self
    }

    /// Generate website schema
    pub fn website_schema(&self, search_url: Option<&str>) -> Value {
        let mut schema = WebsiteSchema::new(
            self.site_name.clone(),
            self.site_url.clone(),
        );

        if let Some(url) = search_url {
            schema.search_url = Some(url.to_string());
        }

        schema.to_json_ld()
    }

    /// Generate organization schema
    pub fn organization_schema(&self) -> Option<Value> {
        self.organization.as_ref().map(|org| org.to_json_ld())
    }

    /// Generate article schema
    pub fn article_schema(&self, data: ArticleData) -> Value {
        let publisher = self.organization.clone().unwrap_or_else(|| {
            OrganizationSchema::new(self.site_name.clone(), self.site_url.clone())
        });

        let author = PersonSchema {
            name: data.author_name,
            url: data.author_url,
            image: data.author_image,
            job_title: None,
            same_as: vec![],
        };

        let article = ArticleSchema {
            headline: data.title,
            description: data.description,
            url: data.url,
            image: data.images,
            author,
            publisher,
            date_published: data.published_at,
            date_modified: data.modified_at,
            article_type: data.article_type,
            word_count: data.word_count,
            keywords: data.keywords,
        };

        article.to_json_ld()
    }

    /// Generate product schema
    pub fn product_schema(&self, data: ProductSchemaData) -> Value {
        let product = ProductSchema {
            name: data.name,
            description: data.description,
            url: data.url,
            image: data.images,
            sku: data.sku,
            brand: data.brand,
            price: data.price,
            currency: data.currency,
            availability: data.availability,
            condition: data.condition,
            rating: data.rating,
            reviews: vec![],
        };

        product.to_json_ld()
    }

    /// Generate breadcrumb schema
    pub fn breadcrumb_schema(&self, breadcrumb: &Breadcrumb) -> Value {
        breadcrumb.to_json_ld()
    }

    /// Generate FAQ schema
    pub fn faq_schema(&self, questions: Vec<(String, String)>) -> Value {
        let faq = FAQSchema {
            questions: questions.into_iter().map(|(q, a)| FAQItem {
                question: q,
                answer: a,
            }).collect(),
        };

        faq.to_json_ld()
    }

    /// Generate local business schema
    pub fn local_business_schema(&self, data: LocalBusinessData) -> Value {
        let schema = LocalBusinessSchema {
            name: data.name,
            business_type: data.business_type,
            url: data.url,
            logo: data.logo,
            image: data.images,
            description: data.description,
            phone: data.phone,
            email: data.email,
            address: data.address,
            geo: data.geo,
            opening_hours: data.opening_hours,
            price_range: data.price_range,
            same_as: data.social_profiles,
        };

        schema.to_json_ld()
    }

    /// Generate all schemas for a page
    pub fn generate_page_schemas(&self, page_type: PageType, data: PageSchemaData) -> Vec<Value> {
        let mut schemas = Vec::new();

        // Always include website schema on homepage
        if matches!(page_type, PageType::Homepage) {
            schemas.push(self.website_schema(data.search_url.as_deref()));
        }

        // Include organization schema
        if let Some(org_schema) = self.organization_schema() {
            schemas.push(org_schema);
        }

        // Include breadcrumb if provided
        if let Some(breadcrumb) = &data.breadcrumb {
            schemas.push(self.breadcrumb_schema(breadcrumb));
        }

        // Page-specific schemas
        match page_type {
            PageType::Article => {
                if let Some(article) = data.article {
                    schemas.push(self.article_schema(article));
                }
            }
            PageType::Product => {
                if let Some(product) = data.product {
                    schemas.push(self.product_schema(product));
                }
            }
            PageType::FAQ => {
                if !data.faq_items.is_empty() {
                    schemas.push(self.faq_schema(data.faq_items));
                }
            }
            PageType::LocalBusiness => {
                if let Some(business) = data.local_business {
                    schemas.push(self.local_business_schema(business));
                }
            }
            _ => {}
        }

        schemas
    }

    /// Generate script tags for all schemas
    pub fn to_html(&self, schemas: &[Value]) -> String {
        schemas.iter().map(|schema| {
            format!(
                "<script type=\"application/ld+json\">\n{}\n</script>\n",
                serde_json::to_string_pretty(schema).unwrap_or_default()
            )
        }).collect()
    }
}

/// Page type for schema selection
#[derive(Debug, Clone, Copy)]
pub enum PageType {
    Homepage,
    Article,
    Product,
    Category,
    FAQ,
    LocalBusiness,
    Contact,
    About,
    Generic,
}

/// Article data for schema
pub struct ArticleData {
    pub title: String,
    pub description: String,
    pub url: String,
    pub images: Vec<String>,
    pub author_name: String,
    pub author_url: Option<String>,
    pub author_image: Option<String>,
    pub published_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub article_type: SchemaType,
    pub word_count: Option<i32>,
    pub keywords: Vec<String>,
}

/// Product data for schema
pub struct ProductSchemaData {
    pub name: String,
    pub description: String,
    pub url: String,
    pub images: Vec<String>,
    pub sku: Option<String>,
    pub brand: Option<String>,
    pub price: String,
    pub currency: String,
    pub availability: ProductAvailability,
    pub condition: ProductCondition,
    pub rating: Option<AggregateRating>,
}

/// Local business data
pub struct LocalBusinessData {
    pub name: String,
    pub business_type: String,
    pub url: String,
    pub logo: Option<String>,
    pub images: Vec<String>,
    pub description: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: AddressSchema,
    pub geo: Option<GeoCoordinates>,
    pub opening_hours: Vec<OpeningHours>,
    pub price_range: Option<String>,
    pub social_profiles: Vec<String>,
}

/// Complete page schema data
pub struct PageSchemaData {
    pub search_url: Option<String>,
    pub breadcrumb: Option<Breadcrumb>,
    pub article: Option<ArticleData>,
    pub product: Option<ProductSchemaData>,
    pub faq_items: Vec<(String, String)>,
    pub local_business: Option<LocalBusinessData>,
}

impl Default for PageSchemaData {
    fn default() -> Self {
        Self {
            search_url: None,
            breadcrumb: None,
            article: None,
            product: None,
            faq_items: vec![],
            local_business: None,
        }
    }
}
