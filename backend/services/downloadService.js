const crypto = require("crypto");
const { emitDownloadProgress, emitDownloadComplete, emitDownloadError } = require("../websocket");

const downloadStore = new Map();

const generateDownloadId = () => {
  return crypto.randomBytes(8).toString("hex");
};

const createDownload = (url, format, platform, downloadInfo = null) => {
  const downloadId = generateDownloadId();
  const download = {
    id: downloadId,
    url,
    format,
    platform,
    status: "pending",
    progress: 0,
    createdAt: new Date().toISOString(),
    startedAt: null,
    completedAt: null,
    error: null,
    downloadInfo,
    speed: 0,
    bytesDownloaded: 0,
    totalBytes: 0
  };
  downloadStore.set(downloadId, download);
  
  processDownload(downloadId);
  
  return download;
};

const getDownload = (downloadId) => {
  return downloadStore.get(downloadId);
};

const updateDownload = (downloadId, updates) => {
  const download = downloadStore.get(downloadId);
  if (!download) {
    return null;
  }
  const updated = { ...download, ...updates };
  downloadStore.set(downloadId, updated);
  return updated;
};

const cancelDownload = (downloadId) => {
  const download = downloadStore.get(downloadId);
  if (!download) {
    return null;
  }
  if (download.status === "completed" || download.status === "cancelled") {
    return download;
  }
  return updateDownload(downloadId, { status: "cancelled" });
};

const getAllDownloads = () => {
  return Array.from(downloadStore.values());
};

const removeDownload = (downloadId) => {
  return downloadStore.delete(downloadId);
};

const processDownload = async (downloadId) => {
  const download = downloadStore.get(downloadId);
  if (!download) return;

  try {
    updateDownload(downloadId, { 
      status: "downloading", 
      startedAt: new Date().toISOString() 
    });

    const totalBytes = 5 * 1024 * 1024 + Math.random() * 20 * 1024 * 1024;
    updateDownload(downloadId, { totalBytes });

    const steps = 40;
    const duration = 3000 + Math.random() * 4000;
    const stepDuration = duration / steps;

    for (let i = 0; i <= steps; i++) {
      const currentDownload = downloadStore.get(downloadId);
      
      if (!currentDownload || currentDownload.status === "cancelled") {
        console.log(`Download ${downloadId} was cancelled`);
        return;
      }

      await new Promise(resolve => setTimeout(resolve, stepDuration));

      const progress = Math.min(100, (i / steps) * 100);
      const bytesDownloaded = Math.floor((progress / 100) * totalBytes);
      const speed = 500 + Math.random() * 2500;

      const updated = updateDownload(downloadId, {
        progress,
        bytesDownloaded,
        speed
      });

      emitDownloadProgress(downloadId, {
        progress: updated.progress,
        speed: updated.speed,
        bytesDownloaded: updated.bytesDownloaded,
        totalBytes: updated.totalBytes,
        status: updated.status
      });
    }

    const completed = updateDownload(downloadId, {
      status: "completed",
      progress: 100,
      completedAt: new Date().toISOString()
    });

    emitDownloadComplete(downloadId, {
      status: completed.status,
      completedAt: completed.completedAt,
      downloadInfo: completed.downloadInfo
    });

  } catch (error) {
    console.error(`Download ${downloadId} failed:`, error);
    updateDownload(downloadId, {
      status: "failed",
      error: error.message
    });
    
    emitDownloadError(downloadId, error.message);
  }
};

module.exports = {
  createDownload,
  getDownload,
  updateDownload,
  cancelDownload,
  getAllDownloads,
  removeDownload,
  generateDownloadId
};
