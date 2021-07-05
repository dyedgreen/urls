export default {
  root: "src",
  alias: {
    "picoql": "./src/picoql.js",
    "@app": "./src/components",
  },
  devOptions: {
    tailwindConfig: "./tailwind.config.js",
  },
  buildOptions: {
    out: "static/dist",
  },
  optimize: {
    entrypoints: [
      "src/account.jsx",
      "src/comments.jsx",
      "src/header.jsx",
      "src/login.jsx",
      "src/register.jsx",
      "src/search.jsx",
    ],
    bundle: true,
    minify: true,
    treeshake: true,
    target: "es2018",
  },
  plugins: [
    "@snowpack/plugin-postcss",
  ],
};
