const supportedPlatforms = require("./platforms");

const toNumber = (value, fallback) => {
  const parsed = Number(value);
  return Number.isNaN(parsed) ? fallback : parsed;
};

const config = {
  app: {
    name: "VDownloader",
    env: process.env.NODE_ENV || "development",
    port: toNumber(process.env.PORT, 4000),
    baseUrl: process.env.APP_BASE_URL || "http://localhost:4000"
  },
  downloader: {
    tempDir: process.env.DOWNLOAD_TEMP_DIR || ".tmp/downloads",
    maxConcurrentJobs: toNumber(process.env.MAX_CONCURRENT_JOBS, 2)
  },
  platforms: supportedPlatforms
};

module.exports = config;
