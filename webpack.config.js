const webpack = require('webpack');
const wasmpack = require('@wasm-tool/wasm-pack-plugin');
const htmlpack = require('html-webpack-plugin');
const path = require('path');

module.exports = (env, args) => {
    const production = (args.mode === 'production');
    return {
        entry: '/src/index.js',
        output: {
            path: path.resolve(__dirname, 'dist'),
            filename: production ? '[name].[contenthash].js' : '[name].[hash].js'
        },
        mode: 'development',
        plugins: [
            new htmlpack({
                template: '/src/index.html'
            }),
            new webpack.ProvidePlugin({
                TextDecoder: ['text-encoding', 'TextDecoder'],
                TextEncoder: ['text-encoding', 'TextEncoder']
            })
        ]
    };
}
