// const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
  purge: {
    enabled: process.env.NODE_ENV === "production",
    mode: "all",
    content: [
      "../src/**/*.rs",
      "../public/index.html",
      "./tailwind.css",
    ]
  },
  darkMode: false,
  theme: {
    extend: {
      // fontFamily: {
      //   "sans": ["Source Sans Pro", ...defaultTheme.fontFamily.sans],
      //   "serif": ["Source Serif Pro", ...defaultTheme.fontFamily.serif],
      //   "mono": ["Source Code Pro", ...defaultTheme.fontFamily.mono],
      // },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
