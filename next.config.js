const withPlugins = require("next-compose-plugins");
const optimizedImages = require("next-optimized-images");

const isProd = process.env.NODE_ENV === "production";

const nextConfig = {
  webpack: (config, options) => {
    return config;
  },
  reactStrictMode: true,
  assetPrefix: isProd ? "https://geoffjay.github.io/" : "",
  images: {
    loader: "imgix",
    path: "",
  },
};

const config = withPlugins(
  [
    [
      optimizedImages,
      {
        optimizeImages: false,
      },
    ],
  ],
  nextConfig,
);

module.exports = config;
