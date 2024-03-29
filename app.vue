<template>
  <NuxtLayout />
</template>

<script setup lang="ts">

</script>

<style lang="stylus">
:root
  // For the scroll inertia
  background-color var(--primary-100)
  font-size 12px

.layout-enter-active, .layout-leave-active
  transition all 0.4s

.layout-enter-from, .layout-leave-to
  filter grayscale(1)

</style>
