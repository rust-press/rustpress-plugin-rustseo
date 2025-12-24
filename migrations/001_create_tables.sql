-- RustSEO Database Migrations
-- Migration 001: Create initial tables

-- SEO Meta table
CREATE TABLE IF NOT EXISTS rustseo_meta (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content_type VARCHAR(50) NOT NULL,
    content_id VARCHAR(100) NOT NULL,

    -- Meta fields
    meta_title VARCHAR(255),
    meta_description TEXT,
    focus_keyword VARCHAR(255),
    secondary_keywords TEXT[], -- Array of keywords
    canonical_url VARCHAR(500),

    -- Robots
    robots_index BOOLEAN DEFAULT true,
    robots_follow BOOLEAN DEFAULT true,
    robots_noarchive BOOLEAN DEFAULT false,
    robots_nosnippet BOOLEAN DEFAULT false,
    robots_noimageindex BOOLEAN DEFAULT false,

    -- OpenGraph
    og_title VARCHAR(255),
    og_description TEXT,
    og_image VARCHAR(500),
    og_type VARCHAR(50),

    -- Twitter
    twitter_title VARCHAR(255),
    twitter_description TEXT,
    twitter_image VARCHAR(500),
    twitter_card_type VARCHAR(50),

    -- Schema
    schema_type VARCHAR(100),
    custom_schema JSONB,

    -- Analysis
    seo_score INTEGER DEFAULT 0,
    readability_score INTEGER DEFAULT 0,
    last_analyzed_at TIMESTAMP WITH TIME ZONE,
    analysis_data JSONB,

    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    -- Unique constraint
    UNIQUE(content_type, content_id)
);

-- Indexes for SEO Meta
CREATE INDEX idx_rustseo_meta_content ON rustseo_meta(content_type, content_id);
CREATE INDEX idx_rustseo_meta_focus_keyword ON rustseo_meta(focus_keyword);
CREATE INDEX idx_rustseo_meta_seo_score ON rustseo_meta(seo_score);

-- Redirects table
CREATE TABLE IF NOT EXISTS rustseo_redirects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_url VARCHAR(1000) NOT NULL,
    target_url VARCHAR(1000) NOT NULL,
    redirect_type SMALLINT DEFAULT 301,
    match_type VARCHAR(20) DEFAULT 'exact',
    is_active BOOLEAN DEFAULT true,

    -- Statistics
    hit_count BIGINT DEFAULT 0,
    last_hit_at TIMESTAMP WITH TIME ZONE,

    -- Metadata
    notes TEXT,
    group_name VARCHAR(100),
    created_by VARCHAR(100),

    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for Redirects
CREATE INDEX idx_rustseo_redirects_source ON rustseo_redirects(source_url);
CREATE INDEX idx_rustseo_redirects_active ON rustseo_redirects(is_active);
CREATE INDEX idx_rustseo_redirects_type ON rustseo_redirects(redirect_type);

-- 404 Log table
CREATE TABLE IF NOT EXISTS rustseo_404_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    url VARCHAR(1000) NOT NULL,
    hit_count BIGINT DEFAULT 1,
    first_seen_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_seen_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    referrer VARCHAR(1000),
    user_agent TEXT,
    has_redirect BOOLEAN DEFAULT false,
    is_ignored BOOLEAN DEFAULT false,

    -- Unique constraint on URL
    UNIQUE(url)
);

-- Indexes for 404 Log
CREATE INDEX idx_rustseo_404_log_url ON rustseo_404_log(url);
CREATE INDEX idx_rustseo_404_log_hits ON rustseo_404_log(hit_count DESC);
CREATE INDEX idx_rustseo_404_log_ignored ON rustseo_404_log(is_ignored);

-- Sitemap Cache table
CREATE TABLE IF NOT EXISTS rustseo_sitemap_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sitemap_type VARCHAR(50) NOT NULL,
    sitemap_index INTEGER DEFAULT 0,
    content TEXT NOT NULL,
    url_count INTEGER DEFAULT 0,
    generated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    -- Unique constraint
    UNIQUE(sitemap_type, sitemap_index)
);

-- Indexes for Sitemap Cache
CREATE INDEX idx_rustseo_sitemap_cache_type ON rustseo_sitemap_cache(sitemap_type);

-- Keyword Tracking table
CREATE TABLE IF NOT EXISTS rustseo_keywords (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    keyword VARCHAR(255) NOT NULL,
    content_type VARCHAR(50),
    content_id VARCHAR(100),

    -- Tracking
    position_history JSONB,
    current_position INTEGER,
    best_position INTEGER,
    clicks BIGINT DEFAULT 0,
    impressions BIGINT DEFAULT 0,

    -- Search Console data
    search_console_data JSONB,
    last_synced_at TIMESTAMP WITH TIME ZONE,

    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for Keywords
CREATE INDEX idx_rustseo_keywords_keyword ON rustseo_keywords(keyword);
CREATE INDEX idx_rustseo_keywords_content ON rustseo_keywords(content_type, content_id);

-- Settings table
CREATE TABLE IF NOT EXISTS rustseo_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    setting_key VARCHAR(100) NOT NULL UNIQUE,
    setting_value JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for Settings
CREATE INDEX idx_rustseo_settings_key ON rustseo_settings(setting_key);

-- Analysis History table
CREATE TABLE IF NOT EXISTS rustseo_analysis_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content_type VARCHAR(50) NOT NULL,
    content_id VARCHAR(100) NOT NULL,
    seo_score INTEGER,
    readability_score INTEGER,
    issues_count INTEGER DEFAULT 0,
    analysis_data JSONB,
    analyzed_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for Analysis History
CREATE INDEX idx_rustseo_analysis_history_content ON rustseo_analysis_history(content_type, content_id);
CREATE INDEX idx_rustseo_analysis_history_date ON rustseo_analysis_history(analyzed_at);

-- Internal Links table
CREATE TABLE IF NOT EXISTS rustseo_internal_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_content_type VARCHAR(50) NOT NULL,
    source_content_id VARCHAR(100) NOT NULL,
    target_url VARCHAR(1000) NOT NULL,
    anchor_text TEXT,
    is_nofollow BOOLEAN DEFAULT false,
    is_broken BOOLEAN DEFAULT false,
    last_checked_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for Internal Links
CREATE INDEX idx_rustseo_internal_links_source ON rustseo_internal_links(source_content_type, source_content_id);
CREATE INDEX idx_rustseo_internal_links_target ON rustseo_internal_links(target_url);
CREATE INDEX idx_rustseo_internal_links_broken ON rustseo_internal_links(is_broken);

-- Search Console Integration table
CREATE TABLE IF NOT EXISTS rustseo_search_console (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    property_url VARCHAR(500) NOT NULL,
    is_connected BOOLEAN DEFAULT false,
    access_token TEXT,
    refresh_token TEXT,
    token_expires_at TIMESTAMP WITH TIME ZONE,
    last_sync_at TIMESTAMP WITH TIME ZONE,
    sync_data JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers for updated_at
CREATE TRIGGER update_rustseo_meta_updated_at
    BEFORE UPDATE ON rustseo_meta
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_rustseo_redirects_updated_at
    BEFORE UPDATE ON rustseo_redirects
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_rustseo_keywords_updated_at
    BEFORE UPDATE ON rustseo_keywords
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_rustseo_settings_updated_at
    BEFORE UPDATE ON rustseo_settings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_rustseo_search_console_updated_at
    BEFORE UPDATE ON rustseo_search_console
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
