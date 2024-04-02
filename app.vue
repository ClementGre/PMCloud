<template>
  <NuxtLayout>
    <NuxtPage/>
  </NuxtLayout>
</template>

<script setup lang="ts">

</script>

<style lang="stylus">
:root
  // For the scroll inertia
  background-color var(--primary-100)
  font-size 12px

.layout-enter-active, .layout-leave-active
  transition filter 0.1s

.layout-enter-from, .layout-leave-to
  //filter blur(10px) saturate(50%);

</style>
