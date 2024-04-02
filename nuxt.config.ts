// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  runtimeConfig: {
    apiSecret: '123',
    public: {
      rootServer: (/true/i).test(process.env.ROOT_SERVER || ""),
    }
  },
  app: {
    head: {
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1',
    },
    pageTransition: { name: 'page', mode: 'out-in' },
    layoutTransition: { name: 'layout', mode: 'out-in' }
  },
  components: [
    {
      path: '~/components',
      pathPrefix: false,
    },
  ],
  modules: [
    'nuxt-primevue',
    '@vueuse/nuxt',
  ],
  primevue: {
    options: {
      ripple: true,
    }
  },
  css: [
      'assets/css/common.styl',
      'primevue/resources/themes/aura-light-green/theme.css',
      'primeicons/primeicons.css'
  ],
  vite: {
    css: {
      preprocessorOptions: {
      }
    }
  },

  $production: {
    routeRules: {
      // Client-side only
      '/': { ssr: false },
      '/admin/**': { ssr: false },
      // Other pages default to CDN cache.
      '/**': { isr: false, swr: false, ssr: false, prerender: false},
    },
  },
  $development: {
    devtools: { enabled: true },
    routeRules: {
      // Client-side only
      '/': { ssr: false },
      '/admin/**': { ssr: false },
      // Other pages default to CDN cache.
      '/**': { isr: false, swr: false, ssr: false, prerender: false},
    },
  }
})
