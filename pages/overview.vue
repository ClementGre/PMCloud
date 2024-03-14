<template>
  <main>
    <p>PictureManager Cloud</p>
    <NuxtLink to="/">Go to the app</NuxtLink>
  </main>
</template>

<script setup lang="ts">

definePageMeta({
  layout: 'default'
})

useSeoMeta({
  title: 'PMCloud Overview',
  ogTitle: 'Overview',
  description: 'An efficient cloud based picture manager that supports automatic grouping of pictures into custom hierarchies, while allowing users to share groups together.',
  ogDescription: 'An efficient cloud based picture manager that supports automatic grouping of pictures into custom hierarchies, while allowing users to share groups together.',
  ogImage: 'https://example.com/image.png'
})

</script>
