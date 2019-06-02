const path = require('path');

const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const resolve = (...p) => {
  return path.resolve.apply(null, [__dirname, ...p])
}

const mode = 'production';

module.exports = {
  entry: {
    'demo': [resolve('src/demo.js')],
  },

  output: {
    publicPath: mode === 'production' ? '/' : 'http://localhost:8088/',
    path: path.join(process.cwd(), 'dist'),
    filename: 'scripts/[name].[hash].js',
  },

  mode,

  module: {
    rules: [
      {
        test: /\.wasm$/,
        // This is needed to make webpack NOT process wasm files.
        // See https://github.com/webpack/webpack/issues/6725
        type: 'javascript/auto',
        loader: 'file-loader',
        options: {
          name: '[name].[hash:5].[ext]',
        },
      },
    ],
  },

  plugins: [
    new HtmlWebpackPlugin({
      template: 'src/index.html',
      chunksSortMode: 'dependency',
    }),
    new CopyWebpackPlugin([
      {
        from: '../pkg/fastblur_bg.wasm',
        to: 'dist/static'
      },
      {
        from: 'static',
        to: 'dist/static'
      },
    ])
  ],

  resolve: {
    modules: ['node_modules'],
    extensions: ['.js', '.scss', '.wasm', '.json'],
  },

  devServer: {
    contentBase: './dist',
    clientLogLevel: 'info',
    port: 8088,
    inline: true,
    historyApiFallback: false,
    watchOptions: {
      aggregateTimeout: 300,
      poll: 500,
    },
  },

  optimization: {
    splitChunks: {
      cacheGroups: {
        commons: {
          test: /[\\/]node_modules[\\/]/,
          name: 'vendor',
          chunks: 'all',
        },
      },
    },
  },

  devtool: 'inline-source-map',
};