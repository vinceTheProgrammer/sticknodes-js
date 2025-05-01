import { defineConfig } from 'vitepress'

const base = '/sticknodes-js/'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "sticknodes-js",
  description: "Site for sticknodes-js",
  base,
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'API Docs', link: `https://vincetheprogrammer.github.io${base}docs/index.html` }
    ],

    sidebar: [
      {
        text: 'Introduction',
        items: [
          { text: 'Getting Started', link: '/getting-started' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/vinceTheProgrammer/sticknodes-js' }
    ]
  }
})
