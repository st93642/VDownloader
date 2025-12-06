const platformService = require("../services/platformService");

const listPlatforms = (req, res) => {
  res.json({
    data: platformService.getAllPlatforms()
  });
};

const listSupportedPlatforms = (req, res) => {
  res.json({
    data: platformService.getSupportedPlatforms()
  });
};

module.exports = {
  listPlatforms,
  listSupportedPlatforms
};
