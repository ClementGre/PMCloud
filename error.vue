<script setup lang="ts">
import type {NuxtError} from "#app";

const props = defineProps({
  error: Object as () => NuxtError
})

let user = useUserStore()
await user.fetchStatus()

const handleError = () => clearError({redirect: '/'})

</script>

<template>
  <NuxtLayout name="noscroll">
    <main>

      <template v-if="props?.error?.statusCode === 404">
        <h1>404 - Page Not Found</h1>
        <template v-if="props?.error?.data.rootServerError">
          <p>This page does not exist on this standalone version of PMCloud, but exists on the PMCloud root server:<br/><a :href="'https://pmcloud.com' + props.error.data.path">{{ 'https://pmcloud.com' + props.error.data.path }}</a></p>
        </template>
        <p v-else>The page you are looking for does not exist.</p>
      </template>
      <template v-else>
        <h1>Error {{props?.error?.statusCode}}</h1>
        <p>Please contact administrators</p>
      </template>

      <Button @click="handleError" label="Go Home" severity="secondary" outlined></Button>
    </main>
  </NuxtLayout>
</template>

<style scoped lang="stylus">
main
  display flex
  flex-direction column
  align-items center
  h1
    color var(--red-600)
  p
    margin-bottom 20px

</style>
