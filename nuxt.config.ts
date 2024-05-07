// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    runtimeConfig: {
        public: {
            rootServer: (/true/i).test(process.env.ROOT_SERVER || ""), // Overrideable at build time only by env ROOT_SERVER
            backendHost: 'http://127.0.0.1:8000', // Overrideable by env NUXT_PUBLIC_BACKEND_HOST
            backendHostSSR: 'http://127.0.0.1:8000', // Overrideable by env NUXT_PUBLIC_BACKEND_HOST_SSR
        }
    },
    app: {
        head: {
            charset: 'utf-8',
            viewport: 'width=device-width, initial-scale=1',
        },
        pageTransition: {name: 'page', mode: 'out-in'},
        layoutTransition: {name: 'layout', mode: 'out-in'}
    },
    components: [
        {
            path: '~/components',
            pathPrefix: false,
        },
    ],
    modules: [
        'nuxt-primevue',
        '@pinia/nuxt',
        '@vueuse/nuxt',
    ],
    primevue: {
        options: {
            ripple: true,
        }
    },
    pinia: {
        storesDirs: ['./stores/**'],
    },
    css: [
        'assets/css/common.styl',
        'primevue/resources/themes/aura-light-green/theme.css',
        'primeicons/primeicons.css'
    ],
    vite: {
        css: {
            preprocessorOptions: {}
        }
    },
    routeRules: {
        // Client-side only
        '/': {ssr: false},
        '/admin/**': {ssr: false},
        // Other pages default to CDN cache.
        '/**': {isr: false, swr: false, ssr: true, prerender: false},
    },
    $production: {

    },
    $development: {
        devtools: {
            enabled: true,
            timeline: {
                enabled: true
            }
        }
    }
})
