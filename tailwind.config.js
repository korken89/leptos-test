/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        extend: {},
    },
    daisyui: {
        themes: ["light", "dark"],
    },
    plugins: [],
}
