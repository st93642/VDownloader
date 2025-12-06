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
