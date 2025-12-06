const rateLimit = require("express-rate-limit");

const createLimiter = (windowMs = 15 * 60 * 1000, max = 30, message = "Too many requests") => {
  return rateLimit({
    windowMs,
    max,
    message,
    standardHeaders: true,
    legacyHeaders: false,
    handler: (req, res) => {
      res.status(429).json({
        success: false,
        error: {
          message,
          code: "RATE_LIMIT_EXCEEDED"
        }
      });
    }
  });
};

const downloadLimiter = createLimiter(
  60 * 60 * 1000, // 1 hour window
  10, // 10 requests per hour
  "Download limit exceeded. Maximum 10 downloads per hour."
);

const validateLimiter = createLimiter(
  60 * 1000, // 1 minute window
  30, // 30 requests per minute
  "Validation limit exceeded. Maximum 30 validations per minute."
);

const statusLimiter = createLimiter(
  60 * 1000, // 1 minute window
  100, // 100 requests per minute
  "Status check limit exceeded."
);

module.exports = {
  createLimiter,
  downloadLimiter,
  validateLimiter,
  statusLimiter
};
