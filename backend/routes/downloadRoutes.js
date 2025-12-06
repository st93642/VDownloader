const { Router } = require("express");
const downloadController = require("../controllers/downloadController");
const { downloadLimiter, validateLimiter, statusLimiter } = require("../middleware/rateLimiter");

const router = Router();

router.post("/validate", validateLimiter, downloadController.validateRequest);
router.post("/download", downloadLimiter, downloadController.initiateDownload);
router.get("/status/:downloadId", statusLimiter, downloadController.getDownloadStatus);
router.delete("/cancel/:downloadId", downloadLimiter, downloadController.cancelDownload);
router.get("/formats/:platform", statusLimiter, downloadController.getFormats);

module.exports = router;
