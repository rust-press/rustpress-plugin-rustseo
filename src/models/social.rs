//! Social Media Meta Models
//!
//! Models for OpenGraph and Twitter Cards meta tags.

use serde::{Deserialize, Serialize};

/// OpenGraph data for social sharing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenGraphData {
    /// og:type (website, article, product, etc.)
    pub og_type: OpenGraphType,

    /// og:title
    pub title: String,

    /// og:description
    pub description: Option<String>,

    /// og:url
    pub url: String,

    /// og:image
    pub image: Option<String>,

    /// og:image:width
    pub image_width: Option<i32>,

    /// og:image:height
    pub image_height: Option<i32>,

    /// og:image:alt
    pub image_alt: Option<String>,

    /// og:site_name
    pub site_name: Option<String>,

    /// og:locale
    pub locale: Option<String>,

    /// og:locale:alternate
    pub locale_alternates: Vec<String>,

    /// Article specific
    pub article: Option<OpenGraphArticle>,

    /// Product specific
    pub product: Option<OpenGraphProduct>,

    /// Video specific
    pub video: Option<OpenGraphVideo>,

    /// Facebook App ID
    pub fb_app_id: Option<String>,
}

/// OpenGraph type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenGraphType {
    Website,
    Article,
    Product,
    Video,
    Music,
    Book,
    Profile,
    Business,
}

impl OpenGraphType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Website => "website",
            Self::Article => "article",
            Self::Product => "product",
            Self::Video => "video.other",
            Self::Music => "music.song",
            Self::Book => "book",
            Self::Profile => "profile",
            Self::Business => "business.business",
        }
    }
}

/// OpenGraph article data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenGraphArticle {
    pub published_time: Option<String>,
    pub modified_time: Option<String>,
    pub expiration_time: Option<String>,
    pub author: Vec<String>,
    pub section: Option<String>,
    pub tag: Vec<String>,
}

/// OpenGraph product data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenGraphProduct {
    pub price_amount: Option<String>,
    pub price_currency: Option<String>,
    pub availability: Option<String>,
    pub condition: Option<String>,
    pub retailer_item_id: Option<String>,
}

/// OpenGraph video data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenGraphVideo {
    pub url: String,
    pub secure_url: Option<String>,
    pub video_type: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
}

impl OpenGraphData {
    pub fn new(og_type: OpenGraphType, title: String, url: String) -> Self {
        Self {
            og_type,
            title,
            description: None,
            url,
            image: None,
            image_width: None,
            image_height: None,
            image_alt: None,
            site_name: None,
            locale: None,
            locale_alternates: vec![],
            article: None,
            product: None,
            video: None,
            fb_app_id: None,
        }
    }

    /// Generate OpenGraph meta tags HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        // Basic tags
        html.push_str(&format!(
            "<meta property=\"og:type\" content=\"{}\">\n",
            self.og_type.as_str()
        ));
        html.push_str(&format!(
            "<meta property=\"og:title\" content=\"{}\">\n",
            html_escape(&self.title)
        ));
        html.push_str(&format!(
            "<meta property=\"og:url\" content=\"{}\">\n",
            html_escape(&self.url)
        ));

        if let Some(desc) = &self.description {
            html.push_str(&format!(
                "<meta property=\"og:description\" content=\"{}\">\n",
                html_escape(desc)
            ));
        }

        if let Some(image) = &self.image {
            html.push_str(&format!(
                "<meta property=\"og:image\" content=\"{}\">\n",
                html_escape(image)
            ));

            if let Some(w) = self.image_width {
                html.push_str(&format!(
                    "<meta property=\"og:image:width\" content=\"{}\">\n",
                    w
                ));
            }

            if let Some(h) = self.image_height {
                html.push_str(&format!(
                    "<meta property=\"og:image:height\" content=\"{}\">\n",
                    h
                ));
            }

            if let Some(alt) = &self.image_alt {
                html.push_str(&format!(
                    "<meta property=\"og:image:alt\" content=\"{}\">\n",
                    html_escape(alt)
                ));
            }
        }

        if let Some(site_name) = &self.site_name {
            html.push_str(&format!(
                "<meta property=\"og:site_name\" content=\"{}\">\n",
                html_escape(site_name)
            ));
        }

        if let Some(locale) = &self.locale {
            html.push_str(&format!(
                "<meta property=\"og:locale\" content=\"{}\">\n",
                html_escape(locale)
            ));
        }

        for alt_locale in &self.locale_alternates {
            html.push_str(&format!(
                "<meta property=\"og:locale:alternate\" content=\"{}\">\n",
                html_escape(alt_locale)
            ));
        }

        // Article specific
        if let Some(article) = &self.article {
            if let Some(pub_time) = &article.published_time {
                html.push_str(&format!(
                    "<meta property=\"article:published_time\" content=\"{}\">\n",
                    pub_time
                ));
            }
            if let Some(mod_time) = &article.modified_time {
                html.push_str(&format!(
                    "<meta property=\"article:modified_time\" content=\"{}\">\n",
                    mod_time
                ));
            }
            for author in &article.author {
                html.push_str(&format!(
                    "<meta property=\"article:author\" content=\"{}\">\n",
                    html_escape(author)
                ));
            }
            if let Some(section) = &article.section {
                html.push_str(&format!(
                    "<meta property=\"article:section\" content=\"{}\">\n",
                    html_escape(section)
                ));
            }
            for tag in &article.tag {
                html.push_str(&format!(
                    "<meta property=\"article:tag\" content=\"{}\">\n",
                    html_escape(tag)
                ));
            }
        }

