const BaseAdapter = require('./baseAdapter');
const axios = require('axios');
const cheerio = require('cheerio');

class InstagramAdapter extends BaseAdapter {
  extractVideoId(url) {
    try {
      const urlObj = new URL(url);
      const pathname = urlObj.pathname;
      
      if (pathname.includes('/p/') || pathname.includes('/reel/')) {
        const parts = pathname.split(/\/p\/|\/reel\//);
        return parts[1]?.split('?')[0]?.split('/')[0];
      }
      
      return null;
    } catch {
      return null;
    }
  }

  async getMetadata(url) {
    try {
      const postId = this.extractVideoId(url);
      if (!postId) {
        throw new Error('Invalid Instagram URL');
      }

      const response = await axios.get(url, {
        headers: {
          'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
        }
      });

      const $ = cheerio.load(response.data);
      const scripts = $('script').toArray();
      
      let videoData = null;
      for (const script of scripts) {
        const scriptContent = $(script).html();
        if (scriptContent && scriptContent.includes('window._sharedData')) {
          const match = scriptContent.match(/window\._sharedData\s*=\s*({.+?});/);
          if (match) {
            try {
              const data = JSON.parse(match[1]);
              const mediaData = data?.entry_data?.PostPage?.[0]?.graphql?.shortcode_media;
              if (mediaData && mediaData.is_video) {
                videoData = mediaData;
                break;
              }
            } catch (e) {
              continue;
            }
          }
        }
      }

      if (!videoData) {
        throw new Error('Could not find video data or post is not a video');
      }

      return {
        title: videoData.edge_media_to_caption?.edges?.[0]?.node?.text || 'Instagram Video',
        duration: videoData.video_duration || 0,
        uploader: videoData.owner?.username || 'Unknown',
        description: videoData.edge_media_to_caption?.edges?.[0]?.node?.text || '',
        thumbnail: videoData.display_url || '',
        viewCount: videoData.video_view_count || 0,
        uploadDate: new Date(parseInt(videoData.taken_at_timestamp) * 1000).toISOString(),
        videoId: postId
      };
    } catch (error) {
      throw new Error(`Failed to extract metadata: ${error.message}`);
    }
  }

  async getDownloadInfo(url, format = 'video', quality = '720p') {
    try {
      const postId = this.extractVideoId(url);
      if (!postId) {
        throw new Error('Invalid Instagram URL');
      }

      const response = await axios.get(url, {
        headers: {
          'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
        }
      });

      const $ = cheerio.load(response.data);
      const scripts = $('script').toArray();
      
      let videoData = null;
      for (const script of scripts) {
        const scriptContent = $(script).html();
        if (scriptContent && scriptContent.includes('window._sharedData')) {
          const match = scriptContent.match(/window\._sharedData\s*=\s*({.+?});/);
          if (match) {
            try {
              const data = JSON.parse(match[1]);
              const mediaData = data?.entry_data?.PostPage?.[0]?.graphql?.shortcode_media;
              if (mediaData && mediaData.is_video) {
                videoData = mediaData;
                break;
              }
            } catch (e) {
              continue;
            }
          }
        }
      }

      if (!videoData) {
        throw new Error('Could not find video data or post is not a video');
      }

      let downloadUrl;
      if (format === 'audio') {
        downloadUrl = videoData.video_url;
      } else {
        downloadUrl = videoData.video_url;
      }

      if (!downloadUrl) {
        throw new Error('Could not find download URL');
      }

      return {
        url: downloadUrl,
        format: format === 'audio' ? 'audio/mp4' : 'video/mp4',
        quality: quality,
        size: null,
        container: 'mp4',
        codecs: format === 'audio' ? 'aac' : 'h264,aac'
      };
    } catch (error) {
      throw new Error(`Failed to get download info: ${error.message}`);
    }
  }

  async getStream(url, format = 'video', quality = '720p') {
    try {
      const downloadInfo = await this.getDownloadInfo(url, format, quality);
      const response = await axios.get(downloadInfo.url, {
        responseType: 'stream',
        headers: {
          'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36',
          'Referer': 'https://www.instagram.com/'
        }
      });
      
      return response.data;
    } catch (error) {
      throw new Error(`Failed to get stream: ${error.message}`);
    }
  }
}

module.exports = InstagramAdapter;