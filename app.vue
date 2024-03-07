<template>
  <NuxtLayout />
</template>

<script setup lang="ts">

</script>

<style>
html, body {
  font-size: 12px;
}

.layout-enter-active,
.layout-leave-active {
  transition: all 0.4s;
}
.layout-enter-from,
.layout-leave-to {
  filter: grayscale(1);
}
</style>
