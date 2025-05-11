const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: './index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
  },
  module: {
    rules: [
      {
        test: /\.s[ac]ss$/i, 
        use: ['style-loader', 'css-loader', 'sass-loader'],
        exclude: /node_modules/
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: 'index.html',
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "."),
      args: '--verbose'
    }),
  ],
  devServer: {
    port: 7001
  },
  mode: 'production', // development
  experiments: {
    asyncWebAssembly: true,
  }
}