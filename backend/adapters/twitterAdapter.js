const BaseAdapter = require('./baseAdapter');
const axios = require('axios');
const cheerio = require('cheerio');

class TwitterAdapter extends BaseAdapter {
  extractVideoId(url) {
    try {
      const urlObj = new URL(url);
      const pathname = urlObj.pathname;
      
      if (pathname.includes('/status/')) {
        const parts = pathname.split('/status/');
        return parts[1]?.split('?')[0]?.split('/')[0];
      }
      
      return null;
    } catch {
      return null;
    }
  }

  async getMetadata(url) {
    try {
      const tweetId = this.extractVideoId(url);
      if (!tweetId) {
        throw new Error('Invalid Twitter/X URL');
      }

      const response = await axios.get(url, {
        headers: {
          'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
        }
      });

      const $ = cheerio.load(response.data);
      
      const title = $('meta[property="og:title"]').attr('content') || 'Twitter Video';
      const description = $('meta[property="og:description"]').attr('content') || '';
      const thumbnail = $('meta[property="og:image"]').attr('content') || '';
      
      const authorElement = $('[data-testid="User-Name"] span').first();
      const uploader = authorElement.text().trim() || 'Unknown';

      return {
        title: title,
        duration: 0,
        uploader: uploader,
        description: description,
        thumbnail: thumbnail,
        viewCount: 0,
        uploadDate: new Date().toISOString(),
        videoId: tweetId
      };
    } catch (error) {
      throw new Error(`Failed to extract metadata: ${error.message}`);
    }
  }

  async getDownloadInfo(url, format = 'video', quality = '720p') {
    try {
      const tweetId = this.extractVideoId(url);
      if (!tweetId) {
        throw new Error('Invalid Twitter/X URL');
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
        if (scriptContent && scriptContent.includes('video_url')) {
          try {
            const videoUrlMatch = scriptContent.match(/video_url":"([^"]+)"/);
            if (videoUrlMatch) {
              videoData = {
                url: videoUrlMatch[1].replace(/\\u002F/g, '/')
              };
              break;
            }
          } catch (e) {
            continue;
          }
        }
      }

      if (!videoData) {
        throw new Error('Could not find video data');
      }

      return {
        url: videoData.url,
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
          'Referer': 'https://twitter.com/'
        }
      });
      
      return response.data;
    } catch (error) {
      throw new Error(`Failed to get stream: ${error.message}`);
    }
  }
}

module.exports = TwitterAdapter;