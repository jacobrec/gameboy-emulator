const {GenerateSW} = require('workbox-webpack-plugin');
const { WebpackManifestPlugin } = require('webpack-manifest-plugin');
const path = require('path');


module.exports = function override(config, env) {
  const wasmExtensionRegExp = /\.wasm$/;

  config.resolve.extensions.push('.wasm');

  config.module.rules.forEach(rule => {
    (rule.oneOf || []).forEach(oneOf => {
      if (oneOf.loader && oneOf.loader.indexOf('file-loader') >= 0) {
        // make file-loader ignore WASM files
        oneOf.exclude.push(wasmExtensionRegExp);
      }
    });
  });

  // add a dedicated loader for WASM
  config.module.rules.push({
    test: wasmExtensionRegExp,
    include: path.resolve(__dirname, 'src'),
    use: [{ loader: require.resolve('wasm-loader'), options: {} }]
  });

  config.plugins.push(new WebpackManifestPlugin({
    "fileName": "assets-manifest.json",
    "writeToFileEmit": true,
  }));
  if (process.env.NODE_ENV != "development") {
    config.plugins.push(new GenerateSW({}));
  }
  return config;
};
