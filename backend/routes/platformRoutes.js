const { Router } = require("express");
const platformController = require("../controllers/platformController");

const router = Router();

router.get("/", platformController.listPlatforms);
router.get("/supported", platformController.listSupportedPlatforms);

module.exports = router;
