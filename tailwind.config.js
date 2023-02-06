/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        '**/*.{html,js,css}',
    ],
    plugins: [
        require('@tailwindcss/typography'),
        require('@tailwindcss/forms'),
    ],
}
