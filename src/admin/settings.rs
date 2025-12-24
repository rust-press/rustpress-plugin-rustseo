//! SEO Settings Admin
//!
//! Settings management for RustSEO plugin.

use serde::{Deserialize, Serialize};

/// General SEO settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub site_name: String,
    pub separator: String,
    pub title_format: TitleFormat,
    pub meta_description_default: String,
    pub knowledge_graph: KnowledgeGraphSettings,
    pub webmaster_tools: WebmasterToolsSettings,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            site_name: String::new(),
            separator: " - ".to_string(),
            title_format: TitleFormat::default(),
            meta_description_default: String::new(),
            knowledge_graph: KnowledgeGraphSettings::default(),
            webmaster_tools: WebmasterToolsSettings::default(),
        }
    }
}

/// Title format settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleFormat {
    pub home: String,
    pub post: String,
    pub page: String,
    pub category: String,
    pub tag: String,
    pub author: String,
    pub archive: String,
    pub search: String,
    pub error_404: String,
}

impl Default for TitleFormat {
    fn default() -> Self {
        Self {
            home: "%%sitename%% %%sep%% %%tagline%%".to_string(),
            post: "%%title%% %%sep%% %%sitename%%".to_string(),
            page: "%%title%% %%sep%% %%sitename%%".to_string(),
            category: "%%term_title%% %%sep%% %%sitename%%".to_string(),
            tag: "%%term_title%% %%sep%% %%sitename%%".to_string(),
            author: "%%name%% %%sep%% %%sitename%%".to_string(),
            archive: "%%date%% %%sep%% %%sitename%%".to_string(),
            search: "Search: %%searchphrase%% %%sep%% %%sitename%%".to_string(),
            error_404: "Page not found %%sep%% %%sitename%%".to_string(),
        }
    }
}

/// Knowledge graph settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraphSettings {
    pub enabled: bool,
    pub entity_type: EntityType,
    pub name: String,
    pub logo: Option<String>,
    pub url: Option<String>,
    pub social_profiles: SocialProfiles,
}

impl Default for KnowledgeGraphSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            entity_type: EntityType::Organization,
            name: String::new(),
            logo: None,
            url: None,
            social_profiles: SocialProfiles::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Person,
    Organization,
    LocalBusiness,
}

/// Social profile URLs
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SocialProfiles {
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub instagram: Option<String>,
    pub linkedin: Option<String>,
    pub youtube: Option<String>,
    pub pinterest: Option<String>,
    pub tiktok: Option<String>,
    pub github: Option<String>,
}

/// Webmaster tools verification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebmasterToolsSettings {
    pub google_verification: Option<String>,
    pub bing_verification: Option<String>,
    pub yandex_verification: Option<String>,
    pub baidu_verification: Option<String>,
    pub pinterest_verification: Option<String>,
}

impl WebmasterToolsSettings {
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        if let Some(ref code) = self.google_verification {
            html.push_str(&format!(
                "<meta name=\"google-site-verification\" content=\"{}\">\n",
                code
            ));
        }
        if let Some(ref code) = self.bing_verification {
            html.push_str(&format!(
                "<meta name=\"msvalidate.01\" content=\"{}\">\n",
                code
            ));
        }
        if let Some(ref code) = self.yandex_verification {
            html.push_str(&format!(
                "<meta name=\"yandex-verification\" content=\"{}\">\n",
                code
            ));
        }
        if let Some(ref code) = self.baidu_verification {
            html.push_str(&format!(
                "<meta name=\"baidu-site-verification\" content=\"{}\">\n",
                code
            ));
        }
        if let Some(ref code) = self.pinterest_verification {
            html.push_str(&format!(
                "<meta name=\"p:domain_verify\" content=\"{}\">\n",
                code
            ));
        }

        html
    }
}

/// Search appearance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAppearanceSettings {
    pub content_types: ContentTypeSettings,
    pub taxonomies: TaxonomySettings,
    pub archives: ArchiveSettings,
    pub breadcrumbs: BreadcrumbSettings,
}

impl Default for SearchAppearanceSettings {
    fn default() -> Self {
        Self {
            content_types: ContentTypeSettings::default(),
            taxonomies: TaxonomySettings::default(),
            archives: ArchiveSettings::default(),
            breadcrumbs: BreadcrumbSettings::default(),
        }
    }
}

/// Content type SEO settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentTypeSettings {
    pub posts: ContentTypeSeo,
    pub pages: ContentTypeSeo,
    pub products: ContentTypeSeo,
    pub custom_types: Vec<ContentTypeSeo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentTypeSeo {
    pub name: String,
    pub show_in_search: bool,
    pub show_seo_settings: bool,
    pub title_template: String,
    pub meta_description_template: String,
    pub schema_type: String,
    pub noindex: bool,
}

