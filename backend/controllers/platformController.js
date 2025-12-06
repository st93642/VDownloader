const platformService = require("../services/platformService");
const platformDownloadService = require("../services/platformDownloadService");

const listPlatforms = (req, res) => {
  res.json({
    success: true,
    data: platformService.getAllPlatforms()
  });
};

const listSupportedPlatforms = (req, res) => {
  res.json({
    success: true,
    data: platformService.getSupportedPlatforms()
  });
};

const getPlatformCapabilities = (req, res) => {
  const platformConfigs = platformService.getSupportedPlatforms();
  const supportedPlatforms = platformDownloadService.getSupportedPlatforms();
  
  const capabilities = platformConfigs.map(platform => ({
    ...platform,
    isSupported: supportedPlatforms.includes(platform.key)
  }));

  res.json({
    success: true,
    data: capabilities
  });
};

module.exports = {
  listPlatforms,
  listSupportedPlatforms,
  getPlatformCapabilities
};
