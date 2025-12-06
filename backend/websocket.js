const { Server } = require("socket.io");

let io;

const setupWebSocket = (server) => {
  io = new Server(server, {
    cors: {
      origin: "*",
      methods: ["GET", "POST"]
    }
  });

  io.on("connection", (socket) => {
    console.log(`Client connected: ${socket.id}`);

    socket.on("subscribe", (downloadId) => {
      socket.join(`download:${downloadId}`);
      console.log(`Client ${socket.id} subscribed to download ${downloadId}`);
    });

    socket.on("unsubscribe", (downloadId) => {
      socket.leave(`download:${downloadId}`);
      console.log(`Client ${socket.id} unsubscribed from download ${downloadId}`);
    });

    socket.on("disconnect", () => {
      console.log(`Client disconnected: ${socket.id}`);
    });
  });

  return io;
};

const getIO = () => {
  if (!io) {
    throw new Error("Socket.io not initialized");
  }
  return io;
};

const emitDownloadProgress = (downloadId, data) => {
  if (io) {
    io.to(`download:${downloadId}`).emit("download:progress", {
      downloadId,
      ...data
    });
  }
};

const emitDownloadComplete = (downloadId, data) => {
  if (io) {
    io.to(`download:${downloadId}`).emit("download:complete", {
      downloadId,
      ...data
    });
  }
};

const emitDownloadError = (downloadId, error) => {
  if (io) {
    io.to(`download:${downloadId}`).emit("download:error", {
      downloadId,
      error
    });
  }
};

module.exports = {
  setupWebSocket,
  getIO,
  emitDownloadProgress,
  emitDownloadComplete,
  emitDownloadError
};
