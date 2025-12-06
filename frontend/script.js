class VDownloader {
    constructor() {
        this.platforms = [];
        this.downloadHistory = this.loadHistory();
        this.activeDownload = null;
        
        this.init();
    }

    async init() {
        await this.loadPlatforms();
        this.setupEventListeners();
        this.renderHistory();
    }

    async loadPlatforms() {
        try {
            const response = await fetch('/api/platforms');
            this.platforms = await response.json();
            this.populatePlatformSelect();
        } catch (error) {
            console.error('Failed to load platforms:', error);
            this.showError('Failed to load platforms. Please refresh the page.');
        }
    }

    populatePlatformSelect() {
        const platformSelect = document.getElementById('platform');
        const enabledPlatforms = this.platforms.filter(p => p.enabled);
        
        enabledPlatforms.forEach(platform => {
            const option = document.createElement('option');
            option.value = platform.key;
            option.textContent = platform.label;
            platformSelect.appendChild(option);
        });
    }

    setupEventListeners() {
        // Form submission
        const form = document.getElementById('downloadForm');
        form.addEventListener('submit', (e) => {
            e.preventDefault();
            this.handleDownload();
        });

        // Platform change
        const platformSelect = document.getElementById('platform');
        platformSelect.addEventListener('change', () => {
            this.updateQualityOptions();
        });

        // Format change
        const formatRadios = document.querySelectorAll('input[name="format"]');
        formatRadios.forEach(radio => {
            radio.addEventListener('change', () => {
                this.updateQualityOptions();
            });
        });

        // URL input validation
        const urlInput = document.getElementById('videoUrl');
        urlInput.addEventListener('input', () => {
            this.validateUrl();
        });
    }

    updateQualityOptions() {
        const platformKey = document.getElementById('platform').value;
        const format = document.querySelector('input[name="format"]:checked').value;
        const qualitySelect = document.getElementById('quality');
        const qualityGroup = document.getElementById('qualityGroup');

        if (!platformKey) {
            qualityGroup.style.display = 'none';
            return;
        }

        const platform = this.platforms.find(p => p.key === platformKey);
        if (!platform) return;

        // Check if platform supports the selected format
        if (!platform.supports.includes(format)) {
            qualityGroup.style.display = 'none';
            this.showError(`${platform.label} does not support ${format} download.`);
            return;
        }

        qualityGroup.style.display = 'block';
        qualitySelect.innerHTML = '<option value="">Select quality</option>';

        platform.qualityOptions.forEach(quality => {
            const option = document.createElement('option');
            option.value = quality;
            option.textContent = quality;
            qualitySelect.appendChild(option);
        });
    }

    validateUrl() {
        const urlInput = document.getElementById('videoUrl');
        const platformKey = document.getElementById('platform').value;
        
        if (!platformKey || !urlInput.value) return;

        const platform = this.platforms.find(p => p.key === platformKey);
        if (!platform) return;

        const isValidDomain = platform.domains.some(domain => 
            urlInput.value.includes(domain)
        );

        if (!isValidDomain) {
            urlInput.setCustomValidity(`Please enter a valid ${platform.label} URL`);
        } else {
            urlInput.setCustomValidity('');
        }
    }

    async handleDownload() {
        const formData = new FormData(document.getElementById('downloadForm'));
        const downloadData = {
            platform: formData.get('platform'),
            url: formData.get('videoUrl'),
            format: formData.get('format'),
            quality: formData.get('quality')
        };

        // Validate
        if (!downloadData.platform || !downloadData.url || !downloadData.quality) {
            this.showError('Please fill in all required fields.');
            return;
        }

        try {
            this.startDownload(downloadData);
            await this.performDownload(downloadData);
        } catch (error) {
            this.showError('Download failed: ' + error.message);
            this.resetDownloadState();
        }
    }

    startDownload(downloadData) {
        this.activeDownload = {
            ...downloadData,
            startTime: Date.now(),
            progress: 0,
            speed: 0,
            status: 'downloading'
        };

        // Show progress section
        const progressSection = document.getElementById('progressSection');
        progressSection.style.display = 'block';
        progressSection.classList.add('fade-in');

        // Update UI
        this.updateProgressUI();
        
        // Disable form
        this.toggleFormState(true);
    }

    async performDownload(downloadData) {
        // Simulate download progress
        const duration = 3000 + Math.random() * 2000; // 3-5 seconds
        const steps = 20;
        const stepDuration = duration / steps;

        for (let i = 0; i <= steps; i++) {
            await new Promise(resolve => setTimeout(resolve, stepDuration));
            
            this.activeDownload.progress = Math.min(100, (i / steps) * 100);
            this.activeDownload.speed = 500 + Math.random() * 1500; // KB/s
            
            this.updateProgressUI();
        }

        // Complete download
        this.completeDownload();
    }

    updateProgressUI() {
        if (!this.activeDownload) return;

        const progressFill = document.getElementById('progressFill');
        const progressPercentage = document.querySelector('.progress-percentage');
        const progressSpeed = document.querySelector('.progress-speed');
        const progressTime = document.querySelector('.progress-time');
        const progressTitle = document.querySelector('.progress-title');

        progressFill.style.width = `${this.activeDownload.progress}%`;
        progressPercentage.textContent = `${Math.round(this.activeDownload.progress)}%`;
        progressSpeed.textContent = `${Math.round(this.activeDownload.speed)} KB/s`;

        // Calculate remaining time
        if (this.activeDownload.progress > 0) {
            const elapsed = Date.now() - this.activeDownload.startTime;
            const totalEstimated = (elapsed / this.activeDownload.progress) * 100;
            const remaining = Math.max(0, totalEstimated - elapsed);
            const remainingSeconds = Math.round(remaining / 1000);
            progressTime.textContent = `Remaining: ${remainingSeconds}s`;
        } else {
            progressTime.textContent = 'Remaining: --';
        }

        // Update title
        const platform = this.platforms.find(p => p.key === this.activeDownload.platform);
        progressTitle.textContent = `Downloading from ${platform?.label || 'Unknown'}...`;
    }

    completeDownload() {
        if (!this.activeDownload) return;

        this.activeDownload.status = 'completed';
        this.activeDownload.completedAt = Date.now();

        // Add to history
        this.addToHistory(this.activeDownload);

        // Show success message
        this.showSuccess('Download completed successfully!');

        // Reset after a delay
        setTimeout(() => {
            this.resetDownloadState();
        }, 2000);
    }

    addToHistory(download) {
        const historyItem = {
            id: Date.now(),
            ...download,
            title: this.extractVideoTitle(download.url),
            thumbnail: this.generateThumbnailUrl(download.url)
        };

        this.downloadHistory.unshift(historyItem);
        if (this.downloadHistory.length > 10) {
            this.downloadHistory = this.downloadHistory.slice(0, 10);
        }

        this.saveHistory();
        this.renderHistory();
    }

    extractVideoTitle(url) {
        // Simple title extraction - in real app, this would come from API
        const platform = this.platforms.find(p => p.key === this.activeDownload.platform);
        const videoId = this.extractVideoId(url, platform);
        return `Video from ${platform?.label || 'Unknown'} (${videoId})`;
    }

    extractVideoId(url, platform) {
        if (!platform) return 'Unknown';
        
        // Simple ID extraction - would be more sophisticated in real app
        for (const domain of platform.domains) {
            if (url.includes(domain)) {
                const parts = url.split('/');
                return parts[parts.length - 1].split('?')[0] || 'Unknown';
            }
        }
        return 'Unknown';
    }

    generateThumbnailUrl(url) {
        // Generate a placeholder thumbnail URL
        return `https://picsum.photos/seed/${encodeURIComponent(url)}/320/180.jpg`;
    }

    renderHistory() {
        const historyEmpty = document.getElementById('historyEmpty');
        const historyList = document.getElementById('historyList');

        if (this.downloadHistory.length === 0) {
            historyEmpty.style.display = 'block';
            historyList.style.display = 'none';
            return;
        }

        historyEmpty.style.display = 'none';
        historyList.style.display = 'grid';
        historyList.innerHTML = '';

        this.downloadHistory.forEach(item => {
            const historyElement = this.createHistoryElement(item);
            historyList.appendChild(historyElement);
        });
    }

    createHistoryElement(item) {
        const div = document.createElement('div');
        div.className = 'history-item fade-in';

        const platform = this.platforms.find(p => p.key === item.platform);
        const iconClass = this.getPlatformIconClass(item.platform);

        div.innerHTML = `
            <div class="history-item-icon">
                <i class="${iconClass}"></i>
            </div>
            <div class="history-item-details">
                <div class="history-item-title">${item.title}</div>
                <div class="history-item-meta">
                    ${platform?.label || 'Unknown'} • ${item.quality} ${item.format} • 
                    ${this.formatDate(item.completedAt)}
                </div>
            </div>
            <div class="history-item-actions">
                <button class="history-item-btn" onclick="app.downloadAgain('${item.id}')">
                    <i class="fas fa-download"></i> Download Again
                </button>
            </div>
        `;

        return div;
    }

    getPlatformIconClass(platformKey) {
        const iconMap = {
            'youtube': 'fab fa-youtube',
            'vimeo': 'fab fa-vimeo',
            'tiktok': 'fab fa-tiktok',
            'twitter': 'fab fa-twitter',
            'instagram': 'fab fa-instagram'
        };
        return iconMap[platformKey] || 'fas fa-video';
    }

    formatDate(timestamp) {
        const date = new Date(timestamp);
        const now = new Date();
        const diffMs = now - date;
        const diffMins = Math.floor(diffMs / 60000);
        const diffHours = Math.floor(diffMs / 3600000);
        const diffDays = Math.floor(diffMs / 86400000);

        if (diffMins < 1) return 'Just now';
        if (diffMins < 60) return `${diffMins} minute${diffMins > 1 ? 's' : ''} ago`;
        if (diffHours < 24) return `${diffHours} hour${diffHours > 1 ? 's' : ''} ago`;
        if (diffDays < 7) return `${diffDays} day${diffDays > 1 ? 's' : ''} ago`;
        
        return date.toLocaleDateString();
    }

    downloadAgain(itemId) {
        const item = this.downloadHistory.find(h => h.id == itemId);
        if (!item) return;

        // Populate form with previous data
        document.getElementById('platform').value = item.platform;
        document.getElementById('videoUrl').value = item.url;
        document.querySelector(`input[name="format"][value="${item.format}"]`).checked = true;
        this.updateQualityOptions();
        document.getElementById('quality').value = item.quality;

        // Scroll to form
        document.querySelector('.download-section').scrollIntoView({ behavior: 'smooth' });
    }

    resetDownloadState() {
        this.activeDownload = null;
        
        // Hide progress section
        const progressSection = document.getElementById('progressSection');
        progressSection.style.display = 'none';

        // Reset form
        document.getElementById('downloadForm').reset();
        document.getElementById('qualityGroup').style.display = 'none';

        // Enable form
        this.toggleFormState(false);
    }

    toggleFormState(disabled) {
        const form = document.getElementById('downloadForm');
        const button = document.getElementById('downloadBtn');
        const inputs = form.querySelectorAll('input, select, button');

        inputs.forEach(input => {
            input.disabled = disabled;
        });

        if (disabled) {
            button.innerHTML = '<span class="spinner"></span> Downloading...';
        } else {
            button.innerHTML = '<i class="fas fa-download"></i> Download';
        }
    }

    showError(message) {
        this.showMessage(message, 'error');
    }

    showSuccess(message) {
        this.showMessage(message, 'success');
    }

    showMessage(message, type) {
        // Create message element
        const messageDiv = document.createElement('div');
        messageDiv.className = `message message-${type} fade-in`;
        messageDiv.innerHTML = `
            <i class="fas fa-${type === 'error' ? 'exclamation-circle' : 'check-circle'}"></i>
            <span>${message}</span>
        `;

        // Add styles if not already added
        if (!document.querySelector('#message-styles')) {
            const style = document.createElement('style');
            style.id = 'message-styles';
            style.textContent = `
                .message {
                    position: fixed;
                    top: 20px;
                    right: 20px;
                    padding: 1rem 1.5rem;
                    border-radius: 8px;
                    color: white;
                    font-weight: 500;
                    z-index: 1000;
                    max-width: 400px;
                    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
                }
                .message-error {
                    background: linear-gradient(135deg, #e74c3c, #c0392b);
                }
                .message-success {
                    background: linear-gradient(135deg, #27ae60, #229954);
                }
                .message i {
                    margin-right: 0.5rem;
                }
            `;
            document.head.appendChild(style);
        }

        // Add to page
        document.body.appendChild(messageDiv);

        // Remove after 5 seconds
        setTimeout(() => {
            messageDiv.style.opacity = '0';
            setTimeout(() => {
                if (messageDiv.parentNode) {
                    messageDiv.parentNode.removeChild(messageDiv);
                }
            }, 300);
        }, 5000);
    }

    loadHistory() {
        try {
            const saved = localStorage.getItem('vdownloader-history');
            return saved ? JSON.parse(saved) : [];
        } catch (error) {
            console.error('Failed to load history:', error);
            return [];
        }
    }

    saveHistory() {
        try {
            localStorage.setItem('vdownloader-history', JSON.stringify(this.downloadHistory));
        } catch (error) {
            console.error('Failed to save history:', error);
        }
    }
}

// Initialize the app when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    window.app = new VDownloader();
});