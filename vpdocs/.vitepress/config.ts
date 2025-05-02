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
          { text: 'About sticknodes-js', link: '/about' },
          { text: 'Getting Started', link: '/getting-started' }
        ]
      },
      {
        text: 'Stickfigures',
        items: [
          { text: 'Anatomy of a Stickfigure', link: '/stickfigures/anatomy' },
          { text: 'Create', link: '/stickfigures/create' },
          { text: 'Modify', link: '/stickfigures/modify' },
          { text: 'Create from bytes', link: '/stickfigures/create-from-bytes' },
          { text: 'Export to bytes', link: '/stickfigures/export-to-bytes' }
        ]
      },
      {
        text: 'Nodes',
        items: [
          { text: 'Anatomy of a Node', link: '/nodes/anatomy' },
          { text: 'Get', link: '/nodes/get' },
          { text: 'Create', link: '/nodes/create' },
          { text: 'Modify', link: '/nodes/modify' },
          { text: 'Delete', link: '/nodes/delete' },
          { text: 'Bulk management', link: '/nodes/bulk' },
          { text: 'Examples', link: '/nodes/examples' },
        ]
      },
      {
        text: 'Polyfills',
        items: [
          { text: 'Anatomy of a Polyfill', link: '/polyfills/anatomy' },
          { text: 'Get', link: '/polyfills/get' },
          { text: 'Create', link: '/polyfills/create' },
          { text: 'Modify', link: '/polyfills/modify' },
          { text: 'Delete', link: '/polyfills/delete' },
          { text: 'Examples', link: '/polyfills/examples' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/vinceTheProgrammer/sticknodes-js' }
    ]
  }
})
