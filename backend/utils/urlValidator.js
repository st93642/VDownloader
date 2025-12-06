const config = require("../../config");

const isValidUrl = (urlString) => {
  try {
    const url = new URL(urlString);
    return url.protocol === "http:" || url.protocol === "https:";
  } catch {
    return false;
  }
};

const getPlatformFromUrl = (urlString) => {
  try {
    const url = new URL(urlString);
    const hostname = url.hostname.replace("www.", "");

    const platform = config.platforms.find((p) =>
      p.domains.some((domain) => hostname.includes(domain))
    );

    return platform || null;
  } catch {
    return null;
  }
};

const isPlatformSupported = (platform) => {
  if (!platform) {
    return false;
  }
  return platform.enabled;
};

const validateUrl = (urlString) => {
  if (!isValidUrl(urlString)) {
    return {
      valid: false,
      error: "Invalid URL format"
    };
  }

  const platform = getPlatformFromUrl(urlString);

  if (!platform) {
    return {
      valid: false,
      error: "URL domain is not recognized"
    };
  }

  if (!isPlatformSupported(platform)) {
    return {
      valid: false,
      error: `Platform '${platform.label}' is not yet supported`
    };
  }

  return {
    valid: true,
    platform: platform.key,
    platformLabel: platform.label
  };
};

module.exports = {
  isValidUrl,
  getPlatformFromUrl,
  isPlatformSupported,
  validateUrl
};
