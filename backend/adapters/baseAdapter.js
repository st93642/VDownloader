class BaseAdapter {
  constructor() {
    if (this.constructor === BaseAdapter) {
      throw new Error("BaseAdapter is an abstract class and cannot be instantiated directly");
    }
  }

  async getMetadata(url) {
    throw new Error("getMetadata method must be implemented by subclass");
  }

  async getDownloadInfo(url, format, quality) {
    throw new Error("getDownloadInfo method must be implemented by subclass");
  }

  async getStream(url, format, quality) {
    throw new Error("getStream method must be implemented by subclass");
  }

  validateUrl(url) {
    try {
      new URL(url);
      return true;
    } catch {
      return false;
    }
  }

  extractVideoId(url) {
    throw new Error("extractVideoId method must be implemented by subclass");
  }

  getSupportedFormats() {
    return ["video", "audio"];
  }

  getSupportedQualities() {
    return ["360p", "480p", "720p", "1080p"];
  }
}

module.exports = BaseAdapter;