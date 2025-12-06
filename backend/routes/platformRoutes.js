const { Router } = require("express");
const platformController = require("../controllers/platformController");

const router = Router();

router.get("/", platformController.listPlatforms);
router.get("/supported", platformController.listSupportedPlatforms);
router.get("/capabilities", platformController.getPlatformCapabilities);

module.exports = router;
