import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const data = {
    entry: './src/index.jsx',
    output: {
        path: join(__dirname, 'dist'),
        filename: 'app.bundle.js'
    },
    // mode: 'production',
    mode: 'development',
    module: {
        rules: [
            {
                test: /\.(js|jsx)?$/,
                exclude: /node_modules/,
                resolve: {
                    extensions: [".js", ".jsx"]
                },
                use: ['babel-loader'],
            },
            {
                test: /\.(css)?$/,
                exclude: /node_modules/,
                use: ['style-loader', "css-loader"],
            },
        ],
    },
};

export default data;
