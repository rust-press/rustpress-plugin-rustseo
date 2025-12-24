//! Schema.org Structured Data Models
//!
//! Models for generating JSON-LD structured data markup.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Schema markup container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaMarkup {
    pub schema_type: SchemaType,
    pub data: Value,
}

/// Supported schema types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SchemaType {
    Article,
    NewsArticle,
    BlogPosting,
    WebPage,
    WebSite,
    Organization,
    LocalBusiness,
    Person,
    Product,
    Review,
    Event,
    Recipe,
    FAQPage,
    HowTo,
    BreadcrumbList,
    SearchAction,
    VideoObject,
    ImageObject,
    Course,
    JobPosting,
}

impl SchemaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Article => "Article",
            Self::NewsArticle => "NewsArticle",
            Self::BlogPosting => "BlogPosting",
            Self::WebPage => "WebPage",
            Self::WebSite => "WebSite",
            Self::Organization => "Organization",
            Self::LocalBusiness => "LocalBusiness",
            Self::Person => "Person",
            Self::Product => "Product",
            Self::Review => "Review",
            Self::Event => "Event",
            Self::Recipe => "Recipe",
            Self::FAQPage => "FAQPage",
            Self::HowTo => "HowTo",
            Self::BreadcrumbList => "BreadcrumbList",
            Self::SearchAction => "SearchAction",
            Self::VideoObject => "VideoObject",
            Self::ImageObject => "ImageObject",
            Self::Course => "Course",
            Self::JobPosting => "JobPosting",
        }
    }
}

impl SchemaMarkup {
    /// Create new schema markup
    pub fn new(schema_type: SchemaType, data: Value) -> Self {
        Self { schema_type, data }
    }

    /// Generate JSON-LD script tag
    pub fn to_json_ld(&self) -> String {
        let json_str = serde_json::to_string_pretty(&self.data).unwrap_or_default();
        format!(
            "<script type=\"application/ld+json\">\n{}\n</script>",
            json_str
        )
    }
}

/// Website schema builder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteSchema {
    pub name: String,
    pub url: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub search_url: Option<String>,
    pub same_as: Vec<String>,
}

impl WebsiteSchema {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            description: None,
            logo: None,
            search_url: None,
            same_as: vec![],
        }
    }

    pub fn to_json_ld(&self) -> Value {
        let mut schema = json!({
            "@context": "https://schema.org",
            "@type": "WebSite",
            "name": self.name,
            "url": self.url
        });

        if let Some(desc) = &self.description {
            schema["description"] = json!(desc);
        }

        if let Some(search) = &self.search_url {
            schema["potentialAction"] = json!({
                "@type": "SearchAction",
                "target": format!("{}?q={{search_term_string}}", search),
                "query-input": "required name=search_term_string"
            });
        }

        if !self.same_as.is_empty() {
            schema["sameAs"] = json!(self.same_as);
        }

        schema
    }
}

/// Organization schema builder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationSchema {
    pub name: String,
    pub url: String,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<AddressSchema>,
    pub same_as: Vec<String>,
    pub founding_date: Option<String>,
    pub founders: Vec<PersonSchema>,
}

impl OrganizationSchema {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            logo: None,
            description: None,
            email: None,
            phone: None,
            address: None,
            same_as: vec![],
            founding_date: None,
            founders: vec![],
        }
    }

    pub fn to_json_ld(&self) -> Value {
        let mut schema = json!({
            "@context": "https://schema.org",
            "@type": "Organization",
            "name": self.name,
            "url": self.url
        });

        if let Some(logo) = &self.logo {
            schema["logo"] = json!(logo);
        }

        if let Some(desc) = &self.description {
            schema["description"] = json!(desc);
        }

        if let Some(email) = &self.email {
            schema["email"] = json!(email);
        }

        if let Some(phone) = &self.phone {
            schema["telephone"] = json!(phone);
        }

        if let Some(addr) = &self.address {
            schema["address"] = addr.to_json();
        }

        if !self.same_as.is_empty() {
            schema["sameAs"] = json!(self.same_as);
        }

        schema
    }
}

