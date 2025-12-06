class VDownloader {
    constructor() {
        this.platforms = [];
        this.downloadHistory = this.loadHistory();
        this.activeDownloads = new Map();
        this.socket = null;
        this.API_BASE = window.location.origin;
        
        this.init();
    }

    async init() {
        await this.loadPlatforms();
        this.setupEventListeners();
        this.renderHistory();
        this.connectWebSocket();
    }

    connectWebSocket() {
        if (typeof io === 'undefined') {
            console.warn('Socket.IO not loaded, will use polling fallback');
            return;
        }

        this.socket = io(this.API_BASE);

        this.socket.on('connect', () => {
            console.log('WebSocket connected');
        });

        this.socket.on('disconnect', () => {
            console.log('WebSocket disconnected');
        });

        this.socket.on('download:progress', (data) => {
            this.handleDownloadProgress(data);
        });

        this.socket.on('download:complete', (data) => {
            this.handleDownloadComplete(data);
        });

        this.socket.on('download:error', (data) => {
            this.handleDownloadError(data);
        });
    }

    subscribeToDownload(downloadId) {
        if (this.socket && this.socket.connected) {
            this.socket.emit('subscribe', downloadId);
        }
    }

    unsubscribeFromDownload(downloadId) {
        if (this.socket && this.socket.connected) {
            this.socket.emit('unsubscribe', downloadId);
        }
    }

    async loadPlatforms() {
        try {
            const response = await fetch(`${this.API_BASE}/api/platforms`);
            const result = await response.json();
            
            if (result.success) {
                this.platforms = result.data;
                this.populatePlatformSelect();
            } else {
                throw new Error(result.error?.message || 'Failed to load platforms');
            }
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
        const form = document.getElementById('downloadForm');
        form.addEventListener('submit', (e) => {
            e.preventDefault();
            this.handleDownload();
        });

        const platformSelect = document.getElementById('platform');
        platformSelect.addEventListener('change', () => {
            this.updateQualityOptions();
        });

        const formatRadios = document.querySelectorAll('input[name="format"]');
        formatRadios.forEach(radio => {
            radio.addEventListener('change', () => {
                this.updateQualityOptions();
            });
        });

        const urlInput = document.getElementById('videoUrl');
        urlInput.addEventListener('input', () => {
            this.validateUrlInput();
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

    validateUrlInput() {
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
            url: formData.get('videoUrl'),
            format: formData.get('format'),
            quality: formData.get('quality')
        };

        if (!downloadData.url || !downloadData.quality) {
            this.showError('Please fill in all required fields.');
            return;
        }

        try {
            this.toggleFormState(true);

            const validated = await this.validateUrl(downloadData.url);
            
            if (!validated.valid) {
                this.showError(validated.error || 'Invalid URL');
                this.toggleFormState(false);
                return;
            }

            const download = await this.initiateDownload(downloadData);
            
            if (download) {
                this.showSuccess('Download started successfully!');
                this.addActiveDownload(download, validated.metadata);
            }

        } catch (error) {
            this.showError('Download failed: ' + error.message);
            this.toggleFormState(false);
        }
    }

    async validateUrl(url) {
        try {
            const response = await fetch(`${this.API_BASE}/api/validate`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ url })
            });

            const result = await response.json();

            if (result.success) {
                return {
                    valid: true,
                    platform: result.data.platform,
                    metadata: result.data.metadata
                };
            } else {
                return {
                    valid: false,
                    error: result.error?.message || 'Validation failed'
                };
            }
        } catch (error) {
            console.error('Validation error:', error);
            return {
                valid: false,
                error: 'Failed to validate URL'
            };
        }
    }

    async initiateDownload(downloadData) {
        try {
            const response = await fetch(`${this.API_BASE}/api/download`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(downloadData)
            });

            const result = await response.json();

            if (result.success) {
                return result.data;
            } else {
                throw new Error(result.error?.message || 'Download initiation failed');
            }
        } catch (error) {
            console.error('Download initiation error:', error);
            throw error;
        }
    }

    addActiveDownload(download, metadata) {
        const downloadInfo = {
            ...download,
            metadata: metadata || {},
            progress: 0,
            speed: 0,
            startTime: Date.now()
        };

        this.activeDownloads.set(download.downloadId, downloadInfo);
        this.subscribeToDownload(download.downloadId);
        this.renderActiveDownloads();
        this.startPolling(download.downloadId);
    }

    startPolling(downloadId) {
        const pollInterval = setInterval(async () => {
            const download = this.activeDownloads.get(downloadId);
            
            if (!download || download.status === 'completed' || download.status === 'failed' || download.status === 'cancelled') {
                clearInterval(pollInterval);
                return;
            }

            if (this.socket && this.socket.connected) {
                return;
            }

            try {
                const response = await fetch(`${this.API_BASE}/api/status/${downloadId}`);
                const result = await response.json();

                if (result.success) {
                    this.handleDownloadProgress({
                        downloadId,
                        ...result.data
                    });

                    if (result.data.status === 'completed') {
                        this.handleDownloadComplete({
                            downloadId,
                            ...result.data
                        });
                    } else if (result.data.status === 'failed') {
                        this.handleDownloadError({
                            downloadId,
                            error: result.data.error
                        });
                    }
                }
            } catch (error) {
                console.error('Polling error:', error);
            }
        }, 1000);
    }

    handleDownloadProgress(data) {
        const download = this.activeDownloads.get(data.downloadId);
        if (!download) return;

        Object.assign(download, {
            progress: data.progress || 0,
            speed: data.speed || 0,
            bytesDownloaded: data.bytesDownloaded || 0,
            totalBytes: data.totalBytes || 0,
            status: data.status
        });

        this.renderActiveDownloads();
    }

    handleDownloadComplete(data) {
        const download = this.activeDownloads.get(data.downloadId);
        if (!download) return;

        download.status = 'completed';
        download.progress = 100;
        download.completedAt = data.completedAt || new Date().toISOString();

        this.addToHistory(download);
        this.unsubscribeFromDownload(data.downloadId);

        setTimeout(() => {
            this.activeDownloads.delete(data.downloadId);
            this.renderActiveDownloads();
            
            if (this.activeDownloads.size === 0) {
                this.toggleFormState(false);
            }
        }, 3000);

        this.renderActiveDownloads();
    }

    handleDownloadError(data) {
        const download = this.activeDownloads.get(data.downloadId);
        if (!download) return;

        download.status = 'failed';
        download.error = data.error;

        this.showError(`Download failed: ${data.error}`);
        this.unsubscribeFromDownload(data.downloadId);

        setTimeout(() => {
            this.activeDownloads.delete(data.downloadId);
            this.renderActiveDownloads();
            
            if (this.activeDownloads.size === 0) {
                this.toggleFormState(false);
            }
        }, 5000);

        this.renderActiveDownloads();
    }

    renderActiveDownloads() {
        const progressSection = document.getElementById('progressSection');
        
        if (this.activeDownloads.size === 0) {
            progressSection.style.display = 'none';
            return;
        }

        progressSection.style.display = 'block';
        progressSection.innerHTML = '';

        this.activeDownloads.forEach((download, downloadId) => {
            const downloadElement = this.createDownloadProgressElement(download);
            progressSection.appendChild(downloadElement);
        });
    }

    createDownloadProgressElement(download) {
        const div = document.createElement('div');
        div.className = 'progress-card fade-in';

        const title = download.metadata?.title || download.url;
        const platform = this.platforms.find(p => p.key === download.platform);
        const platformLabel = platform?.label || 'Unknown';

        const elapsed = Date.now() - download.startTime;
        const totalEstimated = download.progress > 0 ? (elapsed / download.progress) * 100 : 0;
        const remaining = Math.max(0, totalEstimated - elapsed);
        const remainingSeconds = Math.round(remaining / 1000);

        const speedKBs = Math.round(download.speed || 0);
        const progressPercent = Math.round(download.progress || 0);

        const statusClass = download.status === 'completed' ? 'completed' : 
                           download.status === 'failed' ? 'failed' : 'downloading';

        div.innerHTML = `
            <div class="progress-header">
                <div class="progress-title">${this.escapeHtml(title)}</div>
                <div class="progress-platform">${platformLabel} • ${download.format} • ${download.quality || 'Auto'}</div>
            </div>
            <div class="progress-bar">
                <div class="progress-fill ${statusClass}" style="width: ${progressPercent}%"></div>
            </div>
            <div class="progress-info">
                <span class="progress-percentage">${progressPercent}%</span>
                <span class="progress-speed">${speedKBs} KB/s</span>
                <span class="progress-time">${download.status === 'completed' ? 'Completed' : `${remainingSeconds}s remaining`}</span>
            </div>
        `;

        return div;
    }

    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    addToHistory(download) {
        const historyItem = {
            id: Date.now(),
            downloadId: download.downloadId,
            url: download.url,
            platform: download.platform,
            format: download.format,
            quality: download.quality,
            title: download.metadata?.title || this.extractVideoTitle(download.url, download.platform),
            thumbnail: download.metadata?.thumbnail || this.generateThumbnailUrl(download.url),
            completedAt: download.completedAt || Date.now()
        };

        this.downloadHistory.unshift(historyItem);
        if (this.downloadHistory.length > 10) {
            this.downloadHistory = this.downloadHistory.slice(0, 10);
        }

        this.saveHistory();
        this.renderHistory();
    }

    extractVideoTitle(url, platformKey) {
        const platform = this.platforms.find(p => p.key === platformKey);
        const videoId = this.extractVideoId(url, platform);
        return `Video from ${platform?.label || 'Unknown'} (${videoId})`;
    }

    extractVideoId(url, platform) {
        if (!platform) return 'Unknown';
        
        for (const domain of platform.domains) {
            if (url.includes(domain)) {
                const parts = url.split('/');
                return parts[parts.length - 1].split('?')[0] || 'Unknown';
            }
        }
        return 'Unknown';
    }

    generateThumbnailUrl(url) {
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
                <div class="history-item-title">${this.escapeHtml(item.title)}</div>
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
            'instagram': 'fab fa-instagram',
            'reddit': 'fab fa-reddit'
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

        document.getElementById('platform').value = item.platform;
        document.getElementById('videoUrl').value = item.url;
        document.querySelector(`input[name="format"][value="${item.format}"]`).checked = true;
        this.updateQualityOptions();
        document.getElementById('quality').value = item.quality;

        document.querySelector('.download-section').scrollIntoView({ behavior: 'smooth' });
    }

    toggleFormState(disabled) {
        const form = document.getElementById('downloadForm');
        const button = document.getElementById('downloadBtn');
        const inputs = form.querySelectorAll('input, select');

        inputs.forEach(input => {
            input.disabled = disabled;
        });

        button.disabled = disabled;

        if (disabled) {
            button.innerHTML = '<span class="spinner"></span> Processing...';
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
        const messageDiv = document.createElement('div');
        messageDiv.className = `message message-${type} fade-in`;
        messageDiv.innerHTML = `
            <i class="fas fa-${type === 'error' ? 'exclamation-circle' : 'check-circle'}"></i>
            <span>${this.escapeHtml(message)}</span>
        `;

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

        document.body.appendChild(messageDiv);

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

document.addEventListener('DOMContentLoaded', () => {
    window.app = new VDownloader();
});
