const YouTubeAdapter = require('./youtubeAdapter');
const TikTokAdapter = require('./tiktokAdapter');
const TwitterAdapter = require('./twitterAdapter');
const InstagramAdapter = require('./instagramAdapter');
const RedditAdapter = require('./redditAdapter');

class AdapterFactory {
  static adapters = new Map([
    ['youtube', YouTubeAdapter],
    ['tiktok', TikTokAdapter],
    ['twitter', TwitterAdapter],
    ['instagram', InstagramAdapter],
    ['reddit', RedditAdapter]
  ]);

  static getAdapter(platform) {
    const AdapterClass = this.adapters.get(platform);
    if (!AdapterClass) {
      throw new Error(`Unsupported platform: ${platform}`);
    }
    return new AdapterClass();
  }

  static getSupportedPlatforms() {
    return Array.from(this.adapters.keys());
  }

  static isPlatformSupported(platform) {
    return this.adapters.has(platform);
  }
}

module.exports = AdapterFactory;