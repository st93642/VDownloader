const supportedPlatforms = [
  {
    key: "youtube",
    label: "YouTube",
    domains: ["youtube.com", "youtu.be"],
    enabled: true,
    downloader: "ytdl-core",
    supports: ["video", "audio"],
    qualityOptions: ["144p", "240p", "360p", "480p", "720p", "1080p"]
  },
  {
    key: "tiktok",
    label: "TikTok",
    domains: ["tiktok.com", "vm.tiktok.com"],
    enabled: true,
    downloader: "tiktok",
    supports: ["video", "audio"],
    qualityOptions: ["360p", "480p", "720p", "1080p"]
  },
  {
    key: "twitter",
    label: "X/Twitter",
    domains: ["twitter.com", "x.com"],
    enabled: true,
    downloader: "twitter",
    supports: ["video", "audio"],
    qualityOptions: ["360p", "480p", "720p", "1080p"]
  },
  {
    key: "instagram",
    label: "Instagram",
    domains: ["instagram.com", "instagr.am"],
    enabled: true,
    downloader: "instagram",
    supports: ["video", "audio"],
    qualityOptions: ["360p", "480p", "720p", "1080p"]
  },
  {
    key: "reddit",
    label: "Reddit",
    domains: ["reddit.com", "redd.it"],
    enabled: true,
    downloader: "reddit",
    supports: ["video", "audio"],
    qualityOptions: ["360p", "480p", "720p", "1080p"]
  },
  {
    key: "vimeo",
    label: "Vimeo",
    domains: ["vimeo.com"],
    enabled: false,
    downloader: "planned",
    supports: ["video"],
    qualityOptions: ["360p", "480p", "720p"],
    notes: "Placeholder configuration to illustrate upcoming platform support"
  }
];

module.exports = supportedPlatforms;
