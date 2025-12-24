/**
 * RustSEO Admin JavaScript
 */

(function() {
    'use strict';

    // API helper
    const API = {
        baseUrl: '/api/plugins/rustseo',

        async request(endpoint, options = {}) {
            const url = this.baseUrl + endpoint;
            const response = await fetch(url, {
                headers: {
                    'Content-Type': 'application/json',
                    ...options.headers
                },
                ...options
            });

            if (!response.ok) {
                throw new Error(`API error: ${response.status}`);
            }

            return response.json();
        },

        get(endpoint) {
            return this.request(endpoint);
        },

        post(endpoint, data) {
            return this.request(endpoint, {
                method: 'POST',
                body: JSON.stringify(data)
            });
        },

        put(endpoint, data) {
            return this.request(endpoint, {
                method: 'PUT',
                body: JSON.stringify(data)
            });
        },

        delete(endpoint) {
            return this.request(endpoint, {
                method: 'DELETE'
            });
        }
    };

    // Notification system
    const Notify = {
        show(message, type = 'info', duration = 5000) {
            const container = this.getContainer();
            const notification = document.createElement('div');
            notification.className = `notification notification-${type}`;
            notification.innerHTML = `
                <span class="notification-message">${message}</span>
                <button class="notification-close">&times;</button>
            `;

            container.appendChild(notification);

            notification.querySelector('.notification-close').addEventListener('click', () => {
                this.hide(notification);
            });

            if (duration > 0) {
                setTimeout(() => this.hide(notification), duration);
            }

            return notification;
        },

        hide(notification) {
            notification.classList.add('hiding');
            setTimeout(() => notification.remove(), 300);
        },

        getContainer() {
            let container = document.getElementById('notification-container');
            if (!container) {
                container = document.createElement('div');
                container.id = 'notification-container';
                document.body.appendChild(container);
            }
            return container;
        },

        success(message) { return this.show(message, 'success'); },
        error(message) { return this.show(message, 'error'); },
        warning(message) { return this.show(message, 'warning'); },
        info(message) { return this.show(message, 'info'); }
    };

    // SEO Analysis
    const SEOAnalysis = {
        analyze(content, focusKeyword) {
            const result = {
                score: 0,
                issues: [],
                suggestions: []
            };

            // Word count
            const wordCount = this.countWords(content);
            if (wordCount < 300) {
                result.issues.push({
                    type: 'warning',
                    message: `Content is too short (${wordCount} words). Aim for at least 300 words.`
                });
            } else {
                result.score += 15;
            }

            // Keyword analysis
            if (focusKeyword) {
                const keywordAnalysis = this.analyzeKeyword(content, focusKeyword);
                result.score += keywordAnalysis.score;
                result.issues = result.issues.concat(keywordAnalysis.issues);
                result.keyword = keywordAnalysis;
            } else {
                result.issues.push({
                    type: 'critical',
                    message: 'No focus keyword set. Set a focus keyword to optimize your content.'
                });
            }

            // Headings
            const headingAnalysis = this.analyzeHeadings(content);
            result.score += headingAnalysis.score;
            result.issues = result.issues.concat(headingAnalysis.issues);

            // Images
            const imageAnalysis = this.analyzeImages(content, focusKeyword);
            result.score += imageAnalysis.score;
            result.issues = result.issues.concat(imageAnalysis.issues);

            // Links
            const linkAnalysis = this.analyzeLinks(content);
            result.score += linkAnalysis.score;
            result.issues = result.issues.concat(linkAnalysis.issues);

            // Readability
            const readability = this.analyzeReadability(content);
            result.score += readability.score;
            result.readability = readability;

            // Cap score at 100
            result.score = Math.min(100, Math.max(0, Math.round(result.score)));
            result.grade = this.getGrade(result.score);

            return result;
        },

        countWords(text) {
            const cleaned = text.replace(/<[^>]*>/g, ' ').replace(/\s+/g, ' ').trim();
            return cleaned ? cleaned.split(' ').length : 0;
        },

        analyzeKeyword(content, keyword) {
            const result = {
                score: 0,
                issues: [],
                density: 0,
                occurrences: 0
            };

            const lowerContent = content.toLowerCase();
            const lowerKeyword = keyword.toLowerCase();
            const words = this.countWords(content);

            // Count occurrences
            const regex = new RegExp(lowerKeyword.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'gi');
            const matches = content.match(regex);
            result.occurrences = matches ? matches.length : 0;

            // Calculate density
            if (words > 0) {
                const keywordWords = keyword.split(' ').length;
                result.density = ((result.occurrences * keywordWords) / words) * 100;
            }

            // Check density
            if (result.density < 0.5) {
                result.issues.push({
                    type: 'warning',
                    message: `Keyword density is low (${result.density.toFixed(1)}%). Use your keyword more frequently.`
                });
            } else if (result.density > 2.5) {
                result.issues.push({
                    type: 'warning',
                    message: `Keyword density is high (${result.density.toFixed(1)}%). Avoid keyword stuffing.`
                });
                result.score += 5;
            } else {
                result.score += 15;
            }

            // Check if keyword in first paragraph
            const firstPara = content.split('</p>')[0] || content.substring(0, 200);
            if (firstPara.toLowerCase().includes(lowerKeyword)) {
                result.score += 10;
            } else {
                result.issues.push({
                    type: 'suggestion',
                    message: 'Include your focus keyword in the first paragraph.'
                });
            }

            return result;
        },

        analyzeHeadings(content) {
            const result = {
                score: 0,
                issues: [],
                structure: {}
            };

            // Count heading levels
            for (let i = 1; i <= 6; i++) {
                const regex = new RegExp(`<h${i}[^>]*>`, 'gi');
                const matches = content.match(regex);
                result.structure[`h${i}`] = matches ? matches.length : 0;
            }

            // Check H1
            if (result.structure.h1 === 0) {
                result.issues.push({
                    type: 'warning',
                    message: 'No H1 heading found. Add a main heading to your content.'
                });
            } else if (result.structure.h1 > 1) {
                result.issues.push({
                    type: 'warning',
                    message: `Multiple H1 headings found (${result.structure.h1}). Use only one H1 per page.`
                });
            } else {
                result.score += 10;
            }

            // Check for subheadings
            const subheadings = result.structure.h2 + result.structure.h3;
            if (subheadings >= 2) {
                result.score += 10;
            } else {
                result.issues.push({
                    type: 'suggestion',
                    message: 'Add more subheadings (H2, H3) to structure your content.'
                });
            }

            return result;
        },

        analyzeImages(content, focusKeyword) {
            const result = {
                score: 0,
                issues: [],
                total: 0,
                withAlt: 0
            };

            const imgRegex = /<img[^>]*>/gi;
            const images = content.match(imgRegex) || [];
            result.total = images.length;

            images.forEach(img => {
                const altMatch = img.match(/alt=["']([^"']*)/i);
                if (altMatch && altMatch[1]) {
                    result.withAlt++;
                }
            });

            if (result.total === 0) {
                result.issues.push({
                    type: 'suggestion',
                    message: 'Consider adding images to make your content more engaging.'
                });
            } else {
                result.score += 5;

                if (result.withAlt < result.total) {
                    result.issues.push({
                        type: 'warning',
                        message: `${result.total - result.withAlt} image(s) missing alt text.`
                    });
                } else {
                    result.score += 5;
                }
            }

            return result;
        },

        analyzeLinks(content) {
            const result = {
                score: 0,
                issues: [],
                internal: 0,
                external: 0
            };

            const linkRegex = /<a[^>]*href=["']([^"']*)/gi;
            let match;

            while ((match = linkRegex.exec(content)) !== null) {
                const href = match[1];
                if (href.startsWith('http://') || href.startsWith('https://')) {
                    if (href.includes(window.location.hostname)) {
                        result.internal++;
                    } else {
                        result.external++;
                    }
                } else if (href.startsWith('/') || href.startsWith('#')) {
                    result.internal++;
                }
            }

            if (result.internal === 0) {
                result.issues.push({
                    type: 'suggestion',
                    message: 'Add internal links to other content on your site.'
                });
            } else {
                result.score += 5;
            }

            if (result.external > 0) {
                result.score += 5;
            }

            return result;
        },

        analyzeReadability(content) {
            const result = {
                score: 0,
                fleschScore: 0,
                gradeLevel: ''
            };

            // Strip HTML
            const text = content.replace(/<[^>]*>/g, ' ').replace(/\s+/g, ' ').trim();

            if (!text) {
                return result;
            }

            // Count sentences, words, syllables
            const sentences = text.split(/[.!?]+/).filter(s => s.trim().length > 0).length;
            const words = text.split(/\s+/).length;
            const syllables = this.countSyllables(text);

            if (sentences === 0 || words === 0) {
                return result;
            }

            // Flesch Reading Ease
            result.fleschScore = 206.835 - (1.015 * (words / sentences)) - (84.6 * (syllables / words));
            result.fleschScore = Math.max(0, Math.min(100, result.fleschScore));

            // Grade level
            if (result.fleschScore >= 90) {
                result.gradeLevel = '5th grade';
                result.score = 15;
            } else if (result.fleschScore >= 80) {
                result.gradeLevel = '6th grade';
                result.score = 15;
            } else if (result.fleschScore >= 70) {
                result.gradeLevel = '7th grade';
                result.score = 15;
            } else if (result.fleschScore >= 60) {
                result.gradeLevel = '8th-9th grade';
                result.score = 10;
            } else if (result.fleschScore >= 50) {
                result.gradeLevel = '10th-12th grade';
                result.score = 10;
            } else {
                result.gradeLevel = 'College';
                result.score = 5;
            }

            return result;
        },

        countSyllables(text) {
            const words = text.toLowerCase().replace(/[^a-z\s]/g, '').split(/\s+/);
            let count = 0;

            words.forEach(word => {
                if (word.length <= 3) {
                    count += 1;
                } else {
                    word = word.replace(/(?:[^laeiouy]es|ed|[^laeiouy]e)$/, '');
                    word = word.replace(/^y/, '');
                    const matches = word.match(/[aeiouy]{1,2}/g);
                    count += matches ? matches.length : 1;
                }
            });

            return count;
        },

        getGrade(score) {
            if (score >= 90) return 'A';
            if (score >= 80) return 'B';
            if (score >= 70) return 'C';
            if (score >= 60) return 'D';
            return 'F';
        },

        getScoreColor(score) {
            if (score >= 90) return '#22c55e';
            if (score >= 70) return '#84cc16';
            if (score >= 50) return '#f59e0b';
            if (score >= 30) return '#f97316';
            return '#ef4444';
        }
    };

    // SEO Metabox
    const SEOMetabox = {
        init(container, options = {}) {
            this.container = container;
            this.options = options;
            this.render();
            this.bindEvents();
        },

        render() {
            this.container.innerHTML = `
                <div class="seo-metabox">
                    <div class="seo-metabox-header">
                        <h3>
                            <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <circle cx="11" cy="11" r="8"></circle>
                                <path d="m21 21-4.35-4.35"></path>
                            </svg>
                            RustSEO
                        </h3>
                        <div class="seo-score-indicator">
                            <span class="seo-score-label">Score:</span>
                            <div class="seo-score-circle" id="seo-score">--</div>
                        </div>
                    </div>
                    <div class="seo-metabox-content">
                        <div class="seo-tabs">
                            <button class="seo-tab active" data-tab="general">General</button>
                            <button class="seo-tab" data-tab="social">Social</button>
                            <button class="seo-tab" data-tab="advanced">Advanced</button>
                        </div>

                        <div class="seo-tab-content active" id="tab-general">
                            <div class="seo-preview">
                                <div class="seo-preview-title">Google Preview</div>
                                <div class="google-preview">
                                    <div class="google-preview-url" id="preview-url"></div>
                                    <div class="google-preview-title" id="preview-title"></div>
                                    <div class="google-preview-description" id="preview-description"></div>
                                </div>
                            </div>

                            <div class="form-group">
                                <label for="seo-focus-keyword">Focus Keyword</label>
                                <input type="text" id="seo-focus-keyword" placeholder="Enter your focus keyword">
                            </div>

                            <div class="form-group">
                                <label for="seo-title">SEO Title</label>
                                <input type="text" id="seo-title" placeholder="Enter SEO title">
                                <div class="char-counter"><span id="title-count">0</span> / 60</div>
                            </div>

                            <div class="form-group">
                                <label for="seo-description">Meta Description</label>
                                <textarea id="seo-description" rows="3" placeholder="Enter meta description"></textarea>
                                <div class="char-counter"><span id="desc-count">0</span> / 160</div>
                            </div>

                            <div class="keyword-analysis" id="keyword-analysis"></div>
                        </div>

                        <div class="seo-tab-content" id="tab-social">
                            <div class="form-group">
                                <label for="og-title">Facebook Title</label>
                                <input type="text" id="og-title" placeholder="Title for Facebook">
                            </div>
                            <div class="form-group">
                                <label for="og-description">Facebook Description</label>
                                <textarea id="og-description" rows="2" placeholder="Description for Facebook"></textarea>
                            </div>
                            <div class="form-group">
                                <label for="twitter-title">Twitter Title</label>
                                <input type="text" id="twitter-title" placeholder="Title for Twitter">
                            </div>
                        </div>

                        <div class="seo-tab-content" id="tab-advanced">
                            <div class="form-group">
                                <label for="canonical-url">Canonical URL</label>
                                <input type="url" id="canonical-url" placeholder="https://...">
                            </div>
                            <div class="form-group checkbox-group">
                                <label class="checkbox-label">
                                    <input type="checkbox" id="noindex" name="robots_noindex">
                                    <span>Noindex - Prevent search engines from indexing this page</span>
                                </label>
                            </div>
                            <div class="form-group checkbox-group">
                                <label class="checkbox-label">
                                    <input type="checkbox" id="nofollow" name="robots_nofollow">
                                    <span>Nofollow - Prevent search engines from following links</span>
                                </label>
                            </div>
                        </div>
                    </div>
                </div>
            `;
        },

        bindEvents() {
            // Tab switching
            const tabs = this.container.querySelectorAll('.seo-tab');
            const contents = this.container.querySelectorAll('.seo-tab-content');

            tabs.forEach(tab => {
                tab.addEventListener('click', () => {
                    tabs.forEach(t => t.classList.remove('active'));
                    contents.forEach(c => c.classList.remove('active'));
                    tab.classList.add('active');
                    document.getElementById('tab-' + tab.dataset.tab)?.classList.add('active');
                });
            });

            // Character counters
            const titleInput = document.getElementById('seo-title');
            const descInput = document.getElementById('seo-description');

            titleInput?.addEventListener('input', () => {
                document.getElementById('title-count').textContent = titleInput.value.length;
                this.updatePreview();
            });

            descInput?.addEventListener('input', () => {
                document.getElementById('desc-count').textContent = descInput.value.length;
                this.updatePreview();
            });

            // Focus keyword
            document.getElementById('seo-focus-keyword')?.addEventListener('input', () => {
                this.runAnalysis();
            });
        },

        updatePreview() {
            const title = document.getElementById('seo-title')?.value || this.options.defaultTitle || '';
            const description = document.getElementById('seo-description')?.value || '';
            const url = this.options.url || window.location.href;

            document.getElementById('preview-title').textContent = title.substring(0, 60);
            document.getElementById('preview-description').textContent = description.substring(0, 160);
            document.getElementById('preview-url').textContent = url;
        },

        runAnalysis() {
            const content = this.options.getContent?.() || '';
            const keyword = document.getElementById('seo-focus-keyword')?.value || '';

            const result = SEOAnalysis.analyze(content, keyword);

            // Update score
            const scoreEl = document.getElementById('seo-score');
            if (scoreEl) {
                scoreEl.textContent = result.score;
                scoreEl.style.backgroundColor = SEOAnalysis.getScoreColor(result.score);
            }

            // Update analysis
            this.renderAnalysis(result);
        },

        renderAnalysis(result) {
            const container = document.getElementById('keyword-analysis');
            if (!container) return;

            let html = `
                <div class="keyword-analysis-header">
                    <span class="keyword-analysis-title">SEO Analysis</span>
                    ${result.keyword ? `<span class="keyword-density">Density: ${result.keyword.density.toFixed(1)}%</span>` : ''}
                </div>
                <ul class="analysis-checklist">
            `;

            result.issues.forEach(issue => {
                const iconClass = issue.type === 'critical' ? 'fail' :
                                 issue.type === 'warning' ? 'warning' : 'pass';
                html += `
                    <li class="analysis-item">
                        <span class="analysis-item-icon ${iconClass}">
                            ${iconClass === 'pass' ? '✓' : iconClass === 'fail' ? '✗' : '!'}
                        </span>
                        <span>${issue.message}</span>
                    </li>
                `;
            });

            html += '</ul>';
            container.innerHTML = html;
        }
    };

    // Export to global scope
    window.RustSEO = {
        API,
        Notify,
        SEOAnalysis,
        SEOMetabox
    };

    // Auto-initialize on DOM ready
    document.addEventListener('DOMContentLoaded', function() {
        // Initialize any auto-init components
        const metaboxContainer = document.getElementById('rustseo-metabox');
        if (metaboxContainer) {
            SEOMetabox.init(metaboxContainer, {
                url: window.location.href,
                defaultTitle: document.title,
                getContent: () => {
                    // Get content from editor
                    const editor = document.getElementById('content');
                    return editor ? editor.value : '';
                }
            });
        }
    });
})();
