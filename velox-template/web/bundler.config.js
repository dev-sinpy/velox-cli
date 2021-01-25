// Configuration File For Snowpack
// For reference please visit: https://www.snowpack.dev/reference/configuration

module.exports = {
    mount: {
        /* ... */
    },
    plugins: [],
    packageOptions: {
        source: "local"
    },
    devOptions: {
        // DO NOT CHANGE THESE!
        port: 8888,
        open: "none",
        output: "stream",
    },
    buildOptions: {
        out: "dist"
    },
    optimize: {
        "bundle": true,
        "minify": true,
        "target": 'es2018'
    }
};