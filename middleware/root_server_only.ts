export default defineNuxtRouteMiddleware((to, from) => {
    const isRootServer = useRuntimeConfig()?.public?.rootServer
    if (isRootServer) {
        return
    }

    return abortNavigation({
        statusCode: 404,
        message: 'Page not found',
        data: { path: to.path, rootServerError: true}
    })
})
