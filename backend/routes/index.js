const { Router } = require("express");
const healthRoutes = require("./healthRoutes");
const platformRoutes = require("./platformRoutes");
const downloadRoutes = require("./downloadRoutes");

const API_PREFIX = "/api";

const applyRoutes = (app) => {
  const apiRouter = Router();

  apiRouter.use("/health", healthRoutes);
  apiRouter.use("/platforms", platformRoutes);
  apiRouter.use(downloadRoutes);

  app.use(API_PREFIX, apiRouter);
};

module.exports = applyRoutes;