        // Product specific
        if let Some(product) = &self.product {
            if let Some(price) = &product.price_amount {
                html.push_str(&format!(
                    "<meta property=\"product:price:amount\" content=\"{}\">\n",
                    price
                ));
            }
            if let Some(currency) = &product.price_currency {
                html.push_str(&format!(
                    "<meta property=\"product:price:currency\" content=\"{}\">\n",
                    currency
                ));
            }
            if let Some(avail) = &product.availability {
                html.push_str(&format!(
                    "<meta property=\"product:availability\" content=\"{}\">\n",
                    avail
                ));
            }
        }

        // Facebook App ID
        if let Some(fb_id) = &self.fb_app_id {
            html.push_str(&format!(
                "<meta property=\"fb:app_id\" content=\"{}\">\n",
                fb_id
            ));
        }

        html
    }
}

/// Twitter Card data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterCardData {
    /// twitter:card type
    pub card_type: TwitterCardType,

    /// twitter:title
    pub title: String,

    /// twitter:description
    pub description: Option<String>,

    /// twitter:image
    pub image: Option<String>,

    /// twitter:image:alt
    pub image_alt: Option<String>,

    /// twitter:site (@username)
    pub site: Option<String>,

    /// twitter:creator (@username)
    pub creator: Option<String>,

    /// Player card specific
    pub player: Option<TwitterPlayer>,
}

/// Twitter card type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TwitterCardType {
    Summary,
    SummaryLargeImage,
    App,
    Player,
}

impl TwitterCardType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Summary => "summary",
            Self::SummaryLargeImage => "summary_large_image",
            Self::App => "app",
            Self::Player => "player",
        }
    }
}

/// Twitter player card data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterPlayer {
    pub url: String,
    pub width: i32,
    pub height: i32,
    pub stream: Option<String>,
}

impl TwitterCardData {
    pub fn new(card_type: TwitterCardType, title: String) -> Self {
        Self {
            card_type,
            title,
            description: None,
            image: None,
            image_alt: None,
            site: None,
            creator: None,
            player: None,
        }
    }

    /// Generate Twitter Card meta tags HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        html.push_str(&format!(
            "<meta name=\"twitter:card\" content=\"{}\">\n",
            self.card_type.as_str()
        ));
        html.push_str(&format!(
            "<meta name=\"twitter:title\" content=\"{}\">\n",
            html_escape(&self.title)
        ));

        if let Some(desc) = &self.description {
            html.push_str(&format!(
                "<meta name=\"twitter:description\" content=\"{}\">\n",
                html_escape(desc)
            ));
        }

        if let Some(image) = &self.image {
            html.push_str(&format!(
                "<meta name=\"twitter:image\" content=\"{}\">\n",
                html_escape(image)
            ));

            if let Some(alt) = &self.image_alt {
                html.push_str(&format!(
                    "<meta name=\"twitter:image:alt\" content=\"{}\">\n",
                    html_escape(alt)
                ));
            }
        }

        if let Some(site) = &self.site {
            html.push_str(&format!(
                "<meta name=\"twitter:site\" content=\"{}\">\n",
                html_escape(site)
            ));
        }

        if let Some(creator) = &self.creator {
            html.push_str(&format!(
                "<meta name=\"twitter:creator\" content=\"{}\">\n",
                html_escape(creator)
            ));
        }

        if let Some(player) = &self.player {
            html.push_str(&format!(
                "<meta name=\"twitter:player\" content=\"{}\">\n",
                html_escape(&player.url)
            ));
            html.push_str(&format!(
                "<meta name=\"twitter:player:width\" content=\"{}\">\n",
                player.width
            ));
            html.push_str(&format!(
                "<meta name=\"twitter:player:height\" content=\"{}\">\n",
                player.height
            ));
            if let Some(stream) = &player.stream {
                html.push_str(&format!(
                    "<meta name=\"twitter:player:stream\" content=\"{}\">\n",
                    html_escape(stream)
                ));
            }
        }

        html
    }
}

/// Social media settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialSettings {
    pub opengraph_enabled: bool,
    pub twitter_cards_enabled: bool,
    pub default_og_type: OpenGraphType,
    pub default_twitter_card: TwitterCardType,
    pub default_image: Option<String>,
    pub twitter_site: Option<String>,
    pub facebook_app_id: Option<String>,
    pub facebook_admin_ids: Vec<String>,
}

impl Default for SocialSettings {
    fn default() -> Self {
        Self {
            opengraph_enabled: true,
            twitter_cards_enabled: true,
            default_og_type: OpenGraphType::Website,
            default_twitter_card: TwitterCardType::SummaryLargeImage,
            default_image: None,
            twitter_site: None,
            facebook_app_id: None,
            facebook_admin_ids: vec![],
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
