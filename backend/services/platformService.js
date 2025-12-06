const config = require("../../config");

const getAllPlatforms = () => config.platforms;

const getSupportedPlatforms = () => config.platforms.filter((platform) => platform.enabled);

module.exports = {
  getAllPlatforms,
  getSupportedPlatforms
};
