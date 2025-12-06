const BaseAdapter = require('./baseAdapter');
const axios = require('axios');
const cheerio = require('cheerio');

class TikTokAdapter extends BaseAdapter {
  extractVideoId(url) {
    try {
      const urlObj = new URL(url);
      const pathname = urlObj.pathname;
      
      if (pathname.includes('/video/')) {
        const parts = pathname.split('/video/');
        return parts[1]?.split('?')[0]?.split('/')[0];
      }
      
      if (pathname.includes('/t/')) {
        return pathname.split('/t/')[1]?.split('?')[0]?.split('/')[0];
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
        throw new Error('Invalid TikTok URL');
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
        if (scriptContent && scriptContent.includes('__NEXT_DATA__')) {
          const match = scriptContent.match(/__NEXT_DATA__\s*=\s*({.+?});/);
          if (match) {
            try {
              const data = JSON.parse(match[1]);
              videoData = data?.props?.pageProps?.itemInfo?.itemStruct;
              break;
            } catch (e) {
              continue;
            }
          }
        }
      }

      if (!videoData) {
        throw new Error('Could not extract video data');
      }

      return {
        title: videoData.desc || 'TikTok Video',
        duration: videoData.video?.duration || 0,
        uploader: videoData.author?.uniqueId || 'Unknown',
        description: videoData.desc || '',
        thumbnail: videoData.video?.cover || '',
        viewCount: videoData.stats?.playCount || 0,
        uploadDate: new Date(videoData.createTime * 1000).toISOString(),
        videoId: videoId
      };
    } catch (error) {
      throw new Error(`Failed to extract metadata: ${error.message}`);
    }
  }

  async getDownloadInfo(url, format = 'video', quality = '720p') {
    try {
      const videoId = this.extractVideoId(url);
      if (!videoId) {
        throw new Error('Invalid TikTok URL');
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
        if (scriptContent && scriptContent.includes('__NEXT_DATA__')) {
          const match = scriptContent.match(/__NEXT_DATA__\s*=\s*({.+?});/);
          if (match) {
            try {
              const data = JSON.parse(match[1]);
              videoData = data?.props?.pageProps?.itemInfo?.itemStruct;
              break;
            } catch (e) {
              continue;
            }
          }
        }
      }

      if (!videoData) {
        throw new Error('Could not extract video data');
      }

      let downloadUrl;
      if (format === 'audio') {
        downloadUrl = videoData.video?.downloadAddr;
      } else {
        downloadUrl = videoData.video?.playAddr || videoData.video?.downloadAddr;
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
          'Referer': 'https://www.tiktok.com/'
        }
      });
      
      return response.data;
    } catch (error) {
      throw new Error(`Failed to get stream: ${error.message}`);
    }
  }
}

module.exports = TikTokAdapter;