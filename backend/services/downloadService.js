const crypto = require("crypto");

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
    downloadInfo
  };
  downloadStore.set(downloadId, download);
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

module.exports = {
  createDownload,
  getDownload,
  updateDownload,
  cancelDownload,
  getAllDownloads,
  removeDownload,
  generateDownloadId
};
