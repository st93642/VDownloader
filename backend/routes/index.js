const { Router } = require("express");
const healthRoutes = require("./healthRoutes");
const platformRoutes = require("./platformRoutes");

const API_PREFIX = "/api";

const applyRoutes = (app) => {
  const apiRouter = Router();

  apiRouter.use("/health", healthRoutes);
  apiRouter.use("/platforms", platformRoutes);

  app.use(API_PREFIX, apiRouter);
};

module.exports = applyRoutes;
