const withPlugins = require("next-compose-plugins");
const optimizedImages = require("next-optimized-images");

const nextConfig = {
  webpack: (config, options) => {
    return config;
  },
  reactStrictMode: true,
  images: {
    disableStaticImages: true,
    // fake the third-party loader
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