/// Address schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressSchema {
    pub street_address: String,
    pub address_locality: String,
    pub address_region: Option<String>,
    pub postal_code: String,
    pub address_country: String,
}

impl AddressSchema {
    pub fn to_json(&self) -> Value {
        let mut addr = json!({
            "@type": "PostalAddress",
            "streetAddress": self.street_address,
            "addressLocality": self.address_locality,
            "postalCode": self.postal_code,
            "addressCountry": self.address_country
        });

        if let Some(region) = &self.address_region {
            addr["addressRegion"] = json!(region);
        }

        addr
    }
}

/// Person schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonSchema {
    pub name: String,
    pub url: Option<String>,
    pub image: Option<String>,
    pub job_title: Option<String>,
    pub same_as: Vec<String>,
}

impl PersonSchema {
    pub fn new(name: String) -> Self {
        Self {
            name,
            url: None,
            image: None,
            job_title: None,
            same_as: vec![],
        }
    }

    pub fn to_json(&self) -> Value {
        let mut person = json!({
            "@type": "Person",
            "name": self.name
        });

        if let Some(url) = &self.url {
            person["url"] = json!(url);
        }

        if let Some(image) = &self.image {
            person["image"] = json!(image);
        }

        if let Some(job) = &self.job_title {
            person["jobTitle"] = json!(job);
        }

        if !self.same_as.is_empty() {
            person["sameAs"] = json!(self.same_as);
        }

        person
    }
}

/// Article schema builder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleSchema {
    pub headline: String,
    pub description: String,
    pub url: String,
    pub image: Vec<String>,
    pub author: PersonSchema,
    pub publisher: OrganizationSchema,
    pub date_published: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub article_type: SchemaType,
    pub word_count: Option<i32>,
    pub keywords: Vec<String>,
}

impl ArticleSchema {
    pub fn to_json_ld(&self) -> Value {
        let mut schema = json!({
            "@context": "https://schema.org",
            "@type": self.article_type.as_str(),
            "headline": self.headline,
            "description": self.description,
            "url": self.url,
            "author": self.author.to_json(),
            "publisher": {
                "@type": "Organization",
                "name": self.publisher.name,
                "url": self.publisher.url
            },
            "datePublished": self.date_published.to_rfc3339(),
            "dateModified": self.date_modified.to_rfc3339(),
            "mainEntityOfPage": {
                "@type": "WebPage",
                "@id": self.url
            }
        });

        if !self.image.is_empty() {
            schema["image"] = json!(self.image);
        }

        if let Some(logo) = &self.publisher.logo {
            schema["publisher"]["logo"] = json!({
                "@type": "ImageObject",
                "url": logo
            });
        }

        if let Some(wc) = self.word_count {
            schema["wordCount"] = json!(wc);
        }

        if !self.keywords.is_empty() {
            schema["keywords"] = json!(self.keywords.join(", "));
        }

        schema
    }
}

/// Product schema builder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSchema {
    pub name: String,
    pub description: String,
    pub url: String,
    pub image: Vec<String>,
    pub sku: Option<String>,
    pub brand: Option<String>,
    pub price: String,
    pub currency: String,
    pub availability: ProductAvailability,
    pub condition: ProductCondition,
    pub rating: Option<AggregateRating>,
    pub reviews: Vec<ReviewSchema>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ProductAvailability {
    InStock,
    OutOfStock,
    PreOrder,
    BackOrder,
    Discontinued,
}

