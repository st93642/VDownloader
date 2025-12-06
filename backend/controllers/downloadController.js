const downloadService = require("../services/downloadService");
const platformService = require("../services/platformService");
const platformDownloadService = require("../services/platformDownloadService");
const { validateUrl } = require("../utils/urlValidator");
const { APIError } = require("../middleware/errorHandler");

const validateRequest = async (req, res) => {
  const { url } = req.body;

  if (!url) {
    return res.status(400).json({
      success: false,
      error: {
        message: "URL is required",
        code: "MISSING_URL"
      }
    });
  }

  const validation = validateUrl(url);

  if (!validation.valid) {
    return res.status(400).json({
      success: false,
      error: {
        message: validation.error,
        code: "INVALID_URL"
      }
    });
  }

  try {
    const platformValidation = await platformDownloadService.validateAndExtract(url, validation.platform);
    
    return res.json({
      success: true,
      data: {
        valid: true,
        url,
        platform: validation.platform,
        platformLabel: validation.platformLabel,
        metadata: platformValidation.metadata,
        supportedFormats: platformValidation.supportedFormats,
        supportedQualities: platformValidation.supportedQualities
      }
    });
  } catch (error) {
    return res.status(error.statusCode || 500).json({
      success: false,
      error: {
        message: error.message,
        code: error.code || "VALIDATION_ERROR"
      }
    });
  }
};

const initiateDownload = async (req, res) => {
  const { url, format, quality } = req.body;

  if (!url) {
    return res.status(400).json({
      success: false,
      error: {
        message: "URL is required",
        code: "MISSING_URL"
      }
    });
  }

  const validation = validateUrl(url);

  if (!validation.valid) {
    return res.status(400).json({
      success: false,
      error: {
        message: validation.error,
        code: "INVALID_URL"
      }
    });
  }

  if (format && !["video", "audio"].includes(format)) {
    return res.status(400).json({
      success: false,
      error: {
        message: "Invalid format. Must be 'video' or 'audio'",
        code: "INVALID_FORMAT"
      }
    });
  }

  try {
    const downloadInfo = await platformDownloadService.getDownloadInfo(url, validation.platform, format || "video", quality);
    const download = downloadService.createDownload(url, format || "video", validation.platform, downloadInfo);

    return res.status(202).json({
      success: true,
      data: {
        downloadId: download.id,
        status: download.status,
        url: download.url,
        format: download.format,
        platform: download.platform,
        quality: quality,
        downloadInfo: download.downloadInfo,
        createdAt: download.createdAt
      }
    });
  } catch (error) {
    return res.status(error.statusCode || 500).json({
      success: false,
      error: {
        message: error.message,
        code: error.code || "DOWNLOAD_INIT_ERROR"
      }
    });
  }
};

const getDownloadStatus = (req, res) => {
  const { downloadId } = req.params;

  const download = downloadService.getDownload(downloadId);

  if (!download) {
    return res.status(404).json({
      success: false,
      error: {
        message: "Download not found",
        code: "DOWNLOAD_NOT_FOUND"
      }
    });
  }

  return res.json({
    success: true,
    data: {
      downloadId: download.id,
      status: download.status,
      progress: download.progress,
      url: download.url,
      format: download.format,
      platform: download.platform,
      createdAt: download.createdAt,
      startedAt: download.startedAt,
      completedAt: download.completedAt,
      error: download.error
    }
  });
};

const cancelDownload = (req, res) => {
  const { downloadId } = req.params;

  const download = downloadService.getDownload(downloadId);

  if (!download) {
    return res.status(404).json({
      success: false,
      error: {
        message: "Download not found",
        code: "DOWNLOAD_NOT_FOUND"
      }
    });
  }

  if (download.status === "completed") {
    return res.status(400).json({
      success: false,
      error: {
        message: "Cannot cancel a completed download",
        code: "INVALID_STATE"
      }
    });
  }

  if (download.status === "cancelled") {
    return res.status(400).json({
      success: false,
      error: {
        message: "Download is already cancelled",
        code: "INVALID_STATE"
      }
    });
  }

  const cancelled = downloadService.cancelDownload(downloadId);

  return res.json({
    success: true,
    data: {
      downloadId: cancelled.id,
      status: cancelled.status,
      url: cancelled.url,
      cancelledAt: new Date().toISOString()
    }
  });
};

const getFormats = (req, res) => {
  const { platform } = req.params;

  const platformConfig = platformService.getSupportedPlatforms().find((p) => p.key === platform);

  if (!platformConfig) {
    return res.status(404).json({
      success: false,
      error: {
        message: `Platform '${platform}' is not supported`,
        code: "PLATFORM_NOT_SUPPORTED"
      }
    });
  }

  return res.json({
    success: true,
    data: {
      platform: platformConfig.key,
      label: platformConfig.label,
      supports: platformConfig.supports,
      qualityOptions: platformConfig.qualityOptions
    }
  });
};

const getMetadata = async (req, res) => {
  const { url } = req.body;

  if (!url) {
    return res.status(400).json({
      success: false,
      error: {
        message: "URL is required",
        code: "MISSING_URL"
      }
    });
  }

  const validation = validateUrl(url);

  if (!validation.valid) {
    return res.status(400).json({
      success: false,
      error: {
        message: validation.error,
        code: "INVALID_URL"
      }
    });
  }

  try {
    const metadata = await platformDownloadService.getMetadata(url, validation.platform);
    
    return res.json({
      success: true,
      data: {
        url,
        platform: validation.platform,
        platformLabel: validation.platformLabel,
        metadata
      }
    });
  } catch (error) {
    return res.status(error.statusCode || 500).json({
      success: false,
      error: {
        message: error.message,
        code: error.code || "METADATA_ERROR"
      }
    });
  }
};

module.exports = {
  validateRequest,
  initiateDownload,
  getDownloadStatus,
  cancelDownload,
  getFormats,
  getMetadata
};
