const path = require('path');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = {
  mode: 'development',
  entry: './js/index.js',
  plugins: [new MiniCssExtractPlugin()],
  module: {
      rules: [
        {
           test: /\.scss$/,
           use: [MiniCssExtractPlugin.loader, "css-loader", "sass-loader"],
        },
        {
           test: /\.css$/,
           use: [MiniCssExtractPlugin.loader, "css-loader"],
        }
      ]
  },
  output: {
    path: path.resolve(__dirname, '../assets'),
    filename: 'app.js',
  },
};