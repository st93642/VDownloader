const AdapterFactory = require('../adapters/adapterFactory');
const { APIError } = require('../middleware/errorHandler');

class PlatformDownloadService {
  async getMetadata(url, platform) {
    try {
      const adapter = AdapterFactory.getAdapter(platform);
      return await adapter.getMetadata(url);
    } catch (error) {
      throw new APIError(`Failed to extract metadata: ${error.message}`, 'METADATA_EXTRACTION_ERROR', 500);
    }
  }

  async getDownloadInfo(url, platform, format = 'video', quality = '720p') {
    try {
      const adapter = AdapterFactory.getAdapter(platform);
      return await adapter.getDownloadInfo(url, format, quality);
    } catch (error) {
      throw new APIError(`Failed to get download info: ${error.message}`, 'DOWNLOAD_INFO_ERROR', 500);
    }
  }

  async getStream(url, platform, format = 'video', quality = '720p') {
    try {
      const adapter = AdapterFactory.getAdapter(platform);
      return await adapter.getStream(url, format, quality);
    } catch (error) {
      throw new APIError(`Failed to get download stream: ${error.message}`, 'STREAM_ERROR', 500);
    }
  }

  async validateAndExtract(url, platform) {
    try {
      const adapter = AdapterFactory.getAdapter(platform);
      
      if (!adapter.validateUrl(url)) {
        throw new APIError('Invalid URL format', 'INVALID_URL', 400);
      }

      const videoId = adapter.extractVideoId(url);
      if (!videoId) {
        throw new APIError('Could not extract video ID from URL', 'INVALID_VIDEO_ID', 400);
      }

      const metadata = await adapter.getMetadata(url);
      
      return {
        valid: true,
        videoId,
        metadata,
        supportedFormats: adapter.getSupportedFormats(),
        supportedQualities: adapter.getSupportedQualities()
      };
    } catch (error) {
      if (error instanceof APIError) {
        throw error;
      }
      throw new APIError(`Validation failed: ${error.message}`, 'VALIDATION_ERROR', 500);
    }
  }

  getSupportedPlatforms() {
    return AdapterFactory.getSupportedPlatforms();
  }

  isPlatformSupported(platform) {
    return AdapterFactory.isPlatformSupported(platform);
  }
}

module.exports = new PlatformDownloadService();