impl Default for ContentTypeSeo {
    fn default() -> Self {
        Self {
            name: String::new(),
            show_in_search: true,
            show_seo_settings: true,
            title_template: "%%title%% %%sep%% %%sitename%%".to_string(),
            meta_description_template: "%%excerpt%%".to_string(),
            schema_type: "Article".to_string(),
            noindex: false,
        }
    }
}

/// Taxonomy SEO settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomySettings {
    pub categories: TaxonomySeo,
    pub tags: TaxonomySeo,
    pub custom_taxonomies: Vec<TaxonomySeo>,
}

impl Default for TaxonomySettings {
    fn default() -> Self {
        Self {
            categories: TaxonomySeo {
                name: "Categories".to_string(),
                ..Default::default()
            },
            tags: TaxonomySeo {
                name: "Tags".to_string(),
                ..Default::default()
            },
            custom_taxonomies: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomySeo {
    pub name: String,
    pub show_in_search: bool,
    pub title_template: String,
    pub meta_description_template: String,
    pub noindex: bool,
}

impl Default for TaxonomySeo {
    fn default() -> Self {
        Self {
            name: String::new(),
            show_in_search: true,
            title_template: "%%term_title%% %%sep%% %%sitename%%".to_string(),
            meta_description_template: "%%term_description%%".to_string(),
            noindex: false,
        }
    }
}

/// Archive settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveSettings {
    pub author_archives: ArchiveSeo,
    pub date_archives: ArchiveSeo,
}

impl Default for ArchiveSettings {
    fn default() -> Self {
        Self {
            author_archives: ArchiveSeo {
                enabled: true,
                title_template: "%%name%% %%sep%% %%sitename%%".to_string(),
                noindex: false,
            },
            date_archives: ArchiveSeo {
                enabled: true,
                title_template: "%%date%% %%sep%% %%sitename%%".to_string(),
                noindex: true,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveSeo {
    pub enabled: bool,
    pub title_template: String,
    pub noindex: bool,
}

/// Breadcrumb settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreadcrumbSettings {
    pub enabled: bool,
    pub separator: String,
    pub home_text: String,
    pub show_home: bool,
    pub show_current: bool,
    pub bold_current: bool,
    pub schema_enabled: bool,
}

impl Default for BreadcrumbSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            separator: " » ".to_string(),
            home_text: "Home".to_string(),
            show_home: true,
            show_current: true,
            bold_current: true,
            schema_enabled: true,
        }
    }
}

/// Social media settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialSettings {
    pub facebook: FacebookSettings,
    pub twitter: TwitterSettings,
    pub pinterest: PinterestSettings,
    pub default_image: Option<String>,
}

impl Default for SocialSettings {
    fn default() -> Self {
        Self {
            facebook: FacebookSettings::default(),
            twitter: TwitterSettings::default(),
            pinterest: PinterestSettings::default(),
            default_image: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacebookSettings {
    pub opengraph_enabled: bool,
    pub default_image: Option<String>,
    pub app_id: Option<String>,
    pub admin_ids: Vec<String>,
}

impl Default for FacebookSettings {
    fn default() -> Self {
        Self {
            opengraph_enabled: true,
            default_image: None,
            app_id: None,
            admin_ids: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterSettings {
    pub cards_enabled: bool,
    pub card_type: TwitterCardType,
    pub site_username: Option<String>,
    pub default_image: Option<String>,
}

impl Default for TwitterSettings {
    fn default() -> Self {
        Self {
            cards_enabled: true,
            card_type: TwitterCardType::SummaryLargeImage,
            site_username: None,
            default_image: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TwitterCardType {
    Summary,
    SummaryLargeImage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinterestSettings {
    pub enabled: bool,
    pub verification_code: Option<String>,
}

impl Default for PinterestSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            verification_code: None,
        }
    }
}

/// Schema settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaSettings {
    pub enabled: bool,
    pub organization: OrganizationSchema,
    pub local_business: Option<LocalBusinessSchema>,
    pub article_type: ArticleSchemaType,
}

impl Default for SchemaSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            organization: OrganizationSchema::default(),
            local_business: None,
            article_type: ArticleSchemaType::Article,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationSchema {
    pub name: String,
    pub logo: Option<String>,
    pub url: Option<String>,
    pub contact_type: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

impl Default for OrganizationSchema {
    fn default() -> Self {
        Self {
            name: String::new(),
            logo: None,
            url: None,
            contact_type: None,
            phone: None,
            email: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalBusinessSchema {
    pub business_type: String,
    pub name: String,
    pub address: AddressSchema,
    pub phone: Option<String>,
    pub price_range: Option<String>,
    pub opening_hours: Vec<OpeningHours>,
    pub geo: Option<GeoCoordinates>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressSchema {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpeningHours {
    pub day: String,
    pub opens: String,
    pub closes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArticleSchemaType {
    Article,
    NewsArticle,
    BlogPosting,
    TechArticle,
}

/// Tools settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsSettings {
    pub import_export: ImportExportSettings,
    pub bulk_editor: BulkEditorSettings,
    pub file_editor: FileEditorSettings,
}

impl Default for ToolsSettings {
    fn default() -> Self {
        Self {
            import_export: ImportExportSettings::default(),
            bulk_editor: BulkEditorSettings::default(),
            file_editor: FileEditorSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportExportSettings {
    pub allow_import: bool,
    pub allow_export: bool,
}

impl Default for ImportExportSettings {
    fn default() -> Self {
        Self {
            allow_import: true,
            allow_export: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkEditorSettings {
    pub enabled: bool,
    pub items_per_page: i32,
}

impl Default for BulkEditorSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            items_per_page: 25,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEditorSettings {
    pub enable_robots_editor: bool,
    pub enable_htaccess_editor: bool,
}

impl Default for FileEditorSettings {
    fn default() -> Self {
        Self {
            enable_robots_editor: true,
            enable_htaccess_editor: false,
        }
    }
}

/// All RustSEO settings combined
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustSeoSettings {
    pub general: GeneralSettings,
    pub search_appearance: SearchAppearanceSettings,
    pub social: SocialSettings,
    pub schema: SchemaSettings,
    pub tools: ToolsSettings,
}

impl Default for RustSeoSettings {
    fn default() -> Self {
        Self {
            general: GeneralSettings::default(),
            search_appearance: SearchAppearanceSettings::default(),
            social: SocialSettings::default(),
            schema: SchemaSettings::default(),
            tools: ToolsSettings::default(),
        }
    }
}

/// Settings form for admin UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsForm {
    pub sections: Vec<SettingsSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsSection {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub fields: Vec<SettingsField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsField {
    pub id: String,
    pub name: String,
    pub field_type: FieldType,
    pub label: String,
    pub description: Option<String>,
    pub default_value: Option<String>,
    pub options: Option<Vec<FieldOption>>,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    Text,
    Textarea,
    Select,
    Checkbox,
    Radio,
    Image,
    Color,
    Number,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldOption {
    pub value: String,
    pub label: String,
}

/// Get the general settings form
pub fn get_general_settings_form() -> SettingsForm {
    SettingsForm {
        sections: vec![
            SettingsSection {
                id: "site-info".to_string(),
                title: "Site Info".to_string(),
                description: Some("Basic site information for SEO".to_string()),
                fields: vec![
                    SettingsField {
                        id: "site_name".to_string(),
                        name: "general.site_name".to_string(),
                        field_type: FieldType::Text,
                        label: "Site Name".to_string(),
                        description: Some("The name of your site".to_string()),
                        default_value: None,
                        options: None,
                        required: true,
                    },
                    SettingsField {
                        id: "separator".to_string(),
                        name: "general.separator".to_string(),
                        field_type: FieldType::Select,
                        label: "Title Separator".to_string(),
                        description: Some("Character used between title parts".to_string()),
                        default_value: Some(" - ".to_string()),
                        options: Some(vec![
                            FieldOption { value: " - ".to_string(), label: "Dash ( - )".to_string() },
                            FieldOption { value: " | ".to_string(), label: "Pipe ( | )".to_string() },
                            FieldOption { value: " » ".to_string(), label: "Guillemet ( » )".to_string() },
                            FieldOption { value: " • ".to_string(), label: "Bullet ( • )".to_string() },
                        ]),
                        required: false,
                    },
                ],
            },
            SettingsSection {
                id: "webmaster-tools".to_string(),
                title: "Webmaster Tools".to_string(),
                description: Some("Verification codes for search engines".to_string()),
                fields: vec![
                    SettingsField {
                        id: "google_verification".to_string(),
                        name: "general.webmaster_tools.google_verification".to_string(),
                        field_type: FieldType::Text,
                        label: "Google Verification Code".to_string(),
                        description: Some("Google Search Console verification".to_string()),
                        default_value: None,
                        options: None,
                        required: false,
                    },
                    SettingsField {
                        id: "bing_verification".to_string(),
                        name: "general.webmaster_tools.bing_verification".to_string(),
                        field_type: FieldType::Text,
                        label: "Bing Verification Code".to_string(),
                        description: Some("Bing Webmaster Tools verification".to_string()),
                        default_value: None,
                        options: None,
                        required: false,
                    },
                ],
            },
        ],
    }
}
