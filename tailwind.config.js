module.exports = {
    purge: {
        mode: "all",
        content: [
            "./src/**/*.rs",
            "./index.html",
            "./src/**/*.html",
            "./src/**/*.css",
        ],
    },
    theme: {
      extend: {
        colors: {
          bruno: '#333'
        }
      }
    },
    variants: {},
    plugins: [],
};

