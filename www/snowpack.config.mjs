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
    entrypoints: ["src/login.jsx"],
    bundle: true,
    minify: true,
    treeshake: true,
    target: "es2018",
  },
  plugins: [
    "@snowpack/plugin-postcss",
  ],
};