impl ProductAvailability {
    pub fn schema_url(&self) -> &'static str {
        match self {
            Self::InStock => "https://schema.org/InStock",
            Self::OutOfStock => "https://schema.org/OutOfStock",
            Self::PreOrder => "https://schema.org/PreOrder",
            Self::BackOrder => "https://schema.org/BackOrder",
            Self::Discontinued => "https://schema.org/Discontinued",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ProductCondition {
    New,
    Used,
    Refurbished,
    Damaged,
}

impl ProductCondition {
    pub fn schema_url(&self) -> &'static str {
        match self {
            Self::New => "https://schema.org/NewCondition",
            Self::Used => "https://schema.org/UsedCondition",
            Self::Refurbished => "https://schema.org/RefurbishedCondition",
            Self::Damaged => "https://schema.org/DamagedCondition",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateRating {
    pub rating_value: f32,
    pub review_count: i32,
    pub best_rating: f32,
    pub worst_rating: f32,
}

impl AggregateRating {
    pub fn to_json(&self) -> Value {
        json!({
            "@type": "AggregateRating",
            "ratingValue": self.rating_value,
            "reviewCount": self.review_count,
            "bestRating": self.best_rating,
            "worstRating": self.worst_rating
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewSchema {
    pub author: String,
    pub rating: f32,
    pub review_body: String,
    pub date_published: DateTime<Utc>,
}

impl ProductSchema {
    pub fn to_json_ld(&self) -> Value {
        let mut schema = json!({
            "@context": "https://schema.org",
            "@type": "Product",
            "name": self.name,
            "description": self.description,
            "url": self.url,
            "offers": {
                "@type": "Offer",
                "price": self.price,
                "priceCurrency": self.currency,
                "availability": self.availability.schema_url(),
                "itemCondition": self.condition.schema_url()
            }
        });

        if !self.image.is_empty() {
            schema["image"] = json!(self.image);
        }

        if let Some(sku) = &self.sku {
            schema["sku"] = json!(sku);
        }

        if let Some(brand) = &self.brand {
            schema["brand"] = json!({
                "@type": "Brand",
                "name": brand
            });
        }

        if let Some(rating) = &self.rating {
            schema["aggregateRating"] = rating.to_json();
        }

        schema
    }
}

/// FAQ Page schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FAQSchema {
    pub questions: Vec<FAQItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FAQItem {
    pub question: String,
    pub answer: String,
}

impl FAQSchema {
    pub fn to_json_ld(&self) -> Value {
        let items: Vec<Value> = self.questions.iter().map(|q| {
            json!({
                "@type": "Question",
                "name": q.question,
                "acceptedAnswer": {
                    "@type": "Answer",
                    "text": q.answer
                }
            })
        }).collect();

        json!({
            "@context": "https://schema.org",
            "@type": "FAQPage",
            "mainEntity": items
        })
    }
}

/// Local Business schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalBusinessSchema {
    pub name: String,
    pub business_type: String,
    pub url: String,
    pub logo: Option<String>,
    pub image: Vec<String>,
    pub description: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: AddressSchema,
    pub geo: Option<GeoCoordinates>,
    pub opening_hours: Vec<OpeningHours>,
    pub price_range: Option<String>,
    pub same_as: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpeningHours {
    pub day_of_week: Vec<String>,
    pub opens: String,
    pub closes: String,
}

impl LocalBusinessSchema {
    pub fn to_json_ld(&self) -> Value {
        let mut schema = json!({
            "@context": "https://schema.org",
            "@type": self.business_type,
            "name": self.name,
            "url": self.url,
            "address": self.address.to_json()
        });

        if let Some(logo) = &self.logo {
            schema["logo"] = json!(logo);
        }

        if !self.image.is_empty() {
            schema["image"] = json!(self.image);
        }

        if let Some(desc) = &self.description {
            schema["description"] = json!(desc);
        }

        if let Some(phone) = &self.phone {
            schema["telephone"] = json!(phone);
        }

        if let Some(email) = &self.email {
            schema["email"] = json!(email);
        }

        if let Some(geo) = &self.geo {
            schema["geo"] = json!({
                "@type": "GeoCoordinates",
                "latitude": geo.latitude,
                "longitude": geo.longitude
            });
        }

        if !self.opening_hours.is_empty() {
            let hours: Vec<Value> = self.opening_hours.iter().map(|h| {
                json!({
                    "@type": "OpeningHoursSpecification",
                    "dayOfWeek": h.day_of_week,
                    "opens": h.opens,
                    "closes": h.closes
                })
            }).collect();
            schema["openingHoursSpecification"] = json!(hours);
        }

        if let Some(price) = &self.price_range {
            schema["priceRange"] = json!(price);
        }

        if !self.same_as.is_empty() {
            schema["sameAs"] = json!(self.same_as);
        }

        schema
    }
}
