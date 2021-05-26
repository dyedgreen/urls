// Snowpack Configuration File

export default {
  root: "src",
  alias: {
    "picoql": "./src/picoql.js",
    "@app": "./src/components",
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
};
