const BaseAdapter = require('./baseAdapter');
const ytdlCore = require('@distube/ytdl-core');

class YouTubeAdapter extends BaseAdapter {
  extractVideoId(url) {
    try {
      const urlObj = new URL(url);
      
      if (urlObj.hostname.includes('youtu.be')) {
        return urlObj.pathname.slice(1);
      }
      
      if (urlObj.hostname.includes('youtube.com')) {
        const searchParams = urlObj.searchParams;
        return searchParams.get('v');
      }
      
      return null;
    } catch {
      return null;
    }
  }

  async getMetadata(url) {
    try {
      const videoId = this.extractVideoId(url);
      if (!videoId) {
        throw new Error('Invalid YouTube URL');
      }

      const info = await ytdlCore.getInfo(url);
      
      return {
        title: info.videoDetails.title,
        duration: parseInt(info.videoDetails.lengthSeconds),
        uploader: info.videoDetails.author.name,
        description: info.videoDetails.description,
        thumbnail: info.videoDetails.thumbnails[info.videoDetails.thumbnails.length - 1]?.url,
        viewCount: parseInt(info.videoDetails.viewCount),
        uploadDate: info.videoDetails.uploadDate,
        videoId: videoId
      };
    } catch (error) {
      throw new Error(`Failed to extract metadata: ${error.message}`);
    }
  }

  async getDownloadInfo(url, format = 'video', quality = '720p') {
    try {
      const info = await ytdlCore.getInfo(url);
      let formats = info.formats;

      if (format === 'audio') {
        formats = formats.filter(f => f.hasAudio && !f.hasVideo);
      } else {
        formats = formats.filter(f => f.hasVideo);
      }

      const qualityMap = {
        '144p': 'tiny',
        '240p': 'small',
        '360p': 'medium',
        '480p': 'large',
        '720p': 'hd720',
        '1080p': 'hd1080'
      };

      const targetQuality = qualityMap[quality] || 'medium';
      
      let selectedFormat = formats.find(f => 
        f.qualityLabel && f.qualityLabel.toLowerCase().includes(quality.toLowerCase())
      ) || formats.find(f => f.quality === targetQuality) || formats[0];

      if (!selectedFormat) {
        throw new Error('No suitable format found');
      }

      return {
        url: selectedFormat.url,
        format: selectedFormat.mimeType,
        quality: selectedFormat.qualityLabel || quality,
        size: selectedFormat.contentLength,
        container: selectedFormat.container,
        codecs: selectedFormat.codecs
      };
    } catch (error) {
      throw new Error(`Failed to get download info: ${error.message}`);
    }
  }

  async getStream(url, format = 'video', quality = '720p') {
    try {
      const downloadInfo = await this.getDownloadInfo(url, format, quality);
      const response = await fetch(downloadInfo.url);
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      return response.body;
    } catch (error) {
      throw new Error(`Failed to get stream: ${error.message}`);
    }
  }

  getSupportedQualities() {
    return ["144p", "240p", "360p", "480p", "720p", "1080p"];
  }
}

module.exports = YouTubeAdapter;