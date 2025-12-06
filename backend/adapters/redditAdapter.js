const BaseAdapter = require('./baseAdapter');
const axios = require('axios');
const cheerio = require('cheerio');

class RedditAdapter extends BaseAdapter {
  extractVideoId(url) {
    try {
      const urlObj = new URL(url);
      const pathname = urlObj.pathname;
      
      if (pathname.includes('/comments/')) {
        const parts = pathname.split('/comments/');
        return parts[1]?.split('?')[0]?.split('/')[0];
      }
      
      if (urlObj.hostname.includes('redd.it') && pathname.length > 1) {
        return pathname.slice(1).split('?')[0]?.split('/')[0];
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
        throw new Error('Invalid Reddit URL');
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
        if (scriptContent && scriptContent.includes('__r')) {
          const match = scriptContent.match(/window\.__r\s*=\s*({.+?});/);
          if (match) {
            try {
              const data = JSON.parse(match[1]);
              const posts = Object.values(data).find(obj => obj?.posts?.models);
              if (posts && posts.models) {
                const post = Object.values(posts.models)[0];
                if (post?.media?.type === 'video') {
                  videoData = post;
                  break;
                }
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
        title: videoData.title || 'Reddit Video',
        duration: videoData.media?.duration || 0,
        uploader: videoData.author || 'Unknown',
        description: videoData.selftext || '',
        thumbnail: videoData.media?.posterUrl || '',
        viewCount: videoData.viewCount || 0,
        uploadDate: new Date(videoData.created * 1000).toISOString(),
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
        throw new Error('Invalid Reddit URL');
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
        if (scriptContent && scriptContent.includes('__r')) {
          const match = scriptContent.match(/window\.__r\s*=\s*({.+?});/);
          if (match) {
            try {
              const data = JSON.parse(match[1]);
              const posts = Object.values(data).find(obj => obj?.posts?.models);
              if (posts && posts.models) {
                const post = Object.values(posts.models)[0];
                if (post?.media?.type === 'video') {
                  videoData = post;
                  break;
                }
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
        downloadUrl = videoData.media?.audioUrl;
      } else {
        downloadUrl = videoData.media?.hlsUrl || videoData.media?.dashUrl;
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
          'Referer': 'https://www.reddit.com/'
        }
      });
      
      return response.data;
    } catch (error) {
      throw new Error(`Failed to get stream: ${error.message}`);
    }
  }
}

module.exports = RedditAdapter;