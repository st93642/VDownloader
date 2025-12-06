require("dotenv").config();

const app = require("./app");
const config = require("../config");
const { setupWebSocket } = require("./websocket");

const startServer = (portToUse) => {
  const server = app.listen(portToUse, () => {
    console.log(`${config.app.name} running in ${config.app.env} mode on port ${portToUse}`);
  });

  setupWebSocket(server);

  server.once("error", (error) => {
    if (error.code === "EADDRINUSE") {
      const nextPort = portToUse + 1;
      console.warn(`Port ${portToUse} is in use, attempting to use ${nextPort} instead.`);
      startServer(nextPort);
      return;
    }

    throw error;
  });
};

startServer(config.app.port);
