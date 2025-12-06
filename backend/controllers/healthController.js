const pkg = require("../../package.json");
const config = require("../../config");

const getHealthStatus = (req, res) => {
  res.json({
    status: "ok",
    service: config.app.name,
    version: pkg.version,
    environment: config.app.env,
    timestamp: new Date().toISOString(),
    supportedPlatforms: config.platforms.filter((platform) => platform.enabled).map((platform) => platform.key)
  });
};

module.exports = {
  getHealthStatus
};
