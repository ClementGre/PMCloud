// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  runtimeConfig: {
    apiSecret: '123',
  },
  app: {
    head: {
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1',
    },
    pageTransition: { name: 'page', mode: 'out-in' },
    layoutTransition: { name: 'layout', mode: 'out-in' }
  },
  modules: [
    'nuxt-primevue'
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
      '/**': { isr: true }
    }
  },
  $development: {
    devtools: { enabled: true },
    //
  }
})
